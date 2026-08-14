fn add_exclamation(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut s = String::from("hello");
    println!("Before: {s}");
    add_exclamation(&mut s);
    println!("After: {s}");
}
