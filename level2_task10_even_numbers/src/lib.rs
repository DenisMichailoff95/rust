#![allow(dead_code)]
pub fn even_numbers(nums: &[i32]) -> Vec<i32> {
    nums.iter().filter(|x| *x % 2 == 0).copied().collect()
}
