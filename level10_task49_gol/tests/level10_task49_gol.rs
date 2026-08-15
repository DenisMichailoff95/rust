use level10_task49_gol::*;

#[test]
fn test_next_gen() {
    let grid = vec![
        vec![false, true, false],
        vec![false, true, false],
        vec![false, true, false],
    ];
    let next = next_gen(&grid);
    assert_eq!(next[0][0], false);
    assert_eq!(next[0][1], false);
    assert_eq!(next[1][1], true);
    assert_eq!(next[2][1], false);
}
