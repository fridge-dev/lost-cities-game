use tonic::{Request, Response, Status};
use crate::wire_api::proto_lost_cities::proto_lost_cities_server::ProtoLostCities;
use crate::wire_api::proto_lost_cities::{ProtoHostGameReq, ProtoHostGameReply, ProtoJoinGameReq, ProtoJoinGameReply, ProtoGetGameStateReq, ProtoGetGameStateReply, ProtoPlayCardReq, ProtoPlayCardReply, ProtoDescribeGameReq, ProtoQueryGamesReq, ProtoGetMatchableGamesReq, ProtoQueryGamesReply, ProtoDescribeGameReply, ProtoGetMatchableGamesReply, ProtoGameMetadata, ProtoGameStatus};
use game_api::api::GameApi2;
use std::sync::{Mutex, PoisonError};
use tonic::codegen::Arc;
use futures::executor::block_on;
use crate::backend;
use backend_engine::backend_error::{BackendGameError, Cause};
use std::convert::TryInto;
use game_api::types::{GameMetadata, Play};
use chrono::Utc;
use crate::backend::channels::Slots;
use backend_engine::task::backend_task_client::BackendTaskClient;

/// Backend server is the entry point which will implement the gRPC server type.
pub struct LostCitiesBackendServer {
    game_api_slots: Slots<BackendTaskClient>,
}

impl LostCitiesBackendServer {
    pub fn new() -> Self {
        LostCitiesBackendServer {
            game_api_slots: backend::channels::start_backend()
        }
    }
}

#[tonic::async_trait]
impl ProtoLostCities for LostCitiesBackendServer {

    async fn host_game(&self, request: Request<ProtoHostGameReq>) -> Result<Response<ProtoHostGameReply>, Status> {
        let req = request.into_inner();
        println!("{} - [WIRE] {:?}", Utc::now(), req);

        let player_id = req.try_into()?;

        self.game_api_slots
            .get()
        let game_id = {
            match self.game_api.lock() {
                // TODO change `block_on` to use `.await` with channels
                Ok(mut api) => block_on(api.host_game(player_id)),
                Err(e) => Err(convert_lock_error(e)),
            }
        }?;

        let reply = ProtoHostGameReply { game_id };
        println!("{} - [WIRE] {:?}", Utc::now(), reply);
        Ok(Response::new(reply))
    }

    async fn join_game(&self, request: Request<ProtoJoinGameReq>) -> Result<Response<ProtoJoinGameReply>, Status> {
        let req = request.into_inner();
        println!("{} - [WIRE] {:?}", Utc::now(), req);

        let (game_id, player_id) = req.try_into()?;

        let _ = self.game_api_slots
            .get(game_id)
            .join_game(game_id, player_id)
            .await?;

        let reply = ProtoJoinGameReply {};
        println!("{} - [WIRE] {:?}", Utc::now(), reply);
        Ok(Response::new(reply))
    }

    async fn get_game_state(&self, request: Request<ProtoGetGameStateReq>) -> Result<Response<ProtoGetGameStateReply>, Status> {
        let req = request.into_inner();
        println!("{} - [WIRE] {:?}", Utc::now(), req);

        let (game_id, player_id) = req.try_into()?;

        let game_state = self.game_api_slots
            .get(game_id)
            .get_game_state(game_id, player_id).await?;

        let reply = game_state.into();
        println!("{} - [WIRE] {:?}", Utc::now(), reply);
        Ok(Response::new(reply))
    }

    async fn play_card(&self, request: Request<ProtoPlayCardReq>) -> Result<Response<ProtoPlayCardReply>, Status> {
        let req = request.into_inner();
        println!("{} - [WIRE] {:?}", Utc::now(), req);

        let play: Play = req.try_into()?;

        let _ = self.game_api_slots
            .get(play.game_id())
            .play_card(play)
            .await?;

        let reply = ProtoPlayCardReply {};
        println!("{} - [WIRE] {:?}", Utc::now(), reply);
        Ok(Response::new(reply))
    }

    async fn describe_game(&self, request: Request<ProtoDescribeGameReq>) -> Result<Response<ProtoDescribeGameReply>, Status> {
        let req = request.into_inner();
        println!("{} - [WIRE] {:?}", Utc::now(), req);

        let game_id = req.try_into()?;

        let game_metadata = self.game_api_slots
            .get(&game_id)
            .describe_game(game_id)
            .await?;

        let reply = ProtoDescribeGameReply {
            metadata: Some(ProtoGameMetadata::from(game_metadata))
        };
        println!("{} - [WIRE] {:?}", Utc::now(), reply);
        Ok(Response::new(reply))
    }

    async fn query_games(&self, request: Request<ProtoQueryGamesReq>) -> Result<Response<ProtoQueryGamesReply>, Status> {
        let req = request.into_inner();
        println!("{} - [WIRE] {:?}", Utc::now(), req);

        let (player_id, game_status): (String, ProtoGameStatus) = req.try_into()?;
        if game_status == ProtoGameStatus::NoGameStatus {
            return Err(Status::invalid_argument("Unspecified game status"));
        }

        let result = {
            match self.game_api.lock() {
                Err(e) => Err(convert_lock_error(e)),
                Ok(mut api) => {
                    let future_result = match game_status {
                        ProtoGameStatus::YourTurn => api.query_in_progress_games(player_id),
                        ProtoGameStatus::OpponentTurn => api.query_in_progress_games(player_id),
                        ProtoGameStatus::EndWin => api.query_completed_games(player_id),
                        ProtoGameStatus::EndLose => api.query_completed_games(player_id),
                        ProtoGameStatus::EndDraw => api.query_completed_games(player_id),
                        ProtoGameStatus::Unmatched => api.query_unmatched_games(player_id),
                        ProtoGameStatus::NoGameStatus => panic!("Impossible! We short-circuited this arm"),
                    };

                    // TODO change `block_on` to use `.await` with channels
                    block_on(future_result)
                },
            }
        }?;

        let reply = ProtoQueryGamesReply {
            games: into_proto_game_metadata_vec(result)
        };
        println!("{} - [WIRE] {:?}", Utc::now(), reply);
        Ok(Response::new(reply))
    }

    async fn get_matchable_games(&self, request: Request<ProtoGetMatchableGamesReq>) -> Result<Response<ProtoGetMatchableGamesReply>, Status> {
        let req = request.into_inner();
        println!("{} - [WIRE] {:?}", Utc::now(), req);

        let player_id = req.try_into()?;

        let result = {
            match self.game_api.lock() {
                // TODO change `block_on` to use `.await` with channels
                Ok(mut api) => block_on(api.query_all_unmatched_games(player_id)),
                Err(e) => Err(convert_lock_error(e)),
            }
        }?;

        let reply = ProtoGetMatchableGamesReply {
            games: into_proto_game_metadata_vec(result)
        };
        println!("{} - [WIRE] {:?}", Utc::now(), reply);
        Ok(Response::new(reply))
    }
}

fn convert_lock_error<T>(e: PoisonError<T>) -> BackendGameError2 {
    // I don't know how to recover from this. Recreate game handler?
    println!("ERROR: GameApi lock was poisoned. Here's the err: {}", e);
    BackendGameError2::Internal(Cause::Internal("Failed to acquire GameApi lock"))
}

fn into_proto_game_metadata_vec(game_metadata_vec: Vec<GameMetadata>) -> Vec<ProtoGameMetadata> {
    let games: Vec<ProtoGameMetadata> = Vec::with_capacity(game_metadata_vec.len());
    for game_metadata in game_metadata_vec {
        ProtoGameMetadata::from(game_metadata);
    }

    games
}
