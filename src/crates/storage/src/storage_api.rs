use crate::storage_types::{StorageGameMetadata, StorageError, StorageGameState};

/// A GameStore is the storage layer of the game engine. It is responsible for durably persisting the state
/// of the game. It is NOT responsible for applying rules of the game to the provided game board state.
///
/// Planning on the following 4 implementations (via Iterative Development™)
///
/// **When game is run as a single process**
/// 1. ProcessLocalStore - In-memory storage for a game (2 clients & 1 server) run on a single process
///
/// **When game is run as 3 separate processes** (1 server and 2 clients)
/// 2. NetworkStore - For client processes, this is external storage accessible via network with the same interface (gRPC-like)
/// 3. ServerLocalStore - For server process, an in-memory (or disk) storage
/// 4. ServerDbStore - For server process, a storage backed by an external DB
///
/// For future proofing, there should be one method per-table per-access-pattern.
///
/// Methods grouped by classic CURD pattern.
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
