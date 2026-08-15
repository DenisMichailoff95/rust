#![allow(dead_code)]
pub struct Book {
    pub title: String,
    pub author: String,
    year: u16,
}

impl Book {
    pub fn new(title: &str, author: &str, year: u16) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year,
        }
    }

    pub fn description(&self) -> String {
        format!("{} ({}) by {}", self.title, self.year, self.author)
    }
}
