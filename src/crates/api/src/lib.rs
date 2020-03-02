// TODO remove these once I get into the thick of things.
#![allow(dead_code)]
#![allow(unused_variables)]

use types::{GameError, GameMetadata, GameState, Play};
use crate::handler::GameApiHandler;

mod handler;

/// Maybe this is a little too OOP? this is a learning experiment.
pub trait GameApi {

    /// TODO awesome doc
    fn create_game(&self) -> Result<GameMetadata, GameError>;

    /// TODO awesome doc
    fn describe_game(&self, game_id: &str) -> Result<GameMetadata, GameError>;

    /// TODO awesome doc
    fn get_game_state(&self, game_id: &str) -> Result<GameState, GameError>;

    /// TODO awesome doc
    fn play_card(&self, play: Play) -> Result<(), GameError>;
}

// Does this mean every call from main to API will incur the cost of a v-lookup table query?
// Consider removing this interface. See https://stackoverflow.com/a/27570064.
pub fn new_game_api() -> Box<dyn GameApi> {
    Box::new(GameApiHandler)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
