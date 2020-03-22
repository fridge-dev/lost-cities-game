use game_api::types::{Card, CardColor, CardValue};
use rand::thread_rng;
use crate::rand_util::PrngRand;
use rand_core::RngCore;
use rand::seq::SliceRandom;

pub struct DeckFactory {
    unshuffled_deck: Vec<Card>,
}

const CARD_VALUES: [CardValue; 12] = [
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
    CardValue::Wager,
    CardValue::Wager,
];

const CARD_COLORS: [CardColor; 5] = [
    CardColor::Red,
    CardColor::Green,
    CardColor::White,
    CardColor::Blue,
    CardColor::Yellow,
];

impl DeckFactory {
    pub fn new() -> Self {
        let mut unshuffled_deck = Vec::with_capacity(CARD_VALUES.len() * CARD_COLORS.len());

        for value in CARD_VALUES.iter() {
            for &color in CARD_COLORS.iter() {
                unshuffled_deck.push(Card::new(color, *value));
            }
        }

        DeckFactory {
            unshuffled_deck,
        }
    }

    pub fn new_shuffled_deck(&self) -> (Vec<Card>, u64) {
        let seed = thread_rng().next_u64();
        (self.new_shuffled_deck_with_seed(seed), seed)
    }

    fn new_shuffled_deck_with_seed(&self, seed_for_random: u64) -> Vec<Card> {
        let mut deck = self.unshuffled_deck.clone();
        let prng = &mut PrngRand::new(seed_for_random);
        // Let's get wild
        deck.shuffle(prng);
        deck.reverse();
        deck.shuffle(prng);
        deck.reverse();
        deck.shuffle(prng);

        deck
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_seeds_produce_different_decks() {
        let deck_factory = DeckFactory::new();
        let (deck2, seed2)  = deck_factory.new_shuffled_deck();
        let (deck1, seed1)  = deck_factory.new_shuffled_deck();

        assert_ne!(deck1, deck2);
        assert_ne!(seed1, seed2);
    }

    #[test]
    fn same_seed_produces_same_deck() {
        let deck_factory = DeckFactory::new();
        let deck2 = deck_factory.new_shuffled_deck_with_seed(5);
        let deck1 = deck_factory.new_shuffled_deck_with_seed(5);

        assert_eq!(deck1, deck2);
    }
}
