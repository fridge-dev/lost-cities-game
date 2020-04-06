use crate::storage_types::{StorageGameMetadata, StorageError, StorageGameState};
use crate::storage_api::GameStore;
use std::collections::HashMap;

pub struct InMemoryStore {
    // Indexed by game_id
    metadata_map: HashMap<String, StorageGameMetadata>,
    // Indexed by game_id
    state_map: HashMap<String, StorageGameState>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        InMemoryStore {
            metadata_map: HashMap::new(),
            state_map: HashMap::new(),
        }
    }
}

/// Implements game storage by storing in memory.
///
/// All of the read methods return clones of the map's contents. This ensures returned objects can't
/// directly mutate the local store. Mutating the local versions would be a cool optimization, however,
/// this won't accurately reflect the target end state, which is the game's storage being backed by an
/// external DB. So we'll code our interfaces to the target end state and deal with the slight inefficiency
/// in the mean time.
///
/// Read methods return clones of the storage's data, so callers can freely mutate the returned data without
/// corrupting the storage.
impl GameStore for InMemoryStore {

    fn create_game_metadata(&mut self, game_metadata: StorageGameMetadata) -> Result<(), StorageError> {
        if self.metadata_map.contains_key(game_metadata.game_id()) {
            return Err(StorageError::AlreadyExists);
        }

        self.metadata_map.insert(game_metadata.game_id().to_owned(), game_metadata);
        Ok(())
    }

    fn create_game_state(&mut self, game_state: StorageGameState) -> Result<(), StorageError> {
        if self.state_map.contains_key(game_state.game_id()) {
            return Err(StorageError::AlreadyExists);
        }

        self.state_map.insert(game_state.game_id().to_owned(), game_state);
        Ok(())
    }

    fn update_game_metadata(&mut self, game_metadata: StorageGameMetadata) -> Result<(), StorageError> {
        if !self.metadata_map.contains_key(game_metadata.game_id()) {
            return Err(StorageError::NotFound);
        }

        self.metadata_map.insert(game_metadata.game_id().to_owned(), game_metadata);
        Ok(())
    }

    fn update_game_state(&mut self, game_state: StorageGameState) -> Result<(), StorageError> {
        if !self.state_map.contains_key(game_state.game_id()) {
            return Err(StorageError::NotFound);
        }

        self.state_map.insert(game_state.game_id().to_owned(), game_state);
        Ok(())
    }

    fn load_game_metadata(&self, game_id: &str) -> Result<StorageGameMetadata, StorageError> {
        match self.metadata_map.get(game_id) {
            None => Err(StorageError::NotFound),
            Some(game_meta) => Ok((*game_meta).clone()),
        }
    }

    fn load_game_state(&self, game_id: &str) -> Result<StorageGameState, StorageError> {
        match self.state_map.get(game_id) {
            None => Err(StorageError::NotFound),
            Some(game_state) => Ok((*game_state).clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage_types::StorageGameStatus;

    #[test]
    fn create_load_game_metadata() {
        let mut local_store = InMemoryStore::new();

        let metadata = StorageGameMetadata::new(
            "game-123".to_owned(),
            "p1".to_owned(),
            None,
            StorageGameStatus::InProgress
        );

        assert_eq!(
            local_store.load_game_metadata(metadata.game_id()).err().unwrap(),
            StorageError::NotFound
        );
        assert_eq!(
            local_store.create_game_metadata(metadata.clone()).ok().unwrap(),
            ()
        );
        assert_eq!(
            local_store.load_game_metadata(metadata.game_id()).ok().unwrap(),
            metadata
        );
        assert_eq!(
            local_store.create_game_metadata(metadata.clone()).err().unwrap(),
            StorageError::AlreadyExists
        );
    }

    #[test]
    fn update_game_metadata() {
        let mut local_store = InMemoryStore::new();

        let metadata = StorageGameMetadata::new(
            "game-123".to_owned(),
            "p1".to_owned(),
            None,
            StorageGameStatus::InProgress
        );

        assert_eq!(
            local_store.update_game_metadata(metadata.clone()).err().unwrap(),
            StorageError::NotFound
        );
        assert_eq!(
            local_store.create_game_metadata(metadata.clone()).ok().unwrap(),
            ()
        );
        let mut metadata2 = metadata.clone();
        metadata2.set_p2_id("p2p2".to_owned());
        assert_eq!(
            local_store.update_game_metadata(metadata2.clone()).ok().unwrap(),
            ()
        );
        assert_eq!(
            local_store.update_game_metadata(metadata2.clone()).ok().unwrap(),
            ()
        );
        assert_eq!(
            local_store.load_game_metadata(metadata.game_id()).ok().unwrap(),
            metadata2
        );
        assert_ne!(metadata, metadata2);
    }

    #[test]
    fn create_load_game_state() {
        let mut local_store = InMemoryStore::new();

        let game_state = StorageGameState::new(
            "game-123".to_owned(),
            Vec::new(),
            Vec::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            Vec::new(),
            true
        );

        assert_eq!(
            local_store.load_game_state(game_state.game_id()).err().unwrap(),
            StorageError::NotFound
        );
        assert_eq!(
            local_store.create_game_state(game_state.clone()).ok().unwrap(),
            ()
        );
        assert_eq!(
            local_store.load_game_state(game_state.game_id()).ok().unwrap(),
            game_state
        );
        assert_eq!(
            local_store.create_game_state(game_state.clone()).err().unwrap(),
            StorageError::AlreadyExists
        );
    }

    #[test]
    fn update_game_state() {
        let mut local_store = InMemoryStore::new();

        let game_state = StorageGameState::new(
            "game-123".to_owned(),
            Vec::new(),
            Vec::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            Vec::new(),
            true
        );

        assert_eq!(
            local_store.update_game_state(game_state.clone()).err().unwrap(),
            StorageError::NotFound
        );
        assert_eq!(
            local_store.create_game_state(game_state.clone()).ok().unwrap(),
            ()
        );
        let updated_game_state = StorageGameState::new(
            "game-123".to_owned(),
            Vec::new(),
            Vec::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            Vec::new(),
            false
        );
        assert_eq!(
            local_store.update_game_state(updated_game_state.clone()).ok().unwrap(),
            ()
        );
        assert_eq!(
            local_store.update_game_state(updated_game_state.clone()).ok().unwrap(),
            ()
        );
        assert_eq!(
            local_store.load_game_state(game_state.game_id()).ok().unwrap(),
            updated_game_state
        );
        assert_ne!(game_state, updated_game_state);
    }
}
