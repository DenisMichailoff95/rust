#![allow(dead_code)]
use std::collections::HashSet;

pub fn intersection(a: &[i32], b: &[i32]) -> Vec<i32> {
    let set_a: HashSet<_> = a.iter().collect();
    let set_b: HashSet<_> = b.iter().collect();
    set_a.intersection(&set_b).map(|x| **x).collect()
}
