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

#[derive(Debug)]
pub struct GameState {
    game_board: GameBoard,
    p1_hand: [Card; 8],
    p1_turn: bool,
}

impl GameState {
    pub fn new(
        game_board: GameBoard,
        p1_hand: [Card; 8],
        p1_turn: bool
    ) -> Self {
        GameState {
            game_board,
            p1_hand,
            p1_turn
        }
    }

    pub fn game_board(&self) -> &GameBoard {
        &self.game_board
    }

    pub fn p1_hand(&self) -> &[Card; 8] {
        &self.p1_hand
    }

    pub fn p1_turn(&self) -> &bool {
        &self.p1_turn
    }
}

pub struct GameBoard {
    p1_plays: HashMap<CardColor, Vec<CardValue>>,
    p2_plays: HashMap<CardColor, Vec<CardValue>>,
    p1_score: i32,
    p2_score: i32,
    neutral_draw_pile: HashMap<CardColor, (CardValue, u8)>,
    draw_pile_cards_remaining: u8,
}

impl GameBoard {
    pub fn new(
        p1_plays: HashMap<CardColor, Vec<CardValue>>,
        p2_plays: HashMap<CardColor, Vec<CardValue>>,
        p1_score: i32,
        p2_score: i32,
        neutral_draw_pile: HashMap<CardColor, (CardValue, u8)>,
        draw_pile_cards_remaining: u8
    ) -> Self {
        GameBoard {
            p1_plays,
            p2_plays,
            p1_score,
            p2_score,
            neutral_draw_pile,
            draw_pile_cards_remaining
        }
    }

    pub fn p1_plays(&self) -> &HashMap<CardColor, Vec<CardValue>> {
        &self.p1_plays
    }

    pub fn p2_plays(&self) -> &HashMap<CardColor, Vec<CardValue>> {
        &self.p2_plays
    }

    pub fn p1_score(&self) -> &i32 {
        &self.p1_score
    }

    pub fn p2_score(&self) -> &i32 {
        &self.p2_score
    }

    pub fn neutral_draw_pile(&self) -> &HashMap<CardColor, (CardValue, u8)> {
        &self.neutral_draw_pile
    }

    pub fn draw_pile_cards_remaining(&self) -> &u8 {
        &self.draw_pile_cards_remaining
    }
}

impl Debug for GameBoard {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "GameBoard {{ p1_plays: {}, p2_plays: {}, p1_score: {}, p2_score: {}, neutral_draw_pile: {}, draw_pile_cards_remaining: {} }}",
            fmt_hash_map(&self.p1_plays),
            fmt_hash_map(&self.p2_plays),
            self.p1_score,
            self.p2_score,
            fmt_hash_map(&self.neutral_draw_pile),
            self.draw_pile_cards_remaining,
        )
    }
}

fn fmt_hash_map<K: Debug, V: Debug>(map: &HashMap<K, V>) -> String {
    let mut v: Vec<String> = Vec::with_capacity(map.len());
    for (key, val) in map.iter() {
        v.push(format!("{:?}: {:?}", &key, &val));
    }
    format!("HashMap {{ {} }}", v.join(", "))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Card {
    card_color: CardColor,
    card_value: CardValue,
}

impl Card {
    pub fn new(card_color: CardColor, card_value: CardValue) -> Self {
        Card {
            card_color,
            card_value,
        }
    }

    pub fn card_color(&self) -> &CardColor {
        &self.card_color
    }

    pub fn card_value(&self) -> &CardValue {
        &self.card_value
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CardColor {
    Red,
    Green,
    White,
    Blue,
    Yellow,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

mod rand_utils {
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };
    use crate::{Card, CardColor, CardValue};


    impl Distribution<Card> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
            Card {
                card_color: rand::random(),
                card_value: rand::random(),
            }
        }
    }

    impl Distribution<CardColor> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardColor {
            let arr = [
                CardColor::Red,
                CardColor::Green,
                CardColor::White,
                CardColor::Blue,
                CardColor::Yellow,
            ];
            let index: usize = rng.gen_range(0, arr.len());
            arr[index]
        }
    }

    impl Distribution<CardValue> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardValue {
            let arr = [
                CardValue::Two,
                CardValue::Three,
                CardValue::Four,
                CardValue::Five,
                CardValue::Six,
                CardValue::Seven,
                CardValue::Eight,
                CardValue::Nine,
                CardValue::Ten,
                CardValue::Wager,
            ];
            let index: usize = rng.gen_range(0, arr.len());
            arr[index]
        }
    }
}

// I think this usage of lifetimes is "safe" and won't be complicated later. Let's find out!
pub struct Play<'a> {
    game_id: &'a str,
    player_id: &'a str,
    card: &'a Card,
    target: &'a CardTarget,
}

impl<'a> Play<'a> {
    pub fn new(
        game_id: &'a str,
        player_id: &'a str,
        card: &'a Card,
        target: &'a CardTarget,
    ) -> Play<'a> {
        Play {
            game_id,
            player_id,
            card,
            target,
        }
    }

    pub fn game_id(&self) -> &str {
        self.game_id
    }

    pub fn player_id(&self) -> &str {
        self.player_id
    }

    pub fn card(&self) -> &Card {
        self.card
    }

    pub fn target(&self) -> &CardTarget {
        self.target
    }
}

pub enum CardTarget {
    Player,
    Neutral,
}

#[derive(Debug)]
pub enum GameError {
    Internal,
    NotFound,
    GameAlreadyMatched
}

impl Error for GameError {}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            GameError::NotFound => f.write_str("Something not found!"),
            GameError::Internal => f.write_str("Unexpected error."),
            GameError::GameAlreadyMatched => f.write_str("No room for u."),
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
