/// All enums need a convert method like this because prost generated a `from_i32` method via macros
/// which doesn't actually exist in my IDE. Maybe I'm being too IDE dependent, but I hate stuff like
/// this. So I will create explicit methods and contain the "dark magic" within these small methods.
///
/// See https://github.com/danburkert/prost/issues/69
use crate::proto_lost_cities::{ProtoColor, ProtoDrawPile, ProtoGameStatus, ProtoPlayTarget};
use std::convert::TryFrom;

impl TryFrom<i32> for ProtoColor {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        ProtoColor::from_i32(value)
            .ok_or(format!("Illegal Color i32 value '{}'", value))
    }
}

impl TryFrom<i32> for ProtoPlayTarget {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        ProtoPlayTarget::from_i32(value)
            .ok_or(format!("Illegal Color i32 value '{}'", value))
    }
}

impl TryFrom<i32> for ProtoDrawPile {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        ProtoDrawPile::from_i32(value)
            .ok_or(format!("Illegal Color i32 value '{}'", value))
    }
}

impl TryFrom<i32> for ProtoGameStatus {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        ProtoGameStatus::from_i32(value)
            .ok_or(format!("Illegal Color i32 value '{}'", value))
    }
}
