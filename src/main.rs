use std::io;

use board::Board;

pub mod board;

fn main() {
    let mut board = Board::new();
    let mut turn = true;
    let mut choice: u8 = 0;
    println!("PLAYER 1 = x");
    println!("PLAYER 2 = o");
    board.print_board(&(0, String::from("Game started.")));
    loop {
        let mut resulting_flag: (u8, String) = (0, String::new());
        let mut buffer = String::new();
        if let Ok(_) = io::stdin().read_line(&mut buffer) {
            if let Ok(parsed) = buffer.trim().parse::<u8>() {
                choice = parsed - 1;
                if choice > 7 {
                    resulting_flag = (1, String::from("Please give an integer between 1 and 7."));
                }
            } else {
                resulting_flag = (1, String::from("Please give an integer between 1 and 7."));
            }
        } else {
            resulting_flag = (2, String::from("An error occured while taking input."));
        }
        let pegs_count_for_column = board
            .pegs
            .iter()
            .filter(|x| x.1 == choice)
            .collect::<Vec<&(u8, u8, bool)>>()
            .len() as u8;
        if resulting_flag.0 == 0 {
            resulting_flag = board.play(turn, pegs_count_for_column, choice);
        }

        board.print_board(&resulting_flag);
        if board.is_game_finished(pegs_count_for_column, choice, turn) {
            println!(
                "\x1b[12B\x1b[2KPlayer {} won the game.",
                if turn { "1" } else { "2" }
            );
            break;
        }

        if resulting_flag.0 == 0 {
            turn = !turn;
        }
    }
}
