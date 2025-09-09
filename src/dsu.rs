//! Disjoint Set Union (Union-Find) for connectivity problems.

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug, Default)]
pub struct DisjointSet<K: Eq + Hash + Clone> {
    parent: HashMap<K, K>,
    size: HashMap<K, usize>,
    sets: usize,
}

impl<K: Eq + Hash + Clone> DisjointSet<K> {
    pub fn new() -> Self { Self { parent: HashMap::new(), size: HashMap::new(), sets: 0 } }
    fn make_set(&mut self, x: K) {
        if !self.parent.contains_key(&x) {
            self.parent.insert(x.clone(), x.clone());
            self.size.insert(x.clone(), 1);
            self.sets += 1;
        }
    }
    pub fn find(&mut self, x: K) -> K {
        self.make_set(x.clone());
        let p = self.parent.get(&x).cloned().unwrap();
        if p == x { return x; }
        let root = self.find(p.clone());
        self.parent.insert(x.clone(), root.clone());
        root
    }
    pub fn union(&mut self, a: K, b: K) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb { return; }
        let sa = *self.size.get(&ra).unwrap();
        let sb = *self.size.get(&rb).unwrap();
        if sa < sb { std::mem::swap(&mut ra, &mut rb); }
        // attach rb under ra
        self.parent.insert(rb.clone(), ra.clone());
        let new_size = sa + sb;
        self.size.insert(ra.clone(), new_size);
        self.size.remove(&rb);
        self.sets -= 1;
    }
    pub fn connected(&mut self, a: K, b: K) -> bool { self.find(a) == self.find(b) }
    pub fn count(&self) -> usize { self.sets }
    pub fn size_of(&mut self, a: K) -> usize { let r = self.find(a); *self.size.get(&r).unwrap() }
}

#[cfg(test)]
mod tests {
    use super::DisjointSet;
    #[test]
    fn basic() {
        let mut d = DisjointSet::new();
        d.union(1,2); d.union(3,4); d.union(2,3);
        assert!(d.connected(1,4));
        assert_eq!(d.size_of(1), 4);
        assert_eq!(d.count(), 1);
    }
}

