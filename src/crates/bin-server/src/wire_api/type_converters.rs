use crate::wire_api::proto_lost_cities::{
    ProtoCard,
    ProtoColor,
    ProtoDiscardPile,
    ProtoDiscardPileSurface,
    ProtoDrawPile,
    ProtoGame,
    ProtoGameStatus,
    ProtoGetGameStateReply,
    ProtoGetGameStateReq,
    ProtoHostGameReq,
    ProtoJoinGameReq,
    ProtoPlayCardReq,
    ProtoPlayHistory,
    ProtoPlayTarget,
};
use game_api::types::{
    Card,
    CardColor,
    CardTarget,
    CardValue,
    DecoratedCard,
    DrawPile,
    GameResult,
    GameState,
    GameStatus,
    Play,
};
use std::collections::HashMap;
use std::convert::TryFrom;
use tonic::{Code, Status};

// ============================= Request converters ===================================
// ============================= Proto -> App =========================================

impl TryFrom<ProtoHostGameReq> for String {
    type Error = Status;

    fn try_from(req: ProtoHostGameReq) -> Result<Self, Self::Error> {
        if req.player_id.is_empty() {
            return Err(Status::new(Code::InvalidArgument, "Missing PlayedId"));
        }

        Ok(req.player_id)
    }
}

impl TryFrom<ProtoJoinGameReq> for (String, String) {
    type Error = Status;

    fn try_from(req: ProtoJoinGameReq) -> Result<Self, Self::Error> {
        if req.game_id.is_empty() {
            return Err(Status::new(Code::InvalidArgument, "Missing GameId"));
        }
        if req.player_id.is_empty() {
            return Err(Status::new(Code::InvalidArgument, "Missing PlayerId"));
        }

        Ok((req.game_id, req.player_id))
    }
}

impl TryFrom<ProtoGetGameStateReq> for (String, String) {
    type Error = Status;

    fn try_from(req: ProtoGetGameStateReq) -> Result<Self, Self::Error> {
        if req.game_id.is_empty() {
            return Err(Status::new(Code::InvalidArgument, "Missing GameId"));
        }
        if req.player_id.is_empty() {
            return Err(Status::new(Code::InvalidArgument, "Missing PlayerId"));
        }

        Ok((req.game_id, req.player_id))
    }
}

impl TryFrom<ProtoPlayCardReq> for Play {
    type Error = Status;

    fn try_from(req: ProtoPlayCardReq) -> Result<Self, Self::Error> {
        if req.game_id.is_empty() {
            return Err(Status::new(Code::InvalidArgument, "Missing GameId"));
        }
        if req.player_id.is_empty() {
            return Err(Status::new(Code::InvalidArgument, "Missing PlayerId"));
        }
        let card: Card = match req.card {
            None => return Err(Status::new(Code::InvalidArgument, "Missing Card")),
            Some(proto_card) => Card::try_from(proto_card)?,
        };
        let card_target: CardTarget = match ProtoPlayTarget::try_from(req.target)? {
            ProtoPlayTarget::NoPlayTarget => return Err(Status::new(Code::InvalidArgument, "Unspecified PlayTarget")),
            ProtoPlayTarget::PlayerBoard => CardTarget::Player,
            ProtoPlayTarget::Discard => CardTarget::Neutral,
        };
        let draw_pile: DrawPile = match ProtoDrawPile::try_from(req.draw_pile)? {
            ProtoDrawPile::NoDrawPile => return Err(Status::new(Code::InvalidArgument, "Unspecified DrawPile")),
            ProtoDrawPile::MainDraw => DrawPile::Main,
            ProtoDrawPile::DiscardDraw => DrawPile::Neutral(CardColor::try_from(
                ProtoColor::try_from(req.discard_draw_color)?,
            )?),
        };

        Ok(Play::new(
            req.game_id,
            req.player_id,
            card,
            card_target,
            draw_pile,
        ))
    }
}

// ============================= From<Proto> for App ==================================

impl TryFrom<ProtoCard> for Card {
    type Error = Status;

    fn try_from(proto_card: ProtoCard) -> Result<Self, Self::Error> {
        let color = CardColor::try_from(ProtoColor::try_from(proto_card.color)?)?;
        let value = CardValue::try_from(proto_card.value)
            .map_err(|msg| Status::new(Code::InvalidArgument, msg))?;

        Ok(Card::new(color, value))
    }
}

impl TryFrom<ProtoColor> for CardColor {
    type Error = Status;

    fn try_from(proto_color: ProtoColor) -> Result<Self, Self::Error> {
        match proto_color {
            ProtoColor::NoColor => Err(Status::new(Code::InvalidArgument, "Unspecified proto color")),
            ProtoColor::Red => Ok(CardColor::Red),
            ProtoColor::Green => Ok(CardColor::Green),
            ProtoColor::White => Ok(CardColor::White),
            ProtoColor::Blue => Ok(CardColor::Blue),
            ProtoColor::Yellow => Ok(CardColor::Yellow),
        }
    }
}

// ============================= Reply converters =====================================
// ============================= App -> Proto =========================================

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
            opponent_player_id: "TODO".to_string(),
        }
    }
}

// ============================= From<App> for Proto ==================================

fn into_proto_card_vec(hand: &Vec<DecoratedCard>) -> Vec<ProtoCard> {
    hand.iter()
        .map(|card| ProtoCard::from(*card.card()))
        .collect()
}

fn into_proto_play_history(plays: &HashMap<CardColor, Vec<CardValue>>) -> ProtoPlayHistory {
    let inner_converter = |color| {
        plays
            .get(&color)
            .map(|values| {
                values
                    .iter()
                    .map(|card_value| u32::from(*card_value))
                    .collect()
            })
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
        neutral_draw_pile
            .get(&color)
            .map(|(card_value, num_cards)| ProtoDiscardPileSurface {
                value: u32::from(*card_value),
                remaining: *num_cards as u32,
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
        GameStatus::InProgress => {
            if *game_state.is_my_turn() {
                ProtoGameStatus::YourTurn
            } else {
                ProtoGameStatus::OpponentTurn
            }
        }
        GameStatus::Complete(result) => match result {
            GameResult::Win => ProtoGameStatus::EndWin,
            GameResult::Lose => ProtoGameStatus::EndLose,
            GameResult::Draw => ProtoGameStatus::EndDraw,
        },
    }
}

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

impl From<CardTarget> for ProtoPlayTarget {
    fn from(card_target: CardTarget) -> Self {
        match card_target {
            CardTarget::Player => ProtoPlayTarget::PlayerBoard,
            CardTarget::Neutral => ProtoPlayTarget::Discard,
        }
    }
}
