use std::io;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn main() {
    println!("Enter temperature in Celsius:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let c: f64 = input.trim().parse().expect("Please enter a valid number");

    let f = celsius_to_fahrenheit(c);
    println!("{c}°C = {f}°F");

    let back = fahrenheit_to_celsius(f);
    println!("{f}°F = {back}°C");
}
