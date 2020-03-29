/// For impls of the Display trait.
use crate::types::{GameBoard, GameState, Card, CardColor, CardValue, DecoratedCard};
use std::fmt::{Display, Formatter};
use std::fmt;
use std::collections::HashMap;
#[allow(unused_imports)] // Needs to be in scope, despite not used
use std::convert::TryFrom;

const BOARD_ROW_GRID_LINE: &str = "+-----------+-----------+-----------+-----------+-----------+";
const BOARD_ROW_GRID_BLANK: &str = "|           |           |           |           |           |";
const BOARD_NEUTRAL_HEADER: &str = "|    Red    |   Green   |   White   |   Blue    |  Yellow   |";
const BOARD_NEUTRAL_CARD_BORDER: &str = "|  +-----+  |  +-----+  |  +-----+  |  +-----+  |  +-----+  |";
const BOARD_ROW_SIZE: usize = BOARD_ROW_GRID_LINE.len();
const BOARD_PLAY_CARD_BORDER: &str = "  +-----+  ";
const BOARD_PLAY_CARD_BLANK: &str = "           ";
const CARD_BORDER_SINGLE: &str = "+-----+";
const HAND_BORDER: &str = "+-----+ +-----+ +-----+ +-----+ +-----+ +-----+ +-----+ +-----+";
const HAND_ROW_SIZE: usize = HAND_BORDER.len();
const HAND_SELECTION_ROW: &str = "  [1]     [2]     [3]     [4]     [5]     [6]     [7]     [8]";
const COLOR_ORDER: [CardColor; 5] = [
    CardColor::Red,
    CardColor::Green,
    CardColor::White,
    CardColor::Blue,
    CardColor::Yellow
];

/// See repo level README for example.
impl Display for GameState {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let mut lines = Vec::new();

        lines.push("The board:");
        let game_board_str = format!("{}", self.game_board());
        lines.push(&game_board_str);

        lines.push("");
        lines.push("Your hand:");
        lines.push(HAND_BORDER);
        let mut hand_number_line = String::with_capacity(HAND_ROW_SIZE);
        let mut hand_color_line = String::with_capacity(HAND_ROW_SIZE);
        for (i, decorated_card) in self.my_hand().iter().enumerate() {
            if i != 0 {
                hand_number_line.push(' ');
                hand_color_line.push(' ');
            }
            hand_number_line.push_str(&format!("|{:^5}|", decorated_card.card().card_value().to_string_short()));
            hand_color_line.push_str(&format!("|{:^5}|", decorated_card.card().card_color().to_string_short()));
        }
        lines.push(&hand_number_line);
        lines.push(&hand_color_line);
        lines.push(HAND_BORDER);
        lines.push(HAND_SELECTION_ROW);

        f.write_str(&lines.join("\n"))
    }
}

impl Display for GameBoard {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let mut lines = Vec::new();

        // Opponent's score
        lines.push(BOARD_ROW_GRID_LINE);
        let opponent_score_line = format!("| Opponent's score: {:<4}{:36}|", self.op_score(), "");
        lines.push(&opponent_score_line);

        // Opponent's plays
        lines.push(BOARD_ROW_GRID_LINE);
        lines.push(BOARD_ROW_GRID_BLANK);
        let op_plays_lines: Vec<String> = draw_op_plays(self.op_plays());
        for line in op_plays_lines.iter() {
            lines.push(line);
        }
        lines.push(BOARD_ROW_GRID_LINE);

        // Neutral board
        lines.push(BOARD_NEUTRAL_HEADER);
        lines.push(BOARD_ROW_GRID_BLANK);
        lines.push(BOARD_NEUTRAL_CARD_BORDER);
        let (
            neutral_value_line,
            neutral_color_line,
            neutral_draw_deck_size_line
        ) = draw_neutral_board(self.neutral_draw_pile());
        lines.push(&neutral_value_line);
        lines.push(&neutral_color_line);
        lines.push(BOARD_NEUTRAL_CARD_BORDER);
        lines.push(&neutral_draw_deck_size_line);
        lines.push(BOARD_ROW_GRID_BLANK);
        lines.push(BOARD_NEUTRAL_HEADER);

        // My plays
        lines.push(BOARD_ROW_GRID_LINE);
        let my_plays_lines: Vec<String> = draw_my_plays(self.my_plays());
        for line in my_plays_lines.iter() {
            lines.push(line);
        }
        lines.push(BOARD_ROW_GRID_BLANK);

        // My score
        lines.push(BOARD_ROW_GRID_LINE);
        let my_score_line = format!("| Your score: {:<4}{:42}|", self.my_score(), "");
        lines.push(&my_score_line);
        lines.push(BOARD_ROW_GRID_LINE);

        // Draw pile
        let draw_pile_line = format!("Main draw pile: {} cards remaining", self.draw_pile_cards_remaining());
        lines.push(&draw_pile_line);

        // Fin
        f.write_str(&lines.join("\n"))
    }
}

fn draw_op_plays(op_plays: &HashMap<CardColor, Vec<CardValue>>) -> Vec<String> {
    let mut columns_per_color: HashMap<CardColor, Vec<String>> = HashMap::new();
    for (color, card_value_vec) in op_plays.iter() {
        let mut column: Vec<String> = Vec::new();
        for card_val in card_value_vec.iter() {
            column.push(BOARD_PLAY_CARD_BORDER.to_owned());
            column.push(format!("  | {:^3} |  ", card_val.to_string_short()));
        }
        column.push(format!("  | {:^3} |  ", color.to_string_short()));
        column.push(BOARD_PLAY_CARD_BORDER.to_owned());

        columns_per_color.insert(*color, column);
    }

    let num_rows = columns_per_color.values()
        .into_iter()
        .map(|v| v.len())
        .max()
        .unwrap_or(0);

    // Convert columns to rows. Column == color.
    // Horribly inefficient, but it's FE code for a FE I don't plan to support for very long.
    let mut rows: Vec<String> = Vec::with_capacity(num_rows);
    for i in 0..num_rows {
        let mut cells_in_row: Vec<&str> = Vec::with_capacity(COLOR_ORDER.len());

        for color in COLOR_ORDER.iter() {
            if let Some(column) = columns_per_color.get(color) {
                if let Some(cell) = column.get(i) {
                    cells_in_row.push(cell)
                } else {
                    cells_in_row.push(BOARD_PLAY_CARD_BLANK);
                }
            } else {
                cells_in_row.push(BOARD_PLAY_CARD_BLANK)
            }
        }

        rows.push(format!("|{}|", cells_in_row.join("|")));
    }

    rows
}

fn draw_neutral_board(neutral_draw_pile: &HashMap<CardColor, (CardValue, usize)>) -> (String, String, String) {
    let mut neutral_value_line = String::with_capacity(BOARD_ROW_SIZE);
    let mut neutral_color_line = String::with_capacity(BOARD_ROW_SIZE);
    let mut neutral_draw_deck_size_line = String::with_capacity(BOARD_ROW_SIZE);
    neutral_value_line.push('|');
    neutral_color_line.push('|');
    neutral_draw_deck_size_line.push('|');
    for color in COLOR_ORDER.iter() {
        match neutral_draw_pile.get(color) {
            Some((value, size)) => {
                neutral_value_line.push_str(&format!("  | {:^3} |  |", value.to_string_short()));
                neutral_color_line.push_str(&format!("  | {:^3} |  |", color.to_string_short()));
                neutral_draw_deck_size_line.push_str(&format!("{:>2} in pile |", size))
            },
            None => {
                neutral_value_line.push_str("  |     |  |");
                neutral_color_line.push_str("  |     |  |");
                neutral_draw_deck_size_line.push_str(" 0 in pile |");
            }
        }
    }

    (
        neutral_value_line,
        neutral_color_line,
        neutral_draw_deck_size_line
    )
}

fn draw_my_plays(my_plays: &HashMap<CardColor, Vec<CardValue>>) -> Vec<String> {
    let mut columns_per_color: HashMap<CardColor, Vec<String>> = HashMap::new();
    for (color, card_value_vec) in my_plays.iter() {
        let mut column: Vec<String> = Vec::new();
        for card_val in card_value_vec.iter() {
            column.push(BOARD_PLAY_CARD_BORDER.to_owned());
            column.push(format!("  | {:^3} |  ", card_val.to_string_short()));
        }
        column.push(format!("  | {:^3} |  ", color.to_string_short()));
        column.push(BOARD_PLAY_CARD_BORDER.to_owned());

        columns_per_color.insert(*color, column);
    }

    let num_rows = columns_per_color.values()
        .into_iter()
        .map(|v| v.len())
        .max()
        .unwrap_or(0);

    // Convert columns to rows. Column == color.
    // Horribly inefficient, but it's FE code for a FE I don't plan to support for very long.
    let mut rows: Vec<String> = Vec::with_capacity(num_rows);
    for i in 0..num_rows {
        let mut cells_in_row: Vec<&str> = Vec::with_capacity(COLOR_ORDER.len());

        for color in COLOR_ORDER.iter() {
            if let Some(column) = columns_per_color.get(color) {
                if let Some(cell) = column.get(i) {
                    cells_in_row.push(cell)
                } else {
                    cells_in_row.push(BOARD_PLAY_CARD_BLANK);
                }
            } else {
                cells_in_row.push(BOARD_PLAY_CARD_BLANK)
            }
        }

        rows.push(format!("|{}|", cells_in_row.join("|")));
    }

    rows
}

impl Display for DecoratedCard {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.card())
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{} {}",
            self.card_color().to_string_long(),
            self.card_value().to_string_long()
        )
    }
}

impl Card {
    pub fn draw_single(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n",
            CARD_BORDER_SINGLE,
            format!("|{:^5}|", self.card_value().to_string_short()),
            format!("|{:^5}|", self.card_color().to_string_short()),
            CARD_BORDER_SINGLE,
        )
    }
}

impl CardColor {
    fn to_string_long(&self) -> &'static str {
        match self {
            CardColor::Red => "Red",
            CardColor::Green => "Green",
            CardColor::White => "White",
            CardColor::Blue => "Blue",
            CardColor::Yellow => "Yellow",
        }
    }

    fn to_string_short(&self) -> &'static str {
        match self {
            CardColor::Red => "Red",
            CardColor::Green => "Grn",
            CardColor::White => "Wht",
            CardColor::Blue => "Blu",
            CardColor::Yellow => "Ylw",
        }
    }
}

impl CardValue {
    fn to_string_short(&self) -> &'static str {
        match self {
            CardValue::Wager => "wgr",
            CardValue::Two => "2",
            CardValue::Three => "3",
            CardValue::Four => "4",
            CardValue::Five => "5",
            CardValue::Six => "6",
            CardValue::Seven => "7",
            CardValue::Eight => "8",
            CardValue::Nine => "9",
            CardValue::Ten => "10",
        }
    }

    fn to_string_long(&self) -> &'static str {
        match self {
            CardValue::Wager => "Wager",
            CardValue::Two => "2",
            CardValue::Three => "3",
            CardValue::Four => "4",
            CardValue::Five => "5",
            CardValue::Six => "6",
            CardValue::Seven => "7",
            CardValue::Eight => "8",
            CardValue::Nine => "9",
            CardValue::Ten => "10",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::types::GameStatus;

    #[test]
    fn eyeball_stdout_test() {
        let mut my_plays = HashMap::new();
        my_plays.insert(CardColor::Red, vec![
            CardValue::try_from(1).unwrap(),
            CardValue::try_from(1).unwrap(),
            CardValue::try_from(3).unwrap(),
            CardValue::try_from(4).unwrap(),
            CardValue::try_from(5).unwrap(),
        ]);
        my_plays.insert(CardColor::Yellow, vec![
            CardValue::try_from(9).unwrap(),
            CardValue::try_from(10).unwrap(),
        ]);

        let mut op_plays = HashMap::new();
        op_plays.insert(CardColor::Blue, vec![
            CardValue::try_from(1).unwrap(),
            CardValue::try_from(1).unwrap(),
            CardValue::try_from(3).unwrap(),
            CardValue::try_from(4).unwrap(),
            CardValue::try_from(5).unwrap(),
        ]);
        op_plays.insert(CardColor::Green, vec![
            CardValue::try_from(9).unwrap(),
            CardValue::try_from(10).unwrap(),
        ]);

        let mut neutral_draw_pile = HashMap::new();
        neutral_draw_pile.insert(CardColor::White, (CardValue::try_from(1).unwrap(), 3));
        neutral_draw_pile.insert(CardColor::Yellow, (CardValue::try_from(4).unwrap(), 1));
        neutral_draw_pile.insert(CardColor::Green, (CardValue::try_from(10).unwrap(), 12));

        let game_board = GameBoard::new(
            my_plays,
            op_plays,
            100,
            200,
            neutral_draw_pile,
            40
        );

        let my_hand = vec![
            Card::new(CardColor::White, CardValue::try_from(1).unwrap()),
            Card::new(CardColor::White, CardValue::try_from(5).unwrap()),
            Card::new(CardColor::White, CardValue::try_from(10).unwrap()),
            Card::new(CardColor::Blue, CardValue::try_from(3).unwrap()),
            Card::new(CardColor::Green, CardValue::try_from(7).unwrap()),
            Card::new(CardColor::Red, CardValue::try_from(7).unwrap()),
            Card::new(CardColor::Yellow, CardValue::try_from(7).unwrap()),
            Card::new(CardColor::Yellow, CardValue::try_from(9).unwrap()),
        ];
        let my_hand = my_hand.iter()
            .map(|c| DecoratedCard::new(*c, true))
            .collect();

        let game_state = GameState::new(
            game_board,
            my_hand,
            GameStatus::InProgress(true)
        );

        println!();
        println!("{}", game_state);

        assert_eq!(1, 2);
    }
}
