use crate::backend_error::{BackendGameError, Cause};
use crate::game_api::{GameApi2Immut, GameApiResult};
use crate::task::backend_task_event::{BackendTaskEvent, EventPayload};
use crate::task::backend_task_handler::BackendTaskHandler;
use game_api::types::{GameMetadata, GameState, Play};
use std::sync::Arc;
use storage::v2::db_api::GameDatabase;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

pub fn spawn_backend_task(
    db_client: Arc<dyn GameDatabase + Send + Sync>
) -> BackendTaskClientAdapter {
    let (sender, receiver) = mpsc::unbounded_channel::<BackendTaskEvent>();

    let task = BackendTaskHandler::new(receiver, db_client);
    tokio::spawn(task.start_event_loop());

    BackendTaskClientAdapter::new(sender)
}

/// Adapts the task model to the `GameApi2Immut` API model. This is the beauty of interior
/// mutability offered by the shared-nothing task model. The MPSC sender doesn't need to be
/// `mut` but it passes a message to a single-threaded MPSC receiver who has `mut` ownership
/// of its data, and responds back via a Oneshot channel.
///
/// See https://docs.rs/tokio/0.2.13/tokio/sync/index.html#message-passing for a good overview.
pub struct BackendTaskClientAdapter {
    sender: mpsc::UnboundedSender<BackendTaskEvent>,
}

impl BackendTaskClientAdapter {
    fn new(sender: mpsc::UnboundedSender<BackendTaskEvent>) -> Self {
        BackendTaskClientAdapter {
            sender,
        }
    }

    async fn send_and_await<O>(
        &self,
        event: BackendTaskEvent,
        receiver: oneshot::Receiver<GameApiResult<O>>
    ) -> GameApiResult<O> {
        self.sender
            .send(event)
            .map_err(|_| BackendGameError::Internal(Cause::Internal("BackendTask event loop has stopped. This is very bad.")))?;

        receiver
            .await
            .map_err(|_| BackendGameError::Internal(Cause::Internal("BackendTask event loop dropped the oneshot response sender.")))?
    }
}

#[async_trait::async_trait]
impl GameApi2Immut for BackendTaskClientAdapter {
    async fn host_game(&self, game_id: String, p1_id: String) -> GameApiResult<()> {
        let (payload, receiver) = EventPayload::wrap_with_channel((game_id, p1_id));
        self.send_and_await(BackendTaskEvent::HostGame(payload), receiver).await
    }

    async fn join_game(&self, game_id: String, p2_id: String) -> GameApiResult<()> {
        let (payload, receiver) = EventPayload::wrap_with_channel((game_id, p2_id));
        self.send_and_await(BackendTaskEvent::JoinGame(payload), receiver).await
    }

    async fn describe_game(&self, game_id: String) -> GameApiResult<GameMetadata> {
        let (payload, receiver) = EventPayload::wrap_with_channel(game_id);
        self.send_and_await(BackendTaskEvent::GetGameMetadata(payload), receiver).await
    }

    async fn get_game_state(&self, game_id: String, player_id: String) -> GameApiResult<GameState> {
        let (payload, receiver) = EventPayload::wrap_with_channel((game_id, player_id));
        self.send_and_await(BackendTaskEvent::GetGameState(payload), receiver).await
    }

    async fn play_card(&self, play: Play) -> GameApiResult<()> {
        let (payload, receiver) = EventPayload::wrap_with_channel(play);
        self.send_and_await(BackendTaskEvent::PlayCard(payload), receiver).await
    }

    async fn query_unmatched_games(&self, player_id: String) -> GameApiResult<Vec<GameMetadata>> {
        let (payload, receiver) = EventPayload::wrap_with_channel(player_id);
        self.send_and_await(BackendTaskEvent::QueryUnmatchedGames(payload), receiver).await
    }

    async fn query_in_progress_games(&self, player_id: String) -> GameApiResult<Vec<GameMetadata>> {
        let (payload, receiver) = EventPayload::wrap_with_channel(player_id);
        self.send_and_await(BackendTaskEvent::QueryInProgressGames(payload), receiver).await
    }

    async fn query_completed_games(&self, player_id: String) -> GameApiResult<Vec<GameMetadata>> {
        let (payload, receiver) = EventPayload::wrap_with_channel(player_id);
        self.send_and_await(BackendTaskEvent::QueryCompletedGames(payload), receiver).await
    }

    async fn query_all_unmatched_games(&self, player_id: String) -> GameApiResult<Vec<GameMetadata>> {
        let (payload, receiver) = EventPayload::wrap_with_channel(player_id);
        self.send_and_await(BackendTaskEvent::QueryAllUnmatchedGames(payload), receiver).await
    }
}
