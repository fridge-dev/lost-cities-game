use crate::{GameStore, StorageGameMetadata, StorageError, StorageGameState};
use std::collections::HashMap;

pub struct LocalStore {
    // Indexed by game_id
    metadata_map: HashMap<String, StorageGameMetadata>,
    // Indexed by game_id
    state_map: HashMap<String, StorageGameState>,
}

impl LocalStore {
    pub fn new() -> Self {
        LocalStore {
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
impl GameStore for LocalStore {

    fn create_game_metadata(&mut self, game_metadata: StorageGameMetadata) -> Result<(), StorageError> {
        if self.metadata_map.contains_key(&game_metadata.game_id) {
            return Err(StorageError::AlreadyExists);
        }

        self.metadata_map.insert(game_metadata.game_id.clone(), game_metadata);
        Ok(())
    }

    fn create_game_state(&mut self, storage_game_state: StorageGameState) -> Result<(), StorageError> {
        unimplemented!()
    }

    fn update_game_metadata(&mut self, game_metadata: StorageGameMetadata) -> Result<(), StorageError> {
        if !self.metadata_map.contains_key(&game_metadata.game_id) {
            return Err(StorageError::NotFound);
        }

        self.metadata_map.insert(game_metadata.game_id.clone(), game_metadata);
        Ok(())
    }

    fn update_game_state(&mut self, storage_game_state: StorageGameState) -> Result<(), StorageError> {
        unimplemented!()
    }

    fn load_game_metadata(&self, game_id: &str) -> Result<StorageGameMetadata, StorageError> {
        match self.metadata_map.get(game_id) {
            None => Err(StorageError::NotFound),
            Some(game_meta) => Ok((*game_meta).clone()),
        }
    }

    fn load_game_state(&self, game_id: &str) -> Result<StorageGameState, StorageError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GameStatus;

    #[test]
    fn create_load_game_metadata() {
        let mut local_store = LocalStore::new();

        let metadata = StorageGameMetadata::new(
            "game-123".to_owned(),
            "p1".to_owned(),
            None,
            GameStatus::InProgress
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
        let mut local_store = LocalStore::new();

        let mut metadata = StorageGameMetadata::new(
            "game-123".to_owned(),
            "p1".to_owned(),
            None,
            GameStatus::InProgress
        );

        assert_eq!(
            local_store.update_game_metadata(metadata.clone()).err().unwrap(),
            StorageError::NotFound
        );
        assert_eq!(
            local_store.create_game_metadata(metadata.clone()).ok().unwrap(),
            ()
        );
        metadata.set_p2_id("p2p2".to_owned());
        assert_eq!(
            local_store.update_game_metadata(metadata.clone()).ok().unwrap(),
            ()
        );
        assert_eq!(
            local_store.update_game_metadata(metadata.clone()).ok().unwrap(),
            ()
        );
        assert_eq!(
            local_store.load_game_metadata(metadata.game_id()).ok().unwrap(),
            metadata
        );
    }
}
