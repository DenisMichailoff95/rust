use level2_task6_count_vowels::*;

#[test]
fn test_count_vowels_empty() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("   "), 0);
}

#[test]
fn test_count_vowels_english() {
    assert_eq!(count_vowels("hello"), 2);
    assert_eq!(count_vowels("aeiou"), 5);
    assert_eq!(count_vowels("AEIOU"), 5);
    assert_eq!(count_vowels("rhythm"), 1);
}

#[test]
fn test_count_vowels_russian() {
    assert_eq!(count_vowels("Привет"), 2);
    assert_eq!(count_vowels("Аеёиоу"), 6);
}

#[test]
fn test_count_vowels_mixed() {
    assert_eq!(count_vowels("hello world"), 3);
}
