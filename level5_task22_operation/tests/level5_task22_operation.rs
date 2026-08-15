use level5_task22_operation::*;

#[test]
fn test_apply_add() {
    assert_eq!(apply(Operation::Add, 10.0, 5.0), Some(15.0));
}

#[test]
fn test_apply_divide_by_zero() {
    assert_eq!(apply(Operation::Divide, 10.0, 0.0), None);
}

#[test]
fn test_apply_subtract() {
    assert_eq!(apply(Operation::Subtract, 10.0, 5.0), Some(5.0));
}

#[test]
fn test_apply_multiply() {
    assert_eq!(apply(Operation::Multiply, 10.0, 5.0), Some(50.0));
}
