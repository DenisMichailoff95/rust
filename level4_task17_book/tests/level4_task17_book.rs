use level4_task17_book::*;

#[test]
fn test_book_description() {
    let book = Book::new("Rust Book", "Author", 2023);
    assert_eq!(book.description(), "Rust Book (2023) by Author");
}
