use std::collections::HashMap;
use std::fmt::Debug;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct GameMetadata {
    game_id: String,
    host_player_id: String,
    creation_time_ms: u64,
    matched_data: Option<(String, GameStatus)>,
}

impl GameMetadata {
    pub fn new_matched(
        game_id: String,
        host_player_id: String,
        creation_time_ms: u64,
        guest_player_id: String,
        status: GameStatus,
    ) -> Self {
        GameMetadata {
            game_id,
            host_player_id,
            creation_time_ms,
            matched_data: Some((guest_player_id, status)),
        }
    }

    pub fn new_unmatched(
        game_id: String,
        host_player_id: String,
        creation_time_ms: u64,
    ) -> Self {
        GameMetadata {
            game_id,
            host_player_id,
            creation_time_ms,
            matched_data: None,
        }
    }

    pub fn game_id(&self) -> &str {
        &self.game_id
    }

    pub fn host_player_id(&self) -> &str {
        &self.host_player_id
    }

    pub fn creation_time_ms(&self) -> u64 {
        self.creation_time_ms
    }

    pub fn matched_data(&self) -> &Option<(String, GameStatus)> {
        &self.matched_data
    }
}

/// Everything within GameState's hierarchy is in reference to the requesting player.
/// * "my" = the player's data
/// * "op" = the opponent's data
#[derive(Debug)]
pub struct GameState {
    game_board: GameBoard,
    my_hand: Vec<DecoratedCard>,
    status: GameStatus,
}

impl GameState {
    pub fn new(
        game_board: GameBoard,
        my_hand: Vec<DecoratedCard>,
        status: GameStatus,
    ) -> Self {
        GameState {
            game_board,
            my_hand,
            status,
        }
    }

    pub fn game_board(&self) -> &GameBoard {
        &self.game_board
    }

    pub fn my_hand(&self) -> &Vec<DecoratedCard> {
        &self.my_hand
    }

    pub fn status(&self) -> &GameStatus {
        &self.status
    }
}

#[derive(Debug)]
pub struct GameBoard {
    my_plays: HashMap<CardColor, Vec<CardValue>>,
    op_plays: HashMap<CardColor, Vec<CardValue>>,
    my_score_total: i32,
    op_score_total: i32,
    my_score_per_color: HashMap<CardColor, i32>,
    op_score_per_color: HashMap<CardColor, i32>,
    neutral_draw_pile: HashMap<CardColor, (CardValue, usize)>,
    draw_pile_cards_remaining: usize,
}

impl GameBoard {
    pub fn new(
        my_plays: HashMap<CardColor, Vec<CardValue>>,
        op_plays: HashMap<CardColor, Vec<CardValue>>,
        my_score_total: i32,
        op_score_total: i32,
        my_score_per_color: HashMap<CardColor, i32>,
        op_score_per_color: HashMap<CardColor, i32>,
        neutral_draw_pile: HashMap<CardColor, (CardValue, usize)>,
        draw_pile_cards_remaining: usize
    ) -> Self {
        GameBoard {
            my_plays,
            op_plays,
            my_score_total,
            op_score_total,
            my_score_per_color,
            op_score_per_color,
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

    pub fn my_score_total(&self) -> &i32 {
        &self.my_score_total
    }

    pub fn op_score_total(&self) -> &i32 {
        &self.op_score_total
    }

    pub fn my_score_per_color(&self) -> &HashMap<CardColor, i32> {
        &self.my_score_per_color
    }

    pub fn op_score_per_color(&self) -> &HashMap<CardColor, i32> {
        &self.op_score_per_color
    }

    pub fn neutral_draw_pile(&self) -> &HashMap<CardColor, (CardValue, usize)> {
        &self.neutral_draw_pile
    }

    pub fn draw_pile_cards_remaining(&self) -> &usize {
        &self.draw_pile_cards_remaining
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameStatus {
    InProgress(/* Is my turn */ bool),
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
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
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

#[derive(Debug)]
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

    pub fn game_id(&self) -> &String {
        &self.game_id
    }

    pub fn player_id(&self) -> &String {
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
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CardTarget {
    Player,
    Neutral,
}

/// Where to draw the new card from.
#[derive(PartialEq, Eq, Debug)]
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

    #[test]
    fn hand_sorter() {
        let mut hand = vec![
            DecoratedCard::new(Card::new(CardColor::Red, CardValue::Wager), true),
            DecoratedCard::new(Card::new(CardColor::Blue, CardValue::Two), false),
            DecoratedCard::new(Card::new(CardColor::Green, CardValue::Three), true),
            DecoratedCard::new(Card::new(CardColor::White, CardValue::Four), false),
            DecoratedCard::new(Card::new(CardColor::Red, CardValue::Five), false),
            DecoratedCard::new(Card::new(CardColor::Blue, CardValue::Six), true),
            DecoratedCard::new(Card::new(CardColor::Green, CardValue::Seven), false),
            DecoratedCard::new(Card::new(CardColor::White, CardValue::Eight), true),
        ];

        hand.sort();

        assert_eq!(hand, vec![
            DecoratedCard::new(Card::new(CardColor::Red, CardValue::Wager), true),
            DecoratedCard::new(Card::new(CardColor::Red, CardValue::Five), false),
            DecoratedCard::new(Card::new(CardColor::Green, CardValue::Three), true),
            DecoratedCard::new(Card::new(CardColor::Green, CardValue::Seven), false),
            DecoratedCard::new(Card::new(CardColor::White, CardValue::Four), false),
            DecoratedCard::new(Card::new(CardColor::White, CardValue::Eight), true),
            DecoratedCard::new(Card::new(CardColor::Blue, CardValue::Two), false),
            DecoratedCard::new(Card::new(CardColor::Blue, CardValue::Six), true),
        ]);
    }
}