use crate::v2::games::love_letter::{LoveLetterInstanceManager, LoveLetterEvent};
use crate::v2::games::love_letter::impl2::LoveLetterStateMachine;
use crate::v2::games::love_letter::types::{GameInstanceState, GameData, StagedPlay, Card, RoundData};
use futures_util::sink::SinkExt;

impl LoveLetterInstanceManager {
    pub fn handle_event2b(&mut self, event: LoveLetterEvent) {
        let from_state = self.state2.take();
        let to_state = self.state_machine.transition_state2b(from_state, event);
        self.state2.put(to_state);
    }
}

impl LoveLetterStateMachine {
    /// State machine logic:
    ///
    /// Move from FROM_STATE to TO_STATE and mutate internal data as needed.
    ///
    /// This will be a PITA to add Result<> to. Unless Err means game is in corrupt state
    /// and we drop the game instance.
    pub fn transition_state2b(
        &self,
        from_state: GameInstanceState,
        event: LoveLetterEvent,
    ) -> GameInstanceState {
        match from_state {
            GameInstanceState::WaitingForStart => self.transition_from_waiting(event),
            GameInstanceState::InProgress(game_data) => self.transition_from_in_progress(event, game_data),
            GameInstanceState::InProgressStaged(game_data, staged_play) => self.transition_from_in_progress_staged(event, game_data, staged_play),
        }
    }

    fn transition_from_waiting(&self, event: LoveLetterEvent) -> GameInstanceState {
        match event {
            // TODO Phase::Setup refactoring
            LoveLetterEvent::Join(_player_id, _out) => GameInstanceState::WaitingForStart,
            // TODO extract common game-engine functionality
            LoveLetterEvent::GetGameState(_player_id) => GameInstanceState::WaitingForStart,
            LoveLetterEvent::StartGame => {
                let new_game = GameData::new(self.players.player_ids());
                GameInstanceState::InProgress(new_game)
            },
            // TODO Phase::Game refactoring
            _ => {
                // TODO inform caller
                println!("Invalid transition.");
                GameInstanceState::WaitingForStart
            }
        }
    }

    fn transition_from_in_progress(&self, event: LoveLetterEvent, game_data: GameData) -> GameInstanceState {
        match event {
            // TODO extract common game-engine functionality
            LoveLetterEvent::GetGameState(_) => GameInstanceState::InProgress(game_data),
            LoveLetterEvent::PlayCardStaged(player_id, card_source) => {
                if game_data.current_player_turn() != &player_id {
                    self.players.send_err(&player_id, "Not your turn");

                    // No state change
                    return GameInstanceState::InProgress(game_data);
                }

                let card_to_stage = game_data.current_round.get_card_to_play(&player_id, &card_source);

                // TODO if selection not-needed, auto-commit

                GameInstanceState::InProgressStaged(game_data, StagedPlay::new(card_to_stage))
            },
            _ => {
                // TODO inform caller
                println!("Invalid transition.");
                GameInstanceState::InProgress(game_data)
            }
        }
    }

    fn transition_from_in_progress_staged(&self, event: LoveLetterEvent, game_data: GameData, staged_play: StagedPlay) -> GameInstanceState {
        match event {
            // TODO extract common game-engine functionality
            LoveLetterEvent::GetGameState(_) => GameInstanceState::InProgressStaged(game_data, staged_play),
            LoveLetterEvent::SelectTargetPlayer(client_player_id, target_player_id) => {
                let mut staged_play = staged_play;
                staged_play.set_target_player(target_player_id);
                GameInstanceState::InProgressStaged(game_data, staged_play)
            },
            LoveLetterEvent::SelectTargetCard(client_player_id, target_card) => {
                let mut staged_play = staged_play;
                staged_play.set_target_card(target_card);
                GameInstanceState::InProgressStaged(game_data, staged_play)
            },
            LoveLetterEvent::PlayCardCommit(player_id) => {
                let mut game_data = game_data;

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
            // TODO Phase::Game refactoring
            _ => {
                // TODO inform caller
                println!("Invalid transition.");
                GameInstanceState::InProgressStaged(game_data, staged_play)
            }
        }
    }
}