fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let text = "Привет, мир!";
    println!("Original: {text}");
    println!("Reversed: {}", reverse_string(text));
}
