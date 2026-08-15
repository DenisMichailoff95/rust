use level3_task11_length::*;

#[test]
fn test_calculate_length() {
    assert_eq!(calculate_length(&String::from("hello")), 5);
    assert_eq!(calculate_length(&String::from("")), 0);
    assert_eq!(calculate_length(&String::from("rust")), 4);
}

#[test]
fn test_original_still_usable() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    assert_eq!(len, 5);
    assert_eq!(s, "hello");
}
