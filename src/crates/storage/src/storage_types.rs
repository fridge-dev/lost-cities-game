use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use core::fmt;
use types::{Card, CardColor, CardValue}; // This is a broken layer of abstraction. But I'm sick of re-writing the same types for now. I'm trying to learn rust!
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct StorageGameMetadata {
    game_id: String,
    p1_id: String,
    p2_id: Option<String>,
    game_status: GameStatus,
}

impl StorageGameMetadata {
    pub fn new(
        game_id: String,
        p1_id: String,
        p2_id: Option<String>,
        game_status: GameStatus
    ) -> Self {
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

    pub fn p2_id(&self) -> &Option<String> {
        &self.p2_id
    }

    pub fn game_status(&self) -> &GameStatus {
        &self.game_status
    }

    pub fn set_p2_id(&mut self, p2_id: String) -> Result<(), StorageError> {
        match &self.p2_id {
            Some(_existing_p2_id) => Err(StorageError::IllegalModification),
            None => {
                self.p2_id = Some(p2_id);
                Ok(())
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GameStatus {
    InProgress, // "In progress" could also mean host is waiting for a guest
    Completed,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StorageGameState {
    // Maybe metadata should be in here instead? leaving it out for now. Idk.
    game_id: String,

    p1_hand: Vec<Card>,
    p2_hand: Vec<Card>,

    p1_plays: HashMap<CardColor, Vec<CardValue>>,
    p2_plays: HashMap<CardColor, Vec<CardValue>>,

    neutral_draw_pile: HashMap<CardColor, Vec<CardValue>>,
    draw_pile_cards_remaining: Vec<Card>,

    p1_turn: bool,
}

impl StorageGameState {
    pub fn new(
        game_id: String,
        p1_hand: Vec<Card>,
        p2_hand: Vec<Card>,
        p1_plays: HashMap<CardColor, Vec<CardValue>>,
        p2_plays: HashMap<CardColor, Vec<CardValue>>,
        neutral_draw_pile: HashMap<CardColor, Vec<CardValue>>,
        draw_pile_cards_remaining: Vec<Card>,
        p1_turn: bool
    ) -> Self {
        StorageGameState {
            game_id,
            p1_hand,
            p2_hand,
            p1_plays,
            p2_plays,
            neutral_draw_pile,
            draw_pile_cards_remaining,
            p1_turn
        }
    }

    pub fn game_id(&self) -> &str {
        &self.game_id
    }

    pub fn p1_hand(&self) -> &Vec<Card> {
        &self.p1_hand
    }

    pub fn p2_hand(&self) -> &Vec<Card> {
        &self.p2_hand
    }

    pub fn p1_plays(&self) -> &HashMap<CardColor, Vec<CardValue>> {
        &self.p1_plays
    }

    pub fn p2_plays(&self) -> &HashMap<CardColor, Vec<CardValue>> {
        &self.p2_plays
    }

    pub fn neutral_draw_pile(&self) -> &HashMap<CardColor, Vec<CardValue>> {
        &self.neutral_draw_pile
    }

    pub fn draw_pile_cards_remaining(&self) -> &Vec<Card> {
        &self.draw_pile_cards_remaining
    }

    pub fn p1_turn(&self) -> &bool {
        &self.p1_turn
    }
}

#[derive(Debug, PartialEq)]
pub enum StorageError {
    // Client fault
    NotFound,
    AlreadyExists,

    // Server fault
    Internal,
    IllegalModification,
}

impl Error for StorageError {}

impl Display for StorageError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            StorageError::NotFound => f.write_str("Something not found!"),
            StorageError::Internal => f.write_str("Unexpected error."),
            StorageError::AlreadyExists => f.write_str("Already exists STOP"),
            StorageError::IllegalModification => f.write_str("How did this f happen?"),
        }
    }
}
