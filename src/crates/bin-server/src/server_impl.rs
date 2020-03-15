use tonic::{Request, Response, Status};
use wire_types::proto_lost_cities::proto_lost_cities_server::{
    ProtoLostCities
};
use wire_types::proto_lost_cities::{ProtoHostGameReq, ProtoHostGameReply, ProtoJoinGameReq, ProtoJoinGameReply, ProtoGetGameStateReq, ProtoGetGameStateReply, ProtoPlayCardReq, ProtoPlayCardReply};
use api::GameApi;
use crate::type_converters::WireTypeConverter;
use std::sync::{Mutex, PoisonError};
use types::{GameError, Cause};
use tonic::codegen::Arc;

/// Backend server is the entry point which will implement the gRPC server type.
pub struct LostCitiesBackendServer {
    // Mutex because I want a working, multi-tasked prototype for now.
    // I'll change the GameApi to be backed by a mpsc task model with oneshot callbacks.
    game_api: Arc<Mutex<dyn GameApi + Send>>,
}

impl LostCitiesBackendServer {
    pub fn new() -> Self {
        LostCitiesBackendServer {
            game_api: api::new_backend_game_api()
        }
    }
}

fn convert_lock_error<T>(e: PoisonError<T>) -> GameError {
    // I don't know how to recover from this. Recreate game handler?
    println!("ERROR: GameApi lock was poisoned. Here's the err: {}", e);
    GameError::Internal(Cause::Internal("Failed to acquire GameApi lock"))
}

#[tonic::async_trait]
impl ProtoLostCities for LostCitiesBackendServer {

    async fn host_game(&self, request: Request<ProtoHostGameReq>) -> Result<Response<ProtoHostGameReply>, Status> {
        let inner = request.into_inner();
        println!("Rcv: {:?}", inner);

        let player_id = WireTypeConverter::convert_host_game_req(inner)?;

        let result = {
            match self.game_api.lock() {
                Ok(mut api) => api.host_game(player_id),
                Err(e) => Err(convert_lock_error(e)),
            }
        };
        let game_id = result.map_err(|e| WireTypeConverter::convert_error(e))?;

        Ok(Response::new(ProtoHostGameReply {
            game_id
        }))
    }

    async fn join_game(&self, request: Request<ProtoJoinGameReq>) -> Result<Response<ProtoJoinGameReply>, Status> {
        let inner = request.into_inner();
        println!("Rcv: {:?}", inner);

        let (game_id, player_id) = WireTypeConverter::convert_join_game_req(inner)?;

        let result = {
            match self.game_api.lock() {
                Ok(mut api) => api.join_game(game_id, player_id),
                Err(e) => Err(convert_lock_error(e)),
            }
        };
        result.map_err(|e| WireTypeConverter::convert_error(e))?;

        Ok(Response::new(ProtoJoinGameReply {}))
    }

    async fn get_game_state(&self, request: Request<ProtoGetGameStateReq>) -> Result<Response<ProtoGetGameStateReply>, Status> {
        let inner = request.into_inner();
        println!("Rcv: {:?}", inner);

        let (game_id, player_id) = WireTypeConverter::convert_get_game_state_req(inner)?;

        let result = {
            match self.game_api.lock() {
                Ok(api) => api.get_game_state(game_id, player_id),
                Err(e) => Err(convert_lock_error(e)),
            }
        };
        let game_state = result.map_err(|e| WireTypeConverter::convert_error(e))?;
        let reply = WireTypeConverter::convert_game_state(game_state)?;

        Ok(Response::new(reply))
    }

    async fn play_card(&self, request: Request<ProtoPlayCardReq>) -> Result<Response<ProtoPlayCardReply>, Status> {
        let inner = request.into_inner();
        println!("Rcv: {:?}", inner);

        let play = WireTypeConverter::convert_play_card_req(inner)?;

        let result = {
            match self.game_api.lock() {
                Ok(mut api) => api.play_card(play),
                Err(e) => Err(convert_lock_error(e)),
            }
        };
        result.map_err(|e| WireTypeConverter::convert_error(e))?;

        Ok(Response::new(ProtoPlayCardReply {}))
    }
}
