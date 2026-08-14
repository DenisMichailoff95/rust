use std::io;

fn main() {
    println!("Enter a:");
    let mut a_input = String::new();
    io::stdin()
        .read_line(&mut a_input)
        .expect("Failed to read line");
    let a: i32 = a_input
        .trim()
        .parse()
        .expect("Please enter a valid integer");

    println!("Enter b:");
    let mut b_input = String::new();
    io::stdin()
        .read_line(&mut b_input)
        .expect("Failed to read line");
    let b: i32 = b_input
        .trim()
        .parse()
        .expect("Please enter a valid integer");

    let start = a.min(b);
    let end = a.max(b);

    let evens: Vec<i32> = (start..=end).filter(|x| x % 2 == 0).collect();
    println!("Even numbers in [{start}, {end}]: {:?}", evens);
}
