fn first_word(s: &str) -> &str {
    if s.is_empty() {
        return "";
    }
    match s.split_once(' ') {
        Some((word, _)) => word,
        None => s,
    }
}

fn main() {
    let cases = [
        "hello world",
        "rust programming",
        "single",
        "",
        "  leading spaces",
    ];
    for text in &cases {
        println!("{text:?} -> {text:?}", text = first_word(text));
    }
}
