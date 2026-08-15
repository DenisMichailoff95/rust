#![allow(dead_code)]
use std::collections::HashMap;

pub fn group_by_length(strings: &[String]) -> HashMap<usize, Vec<String>> {
    let mut map: HashMap<usize, Vec<String>> = HashMap::new();
    for s in strings {
        map.entry(s.len()).or_default().push(s.clone());
    }
    map
}
