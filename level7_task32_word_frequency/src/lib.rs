#![allow(dead_code)]
use std::collections::HashMap;

pub fn word_frequency(text: &str) -> HashMap<String, u32> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word.to_string()).or_insert(0) += 1;
    }
    freq
}
