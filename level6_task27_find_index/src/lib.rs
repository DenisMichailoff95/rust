#![allow(dead_code)]
pub fn find_index<T: PartialEq>(slice: &[T], item: T) -> Option<usize> {
    for (i, x) in slice.iter().enumerate() {
        if *x == item {
            return Some(i);
        }
    }
    None
}
