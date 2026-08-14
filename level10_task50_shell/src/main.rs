use std::io::{self, Write};

enum Command {
    Cd(String),
    Pwd,
    Ls,
    History,
    Exit,
}

fn parse(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts.get(0)? {
        &"cd" => Some(Command::Cd(parts.get(1)?.to_string())),
        &"pwd" => Some(Command::Pwd),
        &"ls" => Some(Command::Ls),
        &"history" => Some(Command::History),
        &"exit" => Some(Command::Exit),
        _ => None,
    }
}

fn main() {
    let mut history: Vec<String> = Vec::new();
    let mut current_dir = "/".to_string();
    loop {
        print!("{current_dir}$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        history.push(input.to_string());
        match parse(input) {
            Some(Command::Cd(path)) => current_dir = path,
            Some(Command::Pwd) => println!("{current_dir}"),
            Some(Command::Ls) => println!("file1.txt file2.txt dir1/"),
            Some(Command::History) => {
                for (i, cmd) in history.iter().enumerate() {
                    println!("{i}: {cmd}");
                }
            }
            Some(Command::Exit) => break,
            None => println!("Unknown command"),
        }
    }
}
