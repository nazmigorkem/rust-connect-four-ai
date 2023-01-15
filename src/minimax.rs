use crate::board::Board;
pub fn minimax(
    board: &Board,
    is_maximizing: bool,
    depth: u32,
    mut alpha: f32,
    mut beta: f32,
    ai_side: bool,
) -> f32 {
    if depth == 0 || board.is_game_finished() {
        return utility(board, ai_side, depth + 1);
    }
    if is_maximizing {
        let mut value = f32::MIN;
        for child in board.generate_possible_moves(ai_side) {
            let minimax_val = minimax(&child.0, false, depth - 1, alpha, beta, ai_side);
            value = if value < minimax_val {
                minimax_val
            } else {
                value
            };
            if value > beta {
                break;
            }
            alpha = if alpha < value { value } else { alpha };
        }
        return value;
    } else {
        let mut value = f32::MAX;
        for child in board.generate_possible_moves(!ai_side) {
            let minimax_val = minimax(&child.0, true, depth - 1, alpha, beta, ai_side);

            value = if value > minimax_val {
                minimax_val
            } else {
                value
            };
            if value < alpha {
                break;
            }
            beta = if beta > value { value } else { beta };
        }
        return value;
    }
}

pub fn utility(board: &Board, ai_side: bool, depth: u32) -> f32 {
    return if board.is_game_finished() && board.last_move.unwrap().2 == ai_side {
        1. / depth as f32
    } else if board.is_game_finished() && board.last_move.unwrap().2 != ai_side {
        -1. / depth as f32
    } else {
        0.
    };
}
