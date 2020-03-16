use std::error::Error;
use crate::types::{GameState, Play};

/// The application layer API for the game.
///
/// Result::Err type can be different for server and client.
///
/// This API is meant to appear to be stateless and multi-tenanted.
#[async_trait::async_trait]
pub trait GameApi2<E: Error + Send + Sync + 'static> {

    /// Create a new game with only the host player present.
    /// Returns game_id used for all future queries
    async fn host_game(&mut self, p1_id: String) -> Result<String, E>;

    /// Player 2 joins the game.
    async fn join_game(&mut self, game_id: String, p2_id: String) -> Result<(), E>;

    /// Load the state of the game as observed by the requested player.
    async fn get_game_state(&mut self, game_id: String, player_id: String) -> Result<GameState, E>;

    /// Make a turn. Should call get_game_state() after this. Maybe not needed? Idk yet.
    async fn play_card(&mut self, play: Play) -> Result<(), E>;

}