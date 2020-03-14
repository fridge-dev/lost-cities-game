use std::collections::HashMap;
use tonic::{Status, Code};
use types::{
    Play,
    Card,
    CardColor,
    CardValue,
    CardTarget,
    DrawPile,
    GameError,
    Reason,
    Cause,
    GameState,
    GameBoard,
    GameStatus,
    GameResult,
    DecoratedCard
};
use wire_types::proto_lost_cities::{
    ProtoHostGameReq,
    ProtoHostGameReply,
    ProtoJoinGameReq,
    ProtoJoinGameReply,
    ProtoGetGameStateReq,
    ProtoGetGameStateReply,
    ProtoPlayCardReq,
    ProtoPlayCardReply,
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

/// Namespace all public methods to this single struct.
/// Idk if this is a good pattern; we'll see.
pub struct WireTypeConverter;

impl WireTypeConverter {

    pub fn convert_host_game_req(req: ProtoHostGameReq) -> Result<String, Status> {
        if req.player_id.is_empty() {
            return Err(Status::new(Code::InvalidArgument, "Missing PlayedId"));
        }

        Ok(req.player_id)
    }

    pub fn convert_join_game_req(req: ProtoJoinGameReq) -> Result<(String, String), Status> {
        if req.game_id.is_empty() {
            return Err(Status::new(Code::InvalidArgument, "Missing GameId"));
        }
        if req.player_id.is_empty() {
            return Err(Status::new(Code::InvalidArgument, "Missing PlayerId"));
        }

        Ok((req.game_id, req.player_id))
    }

    pub fn convert_get_game_state_req(req: ProtoGetGameStateReq) -> Result<(String, String), Status> {
        if req.game_id.is_empty() {
            return Err(Status::new(Code::InvalidArgument, "Missing GameId"));
        }
        if req.player_id.is_empty() {
            return Err(Status::new(Code::InvalidArgument, "Missing PlayerId"));
        }

        Ok((req.game_id, req.player_id))
    }

    pub fn convert_game_state(game_state: GameState) -> Result<ProtoGetGameStateReply, Status> {
        game_state.try_into_proto()
    }

    pub fn convert_play_card_req(req: ProtoPlayCardReq) -> Result<Play, Status> {
        Ok(Play::try_from_proto(req)?)
    }

    pub fn convert_error(game_error: GameError) -> Status {
        match game_error {
            GameError::NotFound(resource) => Status::new(Code::NotFound, format!("Resource {} not found.", resource)),
            GameError::GameAlreadyMatched => Status::new(Code::AlreadyExists, format!("The game you attempted to join is full.")),
            GameError::InvalidPlay(reason) => Status::new(Code::InvalidArgument, format!("Can't play card. {}", reason)),
            GameError::Internal(cause) => {
                println!("ERROR: Internal failure caused by '{:?}'", cause);
                Status::new(Code::Internal, format!("Internal server failure"))
            },
        }
    }

}

// ================ Generics related to all protobuf types ======================

/// `T`: application layer type
type InputResult<T> = Result<T, InvalidInput>;

struct InvalidInput {
    message: String,
}

trait TryFromProto<P> where Self : Sized {
    fn try_from_proto(proto_type: P) -> InputResult<Self>;
}

impl From<&str> for InvalidInput {
    fn from(message: &str) -> Self {
        InvalidInput {
            message: message.to_owned()
        }
    }
}

impl From<InvalidInput> for Status {
    fn from(invalid_input: InvalidInput) -> Self {
        Status::new(Code::InvalidArgument, invalid_input.message)
    }
}

// ================ Implementations of TryFromProto trait ======================
// Note to future self: If I would move the generated prototypes into this crate,
// I could define TryFrom/TryInto on the types directly. The current "from" and
// "into" verbage would have to swap.

impl TryFromProto<ProtoCard> for Card {
    fn try_from_proto(proto_card: ProtoCard) -> Result<Self, InvalidInput> {
        Ok(Card::new(
            CardColor::try_from_proto(proto_card.color)?,
            CardValue::try_from_proto(proto_card.value)?,
        ))
    }
}

impl TryFromProto<i32> for CardColor {
    fn try_from_proto(proto_card_color: i32) -> Result<Self, InvalidInput> {
        Ok(match ProtoColor::convert_i32(proto_card_color) {
            None => return Err("Missing Color")?,
            Some(proto_color) => match proto_color {
                ProtoColor::NoColor => return Err("Unspecified Color")?,
                ProtoColor::Red => CardColor::Red,
                ProtoColor::Green => CardColor::Green,
                ProtoColor::White => CardColor::White,
                ProtoColor::Blue => CardColor::Blue,
                ProtoColor::Yellow => CardColor::Yellow,
            }
        })
    }
}

impl TryFromProto<u32> for CardValue {
    fn try_from_proto(proto_card_value: u32) -> Result<Self, InvalidInput> {
        Ok(match proto_card_value {
            1 => CardValue::Wager,
            2 => CardValue::Two,
            3 => CardValue::Three,
            4 => CardValue::Four,
            5 => CardValue::Five,
            6 => CardValue::Six,
            7 => CardValue::Seven,
            8 => CardValue::Eight,
            9 => CardValue::Nine,
            10 => CardValue::Ten,
            0 => return Err("Missing CardValue")?,
            _ => return Err("Invalid CardValue")?,
        })
    }
}

impl TryFromProto<ProtoPlayCardReq> for Play {
    fn try_from_proto(req: ProtoPlayCardReq) -> Result<Self, InvalidInput> {
        if req.game_id.is_empty() {
            return Err("Missing GameId")?;
        }
        if req.player_id.is_empty() {
            return Err("Missing PlayerId")?;
        }
        let card: Card = match req.card {
            None => return Err("Missing Card")?,
            Some(proto_card) => Card::try_from_proto(proto_card)?
        };
        let card_target: CardTarget = match ProtoPlayTarget::convert_i32(req.target) {
            None => return Err("Missing PlayTarget")?,
            Some(proto_play_target) => match proto_play_target {
                ProtoPlayTarget::NoPlayTarget => return Err("Unspecified PlayTarget")?,
                ProtoPlayTarget::PlayerBoard => CardTarget::Player,
                ProtoPlayTarget::Discard => CardTarget::Neutral,
            }
        };
        let draw_pile: DrawPile = match ProtoDrawPile::convert_i32(req.draw_pile) {
            None => return Err("Missing DrawPile")?,
            Some(proto_draw_pile) => match proto_draw_pile {
                ProtoDrawPile::NoDrawPile => return Err("Unspecified DrawPile")?,
                ProtoDrawPile::MainDraw => DrawPile::Main,
                ProtoDrawPile::DiscardDraw => DrawPile::Neutral(CardColor::try_from_proto(req.discard_draw_color)?),
            }
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

// =========================== Reply converters ================================

trait TryIntoProto<P> where Self : Sized {
    fn try_into_proto(self) -> Result<P, Status>;
}
trait IntoProto<P> where Self : Sized {
    fn into_proto(self) -> P;
}

impl IntoProto<ProtoCard> for Card {
    fn into_proto(self) -> ProtoCard {
        ProtoCard {
            color: self.card_color().into_proto(),
            value: self.card_value().into_proto(),
        }
    }
}

impl IntoProto<i32> for CardColor {
    fn into_proto(self) -> i32 {
        let proto_color: ProtoColor = match self {
            CardColor::Red => ProtoColor::Red,
            CardColor::Green => ProtoColor::Green,
            CardColor::White => ProtoColor::White,
            CardColor::Blue => ProtoColor::Blue,
            CardColor::Yellow => ProtoColor::Yellow,
        };

        proto_color as i32
    }
}

impl IntoProto<u32> for CardValue {
    fn into_proto(self) -> u32 {
        match self {
            CardValue::Wager => 1,
            CardValue::Two => 2,
            CardValue::Three => 3,
            CardValue::Four => 4,
            CardValue::Five => 5,
            CardValue::Six => 6,
            CardValue::Seven => 7,
            CardValue::Eight => 8,
            CardValue::Nine => 9,
            CardValue::Ten => 10,
        }
    }
}

// Now I'm getting lazy, implementing From/Into...
impl TryIntoProto<ProtoGetGameStateReply> for GameState {
    fn try_into_proto(self) -> Result<ProtoGetGameStateReply, Status> {
        let proto_game = ProtoGame {
            my_hand: convert_hand(self.my_hand()),
            my_plays: Some(convert_plays(self.game_board().my_plays())),
            opponent_plays: Some(convert_plays(self.game_board().op_plays())),
            discard_pile: Some(convert_discard_pile(self.game_board().neutral_draw_pile())),
            draw_pile_cards_remaining: *self.game_board().draw_pile_cards_remaining() as u32,
            status: convert_game_status(self.status(), *self.is_my_turn()) as i32,
            my_score: *self.game_board().my_score(),
            op_score: *self.game_board().op_score(),
        };

        Ok(ProtoGetGameStateReply {
            game: Some(proto_game),
            opponent_player_id: "TODO".to_string()
        })
    }
}

fn convert_hand(hand: &Vec<DecoratedCard>) -> Vec<ProtoCard> {
    hand.iter()
        .map(|card| card.card().into_proto())
        .collect()
}

fn convert_plays(plays: &HashMap<CardColor, Vec<CardValue>>) -> ProtoPlayHistory {
    let inner_converter: Fn(CardColor) -> Vec<u32> = |color| {
        plays.get(&color)
            .map(|values| convert_card_value_vec(values))
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

fn convert_card_value_vec(card_values: &Vec<CardValue>) -> Vec<u32> {
    card_values
        .iter()
        .map(|card_value| card_value.into_proto())
        .collect()
}

fn convert_discard_pile(neutral_draw_pile: &HashMap<CardColor, (CardValue, usize)>) -> ProtoDiscardPile {
    ProtoDiscardPile {
        red: neutral_draw_pile.get(&CardColor::Red).map(|(value, num_cards)| convert_discard_pile_surface(*value, *num_cards)),
        green: neutral_draw_pile.get(&CardColor::Green).map(|(value, num_cards)| convert_discard_pile_surface(*value, *num_cards)),
        white: neutral_draw_pile.get(&CardColor::White).map(|(value, num_cards)| convert_discard_pile_surface(*value, *num_cards)),
        blue: neutral_draw_pile.get(&CardColor::Blue).map(|(value, num_cards)| convert_discard_pile_surface(*value, *num_cards)),
        yellow: neutral_draw_pile.get(&CardColor::Yellow).map(|(value, num_cards)| convert_discard_pile_surface(*value, *num_cards)),
    }
}

fn convert_discard_pile_surface(value: CardValue, num_cards: usize) -> ProtoDiscardPileSurface {
    ProtoDiscardPileSurface {
        value: value.into_proto(),
        remaining: num_cards as u32,
    }
}

fn convert_game_status(status: &GameStatus, is_your_turn: bool) -> ProtoGameStatus {
    match status {
        GameStatus::InProgress => if is_your_turn {
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