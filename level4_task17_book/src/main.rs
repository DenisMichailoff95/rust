struct Book {
    title: String,
    author: String,
    year: u16,
}

impl Book {
    fn new(title: &str, author: &str, year: u16) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year,
        }
    }

    fn description(&self) -> String {
        format!("{} ({}) by {}", self.title, self.year, self.author)
    }
}

fn main() {
    let book = Book::new("Rust Book", "Author", 2023);
    println!("{}", book.description());
}
