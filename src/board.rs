use std::collections::HashSet;

pub struct Board {
    pub pegs: HashSet<(u8, u8, bool)>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            pegs: HashSet::new(),
        }
    }

    pub fn play(&mut self, turn: bool, selected_row: u8, selected_column: u8) -> (u8, String) {
        if selected_row == 8 {
            return (
                3,
                format!(
                    "You cannot put any more peg to column {}.",
                    selected_column + 1
                ),
            );
        }
        self.pegs.insert((selected_row, selected_column, turn));
        return (
            0,
            format!(
                "Player {} put peg in column {}.",
                if turn { 1 } else { 2 },
                selected_column + 1
            ),
        );
    }

    pub fn is_game_finished(&self, i: u8, j: u8, turn: bool) -> bool {
        let directions = [(-1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1)];
        for direction in directions {
            let mut current_check = direction.clone();
            let mut count = 1;
            while let Some(_) =
                self.pegs
                    .get(&(i + current_check.0 as u8, j + current_check.1 as u8, turn))
            {
                current_check.0 += direction.0;
                current_check.1 += direction.1;
                count += 1;
                if count % 4 == 0 {
                    return true;
                }
            }
        }
        false
    }

    pub fn print_board(&self, result: &(u8, String)) {
        let width = 20;
        println!(
            "{:=<width$}\n\x1b[2KLast move result: {}\n{:=<width$}",
            "=", result.1, "=",
        );
        let mut display_board: Vec<Vec<&str>> = vec![vec!["- "; 7]; 8];
        for (i, j, player) in self.pegs.iter() {
            display_board[*i as usize][*j as usize] = if *player { "x " } else { "o " }
        }
        display_board.reverse();
        for i in display_board {
            for j in i {
                print!("{j} ");
            }
            print!("\n");
        }
        print!("\x1b[12F");
    }
}
