use std::collections::HashSet;

#[derive(Clone)]
pub struct Board {
    pub pegs: HashSet<(u8, u8, bool)>,
    pub value: i32,
}

impl Board {
    pub fn new() -> Self {
        Board {
            pegs: HashSet::new(),
            value: 0,
        }
    }

    pub fn play(&mut self, turn: bool, selected_row: u8, selected_column: u8) -> (u8, String) {
        if selected_row == 7 {
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

    pub fn generate_possible_moves(&self, turn: bool) -> Vec<(Board, u8, u8)> {
        let mut outcome: Vec<(Board, u8, u8)> = Vec::new();
        for j in 0..8 {
            let mut board = self.clone();
            let row = self.get_peg_count_in_column(j) as u8;
            let result = board.play(turn, row, j);
            if result.0 == 0 {
                outcome.push((board, row, j));
            }
        }
        outcome
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

    pub fn get_peg_count_in_column(&self, choice: u8) -> usize {
        self.pegs
            .iter()
            .filter(|x| x.1 == choice)
            .collect::<Vec<&(u8, u8, bool)>>()
            .len()
    }

    pub fn print_board(&self, result: &(u8, String)) {
        let width = 20;
        println!(
            "{:=<width$}\n\x1b[2KLast move result: {}\n{:=<width$}",
            "=", result.1, "=",
        );
        let mut display_board: Vec<Vec<&str>> = vec![vec!["- "; 8]; 7];
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
        print!("\x1b[11F");
    }
}

impl Ord for Board {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Board {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cmp(self))
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Board {}
