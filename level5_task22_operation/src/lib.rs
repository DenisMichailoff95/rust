#![allow(dead_code)]
#[derive(Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn apply(op: Operation, a: f64, b: f64) -> Option<f64> {
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
