#![allow(dead_code)]
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn move_point(pos: (i32, i32), dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (pos.0, pos.1 + 1),
        Direction::Down => (pos.0, pos.1 - 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}
