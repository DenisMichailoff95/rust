use std::io;

fn main() {
    let sum: i32 = io::stdin().lines()
        .map_while(Result::ok)
        .filter_map(|l| l.trim().parse::<i32>().ok())
        .sum();
    println!("Sum: {sum}");
}
