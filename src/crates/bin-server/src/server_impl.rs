use tonic::{Request, Response, Status};
use crate::wire_api::proto_lost_cities::proto_lost_cities_server::ProtoLostCities;
use crate::wire_api::proto_lost_cities::{ProtoHostGameReq, ProtoHostGameReply, ProtoJoinGameReq, ProtoJoinGameReply, ProtoGetGameStateReq, ProtoGetGameStateReply, ProtoPlayCardReq, ProtoPlayCardReply};
use game_api::api::GameApi2;
use std::sync::{Mutex, PoisonError};
use tonic::codegen::Arc;
use futures::executor::block_on;
use crate::backend;
use crate::backend::backend_error::{BackendGameError2, Cause};
use std::convert::TryInto;

/// Backend server is the entry point which will implement the gRPC server type.
pub struct LostCitiesBackendServer {
    // Mutex needed for interior mutability because I want a working, multi-tasked prototype for now.
    // The multi-task impl runs on a single, blocking thread, so yeah, it's not great.
    // I'll change the GameApi to be backed by a mpsc task model with oneshot callbacks.
    game_api: Arc<Mutex<dyn GameApi2<BackendGameError2> + Send>>,
}

impl LostCitiesBackendServer {
    pub fn new() -> Self {
        LostCitiesBackendServer {
            game_api: backend::channels::new_backend_game_api()
        }
    }
}

fn convert_lock_error<T>(e: PoisonError<T>) -> BackendGameError2 {
    // I don't know how to recover from this. Recreate game handler?
    println!("ERROR: GameApi lock was poisoned. Here's the err: {}", e);
    BackendGameError2::Internal(Cause::Internal("Failed to acquire GameApi lock"))
}

#[tonic::async_trait]
impl ProtoLostCities for LostCitiesBackendServer {

    async fn host_game(&self, request: Request<ProtoHostGameReq>) -> Result<Response<ProtoHostGameReply>, Status> {
        let inner = request.into_inner();
        println!("Rcv: {:?}", inner);

        let player_id = inner.try_into()?;

        let game_id = {
            match self.game_api.lock() {
                // TODO change `block_on` to use `.await` with channels
                Ok(mut api) => block_on(api.host_game(player_id)),
                Err(e) => Err(convert_lock_error(e)),
            }
        }?;

        Ok(Response::new(ProtoHostGameReply {
            game_id
        }))
    }

    async fn join_game(&self, request: Request<ProtoJoinGameReq>) -> Result<Response<ProtoJoinGameReply>, Status> {
        let inner = request.into_inner();
        println!("Rcv: {:?}", inner);

        let (game_id, player_id) = inner.try_into()?;

        let _ = {
            match self.game_api.lock() {
                // TODO change `block_on` to use `.await` with channels
                Ok(mut api) => block_on(api.join_game(game_id, player_id)),
                Err(e) => Err(convert_lock_error(e)),
            }
        }?;

        Ok(Response::new(ProtoJoinGameReply {}))
    }

    async fn get_game_state(&self, request: Request<ProtoGetGameStateReq>) -> Result<Response<ProtoGetGameStateReply>, Status> {
        let inner = request.into_inner();
        println!("Rcv: {:?}", inner);

        let (game_id, player_id) = inner.try_into()?;

        let game_state = {
            match self.game_api.lock() {
                // TODO change `block_on` to use `.await` with channels
                Ok(mut api) => block_on(api.get_game_state(game_id, player_id)),
                Err(e) => Err(convert_lock_error(e)),
            }
        }?;

        Ok(Response::new(game_state.into()))
    }

    async fn play_card(&self, request: Request<ProtoPlayCardReq>) -> Result<Response<ProtoPlayCardReply>, Status> {
        let inner = request.into_inner();
        println!("Rcv: {:?}", inner);

        let play = inner.try_into()?;

        let _ = {
            match self.game_api.lock() {
                // TODO change `block_on` to use `.await` with channels
                Ok(mut api) => block_on(api.play_card(play)),
                Err(e) => Err(convert_lock_error(e)),
            }
        }?;

        Ok(Response::new(ProtoPlayCardReply {}))
    }
}
