use level3_task15_common_prefix::*;

#[test]
fn test_common_prefix() {
    assert_eq!(common_prefix("hello", "help"), "hel");
    assert_eq!(common_prefix("", "hello"), "");
    assert_eq!(common_prefix("abc", "def"), "");
    assert_eq!(common_prefix("same", "same"), "same");
}
