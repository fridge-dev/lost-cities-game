use game_api::api::GameApi2;
use game_api::types::{GameState, Play, GameMetadata};
use std::borrow::Cow;
use std::convert::TryFrom;
use tonic::transport::{Channel, Endpoint, Error};
use crate::wire_api::proto_lost_cities::{ProtoHostGameReq, ProtoJoinGameReq, ProtoGetGameStateReq, ProtoPlayCardReq, ProtoDescribeGameReq};
use crate::wire_api::proto_lost_cities::proto_lost_cities_client::ProtoLostCitiesClient;
use crate::client_game_api::error::ClientGameError;

pub struct GameClient {
    inner_client: ProtoLostCitiesClient<Channel>,
}

impl GameClient {
    pub async fn new() -> Result<Self, Error> {
        let endpoint = Endpoint::new("http://localhost:50051")?;

        let connection = endpoint.connect().await?;

        Ok(GameClient {
            inner_client: ProtoLostCitiesClient::new(connection)
        })
    }
}

#[async_trait::async_trait]
impl GameApi2<ClientGameError> for GameClient {
    async fn host_game(&mut self, p1_id: String) -> Result<String, ClientGameError> {
        let request = tonic::Request::new(ProtoHostGameReq {
            player_id: p1_id
        });

        self.inner_client.host_game(request)
            .await
            .map_err(|e| handle_error(e))
            .map(|response| response.into_inner().game_id)
    }

    async fn join_game(&mut self, game_id: String, p2_id: String) -> Result<(), ClientGameError> {
        let request = tonic::Request::new(ProtoJoinGameReq {
            game_id,
            player_id: p2_id,
        });

        self.inner_client.join_game(request)
            .await
            .map_err(|e| handle_error(e))
            .map(|_response| ())
    }

    async fn describe_game(&mut self, game_id: String) -> Result<GameMetadata, ClientGameError> {
        let request = tonic::Request::new(ProtoDescribeGameReq {
            game_id
        });

        self.inner_client.describe_game(request)
            .await
            .map_err(|e| handle_error(e))
            .and_then(|response| response.into_inner().metadata.ok_or(ClientGameError::MalformedResponse(Cow::from("Missing GameMetadata inside DescribeGame reply"))))
            .and_then(|proto_game_metadata| GameMetadata::try_from(proto_game_metadata))
    }

    async fn query_unmatched_games(&mut self, player_id: String) -> Result<Vec<GameMetadata>, ClientGameError> {
        unimplemented!()
    }

    async fn query_in_progress_games(&mut self, player_id: String) -> Result<Vec<GameMetadata>, ClientGameError> {
        unimplemented!()
    }

    async fn query_completed_games(&mut self, player_id: String) -> Result<Vec<GameMetadata>, ClientGameError> {
        unimplemented!()
    }

    async fn query_all_unmatched_games(&mut self) -> Result<Vec<GameMetadata>, ClientGameError> {
        unimplemented!()
    }

    async fn get_game_state(&mut self, game_id: String, player_id: String) -> Result<GameState, ClientGameError> {
        let request = tonic::Request::new(ProtoGetGameStateReq {
            game_id,
            player_id
        });

        self.inner_client.get_game_state(request)
            .await
            .map_err(|e| handle_error(e))
            .and_then(|response| response.into_inner().game.ok_or(ClientGameError::MalformedResponse(Cow::from("Missing Game inside GameState"))))
            .and_then(|proto_game| GameState::try_from(proto_game))

    }

    async fn play_card(&mut self, play: Play) -> Result<(), ClientGameError> {
        let request = tonic::Request::new(ProtoPlayCardReq::from(play));

        self.inner_client.play_card(request)
            .await
            .map_err(|e| handle_error(e))
            .map(|_response| ())
    }
}

fn handle_error(status: tonic::Status) -> ClientGameError {
    println!("WARN: Failed backend call: {:?}", status);
    ClientGameError::from(status)
}
