use level8_task38_any_positive::*;

#[test]
fn test_any_positive() {
    assert!(any_positive(&[1, -2, 3]));
    assert!(!any_positive(&[-1, -2, -3]));
    assert!(!any_positive(&[]));
    assert!(any_positive(&[0, 0, 1]));
}
