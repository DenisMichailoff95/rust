use chrono::NaiveDateTime;
use std::io::{self, Write};
use tokio_postgres::{Client, NoTls};

struct DatabaseManager {
    client: Client,
}

impl DatabaseManager {
    async fn connect(
        host: &str,
        port: &str,
        dbname: &str,
        user: &str,
        password: &str,
    ) -> Result<Self, anyhow::Error> {
        let config_str = format!(
            "host={} port={} dbname={} user={} password={}",
            host, port, dbname, user, password
        );

        let (client, connection) = tokio_postgres::connect(&config_str, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        println!("Successfully connected to database!");
        Ok(DatabaseManager { client })
    }

    async fn create_table(&self) -> Result<(), anyhow::Error> {
        self.client
            .execute(
                "CREATE TABLE IF NOT EXISTS notes (
                    id SERIAL PRIMARY KEY,
                    note TEXT NOT NULL,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )",
                &[],
            )
            .await?;

        Ok(())
    }

    async fn insert_note(&self, note: &str) -> Result<(), anyhow::Error> {
        self.client
            .execute("INSERT INTO notes (note) VALUES ($1)", &[&note])
            .await?;

        println!("Note added successfully!");
        Ok(())
    }

    async fn display_all_notes(&self) -> Result<(), anyhow::Error> {
        let rows = self
            .client
            .query("SELECT id, note, created_at FROM notes ORDER BY id", &[])
            .await?;

        if rows.is_empty() {
            println!("\nNo notes found in database.");
        } else {
            println!("\n{}", "=".repeat(80));
            println!("{:<5} {:<50} {:<25}", "ID", "Note", "Created At");
            println!("{}", "-".repeat(80));

            // Используем ссылку на rows
            for row in &rows {
                let id: i32 = row.get(0);
                let note: &str = row.get(1);
                let created_at: NaiveDateTime = row.get(2);

                let mut note_display = note.to_string();
                if note_display.len() > 47 {
                    let truncated: String = note_display.chars().take(44).collect();
                    note_display = format!("{}...", truncated);
                }

                println!(
                    "{:<5} {:<50} {:<25}",
                    id,
                    note_display,
                    created_at.format("%Y-%m-%d %H:%M:%S")
                );
            }
            println!("{}", "=".repeat(80));
            println!("Total: {} note(s)", rows.len());
        }

        Ok(())
    }

    async fn delete_note(&self, id: i32) -> Result<(), anyhow::Error> {
        let affected_rows = self
            .client
            .execute("DELETE FROM notes WHERE id = $1", &[&id])
            .await?;

        if affected_rows > 0 {
            println!("Note deleted successfully!");
        } else {
            println!("Note with ID {} not found!", id);
        }

        Ok(())
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let host = "localhost";
    let port = "5433";
    let dbname = "postgres";
    let user = "postgres";
    let password = "alpine_password"; // Если пароль есть, укажите его

    let db = match DatabaseManager::connect(host, port, dbname, user, password).await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return Ok(());
        }
    };

    if let Err(e) = db.create_table().await {
        eprintln!("Failed to create table: {}", e);
        return Ok(());
    }

    loop {
        println!("\n=== Notes Database Manager ===");
        println!("1. Add new note");
        println!("2. View all notes");
        println!("3. Delete note");
        println!("4. Exit");
        print!("Choose option: ");
        io::stdout().flush().unwrap();

        let choice = read_input("");

        match choice.as_str() {
            "1" => {
                let note = read_input("Enter your note: ");
                if !note.is_empty() {
                    if let Err(e) = db.insert_note(&note).await {
                        eprintln!("Failed to add note: {}", e);
                    }
                } else {
                    println!("Note cannot be empty!");
                }
            }
            "2" => {
                if let Err(e) = db.display_all_notes().await {
                    eprintln!("Failed to display notes: {}", e);
                }
            }
            "3" => {
                let id_str = read_input("Enter note ID to delete: ");
                match id_str.parse::<i32>() {
                    Ok(id) => {
                        if let Err(e) = db.delete_note(id).await {
                            eprintln!("Failed to delete note: {}", e);
                        }
                    }
                    Err(_) => println!("Invalid ID!"),
                }
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option! Please try again."),
        }
    }

    Ok(())
}
