use crate::backend_error::BackendGameError;
use game_api::types::{GameMetadata, Play, GameState};

/// Short hand type to help impl stuff in this crate.
pub(crate) type GameApiResult<O> = Result<O, BackendGameError>;

/// This is a 1:1 representation of the [GameApi2](/game-api/api/trait.GameApi2.html), but with
/// immutable `&self` as the target (instead of `&mut self`).
#[async_trait::async_trait]
pub trait GameApi2Immut {

    /// Create a new game with only the host player present.
    /// Returns game_id used for all future queries
    async fn host_game(&self, game_id: String, p1_id: String) -> GameApiResult<()>;

    /// Player 2 joins the game.
    async fn join_game(&self, game_id: String, p2_id: String) -> GameApiResult<()>;

    /// Get status of a game
    async fn describe_game(&self, game_id: String) -> GameApiResult<GameMetadata>;

    /// Load the state of the game as observed by the requested player.
    async fn get_game_state(&self, game_id: String, player_id: String) -> GameApiResult<GameState>;

    /// Make a turn. Should call get_game_state() after this. Maybe not needed? Idk yet.
    async fn play_card(&self, play: Play) -> GameApiResult<()>;

    /// Get games that I'm hosting/offering
    async fn query_unmatched_games(&self, player_id: String) -> GameApiResult<Vec<GameMetadata>>;

    /// Get games that I'm currently playing
    async fn query_in_progress_games(&self, player_id: String) -> GameApiResult<Vec<GameMetadata>>;

    /// Get games that I've completed
    async fn query_completed_games(&self, player_id: String) -> GameApiResult<Vec<GameMetadata>>;

    /// Get all (global) unmatched games - aka matchmaking LOL
    async fn query_all_unmatched_games(&self, player_id: String) -> GameApiResult<Vec<GameMetadata>>;
}
