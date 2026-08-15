#![allow(dead_code)]
pub fn take_ownership(v: Vec<i32>) {
    let sum: i32 = v.iter().sum();
    println!("Sum: {sum}");
}

pub fn borrow_sum(v: &[i32]) -> i32 {
    v.iter().sum()
}
