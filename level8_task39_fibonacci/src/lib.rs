#![allow(dead_code)]

pub fn fibonacci(n: usize) -> Vec<i32> {
    std::iter::successors(Some((0, 1)), |(a, b)| Some((*b, a + b)))
        .take(n)
        .map(|(a, _)| a)
        .collect()
}
