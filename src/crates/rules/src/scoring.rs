use types::{CardValue, CardColor};
use std::collections::HashMap;

pub fn compute_score(plays: &HashMap<CardColor, Vec<CardValue>>) -> i32 {
    plays
        .values()
        .into_iter()
        .map(|plays| compute_score_for_color(plays))
        .sum()
}

fn compute_score_for_color(column: &Vec<CardValue>) -> i32 {
    if column.len() == 0 {
        return 0;
    }

    let mut score = -20;
    let mut wager_multiplier = 1;
    for card in column.iter() {
        match card {
            CardValue::Wager => wager_multiplier += 1,
            CardValue::Two => score += 2,
            CardValue::Three => score += 3,
            CardValue::Four => score += 4,
            CardValue::Five => score += 5,
            CardValue::Six => score += 6,
            CardValue::Seven => score += 7,
            CardValue::Eight => score += 8,
            CardValue::Nine => score += 9,
            CardValue::Ten => score += 10,
        }
    }

    let bonus = if column.len() >= 8 { 20 } else { 0 };

    return score * wager_multiplier + bonus;
}


#[cfg(test)]
mod tests {
    use super::*;

    fn compute_score_test_helper(expected_score: i32, previous_plays_vec: Vec<(CardColor, u8)>) {
        let previous_plays = previous_plays(previous_plays_vec);

        assert_eq!(compute_score(&previous_plays), expected_score, "Plays {:?} did not match expected score {}", &previous_plays, expected_score);
    }

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

    fn compute_score_for_color_test_helper(expected_score: i32, card_values: Vec<u8>) {
        let mut cards = Vec::new();
        for val in card_values {
            cards.push(CardValue::from_int(val));
        }

        assert_eq!(compute_score_for_color(&cards), expected_score, "Column {:?} did not match expected score {}", &cards, expected_score);
    }

    #[test]
    fn compute_score_no_cards() {
        compute_score_test_helper(0, vec![]);
    }

    #[test]
    fn compute_score_one_color() {
        compute_score_test_helper(4, vec![
            (CardColor::White, 3),
            (CardColor::White, 5),
            (CardColor::White, 7),
            (CardColor::White, 9),
        ]);
        compute_score_test_helper(-15, vec![
            (CardColor::Yellow, 1),
            (CardColor::Yellow, 1),
            (CardColor::Yellow, 4),
            (CardColor::Yellow, 5),
            (CardColor::Yellow, 6),
        ]);
    }

    #[test]
    fn compute_score_many_colors() {
        compute_score_test_helper(-11, vec![
            (CardColor::White, 3),
            (CardColor::White, 5),
            (CardColor::White, 7),
            (CardColor::White, 9),
            (CardColor::Yellow, 1),
            (CardColor::Yellow, 1),
            (CardColor::Yellow, 4),
            (CardColor::Yellow, 5),
            (CardColor::Yellow, 6),
        ]);
    }

    #[test]
    fn compute_score_complex() {
        compute_score_for_color_test_helper(  4, vec![3, 5, 7, 9]);
        compute_score_for_color_test_helper(-15, vec![1, 1, 4, 5, 6]);
        compute_score_for_color_test_helper( 68, vec![1, 1, 3, 4, 5, 7, 8, 9]);
        compute_score_for_color_test_helper(-30, vec![1, 5]);
        compute_score_for_color_test_helper(-17, vec![3]);

        // Sanity check
        assert_eq!(4 + -15 + 68 + -30 + -17, 10);

        compute_score_test_helper(10, vec![
            // 4
            (CardColor::White, 3),
            (CardColor::White, 5),
            (CardColor::White, 7),
            (CardColor::White, 9),
            // -15
            (CardColor::Yellow, 1),
            (CardColor::Yellow, 1),
            (CardColor::Yellow, 4),
            (CardColor::Yellow, 5),
            (CardColor::Yellow, 6),
            // 68 (= (36 - 20) * 3 + 20)
            (CardColor::Green, 1),
            (CardColor::Green, 1),
            (CardColor::Green, 3),
            (CardColor::Green, 4),
            (CardColor::Green, 5),
            (CardColor::Green, 7),
            (CardColor::Green, 8),
            (CardColor::Green, 9),
            // -30
            (CardColor::Red, 1),
            (CardColor::Red, 5),
            // - 17
            (CardColor::Blue, 3),
        ]);
    }

    #[test]
    fn compute_score_for_color_many_test_cases() {
        // Misc individual cards
        compute_score_for_color_test_helper(-18, vec![2]);
        compute_score_for_color_test_helper(-17, vec![3]);
        compute_score_for_color_test_helper(-10, vec![10]);

        // empty with various multipliers
        compute_score_for_color_test_helper(  0, vec![]);
        compute_score_for_color_test_helper(-40, vec![1]);
        compute_score_for_color_test_helper(-60, vec![1, 1]);
        compute_score_for_color_test_helper(-80, vec![1, 1, 1]);

        // 0 sum with various multipliers
        compute_score_for_color_test_helper(  0, vec![2, 3, 7, 8]);
        compute_score_for_color_test_helper(  0, vec![2, 3, 7, 8, 1]);
        compute_score_for_color_test_helper(  0, vec![2, 3, 7, 8, 1, 1]);
        compute_score_for_color_test_helper(  0, vec![2, 3, 7, 8, 1, 1, 1]);

        // positive sum with various multipliers
        compute_score_for_color_test_helper(  4, vec![6, 8, 10]);
        compute_score_for_color_test_helper(  8, vec![6, 8, 10, 1]);
        compute_score_for_color_test_helper( 12, vec![6, 8, 10, 1, 1]);
        compute_score_for_color_test_helper( 16, vec![6, 8, 10, 1, 1, 1]);

        // negative sum with various multipliers
        compute_score_for_color_test_helper( -4, vec![6, 10]);
        compute_score_for_color_test_helper( -8, vec![6, 10, 1]);
        compute_score_for_color_test_helper(-12, vec![6, 10, 1, 1]);
        compute_score_for_color_test_helper(-16, vec![6, 10, 1, 1, 1]);

        // bonus
        compute_score_for_color_test_helper( 15, vec![2, 3, 4, 5, 6, 7, 8]);
        compute_score_for_color_test_helper( 44, vec![2, 3, 4, 5, 6, 7, 8, 9]);
        compute_score_for_color_test_helper( 68, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        compute_score_for_color_test_helper(156, vec![1, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn literally_the_worst_score() {
        compute_score_test_helper(-400, vec![
            (CardColor::White, 1),
            (CardColor::White, 1),
            (CardColor::White, 1),
            (CardColor::Green, 1),
            (CardColor::Green, 1),
            (CardColor::Green, 1),
            (CardColor::Blue, 1),
            (CardColor::Blue, 1),
            (CardColor::Blue, 1),
            (CardColor::Red, 1),
            (CardColor::Red, 1),
            (CardColor::Red, 1),
            (CardColor::Yellow, 1),
            (CardColor::Yellow, 1),
            (CardColor::Yellow, 1),
        ]);
    }

    // It's hard to exhaustively test all scenarios. Only add more tests if I find bugs
    // with existing implementation. #TDD
}
