use futures::executor::block_on;
use game_api::types::{GameState, Play, DrawPile};
use tonic::Code;
use tonic::transport::{Channel, Endpoint, Error};
use wire_api::proto_lost_cities::{ProtoHostGameReq, ProtoJoinGameReq, ProtoGetGameStateReq, ProtoGame, ProtoPlayCardReq, ProtoDrawPile, ProtoColor, ProtoPlayTarget};
use wire_api::proto_lost_cities::proto_lost_cities_client::ProtoLostCitiesClient;
use game_api::api::GameApi2;
use crate::frontend::frontend_error::ClientGameError;

pub struct BackendClient {
    inner_client: ProtoLostCitiesClient<Channel>,
}

impl BackendClient {
    pub async fn new_async() -> Result<Self, Error> {
        let endpoint = Endpoint::new("localhost:50051")?;

        let connection = endpoint.connect().await?;

        Ok(BackendClient {
            inner_client: ProtoLostCitiesClient::new(connection)
        })
    }

    pub fn new_sync() -> Result<Self, Error> {
        block_on(BackendClient::new_async())
    }
}

#[async_trait::async_trait]
impl GameApi2<ClientGameError> for BackendClient {
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

    async fn get_game_state(&mut self, game_id: String, player_id: String) -> Result<GameState, ClientGameError> {
        let request = tonic::Request::new(ProtoGetGameStateReq {
            game_id,
            player_id
        });

        self.inner_client.get_game_state(request)
            .await
            .map_err(|e| handle_error(e))
            .and_then(|response| response.into_inner().game.ok_or(ClientGameError::NotFound))
            .and_then(|proto_game| try_from_proto_game(proto_game))

    }

    async fn play_card(&mut self, play: Play) -> Result<(), ClientGameError> {
        let request = tonic::Request::new(into_proto_play_card_req(play));

        self.inner_client.play_card(request)
            .await
            .map_err(|e| handle_error(e))
            .map(|_response| ())
    }
}

// TODO implement and move to better place
fn try_from_proto_game(proto_game: ProtoGame) -> Result<GameState, ClientGameError> {
    unimplemented!()
}

// TODO implement and move to better place
fn into_proto_play_card_req(play: Play) -> ProtoPlayCardReq {
    let (draw_pile, draw_color) = match play.draw_pile() {
        DrawPile::Main => (ProtoDrawPile::MainDraw, ProtoColor::NoColor),
        DrawPile::Neutral(color) => (ProtoDrawPile::DiscardDraw, ProtoColor::from(*color)),
    };

    ProtoPlayCardReq {
        game_id: play.game_id().to_owned(),
        player_id: play.player_id().to_owned(),
        card: Some((*play.card()).into()),
        target: ProtoPlayTarget::from(*play.target()) as i32,
        draw_pile: draw_pile as i32,
        discard_draw_color: draw_color as i32,
    }
}

// TODO implement and move to better place
fn handle_error(status: tonic::Status) -> ClientGameError {
    println!("WARN: Failed backend call: {:?}", status);
    match status.code() {
        Code::InvalidArgument => ClientGameError::UserInvalidArg,
        Code::AlreadyExists => ClientGameError::UserInvalidArg,
        Code::NotFound => ClientGameError::NotFound,
        _ => ClientGameError::BackendUnknown
    }
}
