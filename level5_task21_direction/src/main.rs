use std::io;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_point(pos: (i32, i32), dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (pos.0, pos.1 + 1),
        Direction::Down => (pos.0, pos.1 - 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}

fn main() {
    let pos = (0, 0);
    let dir = Direction::Up;
    println!("{:?}", move_point(pos, dir));
}
