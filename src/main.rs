use board::Board;

use crate::game::Game;

pub mod board;
pub mod game;
pub mod minimax;

fn main() {
    let mut board = Board::new();
    let mut turn = true;
    println!("PLAYER 1 = x");
    println!("PLAYER 2 = o");

    board.print_board(&(0, String::from("Game started.")));
    loop {
        let (mut resulting_flag, (_, _)) = Game::play_player(&mut board, turn);

        if resulting_flag.0 == 0 {
            if board.pegs.len() == 56 {
                resulting_flag = (4, String::from("There is no available move. It is tie."))
            }
        }
        board.print_board(&resulting_flag);
        if resulting_flag.0 == 4 {
            break;
        }
        if board.is_game_finished_whole_board_check().0 {
            println!(
                "\x1b[12B\x1b[2KPlayer {} won the game.",
                if turn { "1" } else { "2" }
            );
            break;
        } else if resulting_flag.0 == 0 {
            turn = !turn;
            let ai_move = Game::play_ai(&mut board, turn);
            board.play(turn, ai_move.1, ai_move.2);
            board.print_board(&resulting_flag);
            turn = !turn;
        }
    }
}
