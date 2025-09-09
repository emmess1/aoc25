//! Adjacency list/map for graphs.

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Adjacency<N: Eq + Hash + Clone> {
    adj: HashMap<N, Vec<N>>, // directed
}

impl<N: Eq + Hash + Clone> Adjacency<N> {
    pub fn new() -> Self { Self { adj: HashMap::new() } }
    pub fn add_node(&mut self, n: N) { self.adj.entry(n).or_default(); }
    pub fn add_edge(&mut self, from: N, to: N) { self.adj.entry(from).or_default().push(to); }
    pub fn add_undirected(&mut self, a: N, b: N) { self.add_edge(a.clone(), b.clone()); self.add_edge(b, a); }
    pub fn neighbors<'a>(&'a self, n: &N) -> impl Iterator<Item=&N> + 'a { self.adj.get(n).into_iter().flatten() }
    pub fn nodes(&self) -> impl Iterator<Item=&N> { self.adj.keys() }
    pub fn indegrees(&self) -> HashMap<N, usize> {
        let mut indeg: HashMap<N, usize> = self.adj.keys().cloned().map(|n| (n, 0)).collect();
        for v in self.adj.values() { for u in v { *indeg.entry(u.clone()).or_default() += 1; } }
        indeg
    }
}

#[cfg(test)]
mod tests {
    use super::Adjacency;

    #[test]
    fn basic() {
        let mut g = Adjacency::new();
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        let v: Vec<_> = g.neighbors(&1).cloned().collect();
        assert!(v.contains(&2) && v.contains(&3));
        let indeg = g.indegrees();
        assert_eq!(indeg.get(&2), Some(&1));
        assert_eq!(indeg.get(&3), Some(&1));
    }
}

