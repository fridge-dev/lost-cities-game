// TODO remove these once I get into the thick of things.
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::types::{GameMetadata, GameState, Card, GameError};

pub mod types;

/// Create game, return game ID
pub fn create_game() -> GameMetadata {
    GameMetadata::new(
        "g-id".to_owned(),
        "p1p1".to_owned(),
        "p2p2p2".to_owned(),
    )
}

/// Note to future self: something like this could exist.
pub fn describe_game(game_id: &str) -> GameMetadata {
    GameMetadata::new(
        game_id.to_owned(),
        "p1p1".to_owned(),
        "p2p2p2".to_owned(),
    )
}

pub fn get_game_state(game_id: &str) -> GameState {
    panic!("Not implemented");
}

pub fn play_card(game_id: &str, player_id: &str, card: Card) -> Result<(), GameError> {
    panic!("Not implemented");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
