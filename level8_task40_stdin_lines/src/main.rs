use std::io;

fn main() {
    let sum: i32 = io::stdin().lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.trim().parse::<i32>().ok())
        .sum();
    println!("Sum: {sum}");
}
