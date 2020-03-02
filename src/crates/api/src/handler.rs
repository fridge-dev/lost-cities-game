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

    fn create_game(&mut self) -> Result<GameMetadata, GameError> {
        let game_metadata = create_game();
        let storage_result = self.storage.create_game_metadata(StorageGameMetadata::new(
            game_metadata.game_id().to_owned(),
            game_metadata.p1_id().to_owned(),
            game_metadata.p2_id().to_owned(),
            GameStatus::Hosted,
        ));

        // TODO implement Into trait so I can utilize that sweet sweet `?` syntax sugar.
        // This entire match could hypothetically be replaced with:
        // `storage_result.map(|_| Ok(game_metadata))?`
        match storage_result {
            Ok(_) => Ok(game_metadata),
            Err(e) => Err(match e {
                StorageError::Internal => GameError::Internal,
                // These two below are outside the control of the caller, and shouldn't be possible.
                StorageError::AlreadyExists => GameError::Internal,
                StorageError::NotFound => GameError::Internal,
            })
        }
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

/// Create game, return game ID
fn create_game() -> GameMetadata {
    GameMetadata::new(
        "g-id".to_owned(),
        "p1p1".to_owned(),
        "p2p2p2".to_owned(),
    )
}
