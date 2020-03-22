use tokio::sync::mpsc;
use crate::task::backend_task_event::{BackendTaskEvent, EventPayload};
use crate::task::backend_task_handler::BackendTaskHandler;
use game_api::api::GameApi2;
use crate::backend_error::{BackendGameError, Cause};
use game_api::types::{GameMetadata, GameState, Play};
use tokio::sync::oneshot;

pub fn spawn_backend_task() -> BackendTaskClient {
    let (sender, receiver) = mpsc::unbounded_channel::<BackendTaskEvent>();

    let task = BackendTaskHandler::new(receiver);
    tokio::spawn(task.start_event_loop());

    BackendTaskClient::new(sender)
}

pub struct BackendTaskClient {
    sender: mpsc::UnboundedSender<BackendTaskEvent>,
}


impl BackendTaskClient {
    fn new(sender: mpsc::UnboundedSender<BackendTaskEvent>) -> Self {
        BackendTaskClient {
            sender,
        }
    }

    async fn send_and_await<O>(
        &self,
        event: BackendTaskEvent,
        receiver: oneshot::Receiver<Result<O, BackendGameError>>
    ) -> Result<O, BackendGameError> {
        self.sender
            .send(event)
            .map_err(|_| BackendGameError::Internal(Cause::Internal("BackendTask event loop has stopped. This is very bad.")))?;

        receiver
            .await
            .map_err(|_| BackendGameError::Internal(Cause::Internal("BackendTask event loop dropped the oneshot response sender.")))?
    }

    // ==== Immutable methods which are 1:1 with GameApi2 ====

    pub async fn host_game(&self, p1_id: String) -> Result<String, BackendGameError> {
        let (payload, receiver) = EventPayload::wrap_with_channel(p1_id);
        self.send_and_await(BackendTaskEvent::HostGame(payload), receiver).await
    }

    pub async fn join_game(&self, game_id: String, p2_id: String) -> Result<(), BackendGameError> {
        let (payload, receiver) = EventPayload::wrap_with_channel((game_id, p2_id));
        self.send_and_await(BackendTaskEvent::JoinGame(payload), receiver).await
    }

    pub async fn describe_game(&self, game_id: String) -> Result<GameMetadata, BackendGameError> {
        let (payload, receiver) = EventPayload::wrap_with_channel(game_id);
        self.send_and_await(BackendTaskEvent::GetGameMetadata(payload), receiver).await
    }

    pub async fn get_game_state(&self, game_id: String, player_id: String) -> Result<GameState, BackendGameError> {
        let (payload, receiver) = EventPayload::wrap_with_channel((game_id, player_id));
        self.send_and_await(BackendTaskEvent::GetGameState(payload), receiver).await
    }

    pub async fn play_card(&self, play: Play) -> Result<(), BackendGameError> {
        let (payload, receiver) = EventPayload::wrap_with_channel(play);
        self.send_and_await(BackendTaskEvent::PlayCard(payload), receiver).await
    }
}
