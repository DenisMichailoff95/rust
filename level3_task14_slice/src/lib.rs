#![allow(dead_code)]
pub fn first_n_chars(s: &str, n: usize) -> &str {
    if n >= s.len() {
        return s;
    }
    &s[..n]
}
