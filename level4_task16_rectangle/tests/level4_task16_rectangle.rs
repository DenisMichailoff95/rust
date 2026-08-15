use level4_task16_rectangle::*;

#[test]
fn test_rectangle_area() {
    let r = Rectangle {
        width: 10.0,
        height: 5.0,
    };
    assert!((r.area() - 50.0).abs() < 1e-10);
}

#[test]
fn test_rectangle_perimeter() {
    let r = Rectangle {
        width: 10.0,
        height: 5.0,
    };
    assert!((r.perimeter() - 30.0).abs() < 1e-10);
}

#[test]
fn test_rectangle_can_hold() {
    let r1 = Rectangle {
        width: 10.0,
        height: 5.0,
    };
    let r2 = Rectangle {
        width: 3.0,
        height: 2.0,
    };
    assert!(r1.can_hold(&r2));
    assert!(!r2.can_hold(&r1));
}
