#![allow(dead_code)]
pub fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(|c| c.to_lowercase())
        .collect();

    if cleaned.is_empty() {
        return true;
    }

    cleaned.chars().eq(cleaned.chars().rev())
}
