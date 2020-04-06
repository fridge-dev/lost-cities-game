use crate::storage_types::{StorageGameMetadata, StorageError, StorageGameState};

/// A GameStore is the storage layer of the game engine. It is responsible for durably persisting the state
/// of the game. It is NOT responsible for applying rules of the game to the provided game board state.
///
/// For future proofing, there should be one method per-table per-access-pattern. Methods grouped by
/// classic CURD pattern.
///
/// This trait uses `&mut self` and non-`async` for all methods. It was initially designed as an
/// in-memory cache. Since it's not possible to share a storage client across threads as mut
/// (without mutex, gross), this trait should either be considered deprecated or this should be
/// renamed to LocalCache.
///
/// You should instead see `GameDatabase` trait in the v2 API.
pub trait GameStore {

    // C
    fn create_game_metadata(&mut self, game_metadata: StorageGameMetadata) -> Result<(), StorageError>;
    fn create_game_state(&mut self, storage_game_state: StorageGameState) -> Result<(), StorageError>;

    // U
    fn update_game_metadata(&mut self, game_metadata: StorageGameMetadata) -> Result<(), StorageError>;
    fn update_game_state(&mut self, storage_game_state: StorageGameState) -> Result<(), StorageError>;

    // R
    fn load_game_metadata(&self, game_id: &str) -> Result<StorageGameMetadata, StorageError>;
    fn load_game_state(&self, game_id: &str) -> Result<StorageGameState, StorageError>;

    // D
    // none yet
}
