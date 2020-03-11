/// All enums need a convert method like this because prost generated a `from_i32` method via macros
/// which doesn't actually exist in my IDE. Maybe I'm being too IDE dependent, but I hate stuff like
/// this. So I will create explicit methods and contain the "dark magic" within these small methods.
///
/// See https://github.com/danburkert/prost/issues/69
use crate::proto_lost_cities::{
    ProtoColor,
    ProtoPlayTarget,
    ProtoDrawPile,
    ProtoGameStatus
};

impl ProtoColor {
    pub fn convert_i32(val: i32) -> Option<ProtoColor> {
        ProtoColor::from_i32(val)
    }
}

impl ProtoPlayTarget {
    pub fn convert_i32(val: i32) -> Option<ProtoPlayTarget> {
        ProtoPlayTarget::from_i32(val)
    }
}

impl ProtoDrawPile {
    pub fn convert_i32(val: i32) -> Option<ProtoDrawPile> {
        ProtoDrawPile::from_i32(val)
    }
}

impl ProtoGameStatus {
    pub fn convert_i32(val: i32) -> Option<ProtoGameStatus> {
        ProtoGameStatus::from_i32(val)
    }
}
