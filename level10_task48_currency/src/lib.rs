#![allow(dead_code)]
use std::collections::HashMap;

pub fn convert(amount: f64, from: &str, to: &str, rates: &HashMap<String, f64>) -> Result<f64, String> {
    let from_rate = rates.get(from).ok_or("Unknown from currency")?;
    let to_rate = rates.get(to).ok_or("Unknown to currency")?;
    Ok(amount / from_rate * to_rate)
}
