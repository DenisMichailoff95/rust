use level9_task42_calculator::*;

#[test]
fn test_calculate_add() {
    assert_eq!(calculate("1 + 2"), Some(3.0));
}

#[test]
fn test_calculate_multiply() {
    assert_eq!(calculate("2 * 3"), Some(6.0));
}

#[test]
fn test_calculate_invalid() {
    assert_eq!(calculate("1 +"), None);
    assert_eq!(calculate("abc"), None);
}
