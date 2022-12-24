use std::{io, thread, time::Duration};

use board::Board;

pub mod board;

fn main() {
    let mut board = Board::new();
    let mut turn = true;
    let mut choice: u8 = 0;
    loop {
        let mut resulting_flag: u8 = 0;
        let mut buffer = String::new();
        if let Ok(_) = io::stdin().read_line(&mut buffer) {
            if let Ok(parsed) = buffer.trim().parse::<u8>() {
                choice = parsed - 1;
                if choice > 7 {
                    resulting_flag = 1;
                }
            } else {
                resulting_flag = 1;
            }
        } else {
            resulting_flag = 2;
        }
        match resulting_flag {
            0 => {
                board.play(turn, choice);
                board.print_board();
                turn = !turn;
            }
            1 => {
                println!("Please give an integer between 1 and 7.");
                continue;
            }
            2 => {
                println!("Error occured while reading the input.");
                return;
            }
            _ => (),
        }
    }
}
