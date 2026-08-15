use level7_task32_word_frequency::*;

#[test]
fn test_word_frequency() {
    let freq = word_frequency("hello world hello rust");
    assert_eq!(freq.get("hello"), Some(&2));
    assert_eq!(freq.get("world"), Some(&1));
    assert_eq!(freq.get("rust"), Some(&1));
}
