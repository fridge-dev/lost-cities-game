use crate::v2::db_types::{DbGameSummary, DbError, DbGameData};

pub type DbResult<T> = Result<T, DbError>;

/// A GameStore is the storage layer of the game engine. It is responsible for durably persisting the state
/// of the game. It is NOT responsible for applying rules of the game to the provided game board state.
///
/// For future proofing, there should be one method per-table per-access-pattern. Methods grouped by
/// classic CURD pattern.
///
/// In the future, we may need atomic-transactional APIs, however, this won't exist in a noSQL world,
/// so it should be carefully decided for.
#[async_trait::async_trait]
pub trait GameDatabase {

    // C
    async fn create_game_summary(&self, game_summary: DbGameSummary) -> DbResult<()>;
    async fn create_game_data(&self, game_data: DbGameData) -> DbResult<()>;

    // U
    async fn update_game_summary(&self, game_summary: DbGameSummary) -> DbResult<()>;
    async fn update_game_data(&self, game_data: DbGameData) -> DbResult<()>;

    // R
    async fn load_game_summary(&self, game_id: String) -> DbResult<DbGameSummary>;
    async fn load_game_data(&self, game_id: String) -> DbResult<DbGameData>;

    // D
    // none yet
}
