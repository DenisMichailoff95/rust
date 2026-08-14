use std::io;

enum Command {
    Add(f64, f64),
    Sub(f64, f64),
    Quit,
}

fn parse_command(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts.get(0)? {
        &"quit" => Some(Command::Quit),
        &"add" => {
            let a: f64 = parts.get(1)?.parse().ok()?;
            let b: f64 = parts.get(2)?.parse().ok()?;
            Some(Command::Add(a, b))
        }
        &"sub" => {
            let a: f64 = parts.get(1)?.parse().ok()?;
            let b: f64 = parts.get(2)?.parse().ok()?;
            Some(Command::Sub(a, b))
        }
        _ => None,
    }
}

fn main() {
    loop {
        println!("Enter command (add x y / sub x y / quit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let cmd = parse_command(input.trim());
        match cmd {
            Some(Command::Add(a, b)) => println!("Result: {}", a + b),
            Some(Command::Sub(a, b)) => println!("Result: {}", a - b),
            Some(Command::Quit) => break,
            None => println!("Unknown command"),
        }
    }
}
