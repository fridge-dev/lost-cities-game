// TODO remove these once I get into the thick of things.
#![allow(dead_code)]
#![allow(unused_variables)]

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use core::fmt;
use types::{Card, CardColor, CardValue}; // This is a broken layer of abstraction. But I'm sick of re-writing the same types for now. I'm trying to learn rust!
use std::collections::HashMap;

pub mod local_storage;

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
    fn create_game_metadata(&self, game_metadata: StorageGameMetadata) -> Result<(), StorageError>;
    fn create_game_state(&self, storage_game_state: StorageGameState) -> Result<(), StorageError>;

    // U
    fn update_game_state(&self, storage_game_state: StorageGameState) -> Result<(), StorageError>;

    // R
    fn load_game_metadata(&self, game_id: &str) -> Result<StorageGameMetadata, StorageError>;
    fn load_game_state(&self, game_id: &str) -> Result<StorageGameState, StorageError>;

    // D
    // none yet
}

pub struct StorageGameMetadata {
    game_id: String,
    p1_id: String,
    p2_id: String,
    game_status: GameStatus,
}

impl StorageGameMetadata {
    pub fn new(game_id: String, p1_id: String, p2_id: String, game_status: GameStatus) -> Self {
        StorageGameMetadata {
            game_id,
            p1_id,
            p2_id,
            game_status,
        }
    }
    pub fn game_id(&self) -> &str {
        &self.game_id
    }
    pub fn p1_id(&self) -> &str {
        &self.p1_id
    }
    pub fn p2_id(&self) -> &str {
        &self.p2_id
    }
    pub fn game_status(&self) -> &GameStatus {
        &self.game_status
    }
}

pub enum GameStatus {
    Hosted,
    InProgress,
    Completed,
}

pub struct StorageGameState {
    metadata: StorageGameMetadata,

    p1_hand: [Card; 8],
    p2_hand: [Card; 8],

    p1_plays: HashMap<CardColor, Vec<CardValue>>,
    p2_plays: HashMap<CardColor, Vec<CardValue>>,

    neutral_draw_pile: HashMap<CardColor, Vec<CardValue>>,
    draw_pile_cards_remaining: Vec<Card>,

    p1_turn: bool,
}

pub enum StorageError {
    // Client fault
    NotFound,
    AlreadyExists,

    // Server fault
    Internal,
}

impl Error for StorageError {}

impl Display for StorageError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            StorageError::NotFound => f.write_str("Something not found!"),
            StorageError::Internal => f.write_str("Unexpected error."),
            StorageError::AlreadyExists => f.write_str("Already exists STOP"),
        }
    }
}

impl Debug for StorageError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            StorageError::NotFound => f.write_str("Something not found!"),
            StorageError::Internal => f.write_str("Unexpected error."),
            StorageError::AlreadyExists => f.write_str("Already exists STOP"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
