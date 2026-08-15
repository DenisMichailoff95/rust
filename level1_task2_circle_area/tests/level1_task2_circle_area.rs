use level1_task2_circle_area::*;

#[test]
fn test_circle_area() {
    assert!((circle_area(0.0) - 0.0).abs() < 1e-10);
    assert!((circle_area(1.0) - std::f64::consts::PI).abs() < 1e-10);
    assert!((circle_area(2.0) - 4.0 * std::f64::consts::PI).abs() < 1e-10);
    assert!((circle_area(5.0) - 25.0 * std::f64::consts::PI).abs() < 1e-10);
}
