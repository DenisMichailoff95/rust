use std::io;

fn main() {
    let mut x:f64 = 5.1;
    println!("The value of x is: {}", x);
    x = 6.2;
    x = x/3.0;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);


    let a = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);

    for element in &a[1..4] {
        println!("Element: {}", element);
    }

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let mut dynamic_string = String::from("Привет");
    println!("The value of dynamic_string is: {}", dynamic_string);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("The value of months is: {:?}", months);



    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
