#[allow(clippy::ptr_arg)]
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("'{s}' has length {len}");
    println!("Still usable: {s}");
}
