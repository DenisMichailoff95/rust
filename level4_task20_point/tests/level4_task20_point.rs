use level4_task20_point::*;

#[test]
fn test_point_origin() {
    let p = Point::origin();
    assert_eq!(p.x, 0.0);
    assert_eq!(p.y, 0.0);
}

#[test]
fn test_point_distance() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    assert!((p1.distance(&p2) - 5.0).abs() < 1e-10);
}
