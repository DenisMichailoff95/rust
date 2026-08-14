fn main() {
    let fib: Vec<i32> = std::iter::successors(Some((0, 1)), |(a, b)| Some((*b, a + b)))
        .take(10)
        .map(|(a, _)| a)
        .collect();
    println!("{:?}", fib);
}
