use rand_core::RngCore;
use rand::seq::SliceRandom;
use crate::v2::framework::prng::PrngRand;

pub fn shuffle<T>(mut collection: Vec<T>) -> (Vec<T>, u64) {
    let seed = rand::thread_rng().next_u64();
    (shuffle_impl(collection, seed), seed)
}

fn shuffle_impl<T>(mut collection: Vec<T>, seed_for_random: u64) -> Vec<T> {
    let prng = &mut PrngRand::new(seed_for_random);
    // Let's get wild
    collection.shuffle(prng);
    collection.reverse();
    collection.shuffle(prng);
    collection.reverse();
    collection.shuffle(prng);

    collection
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_unshuffled() -> Vec<u8> {
        vec![1, 2, 3, 4, 5]
    }

    #[test]
    fn different_seeds_produce_different_decks() {
        let unshuffled = new_unshuffled();
        let (deck2, seed2) = shuffle(unshuffled.clone());
        let (deck1, seed1) = shuffle(unshuffled.clone());

        assert_ne!(deck1, deck2);
        assert_ne!(seed1, seed2);
    }

    #[test]
    fn same_seed_produces_same_deck() {
        let unshuffled = new_unshuffled();
        let deck2 = shuffle_impl(unshuffled.clone(), 100);
        let deck1 = shuffle_impl(unshuffled.clone(), 100);

        assert_eq!(deck1, deck2);
    }
}
