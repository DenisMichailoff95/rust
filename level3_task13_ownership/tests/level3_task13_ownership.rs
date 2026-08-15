use level3_task13_ownership::*;

#[test]
fn test_take_ownership() {
    let v = vec![1, 2, 3];
    take_ownership(v);
}

#[test]
fn test_borrow_sum() {
    let v = vec![1, 2, 3, 4];
    assert_eq!(borrow_sum(&v), 10);
    assert_eq!(borrow_sum(&[]), 0);
}
