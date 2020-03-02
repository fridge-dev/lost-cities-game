use crate::GameApi;
use types::{GameMetadata, GameError, GameState, Play};

pub struct GameApiHandler;

impl GameApi for GameApiHandler {
    fn create_game(&self) -> Result<GameMetadata, GameError> {
        Ok(create_game())
    }

    fn describe_game(&self, game_id: &str) -> Result<GameMetadata, GameError> {
        Ok(describe_game(game_id))
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

/// Note to future self: something like this could exist.
fn describe_game(game_id: &str) -> GameMetadata {
    GameMetadata::new(
        game_id.to_owned(),
        "p1p1".to_owned(),
        "p2p2p2".to_owned(),
    )
}