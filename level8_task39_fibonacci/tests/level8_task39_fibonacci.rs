use level8_task39_fibonacci::*;

#[test]
fn test_fibonacci() {
    let fib: Vec<i32> = fibonacci(10);
    assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
}

#[test]
fn test_fibonacci_first_two() {
    let fib: Vec<i32> = fibonacci(2);
    assert_eq!(fib, vec![0, 1]);
}
