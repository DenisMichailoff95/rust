use level3_task14_slice::*;

#[test]
fn test_first_n_chars() {
    assert_eq!(first_n_chars("hello", 3), "hel");
    assert_eq!(first_n_chars("hello", 10), "hello");
    assert_eq!(first_n_chars("", 5), "");
}
