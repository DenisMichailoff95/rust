use level1_task3_sum_of_squares::*;

#[test]
fn test_sum_of_squares() {
    assert_eq!(sum_of_squares(1), 1);
    assert_eq!(sum_of_squares(2), 5);
    assert_eq!(sum_of_squares(3), 14);
    assert_eq!(sum_of_squares(10), 385);
}
