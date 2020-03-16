use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::convert::TryFrom;

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

/// Everything within GameState's hierarchy is in reference to the requesting player.
/// * "my" = the player's data
/// * "op" = the opponent's data
#[derive(Debug)]
pub struct GameState {
    game_board: GameBoard,
    my_hand: Vec<DecoratedCard>,
    is_my_turn: bool,
    status: GameStatus,
}

impl GameState {
    pub fn new(
        game_board: GameBoard,
        my_hand: Vec<DecoratedCard>,
        is_my_turn: bool,
        status: GameStatus,
    ) -> Self {
        GameState {
            game_board,
            my_hand,
            is_my_turn,
            status,
        }
    }

    pub fn game_board(&self) -> &GameBoard {
        &self.game_board
    }

    pub fn my_hand(&self) -> &Vec<DecoratedCard> {
        &self.my_hand
    }

    pub fn is_my_turn(&self) -> &bool {
        &self.is_my_turn
    }

    pub fn status(&self) -> &GameStatus {
        &self.status
    }
}

pub struct GameBoard {
    my_plays: HashMap<CardColor, Vec<CardValue>>,
    op_plays: HashMap<CardColor, Vec<CardValue>>,
    my_score: i32,
    op_score: i32,
    neutral_draw_pile: HashMap<CardColor, (CardValue, usize)>,
    draw_pile_cards_remaining: usize,
}

impl GameBoard {
    pub fn new(
        my_plays: HashMap<CardColor, Vec<CardValue>>,
        op_plays: HashMap<CardColor, Vec<CardValue>>,
        my_score: i32,
        op_score: i32,
        neutral_draw_pile: HashMap<CardColor, (CardValue, usize)>,
        draw_pile_cards_remaining: usize
    ) -> Self {
        GameBoard {
            my_plays,
            op_plays,
            my_score,
            op_score,
            neutral_draw_pile,
            draw_pile_cards_remaining
        }
    }

    pub fn my_plays(&self) -> &HashMap<CardColor, Vec<CardValue>> {
        &self.my_plays
    }

    pub fn op_plays(&self) -> &HashMap<CardColor, Vec<CardValue>> {
        &self.op_plays
    }

    pub fn my_score(&self) -> &i32 {
        &self.my_score
    }

    pub fn op_score(&self) -> &i32 {
        &self.op_score
    }

    pub fn neutral_draw_pile(&self) -> &HashMap<CardColor, (CardValue, usize)> {
        &self.neutral_draw_pile
    }

    pub fn draw_pile_cards_remaining(&self) -> &usize {
        &self.draw_pile_cards_remaining
    }
}

impl Debug for GameBoard {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "GameBoard {{ my_plays: {}, op_plays: {}, my_score: {}, op_score: {}, neutral_draw_pile: {}, draw_pile_cards_remaining: {} }}",
            fmt_hash_map(&self.my_plays),
            fmt_hash_map(&self.op_plays),
            self.my_score,
            self.op_score,
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
pub enum GameStatus {
    InProgress,
    Complete(GameResult),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameResult {
    Win,
    Lose,
    Draw
}

/// DecoratedCard is basically the API layer's representation of a "Card" and the
/// Card struct below is the storage layer's representation of a Card.
///
/// Maybe I should change/move the definitions to be as such? Maybe later...
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DecoratedCard {
    card: Card,
    is_playable: bool,
}

impl DecoratedCard {
    pub fn new(card: Card, is_playable: bool) -> Self {
        DecoratedCard {
            card,
            is_playable,
        }
    }

    pub fn card(&self) -> &Card {
        &self.card
    }

    pub fn is_playable(&self) -> &bool {
        &self.is_playable
    }
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CardValue {
    // Force enum variants i32 repr to start from 1.
    Wager = 1,
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

/// IMPORTANT: Wire model uses this, so client and server compatibility are dependent on this not changing.
impl TryFrom<u32> for CardValue {
    type Error = String;

    fn try_from(card_value: u32) -> Result<Self, Self::Error> {
        Ok(match card_value {
            1 => CardValue::Wager,
            2 => CardValue::Two,
            3 => CardValue::Three,
            4 => CardValue::Four,
            5 => CardValue::Five,
            6 => CardValue::Six,
            7 => CardValue::Seven,
            8 => CardValue::Eight,
            9 => CardValue::Nine,
            10 => CardValue::Ten,
            _ => return Err(format!("Illegal card value supplied: {}", card_value)),
        })
    }
}

impl From<CardValue> for u32 {
    fn from(card_value: CardValue) -> Self {
        card_value as u32
    }
}

pub struct Play {
    game_id: String,
    player_id: String,
    card: Card,
    target: CardTarget,
    draw_pile: DrawPile,
}

impl Play {
    pub fn new(
        game_id: String,
        player_id: String,
        card: Card,
        target: CardTarget,
        draw_pile: DrawPile,
    ) -> Play {
        Play {
            game_id,
            player_id,
            card,
            target,
            draw_pile,
        }
    }

    pub fn game_id(&self) -> &str {
        &self.game_id
    }

    pub fn player_id(&self) -> &str {
        &self.player_id
    }

    pub fn card(&self) -> &Card {
        &self.card
    }

    pub fn target(&self) -> &CardTarget {
        &self.target
    }

    pub fn draw_pile(&self) -> &DrawPile {
        &self.draw_pile
    }
}

/// Where to *play* a card.
#[derive(PartialEq, Eq)]
pub enum CardTarget {
    Player,
    Neutral,
}

/// Where to draw the new card from.
#[derive(PartialEq, Eq)]
pub enum DrawPile {
    Main,
    Neutral(CardColor),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_value_converter() {
        let card_value_variants = vec![
            CardValue::Wager,
            CardValue::Two,
            CardValue::Three,
            CardValue::Four,
            CardValue::Five,
            CardValue::Six,
            CardValue::Seven,
            CardValue::Eight,
            CardValue::Nine,
            CardValue::Ten,
        ];

        for card_value in card_value_variants {
            let int_value: u32 = card_value.into();
            assert_eq!(CardValue::try_from(int_value), Ok(card_value));
        }

        assert!(CardValue::try_from(0).is_err())
    }
}