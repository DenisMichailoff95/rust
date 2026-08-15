const ARRAY: [i32; 10] = [4, 2, 9, 6, 1, 8, 3, 7, 5, 0];

fn main() {
    let sum: i32 = ARRAY.iter().sum();
    let count = ARRAY.len() as f64;
    let average = sum as f64 / count;

    let max = *ARRAY.iter().max().unwrap();
    let min = *ARRAY.iter().min().unwrap();

    println!("Array: {:?}", ARRAY);
    println!("Max: {max}");
    println!("Min: {min}");
    println!("Sum: {sum}");
    println!("Average: {average}");
}
