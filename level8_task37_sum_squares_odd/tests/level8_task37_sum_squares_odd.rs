#[test]
fn test_sum_squares_odd() {
    let sum: i32 = (1..100).filter(|x| x % 2 != 0).map(|x| x * x).sum();
    assert_eq!(sum, 166_650);
}

#[test]
fn test_sum_squares_odd_small() {
    let sum: i32 = (1..10).filter(|x| x % 2 != 0).map(|x| x * x).sum();
    assert_eq!(sum, 1 + 9 + 25 + 49 + 81);
}
