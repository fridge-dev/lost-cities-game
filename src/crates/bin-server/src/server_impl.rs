use tonic::{Request, Response, Status};
use crate::wire_api::proto_lost_cities::proto_lost_cities_server::ProtoLostCities;
use crate::wire_api::proto_lost_cities::{ProtoHostGameReq, ProtoHostGameReply, ProtoJoinGameReq, ProtoJoinGameReply, ProtoGetGameStateReq, ProtoGetGameStateReply, ProtoPlayCardReq, ProtoPlayCardReply, ProtoDescribeGameReq, ProtoQueryGamesReq, ProtoGetMatchableGamesReq, ProtoQueryGamesReply, ProtoDescribeGameReply, ProtoGetMatchableGamesReply, ProtoGameMetadata, ProtoGameStatus};
use std::convert::TryInto;
use game_api::types::{GameMetadata, Play};
use chrono::Utc;
use crate::wire_api::error_converters::IntoTonicStatus;
use backend_engine::game_api::GameApi2Immut;

/// Backend server is the entry point which will implement the gRPC server type.
pub struct LostCitiesBackendServer {
    game_api: Box<dyn GameApi2Immut + Send + Sync>,
}

impl LostCitiesBackendServer {
    pub fn new() -> Self {
        LostCitiesBackendServer {
            game_api: backend_engine::start_backend()
        }
    }
}

#[tonic::async_trait]
impl ProtoLostCities for LostCitiesBackendServer {

    async fn host_game(&self, request: Request<ProtoHostGameReq>) -> Result<Response<ProtoHostGameReply>, Status> {
        let req = request.into_inner();
        println!("{} - [WIRE] {:?}", Utc::now(), req);

        let (game_id, player_id) = req.try_into()?;

        let _ = self.game_api
            .host_game(game_id, player_id)
            .await
            .map_err(|e| e.into_status())?;

        let reply = ProtoHostGameReply {};
        println!("{} - [WIRE] {:?}", Utc::now(), reply);
        Ok(Response::new(reply))
    }

    async fn join_game(&self, request: Request<ProtoJoinGameReq>) -> Result<Response<ProtoJoinGameReply>, Status> {
        let req = request.into_inner();
        println!("{} - [WIRE] {:?}", Utc::now(), req);

        let (game_id, player_id) = req.try_into()?;

        let _ = self.game_api
            .join_game(game_id, player_id)
            .await
            .map_err(|e| e.into_status())?;

        let reply = ProtoJoinGameReply {};
        println!("{} - [WIRE] {:?}", Utc::now(), reply);
        Ok(Response::new(reply))
    }

    async fn get_game_state(&self, request: Request<ProtoGetGameStateReq>) -> Result<Response<ProtoGetGameStateReply>, Status> {
        let req = request.into_inner();
        println!("{} - [WIRE] {:?}", Utc::now(), req);

        let (game_id, player_id) = req.try_into()?;

        let game_state = self.game_api
            .get_game_state(game_id, player_id).await
            .map_err(|e| e.into_status())?;

        let reply = game_state.into();
        println!("{} - [WIRE] {:?}", Utc::now(), reply);
        Ok(Response::new(reply))
    }

    async fn play_card(&self, request: Request<ProtoPlayCardReq>) -> Result<Response<ProtoPlayCardReply>, Status> {
        let req = request.into_inner();
        println!("{} - [WIRE] {:?}", Utc::now(), req);

        let play: Play = req.try_into()?;

        let _ = self.game_api
            .play_card(play)
            .await
            .map_err(|e| e.into_status())?;

        let reply = ProtoPlayCardReply {};
        println!("{} - [WIRE] {:?}", Utc::now(), reply);
        Ok(Response::new(reply))
    }

    async fn describe_game(&self, request: Request<ProtoDescribeGameReq>) -> Result<Response<ProtoDescribeGameReply>, Status> {
        let req = request.into_inner();
        println!("{} - [WIRE] {:?}", Utc::now(), req);

        let game_id = req.try_into()?;

        let game_metadata = self.game_api
            .describe_game(game_id)
            .await
            .map_err(|e| e.into_status())?;

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

        let result = match game_status {
            ProtoGameStatus::YourTurn => self.game_api.query_in_progress_games(player_id).await,
            ProtoGameStatus::OpponentTurn => self.game_api.query_in_progress_games(player_id).await,
            ProtoGameStatus::EndWin => self.game_api.query_completed_games(player_id).await,
            ProtoGameStatus::EndLose => self.game_api.query_completed_games(player_id).await,
            ProtoGameStatus::EndDraw => self.game_api.query_completed_games(player_id).await,
            ProtoGameStatus::Unmatched => self.game_api.query_unmatched_games(player_id).await,
            ProtoGameStatus::NoGameStatus => return Err(Status::invalid_argument("Unspecified game status")),
        };
        let games = result.map_err(|e| e.into_status())?;

        let reply = ProtoQueryGamesReply {
            games: into_proto_game_metadata_vec(games)
        };
        println!("{} - [WIRE] {:?}", Utc::now(), reply);
        Ok(Response::new(reply))
    }

    async fn get_matchable_games(&self, request: Request<ProtoGetMatchableGamesReq>) -> Result<Response<ProtoGetMatchableGamesReply>, Status> {
        let req = request.into_inner();
        println!("{} - [WIRE] {:?}", Utc::now(), req);

        let player_id = req.try_into()?;

        let games = self.game_api
            .query_all_unmatched_games(player_id)
            .await
            .map_err(|e| e.into_status())?;

        let reply = ProtoGetMatchableGamesReply {
            games: into_proto_game_metadata_vec(games)
        };
        println!("{} - [WIRE] {:?}", Utc::now(), reply);
        Ok(Response::new(reply))
    }
}

fn into_proto_game_metadata_vec(game_metadata_vec: Vec<GameMetadata>) -> Vec<ProtoGameMetadata> {
    let games: Vec<ProtoGameMetadata> = Vec::with_capacity(game_metadata_vec.len());
    for game_metadata in game_metadata_vec {
        ProtoGameMetadata::from(game_metadata);
    }

    games
}
