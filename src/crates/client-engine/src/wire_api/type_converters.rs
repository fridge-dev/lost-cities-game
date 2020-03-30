use crate::client_game_api::error::ClientGameError;
use crate::wire_api::proto_lost_cities::{ProtoPlayCardReq, ProtoPlayTarget, ProtoDrawPile, ProtoCard, ProtoColor, ProtoGame, ProtoGameStatus, ProtoPlayHistory, ProtoDiscardPile, ProtoDiscardPileSurface, ProtoGameMetadata, ProtoScore};
use game_api::types::{Play, Card, CardColor, CardValue, CardTarget, DrawPile, GameState, GameStatus, GameResult, DecoratedCard, GameBoard, GameMetadata};
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

// ============================= Reply converters =====================================
// ============================= Proto -> App =========================================

fn card_value_from_proto(value_u32: u32) -> Result<CardValue, ClientGameError> {
    CardValue::try_from(value_u32)
        .map_err(|msg| ClientGameError::MalformedResponse(Cow::from(msg)))
}

// ============================= From<Proto> for App ==================================

impl TryFrom<ProtoGameMetadata> for GameMetadata {
    type Error = ClientGameError;

    fn try_from(proto_game_metadata: ProtoGameMetadata) -> Result<Self, Self::Error> {
        let opt_status = match ProtoGameStatus::try_from(proto_game_metadata.status)? {
            ProtoGameStatus::NoGameStatus => return Err(ClientGameError::MalformedResponse(Cow::from("Missing game status"))),
            ProtoGameStatus::YourTurn => Some(GameStatus::InProgress(true)),
            ProtoGameStatus::OpponentTurn => Some(GameStatus::InProgress(false)),
            ProtoGameStatus::EndWin => Some(GameStatus::Complete(GameResult::Win)),
            ProtoGameStatus::EndLose => Some(GameStatus::Complete(GameResult::Lose)),
            ProtoGameStatus::EndDraw => Some(GameStatus::Complete(GameResult::Draw)),
            ProtoGameStatus::Unmatched => None,
        };

        if proto_game_metadata.game_id.is_empty() {
            return Err(ClientGameError::MalformedResponse(Cow::from("Missing GameId")));
        }
        if proto_game_metadata.host_player_id.is_empty() {
            return Err(ClientGameError::MalformedResponse(Cow::from("Missing HostPlayerId")));
        }
        if let Some(status) = opt_status {
            if proto_game_metadata.guest_player_id.is_empty() {
                return Err(ClientGameError::MalformedResponse(Cow::from("Missing GuestPlayerId")));
            }
            Ok(GameMetadata::new_matched(
                proto_game_metadata.game_id,
                proto_game_metadata.host_player_id,
                proto_game_metadata.created_time_ms,
                proto_game_metadata.guest_player_id,
                status,
            ))
        } else {
            Ok(GameMetadata::new_unmatched(
                proto_game_metadata.game_id,
                proto_game_metadata.host_player_id,
                proto_game_metadata.created_time_ms,
            ))
        }
    }
}

impl TryFrom<ProtoGame> for GameState {
    type Error = ClientGameError;

    fn try_from(proto_game: ProtoGame) -> Result<Self, Self::Error> {
        let my_plays = proto_game.my_plays
            .ok_or(ClientGameError::MalformedResponse(Cow::from("Missing required MyPlays")))?
            .try_into()?;
        let op_plays = proto_game.opponent_plays
            .ok_or(ClientGameError::MalformedResponse(Cow::from("Missing required OpponentPlays")))?
            .try_into()?;
        let (my_score_total, my_score_per_color) = proto_game.my_score
            .ok_or(ClientGameError::MalformedResponse(Cow::from("Missing required MyScore")))?
            .into();
        let (op_score_total, op_score_per_color) = proto_game.op_score
            .ok_or(ClientGameError::MalformedResponse(Cow::from("Missing required OpponentScore")))?
            .into();
        let neutral_board = proto_game.discard_pile
            .ok_or(ClientGameError::MalformedResponse(Cow::from("Missing required DiscardPile")))?
            .try_into()?;

        let game_board = GameBoard::new(
            my_plays,
            op_plays,
            my_score_total,
            op_score_total,
            my_score_per_color,
            op_score_per_color,
            neutral_board,
            proto_game.draw_pile_cards_remaining as usize,
        );

        let mut my_hand = Vec::with_capacity(proto_game.my_hand.len());
        for proto_card in proto_game.my_hand {
            my_hand.push(DecoratedCard::new(
                proto_card.try_into()?,
                // From client-side, allow all plays through to the backend.
                // TODO compute this independently in client engine.
                true,
            ))
        }
        my_hand.sort();
        let my_hand = my_hand;

        let status = match ProtoGameStatus::try_from(proto_game.status)? {
            ProtoGameStatus::NoGameStatus => return Err(ClientGameError::MalformedResponse(Cow::from("Missing game status"))),
            // GetGameState is only for in-progress or completed games.
            ProtoGameStatus::Unmatched => return Err(ClientGameError::GameNotStarted),
            // Valid options
            ProtoGameStatus::YourTurn => GameStatus::InProgress(true),
            ProtoGameStatus::OpponentTurn => GameStatus::InProgress(false),
            ProtoGameStatus::EndWin => GameStatus::Complete(GameResult::Win),
            ProtoGameStatus::EndLose => GameStatus::Complete(GameResult::Lose),
            ProtoGameStatus::EndDraw => GameStatus::Complete(GameResult::Draw),
        };

        Ok(GameState::new(
            game_board,
            my_hand,
            status,
        ))
    }
}

impl TryFrom<ProtoPlayHistory> for HashMap<CardColor, Vec<CardValue>> {
    type Error = ClientGameError;

    fn try_from(proto_play_history: ProtoPlayHistory) -> Result<Self, Self::Error> {
        let mut play_history = HashMap::with_capacity(5);

        // Probably a better way to do this. But the '?' operator inside a closure which also
        // has to account for Some/None makes other method signatures ugly and hard to understand.
        for value_u32 in proto_play_history.red {
            play_history
                .entry(CardColor::Red)
                .or_insert_with(|| Vec::new())
                .push(card_value_from_proto(value_u32)?);
        }
        for value_u32 in proto_play_history.blue {
            play_history
                .entry(CardColor::Blue)
                .or_insert_with(|| Vec::new())
                .push(card_value_from_proto(value_u32)?);
        }
        for value_u32 in proto_play_history.green {
            play_history
                .entry(CardColor::Green)
                .or_insert_with(|| Vec::new())
                .push(card_value_from_proto(value_u32)?);
        }
        for value_u32 in proto_play_history.white {
            play_history
                .entry(CardColor::White)
                .or_insert_with(|| Vec::new())
                .push(card_value_from_proto(value_u32)?);
        }
        for value_u32 in proto_play_history.yellow {
            play_history
                .entry(CardColor::Yellow)
                .or_insert_with(|| Vec::new())
                .push(card_value_from_proto(value_u32)?);
        }

        Ok(play_history)
    }
}

impl From<ProtoScore> for (i32, HashMap<CardColor, i32>) {
    fn from(proto_score: ProtoScore) -> Self {
        let mut score_per_color = HashMap::with_capacity(5);
        score_per_color.insert(CardColor::Red, proto_score.red);
        score_per_color.insert(CardColor::Blue, proto_score.blue);
        score_per_color.insert(CardColor::Green, proto_score.green);
        score_per_color.insert(CardColor::White, proto_score.white);
        score_per_color.insert(CardColor::Yellow, proto_score.yellow);

        (proto_score.total, score_per_color)
    }
}

impl TryFrom<ProtoDiscardPile> for HashMap<CardColor, (CardValue, usize)> {
    type Error = ClientGameError;

    fn try_from(proto_discard_pile: ProtoDiscardPile) -> Result<Self, Self::Error> {
        let mut neutral_board: HashMap<CardColor, (CardValue, usize)> = HashMap::with_capacity(5);

        if let Some(surface) = proto_discard_pile.red {
            neutral_board.insert(CardColor::Red, surface.try_into()?);
        }
        if let Some(surface) = proto_discard_pile.blue {
            neutral_board.insert(CardColor::Blue, surface.try_into()?);
        }
        if let Some(surface) = proto_discard_pile.green {
            neutral_board.insert(CardColor::Green, surface.try_into()?);
        }
        if let Some(surface) = proto_discard_pile.white {
            neutral_board.insert(CardColor::White, surface.try_into()?);
        }
        if let Some(surface) = proto_discard_pile.yellow {
            neutral_board.insert(CardColor::Yellow, surface.try_into()?);
        }

        Ok(neutral_board)
    }
}

impl TryFrom<ProtoDiscardPileSurface> for (CardValue, usize) {
    type Error = ClientGameError;

    fn try_from(proto_discard_surface: ProtoDiscardPileSurface) -> Result<Self, Self::Error> {
        let card_value = card_value_from_proto(proto_discard_surface.value)?;
        let remaining = proto_discard_surface.remaining as usize;

        Ok((card_value, remaining))
    }
}

impl TryFrom<ProtoCard> for Card {
    type Error = ClientGameError;

    fn try_from(proto_card: ProtoCard) -> Result<Self, Self::Error> {
        let color = CardColor::try_from(
            ProtoColor::try_from(proto_card.color)?
        )?;
        let value = card_value_from_proto(proto_card.value)?;

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
