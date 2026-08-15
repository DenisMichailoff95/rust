enum FileType {
    Text(String),
    Image { width: u32, height: u32 },
    Binary(Vec<u8>),
}

fn print_info(ft: &FileType) {
    match ft {
        FileType::Text(s) => println!("Text: {s}"),
        FileType::Image { width, height } => println!("Image: {width}x{height}"),
        FileType::Binary(b) => println!("Binary: {} bytes", b.len()),
    }
}

fn main() {
    print_info(&FileType::Text(String::from("hello")));
    print_info(&FileType::Image {
        width: 1920,
        height: 1080,
    });
    print_info(&FileType::Binary(vec![0, 1, 2]));
}
