use std::collections::HashMap;
use tonic::{Status, Code};
use game_api::types::{
    Play,
    Card,
    CardColor,
    CardValue,
    CardTarget,
    DrawPile,
    GameState,
    GameStatus,
    GameResult,
    DecoratedCard
};
use crate::proto_lost_cities::{
    ProtoHostGameReq,
    ProtoJoinGameReq,
    ProtoGetGameStateReq,
    ProtoGetGameStateReply,
    ProtoPlayCardReq,
    ProtoPlayTarget,
    ProtoDrawPile,
    ProtoCard,
    ProtoColor,
    ProtoGame,
    ProtoPlayHistory,
    ProtoDiscardPile,
    ProtoDiscardPileSurface,
    ProtoGameStatus
};
use game_api::backend_errors::BackendGameError;
use std::convert::TryFrom;

// =========================== Proto -> App converters ================================

pub fn try_from_proto_host_game_req(req: ProtoHostGameReq) -> Result<String, Status> {
    if req.player_id.is_empty() {
        return Err(Status::new(Code::InvalidArgument, "Missing PlayedId"));
    }

    Ok(req.player_id)
}

pub fn try_from_proto_join_game_req(req: ProtoJoinGameReq) -> Result<(String, String), Status> {
    if req.game_id.is_empty() {
        return Err(Status::new(Code::InvalidArgument, "Missing GameId"));
    }
    if req.player_id.is_empty() {
        return Err(Status::new(Code::InvalidArgument, "Missing PlayerId"));
    }

    Ok((req.game_id, req.player_id))
}

pub fn try_from_proto_get_game_state_req(req: ProtoGetGameStateReq) -> Result<(String, String), Status> {
    if req.game_id.is_empty() {
        return Err(Status::new(Code::InvalidArgument, "Missing GameId"));
    }
    if req.player_id.is_empty() {
        return Err(Status::new(Code::InvalidArgument, "Missing PlayerId"));
    }

    Ok((req.game_id, req.player_id))
}

pub fn try_from_proto_play_card_req(req: ProtoPlayCardReq) -> Result<Play, Status> {
    Play::try_from(req)
        .map_err(|msg| Status::new(Code::InvalidArgument, msg))
}

// ============================== From<Proto> for App =================================

impl TryFrom<ProtoCard> for Card {
    type Error = String;

    fn try_from(proto_card: ProtoCard) -> Result<Self, Self::Error> {
        Ok(Card::new(
            CardColor::try_from(ProtoColor::try_from(proto_card.color)?)?,
            CardValue::try_from(proto_card.value)?,
        ))

    }
}

impl TryFrom<ProtoColor> for CardColor {
    type Error = &'static str;

    fn try_from(proto_color: ProtoColor) -> Result<Self, Self::Error> {
        match proto_color {
            ProtoColor::NoColor => Err("Unspecified proto color"),
            ProtoColor::Red => Ok(CardColor::Red),
            ProtoColor::Green => Ok(CardColor::Green),
            ProtoColor::White => Ok(CardColor::White),
            ProtoColor::Blue => Ok(CardColor::Blue),
            ProtoColor::Yellow => Ok(CardColor::Yellow),
        }
    }
}

impl TryFrom<ProtoPlayCardReq> for Play {
    type Error = String;

    fn try_from(req: ProtoPlayCardReq) -> Result<Self, Self::Error> {
        if req.game_id.is_empty() {
            return Err("Missing GameId")?;
        }
        if req.player_id.is_empty() {
            return Err("Missing PlayerId")?;
        }
        let card: Card = match req.card {
            None => return Err("Missing Card")?,
            Some(proto_card) => Card::try_from(proto_card)?
        };
        let card_target: CardTarget = match ProtoPlayTarget::try_from(req.target)? {
            ProtoPlayTarget::NoPlayTarget => return Err("Unspecified PlayTarget")?,
            ProtoPlayTarget::PlayerBoard => CardTarget::Player,
            ProtoPlayTarget::Discard => CardTarget::Neutral,
        };
        let draw_pile: DrawPile = match ProtoDrawPile::try_from(req.draw_pile)? {
            ProtoDrawPile::NoDrawPile => return Err("Unspecified DrawPile")?,
            ProtoDrawPile::MainDraw => DrawPile::Main,
            ProtoDrawPile::DiscardDraw => DrawPile::Neutral(
                CardColor::try_from(
                    ProtoColor::try_from(req.discard_draw_color)?
                )?
            ),
        };

        Ok(Play::new(
            req.game_id,
            req.player_id,
            card,
            card_target,
            draw_pile
        ))
    }
}

// =========================== App -> Proto converters ================================

// TODO move BackendGameError into this crate and
// impl From<BackendGameError> for Status
pub fn into_proto_status(game_error: BackendGameError) -> Status {
    match game_error {
        BackendGameError::NotFound(resource) => Status::new(Code::NotFound, format!("Resource {} not found.", resource)),
        BackendGameError::GameAlreadyMatched => Status::new(Code::AlreadyExists, format!("The game you attempted to join is full.")),
        BackendGameError::InvalidPlay(reason) => Status::new(Code::InvalidArgument, format!("Can't play card. {}", reason)),
        BackendGameError::Internal(cause) => {
            println!("ERROR: Internal failure caused by '{:?}'", cause);
            Status::new(Code::Internal, format!("Internal server failure"))
        },
    }
}

fn into_proto_card_vec(hand: &Vec<DecoratedCard>) -> Vec<ProtoCard> {
    hand.iter()
        .map(|card| ProtoCard::from(*card.card()))
        .collect()
}

fn into_proto_play_history(plays: &HashMap<CardColor, Vec<CardValue>>) -> ProtoPlayHistory {
    let inner_converter = |color| {
        plays.get(&color)
            .map(|values| values
                .iter()
                .map(|card_value| u32::from(*card_value))
                .collect())
            .unwrap_or(vec![])
    };

    ProtoPlayHistory {
        red: inner_converter(CardColor::Red),
        blue: inner_converter(CardColor::Blue),
        green: inner_converter(CardColor::Green),
        white: inner_converter(CardColor::White),
        yellow: inner_converter(CardColor::Yellow),
    }
}

fn into_proto_discard_pile(neutral_draw_pile: &HashMap<CardColor, (CardValue, usize)>) -> ProtoDiscardPile {
    let inner_converter = |color| {
        neutral_draw_pile.get(&color)
            .map(|(card_value, num_cards)| {
                ProtoDiscardPileSurface {
                    value: u32::from(*card_value),
                    remaining: *num_cards as u32,
                }
            })
    };

    ProtoDiscardPile {
        red: inner_converter(CardColor::Red),
        green: inner_converter(CardColor::Green),
        white: inner_converter(CardColor::White),
        blue: inner_converter(CardColor::Blue),
        yellow: inner_converter(CardColor::Yellow),
    }
}

fn into_proto_game_status(game_state: &GameState) -> ProtoGameStatus {
    match game_state.status() {
        GameStatus::InProgress => if *game_state.is_my_turn() {
            ProtoGameStatus::YourTurn
        } else {
            ProtoGameStatus::OpponentTurn
        },
        GameStatus::Complete(result) => match result {
            GameResult::Win => ProtoGameStatus::EndWin,
            GameResult::Lose => ProtoGameStatus::EndLose,
            GameResult::Draw => ProtoGameStatus::EndDraw,
        },
    }
}

// ============================== From<App> for Proto =================================

impl From<Card> for ProtoCard {
    fn from(card: Card) -> Self {
        ProtoCard {
            color: ProtoColor::from(*card.card_color()) as i32,
            value: (*card.card_value()).into(),
        }
    }
}

impl From<CardColor> for ProtoColor {
    fn from(card_color: CardColor) -> ProtoColor {
        let proto_color: ProtoColor = match card_color {
            CardColor::Red => ProtoColor::Red,
            CardColor::Green => ProtoColor::Green,
            CardColor::White => ProtoColor::White,
            CardColor::Blue => ProtoColor::Blue,
            CardColor::Yellow => ProtoColor::Yellow,
        };

        proto_color
    }
}

impl From<GameState> for ProtoGetGameStateReply {
    fn from(game_state: GameState) -> Self {
        let proto_game = ProtoGame {
            my_hand: into_proto_card_vec(game_state.my_hand()),
            my_plays: Some(into_proto_play_history(game_state.game_board().my_plays())),
            opponent_plays: Some(into_proto_play_history(game_state.game_board().op_plays())),
            discard_pile: Some(into_proto_discard_pile(game_state.game_board().neutral_draw_pile())),
            draw_pile_cards_remaining: *game_state.game_board().draw_pile_cards_remaining() as u32,
            status: into_proto_game_status(&game_state) as i32,
            my_score: *game_state.game_board().my_score(),
            op_score: *game_state.game_board().op_score(),
        };

        ProtoGetGameStateReply {
            game: Some(proto_game),
            opponent_player_id: "TODO".to_string()
        }
    }
}
