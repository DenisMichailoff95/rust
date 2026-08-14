use std::collections::BinaryHeap;

struct PriorityQueue<T: Ord> {
    heap: BinaryHeap<T>,
}

impl<T: Ord> PriorityQueue<T> {
    fn new() -> Self {
        PriorityQueue { heap: BinaryHeap::new() }
    }

    fn push(&mut self, item: T) {
        self.heap.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.heap.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.heap.peek()
    }
}

fn main() {
    let mut pq: PriorityQueue<i32> = PriorityQueue::new();
    pq.push(3);
    pq.push(1);
    pq.push(2);
    println!("Peek: {:?}", pq.peek());
    println!("Pop: {:?}", pq.pop());
}
