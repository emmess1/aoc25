//! Priority queues (heaps): min-heap and max-heap wrappers for AoC tasks.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Clone, Debug, Default)]
pub struct MaxHeap<T: Ord>(BinaryHeap<T>);

impl<T: Ord> MaxHeap<T> {
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }
    pub fn push(&mut self, x: T) {
        self.0.push(x)
    }
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
    pub fn peek(&self) -> Option<&T> {
        self.0.peek()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug, Default)]
pub struct MinHeap<T: Ord>(BinaryHeap<Reverse<T>>);

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }
    pub fn push(&mut self, x: T) {
        self.0.push(Reverse(x))
    }
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop().map(|r| r.0)
    }
    pub fn peek(&self) -> Option<&T> {
        self.0.peek().map(|r| &r.0)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{MaxHeap, MinHeap};
    #[test]
    fn min_heap_basic() {
        let mut h = MinHeap::new();
        h.push(3);
        h.push(1);
        h.push(2);
        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.pop(), Some(2));
        assert_eq!(h.pop(), Some(3));
        assert!(h.is_empty());
    }
    #[test]
    fn max_heap_basic() {
        let mut h = MaxHeap::new();
        h.push(3);
        h.push(1);
        h.push(2);
        assert_eq!(h.peek(), Some(&3));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), Some(2));
        assert_eq!(h.pop(), Some(1));
        assert!(h.is_empty());
    }
    #[test]
    fn len_after_push_pop() {
        let mut h = MinHeap::new();
        assert!(h.is_empty());
        h.push(10);
        h.push(5);
        assert_eq!(h.len(), 2);
        h.pop();
        assert_eq!(h.len(), 1);
    }

    #[test]
    fn len_max_heap() {
        let mut h = MaxHeap::new();
        assert_eq!(h.len(), 0);
        h.push(1);
        h.push(2);
        assert_eq!(h.len(), 2);
    }
}
