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

    pub fn play(&mut self, turn: bool, selected_column: u8) {
        let pegs_count_for_column = self
            .pegs
            .iter()
            .filter(|x| x.1 == selected_column)
            .collect::<Vec<&(u8, u8, bool)>>()
            .len() as u8;
        if pegs_count_for_column == 8 {
            println!("You cannot put anymore pegs in this column.");
            return;
        }
        self.pegs
            .insert((pegs_count_for_column, selected_column, turn));
    }

    pub fn print_board(&self) {
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
        print!("\x1b[9F")
    }
}
