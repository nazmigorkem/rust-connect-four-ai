use crate::board::Board;
pub fn minimax(board: &mut Board, is_maximizing: bool, depth: u32, i: u8, j: u8) -> Board {
    if depth == 0 || board.is_game_finished(i, j, !is_maximizing) {
        utility(board);
        return board.clone();
    }
    if is_maximizing {
        let mut test_board = Board::new();
        test_board.value = i32::MIN;
        for child in board.generate_possible_moves(is_maximizing).iter_mut() {
            test_board = std::cmp::max(
                test_board,
                minimax(&mut child.0, !is_maximizing, depth - 1, child.1, child.2),
            )
        }
        return test_board;
    } else {
        let mut test_board = Board::new();
        test_board.value = i32::MAX;
        for child in board.generate_possible_moves(is_maximizing).iter_mut() {
            test_board = std::cmp::min(
                test_board,
                minimax(&mut child.0, !is_maximizing, depth - 1, child.1, child.2),
            )
        }
        return test_board;
    }
}

pub fn utility(board: &mut Board) {
    board.value = 5;
}
