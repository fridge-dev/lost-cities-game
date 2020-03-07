/// "Plays" is a module for encapsulating all logic for determining what you are allowed to do in your turn.
/// E.g. *playing* a card, drawing a card.
use types::{Card, CardColor, CardValue, DecoratedCard};
use std::collections::HashMap;

pub fn decorate_hand(hand: Vec<Card>, previous_plays: &HashMap<CardColor, Vec<CardValue>>) -> Vec<DecoratedCard> {
    hand.iter()
        .map(|card| DecoratedCard::new(*card, is_card_playable(card, previous_plays)))
        .collect()
}

/// Consider moving this to UT utility, if that's the only place it's used.
pub fn get_allowed_plays(
    hand: &Vec<Card>,
    previous_plays: &HashMap<CardColor, Vec<CardValue>>
) -> Vec<usize> {
    let mut allowed_indexes = Vec::new();
    for (i, card_in_hand) in hand.iter().enumerate() {
        if is_card_playable(card_in_hand, previous_plays) {
            allowed_indexes.push(i);
        }
    }

    return allowed_indexes;
}

pub fn is_card_playable(
    attempted_play: &Card,
    previous_plays: &HashMap<CardColor, Vec<CardValue>>
) -> bool {
    if let Some(previous_plays_for_color) = previous_plays.get(attempted_play.card_color()) {
        if let Some(top_card) = previous_plays_for_color.last() {
            // If we've previously played a card of this color, only allow it if we're playing a card
            // of greater (or same, in case of wager card) value.
            return attempted_play.card_value() >= top_card;
        } else {
            println!("WARN: previous_plays.get(color) == Some(vec![]). Ideally, the right-hand value should be None.");
            // No previous plays of this color; move is allowed.
            return true;
        }
    } else {
        // No previous plays of this color; move is allowed.
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn previous_plays(previous_plays_vec: Vec<(CardColor, u8)>) -> HashMap<CardColor, Vec<CardValue>> {
        let mut previous_plays_map: HashMap<CardColor, Vec<CardValue>> = HashMap::new();
        for (card_color, card_value) in previous_plays_vec {
            previous_plays_map.entry(card_color)
                .or_insert_with(|| Vec::new())
                .push(CardValue::from_int(card_value));
        }

        for (_, card_value_vec) in &mut previous_plays_map {
            card_value_vec.sort()
        }

        return previous_plays_map;
    }

    // Generic test utility to assert various cases
    fn test_get_allowed_plays(
        previous_plays_vec: Vec<(CardColor, /* Card color */ u8)>,
        hand_with_expected_playable: Vec<(CardColor, /* Card color */ u8, /* Expected playable */ bool)>
    ) {
        // 1. Build the 'hand' and 'expected_output' vecs.
        let mut hand: Vec<Card> = Vec::new();
        let mut expected_output: Vec<usize> = Vec::new();

        for expected_card_playable in hand_with_expected_playable {
            if expected_card_playable.2 {
                expected_output.push(hand.len());
            }
            hand.push(Card::from_int(expected_card_playable.0, expected_card_playable.1));
        }

        // 2. Build the 'previous_plays' map
        let previous_plays_map = previous_plays(previous_plays_vec);

        // 3. Call method under test
        assert_eq!(get_allowed_plays(&hand, &previous_plays_map), expected_output);
    }

    #[test]
    fn can_play_any_with_no_previous_plays() {
        test_get_allowed_plays(
            /* Previous plays */ vec![],
            /* Hand */ vec![
                (CardColor::White, 1, true),
                (CardColor::White, 5, true),
                (CardColor::Green, 3, true),
                (CardColor::Green, 10, true),
                (CardColor::Blue, 4, true),
                (CardColor::Red, 2, true),
                (CardColor::Yellow, 7, true),
                (CardColor::Yellow, 6, true),
            ]
        );
    }

    #[test]
    fn can_play_multiple_wagers_of_same_color() {
        test_get_allowed_plays(
            /* Previous plays */ vec![
                (CardColor::White, 1)
            ],
            /* Hand */ vec![
                (CardColor::White, 1, true)
            ]
        );
    }

    #[test]
    fn cant_play_wager_with_previous_played_number_of_same_color() {
        test_get_allowed_plays(
            /* Previous plays */ vec![
                (CardColor::White, 2)
            ],
            /* Hand */ vec![
                (CardColor::White, 1, false),
                (CardColor::White, 3, true),
                (CardColor::White, 4, true),
                (CardColor::White, 5, true),
                (CardColor::White, 6, true),
                (CardColor::White, 7, true),
                (CardColor::White, 8, true),
                (CardColor::White, 9, true),
                (CardColor::White, 10, true),
            ]
        );
    }

    #[test]
    fn cant_play_lower_number_of_same_color() {
        test_get_allowed_plays(
            /* Previous plays */ vec![
                (CardColor::White, 5)
            ],
            /* Hand */ vec![
                (CardColor::White, 1, false),
                (CardColor::White, 2, false),
                (CardColor::White, 3, false),
                (CardColor::White, 4, false),
                (CardColor::White, 6, true),
                (CardColor::White, 7, true),
                (CardColor::White, 8, true),
                (CardColor::White, 9, true),
                (CardColor::White, 10, true),
            ]
        );
    }

    #[test]
    fn can_play_lower_number_of_diff_color() {
        test_get_allowed_plays(
            /* Previous plays */ vec![
                (CardColor::White, 5)
            ],
            /* Hand */ vec![
                (CardColor::White, 4, false),
                (CardColor::Green, 4, true),
                (CardColor::Blue, 4, true),
                (CardColor::Red, 4, true),
                (CardColor::Yellow, 4, true),
            ],
        );
    }

    #[test]
    fn misc_complex() {
        test_get_allowed_plays(
            /* Previous plays */ vec![
                (CardColor::White, 1),
                (CardColor::White, 3),
                (CardColor::Green, 1),
                (CardColor::Blue, 2),
                (CardColor::Blue, 5),
                (CardColor::Red, 10),
            ],
            /* Hand */ vec![
                (CardColor::White, 1, false),
                (CardColor::White, 4, true),
                (CardColor::Green, 1, true),
                (CardColor::Green, 2, true),
                (CardColor::Green, 10, true),
                (CardColor::Blue, 1, false),
                (CardColor::Blue, 4, false),
                (CardColor::Blue, 6, true),
                (CardColor::Blue, 8, true),
                (CardColor::Red, 1, false),
                (CardColor::Red, 9, false),
                (CardColor::Yellow, 1, true),
                (CardColor::Yellow, 2, true),
                (CardColor::Yellow, 10, true),
            ],
        );
    }

    // It's hard to exhaustively test all scenarios. Only add more tests if I find bugs
    // with existing implementation. #TDD
}
