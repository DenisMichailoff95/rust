#![allow(dead_code)]
use std::collections::VecDeque;

pub fn generate_password(length: usize) -> String {
    let chars: Vec<u8> = (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .chain(b'0'..=b'9')
        .collect();
    let mut queue: VecDeque<u8> = VecDeque::new();
    for i in 0..length {
        let idx = (i * 7 + 13) % chars.len();
        queue.push_back(chars[idx]);
    }
    queue.iter().map(|&b| b as char).collect()
}
