#![allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Command {
    Cd(String),
    Pwd,
    Ls,
    History,
    Exit,
}

pub fn parse(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    match *parts.first()? {
        "cd" => Some(Command::Cd(parts.get(1)?.to_string())),
        "pwd" => Some(Command::Pwd),
        "ls" => Some(Command::Ls),
        "history" => Some(Command::History),
        "exit" => Some(Command::Exit),
        _ => None,
    }
}
