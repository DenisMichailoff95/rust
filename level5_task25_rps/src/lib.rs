#![allow(dead_code)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

pub fn beats(a: Move, b: Move) -> bool {
    matches!(
        (a, b),
        (Move::Rock, Move::Scissors) | (Move::Paper, Move::Rock) | (Move::Scissors, Move::Paper)
    )
}
