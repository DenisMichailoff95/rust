use level9_task44_priority_queue::*;

#[test]
fn test_priority_queue() {
    let mut pq: PriorityQueue<i32> = PriorityQueue::new();
    pq.push(3);
    pq.push(1);
    pq.push(2);
    assert_eq!(pq.heap.pop(), Some(3));
    assert_eq!(pq.heap.pop(), Some(2));
    assert_eq!(pq.heap.pop(), Some(1));
}
