#![allow(dead_code)]
pub fn first_word(s: &str) -> &str {
    if s.is_empty() {
        return "";
    }
    let trimmed = s.trim_start();
    match trimmed.split_once(' ') {
        Some((word, _)) => word,
        None => trimmed,
    }
}
