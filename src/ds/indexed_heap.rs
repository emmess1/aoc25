//! Indexed minimum heap supporting decrease-key by item index.
//!
//! Why this exists
//! - Classic priority queues (binary heaps) do not support updating an item’s
//!   priority efficiently once inside the heap. For Dijkstra/A*, we often need
//!   to lower a node’s priority (decrease-key) when we discover a better path.
//!
//! Design
//! - `heap`: stores the item indices in heap order
//! - `pos`: maps an item index to its current heap position (or None if absent)
//! - `prio`: stores the current priority for each item (or None if absent)
//! - `set(idx, p)`: inserts or updates priority; it fixes heap order by
//!   bubbling up or down depending on whether the priority improved or worsened.

#[derive(Clone, Debug)]
pub struct IndexedMinHeap<P: Ord + Copy> {
    heap: Vec<usize>,        // stores item indices
    pos: Vec<Option<usize>>, // position of item in heap
    prio: Vec<Option<P>>,    // priority per item
}

impl<P: Ord + Copy> IndexedMinHeap<P> {
    /// Create an empty indexed heap for `n` potential items (0..n-1).
    pub fn with_items(n: usize) -> Self {
        Self {
            heap: Vec::new(),
            pos: vec![None; n],
            prio: vec![None; n],
        }
    }
    fn better(a: P, b: P) -> bool {
        a < b
    }
    fn less(&self, i: usize, j: usize) -> bool {
        let ai = self.heap[i];
        let aj = self.heap[j];
        Self::better(self.prio[ai].unwrap(), self.prio[aj].unwrap())
    }
    fn swap_pos(&mut self, i: usize, j: usize) {
        let (a, b) = (self.heap[i], self.heap[j]);
        self.heap.swap(i, j);
        self.pos[a] = Some(j);
        self.pos[b] = Some(i);
    }
    /// Restore heap property by moving an element up toward the root.
    fn up(&mut self, mut i: usize) {
        while i > 0 {
            let p = (i - 1) / 2;
            if self.less(i, p) {
                self.swap_pos(i, p);
                i = p;
            } else {
                break;
            }
        }
    }
    /// Restore heap property by moving an element down toward the leaves.
    fn down(&mut self, mut i: usize) {
        let n = self.heap.len();
        loop {
            let l = 2 * i + 1;
            let r = l + 1;
            let mut m = i;
            if l < n && self.less(l, m) {
                m = l
            }
            if r < n && self.less(r, m) {
                m = r
            }
            if m == i {
                break;
            }
            self.swap_pos(i, m);
            i = m;
        }
    }
    /// Insert or update priority of item `idx`.
    /// Insert `idx` with priority `p`, or update its priority if present.
    pub fn set(&mut self, idx: usize, p: P) {
        match (self.pos[idx], self.prio[idx]) {
            (Some(i), Some(old)) => {
                self.prio[idx] = Some(p);
                if Self::better(p, old) {
                    self.up(i);
                } else {
                    self.down(i);
                }
            }
            _ => {
                self.prio[idx] = Some(p);
                let i = self.heap.len();
                self.heap.push(idx);
                self.pos[idx] = Some(i);
                self.up(i);
            }
        }
    }
    /// Check whether `idx` is currently in the heap.
    pub fn contains(&self, idx: usize) -> bool {
        self.pos[idx].is_some()
    }
    /// Pop the minimum (idx, priority) if present.
    pub fn pop_min(&mut self) -> Option<(usize, P)> {
        if self.heap.is_empty() {
            return None;
        }
        let root = self.heap[0];
        let pr = self.prio[root].take().unwrap();
        let last = self.heap.pop().unwrap();
        self.pos[root] = None;
        if !self.heap.is_empty() {
            self.heap[0] = last;
            self.pos[last] = Some(0);
            self.down(0);
        }
        Some((root, pr))
    }
}

#[cfg(test)]
mod tests {
    use super::IndexedMinHeap;
    #[test]
    fn decrease_key() {
        let mut h = IndexedMinHeap::with_items(5);
        h.set(2, 10);
        h.set(3, 5);
        h.set(1, 7);
        assert!(h.contains(2));
        // decrease key of 2 to become minimum
        h.set(2, 1);
        assert_eq!(h.pop_min(), Some((2, 1)));
        assert_eq!(h.pop_min().map(|x| x.0), Some(3));
        assert_eq!(h.pop_min().map(|x| x.0), Some(1));
        assert_eq!(h.pop_min(), None);
    }
    #[test]
    fn increase_key_triggers_down() {
        let mut h = IndexedMinHeap::with_items(4);
        h.set(0, 1);
        h.set(1, 2);
        h.set(2, 3);
        // Increase priority of 0 (worse), should stay valid and pop order remains 1, then 0, then 2
        h.set(0, 5);
        assert_eq!(h.pop_min(), Some((1, 2)));
        assert_eq!(h.pop_min(), Some((2, 3)));
        assert_eq!(h.pop_min(), Some((0, 5)));
        assert_eq!(h.pop_min(), None);
    }
}
