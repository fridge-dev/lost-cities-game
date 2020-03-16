use game_api::types::{GameBoard, GameStatus, GameResult};

pub fn get_game_status(game_board: &GameBoard) -> GameStatus {
    if *game_board.draw_pile_cards_remaining() > 0 {
        GameStatus::InProgress
    } else {
        if game_board.my_score() > game_board.op_score() {
            GameStatus::Complete(GameResult::Win)
        } else if game_board.my_score() == game_board.op_score() {
            GameStatus::Complete(GameResult::Draw)
        } else {
            GameStatus::Complete(GameResult::Lose)
        }
    }
}
