use board::Board;

use crate::game::Game;

pub mod board;
pub mod enums;
pub mod game;
pub mod minimax;
fn main() {
    Game::main_loop();
}
