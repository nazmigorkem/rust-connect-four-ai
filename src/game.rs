use std::io;

use crate::{
    board::Board,
    enums::{GameState, PlayType},
    minimax::minimax,
};

pub struct Game;

impl Game {
    pub fn main_loop() {
        let mut board = Board::new();
        let mut turn = true;
        let option = Game::select_play_type();
        let human_side = if option == PlayType::HumanVsAI {
            Game::select_side()
        } else {
            true
        };

        println!("PLAYER 1 = x");
        println!("PLAYER 2 = o");

        board.print_board(&(GameState::NoError, String::from("Game started.")));
        loop {
            let mut resulting_flag = (GameState::NoError, String::from(""));
            if option == PlayType::HumanVsHuman
                || (option == PlayType::HumanVsAI && turn == human_side)
            {
                resulting_flag = Game::play_player(&mut board, turn);
            } else if option == PlayType::AIVsAI
                || (option == PlayType::HumanVsAI && turn != human_side)
            {
                resulting_flag = Game::play_ai(&mut board, turn, !human_side);
            }

            if resulting_flag.0 == GameState::NoError {
                if board.pegs.len() == 56 {
                    resulting_flag = (
                        GameState::Finished,
                        String::from("There is no available move. It is tie."),
                    )
                }
            }
            board.print_board(&resulting_flag);
            if resulting_flag.0 == GameState::Finished {
                println!(
                    "\x1b[11B\x1b[2KPlayer {} won the game.",
                    if turn { "1" } else { "2" }
                );
                break;
            }
            if resulting_flag.0 == GameState::NoError {
                turn = !turn;
            }
        }
    }

    pub fn play_ai(board: &mut Board, turn: bool, ai_side: bool) -> (GameState, String) {
        let mut resulting_flag: (GameState, String);

        let moves = board.generate_possible_moves(turn);

        let mut current_move_board = moves[0].clone();

        let mut best_move = None;
        for move_ in moves {
            let current_move = minimax(&move_.0, true, 6, f32::MIN, f32::MAX, ai_side);
            println!("{current_move}");
            if best_move.is_none() {
                best_move = Some(current_move);
            } else if best_move.unwrap() < current_move {
                best_move = Some(current_move);
                current_move_board = move_;
            }
        }
        resulting_flag = board.play(turn, current_move_board.1, current_move_board.2);
        if board.is_game_finished() {
            resulting_flag = (GameState::Finished, String::from("Game finished."));
        }
        return resulting_flag;
    }

    pub fn play_player(board: &mut Board, turn: bool) -> (GameState, String) {
        let mut buffer = String::new();
        let mut resulting_flag: (GameState, String) = (GameState::NoError, String::new());
        let mut choice = 0;
        if let Ok(_) = io::stdin().read_line(&mut buffer) {
            if let Ok(parsed) = buffer.trim().parse::<u8>() {
                choice = parsed - 1;
                if choice > 7 {
                    resulting_flag = (
                        GameState::IntegerError,
                        String::from("Please give an integer between 1 and 8."),
                    );
                }
            } else {
                resulting_flag = (
                    GameState::IntegerError,
                    String::from("Please give an integer between 1 and 8."),
                );
            }
            print!("\x1b[1A\x1b[0J");
        } else {
            resulting_flag = (
                GameState::InputError,
                String::from("An error occured while taking input."),
            );
        }
        let i = board.get_peg_count_in_column(choice) as u8;
        if resulting_flag.0 == GameState::NoError {
            resulting_flag = board.play(turn, i, choice);
        }
        if board.is_game_finished() {
            resulting_flag = (GameState::Finished, String::from("Game finished."));
        }
        return resulting_flag;
    }

    pub fn select_play_type() -> PlayType {
        let mut option: Option<PlayType>;
        println!("Select type:");
        println!("1) Human vs Human");
        println!("2) Human vs AI");
        println!("3) AI vs AI");
        loop {
            let mut buffer = String::new();
            if let Ok(_) = io::stdin().read_line(&mut buffer) {
                option = match buffer.trim() {
                    "1" => Some(PlayType::HumanVsHuman),
                    "2" => Some(PlayType::HumanVsAI),
                    "3" => Some(PlayType::AIVsAI),
                    _ => None,
                };
                if option.is_some() {
                    return option.unwrap();
                }
            }
        }
    }

    pub fn select_side() -> bool {
        let mut option: Option<bool>;
        println!("Select side:");
        println!("1) X");
        println!("2) O");
        loop {
            let mut buffer = String::new();
            if let Ok(_) = io::stdin().read_line(&mut buffer) {
                option = match buffer.trim() {
                    "1" => Some(true),
                    "2" => Some(false),
                    _ => None,
                };
                if option.is_some() {
                    return option.unwrap();
                }
            }
        }
    }
}
