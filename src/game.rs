use std::io;

use crate::{board::Board, minimax::minimax};

pub struct Game;

impl Game {
    pub fn play_player(board: &mut Board, turn: bool) -> ((u8, String), (u8, u8)) {
        let mut buffer = String::new();
        let mut resulting_flag: (u8, String) = (0, String::new());
        let mut choice = 0;
        if let Ok(_) = io::stdin().read_line(&mut buffer) {
            if let Ok(parsed) = buffer.trim().parse::<u8>() {
                choice = parsed - 1;
                if choice > 7 {
                    resulting_flag = (1, String::from("Please give an integer between 1 and 8."));
                }
            } else {
                resulting_flag = (1, String::from("Please give an integer between 1 and 8."));
            }
        } else {
            resulting_flag = (2, String::from("An error occured while taking input."));
        }
        let i = board.get_peg_count_in_column(choice) as u8;
        if resulting_flag.0 == 0 {
            resulting_flag = board.play(turn, i, choice);
        }

        return (resulting_flag, (i, choice));
    }

    pub fn play_ai(board: &mut Board) {
        let res = minimax(board, true, 6, 0, 0);
        res.print_board(&(0, String::from("_")));
        println!("{}", res.value);
    }
}
