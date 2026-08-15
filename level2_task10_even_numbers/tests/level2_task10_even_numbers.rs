use level2_task10_even_numbers::*;

#[test]
fn test_even_numbers() {
    assert_eq!(even_numbers(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
    assert_eq!(even_numbers(&[]), Vec::<i32>::new());
    assert_eq!(even_numbers(&[1, 3, 5]), Vec::<i32>::new());
}
