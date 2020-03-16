use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use core::fmt;
// This is a broken layer of abstraction. But I'm sick of re-writing the same types for now. I'm trying to learn rust!
use game_api::types::{Card, CardColor, CardValue};

const MISSING_P2_ID_MSG: &str = "Player 2 id is missing from metadata. If this happens, I was probably not as careful as I assumed and I should rename this method.";

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

    pub fn p2_id_opt(&self) -> &Option<String> {
        &self.p2_id
    }

    pub fn p2_id(&self) -> &str {
        self.p2_id.as_ref().expect(MISSING_P2_ID_MSG)
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
    main_draw_pile: Vec<Card>,

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
        main_draw_pile: Vec<Card>,
        p1_turn: bool
    ) -> Self {
        StorageGameState {
            game_id,
            p1_hand,
            p2_hand,
            p1_plays,
            p2_plays,
            neutral_draw_pile,
            main_draw_pile,
            p1_turn
        }
    }

    pub fn game_id(&self) -> &str {
        &self.game_id
    }

    pub fn p1_hand(&self) -> &Vec<Card> {
        &self.p1_hand
    }

    pub fn p1_hand_mut(&mut self) -> &mut Vec<Card> {
        &mut self.p1_hand
    }

    pub fn p2_hand(&self) -> &Vec<Card> {
        &self.p2_hand
    }

    pub fn p2_hand_mut(&mut self) -> &mut Vec<Card> {
        &mut self.p2_hand
    }

    pub fn p1_plays(&self) -> &HashMap<CardColor, Vec<CardValue>> {
        &self.p1_plays
    }

    pub fn p1_plays_mut(&mut self) -> &mut HashMap<CardColor, Vec<CardValue>> {
        &mut self.p1_plays
    }

    pub fn p2_plays(&self) -> &HashMap<CardColor, Vec<CardValue>> {
        &self.p2_plays
    }

    pub fn p2_plays_mut(&mut self) -> &mut HashMap<CardColor, Vec<CardValue>> {
        &mut self.p2_plays
    }

    pub fn neutral_draw_pile(&self) -> &HashMap<CardColor, Vec<CardValue>> {
        &self.neutral_draw_pile
    }

    pub fn neutral_draw_pile_mut(&mut self) -> &mut HashMap<CardColor, Vec<CardValue>> {
        &mut self.neutral_draw_pile
    }

    pub fn main_draw_pile(&self) -> &Vec<Card> {
        &self.main_draw_pile
    }

    pub fn main_draw_pile_mut(&mut self) -> &mut Vec<Card> {
        &mut self.main_draw_pile
    }

    pub fn p1_turn(&self) -> &bool {
        &self.p1_turn
    }

    pub fn swap_turn(&mut self) {
        self.p1_turn = !self.p1_turn
    }

    pub fn convert_to_player_aware(self, is_player_1: bool) -> PlayerAwareStorageGameState {
        PlayerAwareStorageGameState {
            inner: self,
            is_player_1
        }
    }
}

pub struct PlayerAwareStorageGameState {
    inner: StorageGameState,
    is_player_1: bool,
}

impl PlayerAwareStorageGameState {

    pub fn inner(&self) -> &StorageGameState {
        &self.inner
    }

    pub fn my_hand(&self) -> &Vec<Card> {
        if self.is_player_1 {
            &self.inner.p1_hand
        } else {
            &self.inner.p2_hand
        }
    }

    pub fn my_hand_mut(&mut self) -> &mut Vec<Card> {
        if self.is_player_1 {
            &mut self.inner.p1_hand
        } else {
            &mut self.inner.p2_hand
        }
    }

    pub fn my_plays(&self) -> &HashMap<CardColor, Vec<CardValue>> {
        if self.is_player_1 {
            &self.inner.p1_plays
        } else {
            &self.inner.p2_plays
        }
    }

    pub fn my_plays_mut(&mut self) -> &mut HashMap<CardColor, Vec<CardValue>> {
        if self.is_player_1 {
            &mut self.inner.p1_plays
        } else {
            &mut self.inner.p2_plays
        }
    }

    pub fn is_my_turn(&self) -> bool {
        self.is_player_1 ^ !self.inner.p1_turn
    }

    pub fn neutral_draw_pile(&self) -> &HashMap<CardColor, Vec<CardValue>> {
        &self.inner.neutral_draw_pile
    }

    pub fn neutral_draw_pile_mut(&mut self) -> &mut HashMap<CardColor, Vec<CardValue>> {
        &mut self.inner.neutral_draw_pile
    }

    pub fn main_draw_pile(&self) -> &Vec<Card> {
        &self.inner.main_draw_pile
    }

    pub fn main_draw_pile_mut(&mut self) -> &mut Vec<Card> {
        &mut self.inner.main_draw_pile
    }

    pub fn convert_to_inner(self) -> StorageGameState {
        self.inner
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
