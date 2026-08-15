use level5_task21_direction::*;

#[test]
fn test_move_point() {
    assert_eq!(move_point((0, 0), Direction::Up), (0, 1));
    assert_eq!(move_point((0, 0), Direction::Down), (0, -1));
    assert_eq!(move_point((0, 0), Direction::Left), (-1, 0));
    assert_eq!(move_point((0, 0), Direction::Right), (1, 0));
}

#[test]
fn test_move_point_multiple() {
    let pos = (5, 5);
    assert_eq!(move_point(pos, Direction::Up), (5, 6));
    assert_eq!(move_point(pos, Direction::Right), (6, 5));
    assert_eq!(
        move_point(move_point(pos, Direction::Up), Direction::Left),
        (4, 6)
    );
}
