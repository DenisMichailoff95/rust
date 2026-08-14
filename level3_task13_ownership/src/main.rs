fn take_ownership(v: Vec<i32>) {
    let sum: i32 = v.iter().sum();
    println!("Sum: {sum}");
}

fn borrow_sum(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let sum = borrow_sum(&v);
    println!("Sum via borrow: {sum}");
    println!("Vector still usable: {:?}", v);
}
