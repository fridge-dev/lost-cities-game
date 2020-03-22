use crate::backend_error::BackendGameError;
use game_api::types::{GameMetadata, GameState, Play};
use std::fmt::{Debug, Formatter};
use std::fmt;
use tokio::sync::oneshot;

/// This is a 1:1 representation of the GameApi2.
#[derive(Debug)]
pub enum BackendTaskEvent {
    HostGame(EventPayload<String, String>),
    JoinGame(EventPayload<(String, String), ()>),
    GetGameMetadata(EventPayload<String, GameMetadata>),
    GetGameState(EventPayload<(String, String), GameState>),
    PlayCard(EventPayload<Play, ()>),
}

pub struct EventPayload<I, O> {
    pub input: I,
    pub output_sender: oneshot::Sender<Result<O, BackendGameError>>,
}

impl<I, O> EventPayload<I, O> {
    pub fn wrap_with_channel(input: I) -> (Self, oneshot::Receiver<Result<O, BackendGameError>>) {
        let (output_sender, output_receiver) = oneshot::channel::<Result<O, BackendGameError>>();

        let payload = EventPayload {
            input,
            output_sender
        };

        (payload, output_receiver)
    }
}

impl<I, O> Debug for EventPayload<I, O> where I: Debug {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "EventPayload {{ input={:?} }}", &self.input)
    }
}