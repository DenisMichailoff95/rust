#![allow(dead_code)]
pub fn read_file_to_string(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
