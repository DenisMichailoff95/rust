use std::collections::HashMap;

fn convert(amount: f64, from: &str, to: &str, rates: &HashMap<String, f64>) -> Result<f64, String> {
    let from_rate = rates.get(from).ok_or("Unknown from currency")?;
    let to_rate = rates.get(to).ok_or("Unknown to currency")?;
    Ok(amount / from_rate * to_rate)
}

fn main() {
    let mut rates = HashMap::new();
    rates.insert("USD".to_string(), 1.0);
    rates.insert("EUR".to_string(), 0.85);
    println!("{:?}", convert(100.0, "USD", "EUR", &rates));
    println!("{:?}", convert(100.0, "USD", "GBP", &rates));
}
