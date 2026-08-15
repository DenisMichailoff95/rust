use level9_task45_maze::*;

#[test]
fn test_solve_maze() {
    let mut maze = vec![
        vec![1, 1, 1],
        vec![1, 0, 1],
        vec![1, 1, 1],
    ];
    let result = solve(&mut maze, (0, 0), (2, 2));
    assert!(result.is_some());
}
