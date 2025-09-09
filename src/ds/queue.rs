//! Queue/Deque: FIFO and double-ended queues for BFS and sliding windows.

use std::collections::VecDeque;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Queue<T> { q: VecDeque<T> }

impl<T> Queue<T> {
    pub fn new() -> Self { Self { q: VecDeque::new() } }
    pub fn push(&mut self, x: T) { self.q.push_back(x) }
    pub fn pop(&mut self) -> Option<T> { self.q.pop_front() }
    pub fn peek(&self) -> Option<&T> { self.q.front() }
    pub fn is_empty(&self) -> bool { self.q.is_empty() }
    pub fn len(&self) -> usize { self.q.len() }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Deque<T> { d: VecDeque<T> }

impl<T> Deque<T> {
    pub fn new() -> Self { Self { d: VecDeque::new() } }
    pub fn push_front(&mut self, x: T) { self.d.push_front(x) }
    pub fn push_back(&mut self, x: T) { self.d.push_back(x) }
    pub fn pop_front(&mut self) -> Option<T> { self.d.pop_front() }
    pub fn pop_back(&mut self) -> Option<T> { self.d.pop_back() }
    pub fn front(&self) -> Option<&T> { self.d.front() }
    pub fn back(&self) -> Option<&T> { self.d.back() }
    pub fn is_empty(&self) -> bool { self.d.is_empty() }
    pub fn len(&self) -> usize { self.d.len() }
}

#[cfg(test)]
mod tests {
    use super::{Queue, Deque};
    #[test]
    fn queue_basic() {
        let mut q = Queue::new();
        q.push(1); q.push(2);
        assert_eq!(q.peek(), Some(&1));
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), Some(2));
        assert_eq!(q.pop(), None);
        assert!(q.is_empty());
    }
    #[test]
    fn deque_basic() {
        let mut d = Deque::new();
        d.push_back(2); d.push_front(1); d.push_back(3);
        assert_eq!(d.front(), Some(&1));
        assert_eq!(d.back(), Some(&3));
        assert_eq!(d.pop_front(), Some(1));
        assert_eq!(d.pop_back(), Some(3));
        assert_eq!(d.pop_back(), Some(2));
        assert!(d.is_empty());
        assert_eq!(d.len(), 0);
    }

    #[test]
    fn queue_len() {
        let mut q = Queue::new();
        assert_eq!(q.len(), 0);
        q.push(1);
        assert_eq!(q.len(), 1);
    }
}
