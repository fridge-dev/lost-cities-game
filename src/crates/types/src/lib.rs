// TODO remove these once I get into the thick of things.
#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::error::Error;
use core::fmt;

pub struct GameMetadata {
    game_id: String,
    p1_id: String,
    p2_id: String,
}

impl GameMetadata {
    pub fn new(game_id: String, p1_id: String, p2_id: String) -> Self {
        GameMetadata {
            game_id,
            p1_id,
            p2_id
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
}

pub struct GameState {
    game_board: GameBoard,
    p1_hand: [Card; 8],
    p1_turn: bool,
}

pub struct GameBoard {
    p1_plays: HashMap<CardColor, Vec<CardValue>>,
    p2_plays: HashMap<CardColor, Vec<CardValue>>,
    p1_score: i32,
    p2_score: i32,
    neutral_draw_pile: HashMap<CardColor, (CardValue, u8)>,
    draw_pile_cards_remaining: u8,
}

pub struct Card {
    card_color: CardColor,
    card_value: CardValue,
}

pub enum CardColor {
    Red,
    Green,
    White,
    Blue,
    Yellow,
}

pub enum CardValue {
    Wager,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

// I think this usage of lifetimes is "safe" and won't be complicated later. Let's find out!
pub struct Play<'a> {
    game_id: &'a str,
    player_id: &'a str,
    card: Card,
    target: CardTarget,
}

pub enum CardTarget {
    Player,
    Neutral,
}

#[derive(Debug)]
pub enum GameError {
    Internal,
    NotFound,
}

impl Error for GameError {}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            GameError::NotFound => f.write_str("Something not found!"),
            GameError::Internal => f.write_str("Unexpected error."),
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
