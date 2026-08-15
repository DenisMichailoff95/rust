use level2_task8_first_word::*;

#[test]
fn test_first_word() {
    assert_eq!(first_word("hello world"), "hello");
    assert_eq!(first_word(""), "");
    assert_eq!(first_word("single"), "single");
    assert_eq!(first_word("  leading"), "leading");
}
