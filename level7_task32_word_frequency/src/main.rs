use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, u32> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word.to_string()).or_insert(0) += 1;
    }
    freq
}

fn main() {
    let text = "hello world hello rust world rust rust";
    let freq = word_frequency(text);
    let mut vec: Vec<_> = freq.iter().collect();
    vec.sort_by_key(|(_, c)| **c);
    vec.reverse();
    for (word, count) in vec.iter().take(3) {
        println!("{word}: {count}");
    }
}
