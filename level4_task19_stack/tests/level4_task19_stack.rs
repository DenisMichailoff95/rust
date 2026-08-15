use level4_task19_stack::*;

#[test]
fn test_stack_new() {
    let stack: Stack<i32> = Stack::new();
    assert!(stack.data.is_empty());
}

#[test]
fn test_stack_push_pop() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.data, vec![1, 2]);
}
