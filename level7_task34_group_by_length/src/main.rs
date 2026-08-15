use std::collections::HashMap;

fn group_by_length(strings: &[String]) -> HashMap<usize, Vec<String>> {
    let mut map: HashMap<usize, Vec<String>> = HashMap::new();
    for s in strings {
        map.entry(s.len()).or_default().push(s.clone());
    }
    map
}

fn main() {
    let strings = vec![
        "a".into(),
        "ab".into(),
        "abc".into(),
        "de".into(),
        "fgh".into(),
    ];
    let grouped = group_by_length(&strings);
    for (len, words) in &grouped {
        println!("{len}: {:?}", words);
    }
}
