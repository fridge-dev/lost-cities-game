// TODO remove
#![allow(unused_variables)]
use crate::GameApi;
use types::{GameError, GameState, Play, Reason};
use wire_types::proto_lost_cities::proto_lost_cities_client::ProtoLostCitiesClient;
use tonic::transport::{Channel, Endpoint, Error};
use wire_types::proto_lost_cities::{ProtoHostGameReq, ProtoHostGameReply};
use tonic::Code;
use futures::executor::block_on;

pub struct BackendClient {
    inner_client: ProtoLostCitiesClient<Channel>,
}

/// Async methods - In the long term, I'd like that only these exist.
/// For that reason, this is where the actual logic will be.
///
/// Blocked on refactor to separate interfaces for client and server.
impl BackendClient {
    pub async fn new_async() -> Result<Self, Error> {
        let endpoint = Endpoint::new("localhost:50051")?;

        let connection = endpoint.connect().await?;

        Ok(BackendClient {
            inner_client: ProtoLostCitiesClient::new(connection)
        })
    }

    pub async fn host_game_async(&mut self, p1_id: String) -> Result<String, GameError> {
        let request = tonic::Request::new(ProtoHostGameReq {
            player_id: p1_id
        });

        self.inner_client.host_game(request).await
            .map(|response| convert_response(response))
            .map_err(|e| handle_error(e))
    }

    pub async fn join_game_async(&mut self, game_id: String, p2_id: String) -> Result<(), GameError> {
        unimplemented!()
    }

    pub async fn get_game_state_async(&self, game_id: String, player_id: String) -> Result<GameState, GameError> {
        unimplemented!()
    }

    pub async fn play_card_async(&mut self, play: Play) -> Result<(), GameError> {
        unimplemented!()
    }
}

/// Sync methods - these exist to more easily be used by the blocking main.rs loop.
/// Instead of blocking, I should use non-blocking via tokio main. That will require
/// a larger refactor of separating client and server interfaces.
impl BackendClient {
    pub fn new() -> Result<Self, Error> {
        block_on(BackendClient::new_async())
    }
}

/// Sync methods - these exist to more easily be used by the blocking main.rs loop.
/// Instead of blocking, I should use non-blocking via tokio main. That will require
/// a larger refactor of separating client and server interfaces.
impl GameApi for BackendClient {
    fn host_game(&mut self, p1_id: String) -> Result<String, GameError> {
        block_on(self.host_game_async(p1_id))
    }

    fn join_game(&mut self, game_id: String, p2_id: String) -> Result<(), GameError> {
        block_on(self.join_game_async(game_id, p2_id))
    }

    fn get_game_state(&self, game_id: String, player_id: String) -> Result<GameState, GameError> {
        block_on(self.get_game_state_async(game_id, player_id))
    }

    fn play_card(&mut self, play: Play) -> Result<(), GameError> {
        block_on(self.play_card_async(play))
    }
}

fn convert_response(response: tonic::Response<ProtoHostGameReply>) -> String {
    response.into_inner().game_id
}

fn handle_error(status: tonic::Status) -> GameError {
    println!("WARN: Failed backend call: {:?}", status);
    match status.code() {
        // TODO use more accurate message
        Code::InvalidArgument => GameError::InvalidPlay(Reason::NeutralDrawPileEmpty),
        Code::NotFound => GameError::NotFound("backend call, no context, idk"),
        Code::AlreadyExists => GameError::GameAlreadyMatched,
        Code::DeadlineExceeded => GameError::BackendTimeout,
        Code::Internal => GameError::BackendFault,
        _ => GameError::BackendUnknown
    }
}