fn first_n_chars(s: &str, n: usize) -> &str {
    if n >= s.len() {
        return s;
    }
    &s[..n]
}

fn main() {
    let s = "hello world";
    println!("{}", first_n_chars(s, 5));
    println!("{}", first_n_chars(s, 100));
}
