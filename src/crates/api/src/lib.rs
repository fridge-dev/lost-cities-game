use std::collections::HashMap;

pub struct GameState {
    game_board: GameBoard,
    p1_hand: [Card; 8],
    // TODO will probably need some player_id to pass in on each request
}

pub struct GameBoard {
    p1_plays: HashMap<CardColor, Vec<CardValue>>,
    p2_plays: HashMap<CardColor, Vec<CardValue>>,
    p1_score: i32,
    p2_score: i32,
    neutral_draw_pile: HashMap<CardColor, (CardValue, u8)>,
    draw_pile_cards_remaining: u8,
}

pub struct Card {
    card_color: CardColor,
    card_value: CardValue,
}

pub enum CardColor {
    Red,
    Green,
    White,
    Blue,
    Yellow,
}

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

pub fn get_game_state() -> GameState {
    panic!("Not implemented");
}

pub fn play_card() {
    panic!("Not implemented");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
