#![allow(dead_code)]
pub fn calculate(expr: &str) -> Option<f64> {
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.len() != 3 {
        return None;
    }
    let a: f64 = parts[0].parse().ok()?;
    let op = parts[1];
    let b: f64 = parts[2].parse().ok()?;
    match op {
        "+" => Some(a + b),
        "*" => Some(a * b),
        _ => None,
    }
}
