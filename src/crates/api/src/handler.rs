use crate::GameApi;
use types::{GameMetadata, GameError, GameState, Play};
use storage::{GameStore, StorageGameMetadata, GameStatus, StorageError};
use storage::local_storage::LocalStore;

pub struct GameApiHandler {
    storage: Box<dyn GameStore>,
}

impl GameApiHandler {
    pub fn new() -> Self {
        GameApiHandler {
            storage: Box::new(LocalStore::new()),
        }
    }
}

impl GameApi for GameApiHandler {

    fn host_game(&mut self, p1_id: &str) -> Result<String, GameError> {
        let game_id = create_game_id();
        let storage_result = self.storage.create_game_metadata(StorageGameMetadata::new(
            game_id.clone(),
            p1_id.to_owned(),
            GameStatus::Hosted,
        ));

        // TODO implement Into trait so I can utilize that sweet sweet `?` syntax sugar.
        // This entire match could hypothetically be replaced with:
        // `storage_result.map(|_| Ok(game_metadata))?`
        match storage_result {
            Ok(_) => Ok(game_id),
            Err(e) => Err(match e {
                StorageError::Internal => GameError::Internal,
                // These two below are outside the control of the caller, and shouldn't be possible.
                StorageError::AlreadyExists => GameError::Internal,
                StorageError::NotFound => GameError::Internal,
                StorageError::IllegalModification => GameError::Internal,
            })
        }
    }

    fn join_game(&mut self, game_id: &str, p2_id: &str) -> Result<(), GameError> {
        unimplemented!()
    }

    fn describe_game(&self, game_id: &str) -> Result<GameMetadata, GameError> {
        unimplemented!()
    }

    fn get_game_state(&self, game_id: &str) -> Result<GameState, GameError> {
        unimplemented!()
    }

    fn play_card(&self, play: Play) -> Result<(), GameError> {
        unimplemented!()
    }
}

fn create_game_id() -> String {
    "g-id".to_owned()
}
