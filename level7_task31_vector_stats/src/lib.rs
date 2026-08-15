#![allow(dead_code)]
pub fn vector_stats(v: Vec<i32>) -> (i32, i32, i32, f64) {
    let min = *v.iter().min().unwrap();
    let max = *v.iter().max().unwrap();
    let sum: i32 = v.iter().sum();
    let avg = sum as f64 / v.len() as f64;
    (min, max, sum, avg)
}
