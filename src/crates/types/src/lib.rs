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

/// Everything within GameState's hierarchy is in reference to the requesting player.
/// * "my" = the player's data
/// * "op" = the opponent's data
#[derive(Debug)]
pub struct GameState {
    game_board: GameBoard,
    my_hand: Vec<DecoratedCard>,
    is_my_turn: bool,
}

impl GameState {
    pub fn new(
        game_board: GameBoard,
        my_hand: Vec<DecoratedCard>,
        is_my_turn: bool
    ) -> Self {
        GameState {
            game_board,
            my_hand,
            is_my_turn
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

    /// This method is probably only for UT readability. Not sure where is the best place to put such methods.
    pub fn from_int(card_color: CardColor, card_value: u8) -> Self {
        Card::new(card_color, CardValue::from_int(card_value))
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

impl CardValue {
    /// This method is probably only for UT readability. Not sure where is the best place to put such methods.
    pub fn from_int(card_value: u8) -> Self {
        match card_value {
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
            _ => panic!(format!("Illegal card value supplied: {}", card_value)),
        }
    }
}

/// TODO This is only needed for UTs ... Find a suitable replacement or remove this comment if this is really the best.
mod rand_utils {
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };
    use crate::{Card, CardColor, CardValue};

    impl Distribution<Card> for Standard {
        fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Card {
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
    target: CardTarget,
    draw_pile: DrawPile,
}

impl<'a> Play<'a> {
    pub fn new(
        game_id: &'a str,
        player_id: &'a str,
        card: &'a Card,
        target: CardTarget,
        draw_pile: DrawPile,
    ) -> Play<'a> {
        Play {
            game_id,
            player_id,
            card,
            target,
            draw_pile,
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

// Naming starts to get confusing here... GL future me!
#[derive(Debug)]
pub enum GameError {
    Internal(Cause),
    NotFound(&'static str),
    GameAlreadyMatched,
    InvalidPlay(Reason),
}

/// Causes of `GameError::Internal` errors.
#[derive(Debug)]
pub enum Cause {

    /// Error caused by internal/dependency storage layer
    Storage(&'static str, Box<dyn Error>),

    /// Error caused by some impossible circumstance, but an error is needed for rust code to compile.
    ///
    /// Example:
    /// ```
    /// use types::{GameError, Cause};
    /// let mut v = vec![1, 2, 3];
    /// let first = v.pop().ok_or(GameError::Internal(Cause::Impossible));
    /// ```
    ///
    /// I truly expect this to never happen. #FamousLastWords
    Impossible,
}

// These may end up being my way of educating new users to the rules of the game. Consider giving this
// a really descriptive Display impl.
#[derive(Debug)]
pub enum Reason {
    NotYourTurn,
    CardNotInHand,
    CantPlayDecreasingCardValue,
    NeutralDrawPileEmpty,
    CantRedrawCardJustPlayed,
}

impl Error for GameError {}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            GameError::NotFound(entity) => f.write_str(&format!("{} not found!", entity)),
            GameError::Internal(cause) => f.write_str(&format!("Unexpected error: {:?}", cause)),
            GameError::GameAlreadyMatched => f.write_str("No room for u."),
            GameError::InvalidPlay(reason) => f.write_str(&format!("You cannot make that play: {:?}", reason)),
        }
    }
}
