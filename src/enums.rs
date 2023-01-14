#[derive(PartialEq, Eq)]

pub enum GameState {
    NoError,
    Finished,
    IntegerError,
    ColumnIsFull,
    InputError,
}
#[derive(PartialEq, Eq)]
pub enum PlayType {
    HumanVsHuman,
    HumanVsAI,
    AIVsAI,
}
