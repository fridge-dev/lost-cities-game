use crate::GameApi;
use types::{GameError, GameState, Play, GameBoard};
use storage::{GameStore, StorageGameMetadata, GameStatus, StorageError, StorageGameState};
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
            None,
            GameStatus::InProgress,
        ));

        match storage_result {
            Ok(_) => Ok(game_id),
            // This should never fail.
            Err(e) => Err(GameError::Internal)
        }
    }

    fn join_game(&mut self, game_id: &str, p2_id: &str) -> Result<(), GameError> {
        let mut metadata = match self.storage.load_game_metadata(game_id) {
            Ok(v) => v,
            Err(e) => return Err(match e {
                StorageError::NotFound => GameError::NotFound,
                _ => GameError::Internal
            })
        };

        if let Err(e) = metadata.set_p2_id(p2_id.to_string()) {
            return Err(match e {
                StorageError::IllegalModification => GameError::GameAlreadyMatched,
                _ => GameError::Internal,
            });
        }

        self.storage.update_game_metadata(metadata)
            .map_err(|e| match e {
                StorageError::NotFound => GameError::NotFound,
                _ => GameError::Internal
            })

        // TODO start here
//        self.storage.create_game_state(StorageGameState::new(
//
//        ))
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
