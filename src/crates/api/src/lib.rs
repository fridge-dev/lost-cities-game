// TODO remove these once I get into the thick of things.
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::types::{GameError, GameMetadata, GameState, Play};

pub mod types;

/// Maybe this is a little too OOP? oh well...
/// But maybe I can move all of the impl logic into another file. that'd be cool!
pub trait GameApi {
    fn create_game(&self) -> Result<GameMetadata, GameError>;
    fn describe_game(&self, game_id: &str) -> Result<GameMetadata, GameError>;
    fn get_game_state(&self, game_id: &str) -> Result<GameState, GameError>;
    fn play_card(&self, play: Play) -> Result<(), GameError>;
}

// Does this mean every call from main to API will incur the cost of a v-lookup table query?
// Consider removing this interface. See https://stackoverflow.com/a/27570064.
pub fn new_api() -> Box<dyn GameApi> {
    Box::new(GameApiHandler)
}

struct GameApiHandler;

impl GameApi for GameApiHandler {
    fn create_game(&self) -> Result<GameMetadata, GameError> {
        Ok(create_game())
    }

    fn describe_game(&self, game_id: &str) -> Result<GameMetadata, GameError> {
        Ok(describe_game(game_id))
    }

    fn get_game_state(&self, game_id: &str) -> Result<GameState, GameError> {
        unimplemented!()
    }

    fn play_card(&self, play: Play) -> Result<(), GameError> {
        unimplemented!()
    }
}

/// Create game, return game ID
fn create_game() -> GameMetadata {
    GameMetadata::new(
        "g-id".to_owned(),
        "p1p1".to_owned(),
        "p2p2p2".to_owned(),
    )
}

/// Note to future self: something like this could exist.
fn describe_game(game_id: &str) -> GameMetadata {
    GameMetadata::new(
        game_id.to_owned(),
        "p1p1".to_owned(),
        "p2p2p2".to_owned(),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
