#![allow(dead_code)]
use game_api::types::{GameState, Play};
use crate::backend_client::BackendClient;
use game_api::backend_errors::BackendGameError;

mod backend_client;

/// Maybe having this as a trait is a little too OOP? this is a learning experiment.
///
/// Generally, all APIs have owned inputs and owned outputs. We'll see how the usage/impl of this works.
///
/// This API is meant to appear to be stateless and multi-tenanted.
pub trait GameApi {

    /// Create a new game with only the host player present.
    /// Returns game_id used for all future queries
    fn host_game(&mut self, p1_id: String) -> Result<String, BackendGameError>;

    /// Player 2 joins the game.
    fn join_game(&mut self, game_id: String, p2_id: String) -> Result<(), BackendGameError>;

    /// Load the state of the game as observed by the requested player.
    fn get_game_state(&self, game_id: String, player_id: String) -> Result<GameState, BackendGameError>;

    /// Make a turn. Should call get_game_state() after this. Maybe not needed? Idk yet.
    fn play_card(&mut self, play: Play) -> Result<(), BackendGameError>;
}

/// Does this mean every call from main to API will incur the cost of a v-lookup table query?
/// Consider removing this interface.
///
/// See:
/// * https://stackoverflow.com/a/27570064
/// * https://stackoverflow.com/questions/28621980/what-are-the-actual-runtime-performance-costs-of-dynamic-dispatch
pub fn new_frontend_game_api() -> Box<dyn GameApi> {
    match BackendClient::new() {
        Ok(client) => Box::new(client),
        Err(e) => panic!(format!("Failed to connect to backend. AAAAAA! {:?}", e)),
    }
}
