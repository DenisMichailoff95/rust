use level10_task48_currency::*;

#[test]
fn test_convert() {
    use std::collections::HashMap;
    let mut rates = HashMap::new();
    rates.insert("USD".to_string(), 1.0);
    rates.insert("EUR".to_string(), 0.92);
    assert_eq!(convert(100.0, "USD", "EUR", &rates), Ok(92.0));
}

#[test]
fn test_convert_unknown_currency() {
    use std::collections::HashMap;
    let mut rates = HashMap::new();
    rates.insert("USD".to_string(), 1.0);
    assert!(convert(100.0, "UNKNOWN", "USD", &rates).is_err());
}
