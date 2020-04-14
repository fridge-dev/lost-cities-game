use crate::v2::games::love_letter::{LoveLetterInstanceManager, LoveLetterEvent, PlayCardSource};
use crate::v2::games::love_letter::types::{GameData, Card, StagedPlay, GameInstanceState, RoundData};
use crate::v2::framework::ClientOut;
use futures_util::sink::SinkExt;
use futures_util::io::AsyncReadExt;

const MAX_PLAYERS: usize = 4;
const MIN_PLAYERS: usize = 2;

impl LoveLetterInstanceManager {
    pub async fn handle_event(&mut self, event: LoveLetterEvent) -> Result<(), ()> {
        match event {
            LoveLetterEvent::Join(player_id, client_out) => {
                self.join(player_id, client_out);
            },
            LoveLetterEvent::StartGame => {
                self.start_game();
            },
            LoveLetterEvent::GetGameState(player_id) => {
                self.get_game_state(player_id);
            },
            LoveLetterEvent::PlayCardStaged(player_id, card_source) => {
                self.play_card_staged(player_id, card_source);
            },
            LoveLetterEvent::SelectTargetPlayer(client_player_id, target_player_id) => {
                self.select_target_player(client_player_id, target_player_id);
            },
            LoveLetterEvent::SelectTargetCard(client_player_id, target_card) => {
                self.select_target_card(client_player_id, target_card);
            },
            LoveLetterEvent::PlayCardCommit(player_id) => {
                self.play_card_commit(player_id);
            },
        }

        Ok(())
    }

    fn join(&mut self, player_id: String, client_out: ClientOut) {
        // Reconnect
        if self.players.contains_key(&player_id) {
            self.players.insert(player_id, client_out);
            return;
        }

        // Game in progress
        if &GameInstanceState::WaitingForStart != self.state2.peek() {
            client_out.send_err("Can't join, game has started");
            return;
        }

        // Player count
        if self.players.len() >= MAX_PLAYERS {
            client_out.send_err("Can't join, game has max players");
            return;
        }

        self.players.insert(player_id, client_out);
    }

    fn start_game(&mut self) {
        let state = self.state2.take();

        // Game already started
        if GameInstanceState::WaitingForStart != state {
            // TODO notify caller of err?
            // TODO idempotency?
            self.state2.put(state);
            return;
        }

        // Not enough players
        if self.players.len() < MIN_PLAYERS {
            // TODO notify caller of err?
            return;
        }

        let player_ids = self.players.keys()
            .clone()
            .collect();
        let game_data = GameData::new(player_ids);
        self.state2.put(GameInstanceState::InProgress(game_data));
    }

    fn get_game_state(&self, player_id: String) {
        if let Some(client_out) = self.players.get(&player_id) {
            client_out.send(&self.state);
        } else {
            println!("INFO: Non-game-member '{}' requested game state.");
        }
    }

    fn play_card_staged(&mut self, player_id: String, card_source: PlayCardSource) {
        if !self.players.contains_key(&player_id) {
            // TODO notify caller of err?
            return;
        }

        let state = self.state2.take();
        match state {
            GameInstanceState::WaitingForStart => {
                // TODO idempotency?
                self.players
                    .get(&player_id)
                    .expect("player map")
                    .send_err("Can't play before game has started");
                self.state2.put(state);
                return;
            },
            GameInstanceState::InProgressStaged(game_data, staged_play) => {
                // Is my turn
                if &player_id != game_data.current_player_turn() {
                    self.players
                        .get(&player_id)
                        .expect("player map")
                        .send_err("Can't play card, not your turn");
                    return;
                }

                // Idempotent check
                let card_to_stage = game_data.current_round.get_card_to_play(&player_id, &card_source);
                if card_to_stage == &staged_play.card {
                    // TODO send ACK to only requesting player
                    // Or send player some type of message telling
                    // them to re-get state
                } else {
                    self.players
                        .get(&player_id)
                        .expect("player map")
                        .send_err("Can't play card while pending commit");
                }

                self.state2.put(GameInstanceState::InProgressStaged(game_data, staged_play));
                return;
            },
            GameInstanceState::InProgress(game_data) => {
                if game_data.current_player_turn() != &player_id {
                    self.players
                        .get(&player_id)
                        .expect("player map")
                        .send_err("Not your turn");
                    return;
                }

                let card_to_stage = game_data.current_round.get_card_to_play(&player_id, &card_source);
                self.state2.put(GameInstanceState::InProgressStaged(game_data, StagedPlay::new(card_to_stage)));

                // TODO if selection not-needed, auto-commit
            },
        }
    }

    fn select_target_player(&mut self, client_player_id: String, target_player_id: String) {
        // TODO being lazy, fill out full match statement... Only happy path for now
        let mut state = self.state2.take();
        if let GameInstanceState::InProgressStaged(game_data, staged_play) = &mut state {
            staged_play.set_target_player(target_player_id);
        }

        self.state2.put(state);
    }

    fn select_target_card(&mut self, client_player_id: String, target_card: Card) {
        // TODO being lazy, fill out full match statement... Only happy path for now
        let mut state = self.state2.take();
        if let GameInstanceState::InProgressStaged(game_data, staged_play) = &mut state {
            staged_play.set_target_card(target_card);
        }

        self.state2.put(state);

    }

    fn play_card_commit(&mut self, player_id: String) {
        // TODO being lazy, fill out full match statement... Only happy path for now
        let state = self.state2.take();
        let new_state = match state {
            GameInstanceState::WaitingForStart => {
                self.players.get(&player_id)
                    .expect("player map")
                    .send_err("Can't play card before game start");
                GameInstanceState::WaitingForStart
            },
            GameInstanceState::InProgress(game_data) => {
                // TODO if selection not-needed, auto-commit
                GameInstanceState::InProgress(game_data)
            },
            GameInstanceState::InProgressStaged(mut game_data, staged_play) => {
                // Perform action
                match staged_play.card {
                    Card::Guard => {
                        let target_player = staged_play.target_player.expect("Rule");
                        let guessed_card = staged_play.target_card.expect("Rule");
                        let actual_card = game_data.current_round.player_cards.get(&target_player);
                        if guessed_card == actual_card {
                            // Player is out!
                            game_data.current_round.player_cards.remove(&target_player);
                            // TODO send result to all players
                        } else {
                            // TODO send result to all players
                        }
                    },
                    Card::Priest => {},
                    Card::Baron => {},
                    Card::Handmaid => {},
                    Card::Prince => {},
                    Card::King => {},
                    Card::Countess => {},
                    Card::Princess => {},
                }

                // Update current-player hand
                // TODO do this during staging

                // Send next card to next player
                let next_card_opt = game_data.current_round.remaining_cards.last();
                match next_card_opt {
                    None => {
                        // Round over
                        let (winner, mut high_card) = game_data.current_round
                            .player_cards
                            .remove_entry(&player_id)
                            .expect("impossible");
                        let mut winners = vec![winner];
                        for (player_id, card) in game_data.current_round.player_cards {
                            if card > high_card {
                                winners.clear();
                                winners.push(player_id);
                                high_card = card;
                            } else if card == high_card {
                                winners.push(player_id);
                            }
                        }

                        for player_id in winners {
                            *game_data.wins_per_player.entry(player_id).or_insert(0) += 1;
                        }

                        // TODO notify players

                        // Re-deal
                        game_data.current_round = RoundData::new(&game_data.player_id_turn_order)
                    },
                    Some(next_card) => {
                        game_data.current_round.turn_cursor = (game_data.current_round.turn_cursor + 1) % game_data.player_id_turn_order.len();
                        let next_player = game_data.current_player_turn();
                        self.players
                            .get(&next_player)
                            .expect("player map")
                            .send(format!("New card: {}", next_card));
                    },
                }

                GameInstanceState::InProgress(game_data)
            },
        };

        self.state2.put(new_state);
    }
}
