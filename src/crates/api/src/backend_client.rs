// TODO remove
#![allow(unused_variables)]
use crate::GameApi;
use game_api::types::{GameState, Play};
use wire_api::proto_lost_cities::proto_lost_cities_client::ProtoLostCitiesClient;
use tonic::transport::{Channel, Endpoint, Error};
use wire_api::proto_lost_cities::{ProtoHostGameReq, ProtoHostGameReply};
use tonic::Code;
use futures::executor::block_on;
use game_api::backend_errors::{BackendGameError, Cause, Reason};

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

    pub async fn host_game_async(&mut self, p1_id: String) -> Result<String, BackendGameError> {
        let request = tonic::Request::new(ProtoHostGameReq {
            player_id: p1_id
        });

        self.inner_client.host_game(request).await
            .map(|response| convert_response(response))
            .map_err(|e| handle_error(e))
    }

    pub async fn join_game_async(&mut self, game_id: String, p2_id: String) -> Result<(), BackendGameError> {
        unimplemented!()
    }

    pub async fn get_game_state_async(&self, game_id: String, player_id: String) -> Result<GameState, BackendGameError> {
        unimplemented!()
    }

    pub async fn play_card_async(&mut self, play: Play) -> Result<(), BackendGameError> {
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
    fn host_game(&mut self, p1_id: String) -> Result<String, BackendGameError> {
        block_on(self.host_game_async(p1_id))
    }

    fn join_game(&mut self, game_id: String, p2_id: String) -> Result<(), BackendGameError> {
        block_on(self.join_game_async(game_id, p2_id))
    }

    fn get_game_state(&self, game_id: String, player_id: String) -> Result<GameState, BackendGameError> {
        block_on(self.get_game_state_async(game_id, player_id))
    }

    fn play_card(&mut self, play: Play) -> Result<(), BackendGameError> {
        block_on(self.play_card_async(play))
    }
}

fn convert_response(response: tonic::Response<ProtoHostGameReply>) -> String {
    response.into_inner().game_id
}

fn handle_error(status: tonic::Status) -> BackendGameError {
    println!("WARN: Failed backend call: {:?}", status);
    match status.code() {
        // TODO use more accurate message
        Code::InvalidArgument => BackendGameError::InvalidPlay(Reason::NeutralDrawPileEmpty),
        Code::NotFound => BackendGameError::NotFound("backend call, no context, idk"),
        Code::AlreadyExists => BackendGameError::GameAlreadyMatched,
        _ => BackendGameError::Internal(Cause::Internal("TODO Map exceptions"))
    }
}
