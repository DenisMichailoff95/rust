#![allow(dead_code)]
pub fn sum_of_squares(n: u32) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=n {
        sum += (i as u64) * (i as u64);
    }
    sum
}
