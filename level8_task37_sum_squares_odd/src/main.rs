fn main() {
    let sum: i32 = (1..100)
        .filter(|x| x % 2 != 0)
        .map(|x| x * x)
        .sum();
    println!("{sum}");
}
