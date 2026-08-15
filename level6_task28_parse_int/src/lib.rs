#![allow(dead_code)]
pub fn parse_int(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("'{s}' is not a valid integer"))
}
