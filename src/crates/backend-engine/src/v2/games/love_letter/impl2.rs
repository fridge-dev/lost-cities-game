use crate::v2::games::love_letter::{LoveLetterInstanceManager, LoveLetterEvent, PlayCardSource};
use crate::v2::games::love_letter::types::{GameInstanceState, RoundData, Card, StagedPlay, GameData};
use crate::v2::framework::{ClientOut, Players};
use std::collections::HashMap;

const MAX_PLAYERS: usize = 4;
const MIN_PLAYERS: usize = 2;

impl LoveLetterInstanceManager {
    pub fn handle_event2(&mut self, event: LoveLetterEvent) {
        let from_state = self.state2.take();
        let to_state = self.state_machine.transition_state2(from_state, event);
        self.state2.put(to_state);
    }
}

pub struct LoveLetterStateMachine {
    // TODO will be private when I re-organized modules
    pub players: Players
}

impl LoveLetterStateMachine {
    pub fn new(players: Players) -> Self {
        LoveLetterStateMachine {
            players
        }
    }

    /// State machine logic:
    ///
    /// Move from FROM_STATE to TO_STATE and mutate internal data as needed.
    ///
    /// This will be a PITA to add Result<> to. Unless Err means game is in corrupt state
    /// and we drop the game instance.
    pub fn transition_state2(
        &self,
        from_state: GameInstanceState,
        event: LoveLetterEvent,
    ) -> GameInstanceState {
        match event {
            // I will refactor these to be in Phase::Setup and game-specific below will be Phase::Game
            LoveLetterEvent::Join(_, _) => from_state,
            LoveLetterEvent::StartGame => from_state,
            LoveLetterEvent::GetGameState(_) => from_state,

            // Phase::Game
            LoveLetterEvent::PlayCardStaged(player_id, card_source) => self.play_card_staged(from_state, player_id, card_source),
            LoveLetterEvent::SelectTargetPlayer(client_player_id, target_player_id) => self.select_target_player(from_state, client_player_id, target_player_id),
            LoveLetterEvent::SelectTargetCard(client_player_id, target_card) => self.select_target_card(from_state, client_player_id, target_card),
            LoveLetterEvent::PlayCardCommit(player_id) => self.play_card_commit(from_state, player_id),
        }
    }

    fn play_card_staged(&self, from_state: GameInstanceState, player_id: String, card_source: PlayCardSource) -> GameInstanceState {
        if !self.players.contains(&player_id) {
            // TODO notify caller of err?
            return from_state;
        }

        match from_state {
            GameInstanceState::WaitingForStart => {
                // TODO idempotency?
                self.players.send_err(&player_id, "Can't play before game has started");
                self.state2.put(state);

                // No state change
                GameInstanceState::WaitingForStart
            },
            GameInstanceState::InProgressStaged(game_data, staged_play) => {
                // Is my turn
                if &player_id != game_data.current_player_turn() {
                    self.players.send_err(&player_id, "Can't play card, not your turn");
                    return GameInstanceState::InProgressStaged(game_data, staged_play)
                }

                // Idempotent check
                let card_to_stage = game_data.current_round.get_card_to_play(&player_id, &card_source);
                if card_to_stage == &staged_play.card {
                    // TODO send ACK to only requesting player
                    // Or send player some type of message telling
                    // them to re-get state
                } else {
                    self.players.send_err(&player_id, "Can't play card while pending commit");
                }

                // No state change
                GameInstanceState::InProgressStaged(game_data, staged_play)
            },
            GameInstanceState::InProgress(game_data) => {
                if game_data.current_player_turn() != &player_id {
                    self.players.send_err(&player_id, "Not your turn");

                    // No state change
                    return GameInstanceState::InProgress(game_data);
                }

                let card_to_stage = game_data.current_round.get_card_to_play(&player_id, &card_source);

                // TODO if selection not-needed, auto-commit

                GameInstanceState::InProgressStaged(game_data, StagedPlay::new(card_to_stage))
            },
        }
    }

    fn select_target_player(&self, from_state: GameInstanceState, client_player_id: String, target_player_id: String) -> GameInstanceState {
        // TODO being lazy, fill out full match statement... Only happy path for now
        match from_state {
            GameInstanceState::WaitingForStart => from_state,
            GameInstanceState::InProgress(_) => from_state,
            GameInstanceState::InProgressStaged(game_data, mut staged_play) => {
                staged_play.set_target_player(target_player_id);
                GameInstanceState::InProgressStaged(game_data, staged_play)
            },
        }
    }

    fn select_target_card(&self, from_state: GameInstanceState, client_player_id: String, target_card: Card) -> GameInstanceState {
        // TODO being lazy, fill out full match statement... Only happy path for now
        match from_state {
            GameInstanceState::WaitingForStart => from_state,
            GameInstanceState::InProgress(_) => from_state,
            GameInstanceState::InProgressStaged(game_data, mut staged_play) => {
                staged_play.set_target_card(target_player_id);
                GameInstanceState::InProgressStaged(game_data, staged_play)
            },
        }
    }

    fn play_card_commit(&self, from_state: GameInstanceState, player_id: String) -> GameInstanceState {
        match from_state {
            GameInstanceState::WaitingForStart => {
                self.players.send_err(&player_id, "Can't play card before game start");
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
                        // TODO more robust way of expecting staging (micro states?)
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
                        self.players.send_msg(&next_player, format!("New card: {}", next_card));
                    },
                }

                GameInstanceState::InProgress(game_data)
            },
        }
    }
}