use crate::GameApi;
use types::{GameError, GameState, Play, Card, Cause, GameBoard};
use storage::storage_types::{StorageGameMetadata, GameStatus, StorageError, StorageGameState};
use storage::storage_api::GameStore;
use storage::local_storage::LocalStore;
use rules::deck::DeckFactory;
use std::collections::HashMap;

pub struct GameApiHandler {
    storage: Box<dyn GameStore>,
    deck_factory: DeckFactory,
}

impl GameApiHandler {
    pub fn new() -> Self {
        GameApiHandler {
            storage: Box::new(LocalStore::new()),
            deck_factory: DeckFactory::new(),
        }
    }

    fn update_game_metadata(&mut self, game_id: &str, p2_id: &str) -> Result<(), GameError> {
        let mut metadata = self.storage.load_game_metadata(game_id)
            .map_err(|e| match e {
                StorageError::NotFound => GameError::NotFound("Game metadata"),
                _ => GameError::Internal(Cause::Storage("Failed to load game metadata", Box::new(e)))
            })?;
        metadata.set_p2_id(p2_id.to_string())
            .map_err(|e| match e {
                StorageError::IllegalModification => GameError::GameAlreadyMatched,
                _ => GameError::Internal(Cause::Storage("Failed to mutate game metadata", Box::new(e))),
            })?;
        self.storage.update_game_metadata(metadata)
            .map_err(|e| match e {
                StorageError::NotFound => GameError::NotFound("Game metadata"),
                _ => GameError::Internal(Cause::Storage("Failed to save game metadata", Box::new(e)))
            })
    }

    fn create_initial_game_state(&mut self, game_id: &str) -> Result<(), GameError> {
        let mut deck = self.deck_factory.new_shuffled_deck();

        let mut p1_hand: Vec<Card> = Vec::with_capacity(8);
        let mut p2_hand: Vec<Card> = Vec::with_capacity(8);
        for _ in 0..8 {
            p1_hand.push(deck.pop().unwrap());
            p2_hand.push(deck.pop().unwrap());
        }

        let game_state = StorageGameState::new(
            game_id.to_owned(),
            p1_hand,
            p2_hand,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            deck,
            is_first_turn_p1(),
        );

        self.storage.create_game_state(game_state)
            .map_err(|e| GameError::Internal(Cause::Storage("Failed to save initial game state", Box::new(e))))
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
            Err(e) => Err(GameError::Internal(Cause::Storage("Failed to list game as hosted.", Box::new(e))))
        }
    }

    fn join_game(&mut self, game_id: &str, p2_id: &str) -> Result<(), GameError> {
        self.update_game_metadata(game_id, p2_id)?;
        self.create_initial_game_state(game_id)
    }

    fn get_game_state(&self, game_id: &str, player_id: &str) -> Result<GameState, GameError> {
        let metadata = self.storage.load_game_metadata(game_id)
            .map_err(|e| match e {
                StorageError::NotFound => GameError::NotFound("Game metadata"),
                _ => GameError::Internal(Cause::Storage("Failed to load game", Box::new(e)))
            })?;

        let is_player_1 = if player_id == metadata.p1_id() {
            true
        } else if player_id == metadata.p2_id() {
            false
        } else {
            return Err(GameError::NotFound("Player in game"));
        };

        let storage_game_state = self.storage.load_game_state(game_id)
            .map_err(|e| match e {
                StorageError::NotFound => GameError::NotFound("Game state"),
                _ => GameError::Internal(Cause::Storage("Failed to load game state.", Box::new(e))),
            })?;

        println!("DEBUG: Loaded game state: {:?}", storage_game_state);

        // Expensive cloning incoming... :P

        // Here is where we only show what the player is allowed to see.
        // TODO use storage_game_state.neutral_draw_pile()
        let concealed_neutral_draw_pile = HashMap::new();

        let game_board = GameBoard::new(
            storage_game_state.p1_plays().to_owned(),
            storage_game_state.p2_plays().to_owned(),
            // TODO implement scoring
            0,
            0,
            concealed_neutral_draw_pile,
            storage_game_state.draw_pile_cards_remaining().len(),
        );

        let (my_hand, is_my_turn) = if is_player_1 {
            (storage_game_state.p1_hand(), *storage_game_state.p1_turn())
        } else {
            (storage_game_state.p2_hand(), !*storage_game_state.p1_turn())
        };

        let game_state = GameState::new(
            game_board,
            my_hand.to_owned(),
            is_my_turn
        );

        return Ok(game_state);
    }

    fn play_card(&self, play: Play) -> Result<(), GameError> {
        unimplemented!()
    }
}

fn create_game_id() -> String {
    // random hex string
    format!("{:x}", rand::random::<u128>())
}

fn is_first_turn_p1() -> bool {
    rand::random()
}
