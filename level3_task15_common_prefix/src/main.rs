fn common_prefix<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    let mut i = 0;
    let min_len = s1.len().min(s2.len());
    while i < min_len && s1.as_bytes()[i] == s2.as_bytes()[i] {
        i += 1;
    }
    &s1[..i]
}

fn main() {
    println!("{}", common_prefix("hello", "help"));
    println!("{}", common_prefix("rust", "rustacean"));
    println!("{}", common_prefix("abc", "def"));
}
