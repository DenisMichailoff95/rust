use std::collections::HashSet;

fn intersection(a: &[i32], b: &[i32]) -> Vec<i32> {
    let set_a: HashSet<_> = a.iter().collect();
    let set_b: HashSet<_> = b.iter().collect();
    set_a.intersection(&set_b).map(|x| **x).collect()
}

fn main() {
    let a = vec![1, 2, 3, 4, 5];
    let b = vec![4, 5, 6, 7, 8];
    println!("{:?}", intersection(&a, &b));
}
