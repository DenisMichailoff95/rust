use level2_task7_reverse_string::*;

#[test]
fn test_reverse_string() {
    assert_eq!(reverse_string("hello"), "olleh");
    assert_eq!(reverse_string(""), "");
    assert_eq!(reverse_string("a"), "a");
    assert_eq!(reverse_string("Rust"), "tsuR");
}
