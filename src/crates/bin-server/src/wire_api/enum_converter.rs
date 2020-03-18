use crate::wire_api::proto_lost_cities::{
    ProtoColor, ProtoDrawPile, ProtoGameStatus, ProtoPlayTarget,
};
/// All enums need a convert method like this because prost generated a `from_i32` method via macros
/// which doesn't actually exist in my IDE. Maybe I'm being too IDE dependent, but I hate stuff like
/// this. So I will create explicit methods and contain the "dark magic" within these small methods.
///
/// See https://github.com/danburkert/prost/issues/69
use std::convert::TryFrom;
use tonic::{Code, Status};

impl TryFrom<i32> for ProtoColor {
    type Error = Status;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        ProtoColor::from_i32(value).ok_or(Status::new(
            Code::InvalidArgument,
            format!("Illegal Color i32 value '{}'", value),
        ))
    }
}

impl TryFrom<i32> for ProtoPlayTarget {
    type Error = Status;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        ProtoPlayTarget::from_i32(value).ok_or(Status::new(
            Code::InvalidArgument,
            format!("Illegal Color i32 value '{}'", value),
        ))
    }
}

impl TryFrom<i32> for ProtoDrawPile {
    type Error = Status;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        ProtoDrawPile::from_i32(value).ok_or(Status::new(
            Code::InvalidArgument,
            format!("Illegal Color i32 value '{}'", value),
        ))
    }
}

impl TryFrom<i32> for ProtoGameStatus {
    type Error = Status;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        ProtoGameStatus::from_i32(value).ok_or(Status::new(
            Code::InvalidArgument,
            format!("Illegal Color i32 value '{}'", value),
        ))
    }
}
