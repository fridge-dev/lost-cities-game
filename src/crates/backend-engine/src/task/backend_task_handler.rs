use crate::backend_error::BackendGameError;
use game_api::api::GameApi2;
use tokio::sync::mpsc;
use crate::game_engine::backend_game_api::BackendGameApi;
use crate::task::backend_task_event::BackendTaskEvent;

pub struct BackendTaskHandler {
    receiver: mpsc::UnboundedReceiver<BackendTaskEvent>,
    game_api: Box<dyn GameApi2<BackendGameError> + Send>,
}

impl BackendTaskHandler {
    pub fn new(receiver: mpsc::UnboundedReceiver<BackendTaskEvent>) -> Self {
        BackendTaskHandler {
            receiver,
            game_api: Box::new(BackendGameApi::new()),
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
        // TODO make DRY, too WET
        match event {
            BackendTaskEvent::HostGame(payload) => {
                let result = self.game_api.host_game(payload.input).await;
                if let Err(result_failed_to_send) = payload.output_sender.send(result) {
                    println!("ERROR: Sender dropped. Dropping result: {:?}", result_failed_to_send)
                }
            },
            BackendTaskEvent::JoinGame(payload) => {
                let (game_id, player_id) = payload.input;
                let result = self.game_api.join_game(game_id, player_id).await;
                if let Err(result_failed_to_send) = payload.output_sender.send(result) {
                    println!("ERROR: Sender dropped. Dropping result: {:?}", result_failed_to_send)
                }
            },
            BackendTaskEvent::GetGameMetadata(payload) => {
                let result = self.game_api.describe_game(payload.input).await;
                if let Err(result_failed_to_send) = payload.output_sender.send(result) {
                    println!("ERROR: Sender dropped. Dropping result: {:?}", result_failed_to_send)
                }
            },
            BackendTaskEvent::GetGameState(payload) => {
                let (game_id, player_id) = payload.input;
                let result = self.game_api.get_game_state(game_id, player_id).await;
                if let Err(result_failed_to_send) = payload.output_sender.send(result) {
                    println!("ERROR: Sender dropped. Dropping result: {:?}", result_failed_to_send)
                }
            },
            BackendTaskEvent::PlayCard(payload) => {
                let result = self.game_api.play_card(payload.input).await;
                if let Err(result_failed_to_send) = payload.output_sender.send(result) {
                    println!("ERROR: Sender dropped. Dropping result: {:?}", result_failed_to_send)
                }
            },
        }
    }
}