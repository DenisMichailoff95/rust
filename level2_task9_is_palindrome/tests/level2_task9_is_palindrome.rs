use level2_task9_is_palindrome::*;

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome("racecar"));
    assert!(is_palindrome("madam"));
    assert!(!is_palindrome("hello"));
    assert!(is_palindrome("A man a plan a canal Panama"));
}
