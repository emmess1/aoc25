//! Indexed minimum heap supporting decrease-key by item index.

#[derive(Clone, Debug)]
pub struct IndexedMinHeap<P: Ord + Copy> {
    heap: Vec<usize>,        // stores item indices
    pos: Vec<Option<usize>>, // position of item in heap
    prio: Vec<Option<P>>,    // priority per item
}

impl<P: Ord + Copy> IndexedMinHeap<P> {
    pub fn with_items(n: usize) -> Self {
        Self { heap: Vec::new(), pos: vec![None; n], prio: vec![None; n] }
    }
    fn better(a: P, b: P) -> bool { a < b }
    fn less(&self, i: usize, j: usize) -> bool {
        let ai = self.heap[i]; let aj = self.heap[j];
        Self::better(self.prio[ai].unwrap(), self.prio[aj].unwrap())
    }
    fn swap_pos(&mut self, i: usize, j: usize) { let (a,b)=(self.heap[i], self.heap[j]); self.heap.swap(i,j); self.pos[a]=Some(j); self.pos[b]=Some(i); }
    fn up(&mut self, mut i: usize) { while i>0 { let p=(i-1)/2; if self.less(i,p){ self.swap_pos(i,p); i=p;} else {break;} } }
    fn down(&mut self, mut i: usize) { let n=self.heap.len(); loop { let l=2*i+1; let r=l+1; let mut m=i; if l<n && self.less(l,m){m=l} if r<n && self.less(r,m){m=r} if m==i {break;} self.swap_pos(i,m); i=m; } }
    /// Insert or update priority of item `idx`.
    pub fn set(&mut self, idx: usize, p: P) {
        match (self.pos[idx], self.prio[idx]) {
            (Some(i), Some(old)) => { self.prio[idx]=Some(p); if Self::better(p, old) { self.up(i); } else { self.down(i); } }
            _ => { self.prio[idx]=Some(p); let i=self.heap.len(); self.heap.push(idx); self.pos[idx]=Some(i); self.up(i); }
        }
    }
    pub fn contains(&self, idx: usize) -> bool { self.pos[idx].is_some() }
    pub fn pop_min(&mut self) -> Option<(usize,P)> {
        if self.heap.is_empty() { return None; }
        let root = self.heap[0]; let pr = self.prio[root].take().unwrap();
        let last = self.heap.pop().unwrap();
        self.pos[root]=None;
        if !self.heap.is_empty() {
            self.heap[0]=last; self.pos[last]=Some(0); self.down(0);
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
        h.set(2, 10); h.set(3, 5); h.set(1, 7);
        assert!(h.contains(2));
        // decrease key of 2 to become minimum
        h.set(2, 1);
        assert_eq!(h.pop_min(), Some((2,1)));
        assert_eq!(h.pop_min().map(|x| x.0), Some(3));
        assert_eq!(h.pop_min().map(|x| x.0), Some(1));
        assert_eq!(h.pop_min(), None);
    }
}

