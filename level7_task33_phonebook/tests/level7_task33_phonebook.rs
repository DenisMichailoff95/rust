#[test]
fn test_phonebook_smoke() {
    use std::collections::HashMap;
    let mut book: HashMap<String, String> = HashMap::new();
    book.insert("Alice".to_string(), "123".to_string());
    assert_eq!(book.get("Alice"), Some(&"123".to_string()));
}
