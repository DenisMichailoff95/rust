fn any_positive(vec: &[i32]) -> bool {
    vec.iter().any(|x| *x > 0)
}

fn main() {
    println!("{}", any_positive(&[1, 2, 3]));
    println!("{}", any_positive(&[-1, -2, -3]));
    println!("{}", any_positive(&[]));
}
