fn is_palindrome(s: &str) -> bool {
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

fn main() {
    let cases = [
        "А роза упала на лапу Азора",
        "racecar",
        "hello",
        "",
        "     ",
    ];
    for text in &cases {
        println!("{text:?} is palindrome: {}", is_palindrome(text));
    }
}
