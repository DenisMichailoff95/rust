#![allow(dead_code, clippy::new_without_default)]
use std::collections::BinaryHeap;

pub struct PriorityQueue<T: Ord> {
    pub heap: BinaryHeap<T>,
}

impl<T: Ord> PriorityQueue<T> {
    pub fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.heap.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek()
    }
}
