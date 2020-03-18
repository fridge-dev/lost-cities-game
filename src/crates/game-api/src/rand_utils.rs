/// This is only needed for UTs
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use crate::types::{Card, CardColor, CardValue};

impl Distribution<Card> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Card {
        Card::new(
            rand::random(),
            rand::random(),
        )
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