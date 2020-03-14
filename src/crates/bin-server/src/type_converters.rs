use tonic::{Status, Code};
use types::{Play, Card, CardColor, CardValue, CardTarget, DrawPile, GameError, Reason, Cause};
use wire_types::proto_lost_cities::{ProtoHostGameReq, ProtoHostGameReply, ProtoJoinGameReq, ProtoJoinGameReply, ProtoGetGameStateReq, ProtoGetGameStateReply, ProtoPlayCardReq, ProtoPlayCardReply, ProtoPlayTarget, ProtoDrawPile, ProtoCard, ProtoColor};

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

