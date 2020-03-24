use std::error::Error;
use crate::types::{GameState, Play, GameMetadata};

/// The application layer API for the game.
///
/// Result::Err type can be different for server and client.
///
/// This API is meant to appear to be stateless and multi-tenanted.
#[async_trait::async_trait]
pub trait GameApi2<E: Error + Send + Sync + 'static> {

    /// Create a new game with only the host player present.
    /// Returns game_id used for all future queries
    async fn host_game(&mut self, game_id: String, p1_id: String) -> Result<(), E>;

    /// Player 2 joins the game.
    async fn join_game(&mut self, game_id: String, p2_id: String) -> Result<(), E>;

    /// Get status of a game
    async fn describe_game(&mut self, game_id: String) -> Result<GameMetadata, E>;

    /// Get games that I'm hosting/offering
    async fn query_unmatched_games(&mut self, player_id: String) -> Result<Vec<GameMetadata>, E>;

    /// Get games that I'm currently playing
    async fn query_in_progress_games(&mut self, player_id: String) -> Result<Vec<GameMetadata>, E>;

    /// Get games that I've completed
    async fn query_completed_games(&mut self, player_id: String) -> Result<Vec<GameMetadata>, E>;

    /// Get all (global) unmatched games - aka matchmaking LOL
    async fn query_all_unmatched_games(&mut self, player_id: String) -> Result<Vec<GameMetadata>, E>;

    /// Load the state of the game as observed by the requested player.
    async fn get_game_state(&mut self, game_id: String, player_id: String) -> Result<GameState, E>;

    /// Make a turn. Should call get_game_state() after this. Maybe not needed? Idk yet.
    async fn play_card(&mut self, play: Play) -> Result<(), E>;

}