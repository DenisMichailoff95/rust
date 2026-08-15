use level8_task36_chain::*;

#[test]
fn test_chain_evens_squared() {
    let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter_solution: Vec<i32> = nums.iter().filter(|x| *x % 2 == 0).map(|x| x * x).collect();
    let mut loop_solution = Vec::new();
    for x in nums.iter() {
        if x % 2 == 0 {
            loop_solution.push(x * x);
        }
    }
    assert_eq!(iter_solution, loop_solution);
    assert_eq!(iter_solution, vec![4, 16, 36, 64, 100]);
}
