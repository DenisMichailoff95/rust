fn vector_stats(v: Vec<i32>) -> (i32, i32, i32, f64) {
    let min = *v.iter().min().unwrap();
    let max = *v.iter().max().unwrap();
    let sum: i32 = v.iter().sum();
    let avg = sum as f64 / v.len() as f64;
    (min, max, sum, avg)
}

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (min, max, sum, avg) = vector_stats(v);
    println!("min={min}, max={max}, sum={sum}, avg={avg}");
}
