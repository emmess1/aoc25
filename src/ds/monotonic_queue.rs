//! Monotonic queues for sliding window min/max in O(N).

use std::collections::VecDeque;

#[derive(Clone, Debug, Default)]
pub struct MonotonicQueueMin<T: Ord + Copy> {
    dq: VecDeque<T>,
}
impl<T: Ord + Copy> MonotonicQueueMin<T> {
    pub fn new() -> Self { Self { dq: VecDeque::new() } }
    /// Push value; removes larger elements from the back.
    pub fn push(&mut self, x: T) {
        while let Some(&back) = self.dq.back() { if back > x { self.dq.pop_back(); } else { break; } }
        self.dq.push_back(x);
    }
    /// Pop from front if it equals x (use when sliding window removes x).
    pub fn pop_if(&mut self, x: T) { if self.dq.front().copied() == Some(x) { self.dq.pop_front(); } }
    pub fn min(&self) -> Option<T> { self.dq.front().copied() }
    pub fn len(&self) -> usize { self.dq.len() }
    pub fn is_empty(&self) -> bool { self.dq.is_empty() }
}

#[derive(Clone, Debug, Default)]
pub struct MonotonicQueueMax<T: Ord + Copy> { dq: VecDeque<T> }
impl<T: Ord + Copy> MonotonicQueueMax<T> {
    pub fn new() -> Self { Self { dq: VecDeque::new() } }
    pub fn push(&mut self, x: T) { while let Some(&b)=self.dq.back(){ if b < x { self.dq.pop_back(); } else { break; }} self.dq.push_back(x); }
    pub fn pop_if(&mut self, x: T) { if self.dq.front().copied() == Some(x) { self.dq.pop_front(); } }
    pub fn max(&self) -> Option<T> { self.dq.front().copied() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sliding_min() {
        let a = [4,2,12,3,1,5];
        let mut mq = MonotonicQueueMin::new();
        let k = 3; let mut out = Vec::new();
        for i in 0..a.len() {
            mq.push(a[i]);
            if i >= k-1 { out.push(mq.min().unwrap()); mq.pop_if(a[i+1-k]); }
        }
        assert_eq!(out, vec![2,2,1,1]);
    }
    #[test]
    fn sliding_max() {
        let a = [1,3,2,5,4];
        let mut mq = MonotonicQueueMax::new();
        let k = 2; let mut out = Vec::new();
        for i in 0..a.len() { mq.push(a[i]); if i>=k-1 { out.push(mq.max().unwrap()); mq.pop_if(a[i+1-k]); } }
        assert_eq!(out, vec![3,3,5,5]);
    }
    #[test]
    fn pop_if_noop_when_not_front() {
        let mut mq = MonotonicQueueMin::new();
        mq.push(2); mq.push(3); // 2 is front
        mq.pop_if(99); // no-op
        assert_eq!(mq.min(), Some(2));
    }

    #[test]
    fn lens_and_empty() {
        let mut mq = MonotonicQueueMin::new();
        assert!(mq.is_empty());
        mq.push(5);
        assert_eq!(mq.len(), 1);

        let mut mx = MonotonicQueueMax::new();
        assert!(mx.max().is_none());
        mx.push(1); mx.push(2);
        assert_eq!(mx.max(), Some(2));
    }
}
