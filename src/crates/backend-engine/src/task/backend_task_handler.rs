use crate::backend_error::BackendGameError;
use game_api::api::GameApi2;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use crate::game_engine::backend_game_api::BackendGameApi;
use crate::task::backend_task_event::BackendTaskEvent;
use std::fmt::Debug;
use storage::v2::db_api::GameDatabase;
use std::sync::Arc;

pub struct BackendTaskHandler {
    receiver: mpsc::UnboundedReceiver<BackendTaskEvent>,
    game_api: Box<dyn GameApi2<BackendGameError> + Send>,
}

impl BackendTaskHandler {
    pub fn new(
        receiver: mpsc::UnboundedReceiver<BackendTaskEvent>,
        db_client: Arc<dyn GameDatabase + Send + Sync>
    ) -> Self {
        BackendTaskHandler {
            receiver,
            game_api: Box::new(BackendGameApi::new(db_client)),
        }
    }

    pub async fn start_event_loop(mut self) {
        while let Some(event) = self.receiver.recv().await {
            println!("Received {:?}", event);
            self.handle_event(event).await;
        }

        println!("Exiting event loop.");
    }

    async fn handle_event(&mut self, event: BackendTaskEvent) {
        match event {
            BackendTaskEvent::HostGame(payload) => {
                let (game_id, player_id) = payload.input;
                pipe_result_to_sender(
                    self.game_api.host_game(game_id, player_id).await,
                    payload.output_sender
                );
            },
            BackendTaskEvent::JoinGame(payload) => {
                let (game_id, player_id) = payload.input;
                pipe_result_to_sender(
                    self.game_api.join_game(game_id, player_id).await,
                    payload.output_sender
                );
            },
            BackendTaskEvent::GetGameMetadata(payload) => {
                let game_id = payload.input;
                pipe_result_to_sender(
                    self.game_api.describe_game(game_id).await,
                    payload.output_sender
                );
            },
            BackendTaskEvent::GetGameState(payload) => {
                let (game_id, player_id) = payload.input;
                pipe_result_to_sender(
                    self.game_api.get_game_state(game_id, player_id).await,
                    payload.output_sender
                );
            },
            BackendTaskEvent::PlayCard(payload) => {
                let play = payload.input;
                pipe_result_to_sender(
                    self.game_api.play_card(play).await,
                    payload.output_sender
                );
            },
            BackendTaskEvent::QueryUnmatchedGames(payload) => {
                let player_id = payload.input;
                pipe_result_to_sender(
                    self.game_api.query_unmatched_games(player_id).await,
                    payload.output_sender
                )
            }
            BackendTaskEvent::QueryInProgressGames(payload) => {
                let player_id = payload.input;
                pipe_result_to_sender(
                    self.game_api.query_in_progress_games(player_id).await,
                    payload.output_sender
                )
            }
            BackendTaskEvent::QueryCompletedGames(payload) => {
                let player_id = payload.input;
                pipe_result_to_sender(
                    self.game_api.query_completed_games(player_id).await,
                    payload.output_sender
                )
            }
            BackendTaskEvent::QueryAllUnmatchedGames(payload) => {
                let player_id = payload.input;
                pipe_result_to_sender(
                    self.game_api.query_all_unmatched_games(player_id).await,
                    payload.output_sender
                )
            }
        }
    }
}

fn pipe_result_to_sender<O: Debug>(
    result: Result<O, BackendGameError>,
    sender: oneshot::Sender<Result<O, BackendGameError>>,
) {
    if let Err(result_failed_to_send) = sender.send(result) {
        println!("ERROR: Sender dropped. Dropping result: {:?}", result_failed_to_send)
    }
}
