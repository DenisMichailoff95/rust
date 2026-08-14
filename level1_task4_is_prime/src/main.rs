fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }
    let limit = (n as f64).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        if n.is_multiple_of(i) {
            return false;
        }
    }
    true
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 16, 17, 19, 20, 23, 29, 97];
    for &num in &numbers {
        println!("{num} is prime: {}", is_prime(num));
    }
}
