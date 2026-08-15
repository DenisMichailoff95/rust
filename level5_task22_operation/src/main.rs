#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn apply(op: Operation, a: f64, b: f64) -> Option<f64> {
    match op {
        Operation::Add => Some(a + b),
        Operation::Subtract => Some(a - b),
        Operation::Multiply => Some(a * b),
        Operation::Divide => {
            if b == 0.0 {
                None
            } else {
                Some(a / b)
            }
        }
    }
}

fn main() {
    println!("{:?}", apply(Operation::Add, 10.0, 5.0));
    println!("{:?}", apply(Operation::Subtract, 10.0, 5.0));
    println!("{:?}", apply(Operation::Multiply, 10.0, 5.0));
    println!("{:?}", apply(Operation::Divide, 10.0, 0.0));
}
