fn main() {
    let a: &str = "world";  // Declare and initialize a

    let i: i32 = 0;
    let mut j: i32 = 0;


    j = j + (i+10);

    println!("{}",j);
    
    println!("Hello, world! {}", a);  // Using correct Rust format
}