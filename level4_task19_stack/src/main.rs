struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

fn main() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    println!("Peek: {:?}", stack.peek());
    println!("Pop: {:?}", stack.pop());
    println!("Empty: {}", stack.is_empty());
}
