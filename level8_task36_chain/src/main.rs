fn main() {
    let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter_solution: Vec<i32> = nums.iter().filter(|x| *x % 2 == 0).map(|x| x * x).collect();

    let mut loop_solution = Vec::new();
    for x in nums.iter() {
        if x % 2 == 0 {
            loop_solution.push(x * x);
        }
    }

    println!("Iterators: {:?}", iter_solution);
    println!("Loop: {:?}", loop_solution);
}
