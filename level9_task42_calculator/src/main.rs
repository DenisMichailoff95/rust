use std::io;

fn calculate(expr: &str) -> Option<f64> {
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.len() != 3 {
        return None;
    }
    let a: f64 = parts[0].parse().ok()?;
    let op = parts[1];
    let b: f64 = parts[2].parse().ok()?;
    match op {
        "+" => Some(a + b),
        "*" => Some(a * b),
        _ => None,
    }
}

fn main() {
    loop {
        println!("Enter expression (or quit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "quit" {
            break;
        }
        match calculate(input) {
            Some(r) => println!("= {r}"),
            None => println!("Error"),
        }
    }
}
