use std::collections::HashSet;

pub struct Board {
    pegs: HashSet<(u8, u8, bool)>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            pegs: HashSet::new(),
        }
    }

    pub fn play(&mut self, turn: bool, selected_column: u8) -> (u8, String) {
        let pegs_count_for_column = self
            .pegs
            .iter()
            .filter(|x| x.1 == selected_column)
            .collect::<Vec<&(u8, u8, bool)>>()
            .len() as u8;
        if pegs_count_for_column == 8 {
            return (
                3,
                format!(
                    "You cannot put any more peg to column {}.",
                    selected_column + 1
                ),
            );
        }
        self.pegs
            .insert((pegs_count_for_column, selected_column, turn));
        return (
            0,
            format!(
                "Player {}, put peg in column {}.",
                if turn { 1 } else { 2 },
                selected_column + 1
            ),
        );
    }

    pub fn is_game_finished(&self, i: u8, j: u8) -> bool {
        true
    }

    pub fn print_board(&self, result: (u8, String)) {
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
