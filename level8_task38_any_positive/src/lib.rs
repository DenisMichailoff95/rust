#![allow(dead_code)]
pub fn any_positive(vec: &[i32]) -> bool {
    vec.iter().any(|x| *x > 0)
}
