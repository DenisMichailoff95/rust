use level1_task1_temperature::*;

#[test]
fn test_fahrenheit_to_celsius() {
    assert!((fahrenheit_to_celsius(32.0) - 0.0).abs() < 1e-10);
    assert!((fahrenheit_to_celsius(212.0) - 100.0).abs() < 1e-10);
    assert!((fahrenheit_to_celsius(-40.0) - (-40.0)).abs() < 1e-10);
}

#[test]
fn test_celsius_to_fahrenheit() {
    assert!((celsius_to_fahrenheit(0.0) - 32.0).abs() < 1e-10);
    assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < 1e-10);
    assert!((celsius_to_fahrenheit(-40.0) - (-40.0)).abs() < 1e-10);
}

#[test]
fn test_roundtrip() {
    for c in [-40.0, 0.0, 20.0, 100.0] {
        let f = celsius_to_fahrenheit(c);
        let back = fahrenheit_to_celsius(f);
        assert!((back - c).abs() < 1e-10);
    }
}
