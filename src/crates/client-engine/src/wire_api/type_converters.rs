use crate::client_game_api::error::ClientGameError;
use crate::wire_api::proto_lost_cities::{
    ProtoPlayCardReq,
    ProtoPlayTarget,
    ProtoDrawPile,
    ProtoCard,
    ProtoColor,
    ProtoGame,
    ProtoGameStatus
};
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
    DecoratedCard,
    GameBoard,
};
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

// ============================= Reply converters =====================================
// ============================= Proto -> App =========================================

impl TryFrom<ProtoGame> for GameState {
    type Error = ClientGameError;

    fn try_from(proto_game: ProtoGame) -> Result<Self, Self::Error> {
        let game_board = GameBoard::new(
            HashMap::new(),
            HashMap::new(),
            0,
            0,
            HashMap::new(),
            0
        );

        let mut my_hand = Vec::with_capacity(proto_game.my_hand.len());
        for proto_card in proto_game.my_hand {
            my_hand.push(DecoratedCard::new(
                proto_card.try_into()?,
                true, // TODO isPlayable
            ))
        }
        let my_hand = my_hand;

        let (status, is_my_turn) = ProtoGameStatus::try_from(proto_game.status)?.try_into()?;

        Ok(GameState::new(
            game_board,
            my_hand,
            is_my_turn,
            status,
        ))
    }
}

// ============================= From<Proto> for App ==================================

impl TryFrom<ProtoGameStatus> for (GameStatus, bool) {
    type Error = ClientGameError;

    fn try_from(proto_game_status: ProtoGameStatus) -> Result<Self, Self::Error> {
        match proto_game_status {
            ProtoGameStatus::NoGameStatus => Err(ClientGameError::MalformedResponse(Cow::from("Missing game status"))),
            ProtoGameStatus::YourTurn => Ok((GameStatus::InProgress, true)),
            ProtoGameStatus::OpponentTurn => Ok((GameStatus::InProgress, false)),
            ProtoGameStatus::EndWin => Ok((GameStatus::Complete(GameResult::Win), false)),
            ProtoGameStatus::EndLose => Ok((GameStatus::Complete(GameResult::Lose), false)),
            ProtoGameStatus::EndDraw => Ok((GameStatus::Complete(GameResult::Draw), false)),
        }
    }
}

impl TryFrom<ProtoCard> for Card {
    type Error = ClientGameError;

    fn try_from(proto_card: ProtoCard) -> Result<Self, Self::Error> {
        let color = CardColor::try_from(
            ProtoColor::try_from(proto_card.color)?
        )?;
        let value = CardValue::try_from(proto_card.value)
            .map_err(|msg| ClientGameError::MalformedResponse(Cow::from(msg)))?;

        Ok(Card::new(
            color,
            value,
        ))
    }
}

impl TryFrom<ProtoColor> for CardColor {
    type Error = ClientGameError;

    fn try_from(proto_color: ProtoColor) -> Result<Self, Self::Error> {
        match proto_color {
            ProtoColor::NoColor => Err(ClientGameError::MalformedResponse(Cow::from("Unspecified proto color"))),
            ProtoColor::Red => Ok(CardColor::Red),
            ProtoColor::Green => Ok(CardColor::Green),
            ProtoColor::White => Ok(CardColor::White),
            ProtoColor::Blue => Ok(CardColor::Blue),
            ProtoColor::Yellow => Ok(CardColor::Yellow),
        }
    }
}

// ============================= Request converters ===================================

impl From<Play> for ProtoPlayCardReq {
    fn from(play: Play) -> Self {
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
}

// ============================= App -> Proto =========================================
// ============================= From<App> for Proto ==================================

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
