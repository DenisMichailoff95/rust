use level6_task26_safe_div::*;

#[test]
fn test_safe_div() {
    assert_eq!(safe_div(10.0, 2.0), Some(5.0));
    assert_eq!(safe_div(10.0, 0.0), None);
    assert_eq!(safe_div(0.0, 5.0), Some(0.0));
}
