use std::collections::HashMap;
use std::io;

fn main() {
    let mut book: HashMap<String, String> = HashMap::new();
    loop {
        println!("1) add  2) find  3) remove  4) list  5) quit");
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();
        match cmd.trim() {
            "1" => {
                println!("Name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                println!("Phone:");
                let mut phone = String::new();
                io::stdin().read_line(&mut phone).unwrap();
                book.insert(name.trim().to_string(), phone.trim().to_string());
            }
            "2" => {
                println!("Name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                match book.get(name.trim()) {
                    Some(p) => println!("Phone: {p}"),
                    None => println!("Not found"),
                }
            }
            "3" => {
                println!("Name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                book.remove(name.trim());
            }
            "4" => {
                for (name, phone) in &book {
                    println!("{name}: {phone}");
                }
            }
            "5" => break,
            _ => println!("Unknown command"),
        }
    }
}
