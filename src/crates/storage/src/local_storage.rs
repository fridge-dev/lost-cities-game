use crate::{GameStore, StorageGameMetadata, StorageError, StorageGameState};

pub struct LocalStore {

}

/// Implements game storage by storing in memory.
impl GameStore for LocalStore {
    fn create_game_metadata(&self, game_metadata: StorageGameMetadata) -> Result<(), StorageError> {
        unimplemented!()
    }

    fn create_game_state(&self, storage_game_state: StorageGameState) -> Result<(), StorageError> {
        unimplemented!()
    }

    fn update_game_state(&self, storage_game_state: StorageGameState) -> Result<(), StorageError> {
        unimplemented!()
    }

    fn load_game_metadata(&self, game_id: &str) -> Result<StorageGameMetadata, StorageError> {
        unimplemented!()
    }

    fn load_game_state(&self, game_id: &str) -> Result<StorageGameState, StorageError> {
        unimplemented!()
    }
}