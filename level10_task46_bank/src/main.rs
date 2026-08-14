use std::collections::HashMap;

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        Ok(())
    }
}

fn main() {
    let mut accounts: HashMap<u32, BankAccount> = HashMap::new();
    let mut id = 1;
    loop {
        println!("1) create  2) deposit  3) withdraw  4) list  5) quit");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => {
                println!("Owner:");
                let mut owner = String::new();
                std::io::stdin().read_line(&mut owner).unwrap();
                accounts.insert(id, BankAccount { owner: owner.trim().to_string(), balance: 0.0 });
                println!("Created id={}", id);
                id += 1;
            }
            "2" => {
                println!("Id:");
                let mut id_str = String::new();
                std::io::stdin().read_line(&mut id_str).unwrap();
                let id: u32 = id_str.trim().parse().unwrap_or(0);
                println!("Amount:");
                let mut amount = String::new();
                std::io::stdin().read_line(&mut amount).unwrap();
                if let Some(acc) = accounts.get_mut(&id) {
                    println!("{:?}", acc.deposit(amount.trim().parse().unwrap_or(0.0)));
                }
            }
            "3" => {
                println!("Id:");
                let mut id_str = String::new();
                std::io::stdin().read_line(&mut id_str).unwrap();
                let id: u32 = id_str.trim().parse().unwrap_or(0);
                println!("Amount:");
                let mut amount = String::new();
                std::io::stdin().read_line(&mut amount).unwrap();
                if let Some(acc) = accounts.get_mut(&id) {
                    println!("{:?}", acc.withdraw(amount.trim().parse().unwrap_or(0.0)));
                }
            }
            "4" => {
                for (id, acc) in &accounts {
                    println!("{id}: {} -> {:.2}", acc.owner, acc.balance);
                }
            }
            "5" => break,
            _ => println!("Unknown"),
        }
    }
}
