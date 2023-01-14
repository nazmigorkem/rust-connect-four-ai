use crate::board::Board;
pub fn minimax(board: &mut Board, is_maximizing: bool, depth: u32) -> i32 {
    if depth == 0 || board.is_game_finished_whole_board_check().0 {
        return utility(board);
    }
    if is_maximizing {
        let mut value = i32::MIN;
        for child in board.generate_possible_moves(is_maximizing).iter_mut() {
            value = std::cmp::max(value, minimax(&mut child.0, !is_maximizing, depth - 1))
        }
        return value;
    } else {
        let mut value = i32::MAX;
        for child in board.generate_possible_moves(is_maximizing).iter_mut() {
            value = std::cmp::min(value, minimax(&mut child.0, !is_maximizing, depth - 1))
        }
        return value;
    }
}

pub fn utility(board: &mut Board) -> i32 {
    5
}
