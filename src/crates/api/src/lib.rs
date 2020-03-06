use types::{GameError, GameState, Play};
use crate::handler::GameApiHandler;

mod handler;

/// Maybe having this as a trait is a little too OOP? this is a learning experiment.
///
/// Generally, all APIs have borrow inputs and owned outputs. We'll see how the usage/impl of this works.
///
/// This API is meant to appear to be stateless and multi-tenanted.
pub trait GameApi {

    /// Create a new game with only the host player present.
    /// Returns game_id used for all future queries
    fn host_game(&mut self, p1_id: &str) -> Result<String, GameError>;

    /// Player 2 joins the game.
    fn join_game(&mut self, game_id: &str, p2_id: &str) -> Result<(), GameError>;

    /// Load the state of the game as observed by the requested player.
    fn get_game_state(&self, game_id: &str, player_id: &str) -> Result<GameState, GameError>;

    /// Make a turn. Should call get_game_state() after this. Maybe not needed? Idk yet.
    fn play_card(&self, play: Play) -> Result<(), GameError>;
}

// Does this mean every call from main to API will incur the cost of a v-lookup table query?
// Consider removing this interface. See https://stackoverflow.com/a/27570064.
pub fn new_game_api() -> Box<dyn GameApi> {
    Box::new(GameApiHandler::new())
}
