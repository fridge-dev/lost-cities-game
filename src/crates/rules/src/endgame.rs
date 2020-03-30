use game_api::types::{GameBoard, GameStatus, GameResult};

pub fn get_game_status(game_board: &GameBoard, is_my_turn: bool) -> GameStatus {
    if *game_board.draw_pile_cards_remaining() > 0 {
        GameStatus::InProgress(is_my_turn)
    } else {
        if game_board.my_score_total() > game_board.op_score_total() {
            GameStatus::Complete(GameResult::Win)
        } else if game_board.my_score_total() == game_board.op_score_total() {
            GameStatus::Complete(GameResult::Draw)
        } else {
            GameStatus::Complete(GameResult::Lose)
        }
    }
}
