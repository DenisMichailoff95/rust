#![allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum Command {
    Add(f64, f64),
    Sub(f64, f64),
    Quit,
}

pub fn parse_command(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    match *parts.first()? {
        "quit" => Some(Command::Quit),
        "add" => {
            let a: f64 = parts.get(1)?.parse().ok()?;
            let b: f64 = parts.get(2)?.parse().ok()?;
            Some(Command::Add(a, b))
        }
        "sub" => {
            let a: f64 = parts.get(1)?.parse().ok()?;
            let b: f64 = parts.get(2)?.parse().ok()?;
            Some(Command::Sub(a, b))
        }
        _ => None,
    }
}
