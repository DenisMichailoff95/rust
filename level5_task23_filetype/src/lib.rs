#![allow(dead_code)]
pub enum FileType {
    Text(String),
    Image { width: u32, height: u32 },
    Binary(Vec<u8>),
}

pub fn print_info(ft: &FileType) {
    match ft {
        FileType::Text(s) => println!("Text: {s}"),
        FileType::Image { width, height } => println!("Image: {width}x{height}"),
        FileType::Binary(b) => println!("Binary: {} bytes", b.len()),
    }
}
