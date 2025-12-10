//! Adjacency list/map for graphs.
//!
//! This is a lightweight, readable structure for AoC-style graphs where
//! nodes are hashable (numbers, strings, etc.). It stores a directed adjacency
//! list and provides helpers for adding edges, iterating neighbors, and
//! computing indegrees (useful for topological sorting).

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Adjacency<N: Eq + Hash + Clone> {
    adj: HashMap<N, Vec<N>>, // directed
}

impl<N: Eq + Hash + Clone> Adjacency<N> {
    pub fn new() -> Self {
        Self {
            adj: HashMap::new(),
        }
    }
    /// Ensure a node exists even if it has no outgoing edges.
    pub fn add_node(&mut self, n: N) {
        self.adj.entry(n).or_default();
    }
    /// Add a directed edge `from -> to`.
    pub fn add_edge(&mut self, from: N, to: N) {
        self.adj.entry(from).or_default().push(to);
    }
    /// Add an undirected edge by inserting both directions.
    pub fn add_undirected(&mut self, a: N, b: N) {
        self.add_edge(a.clone(), b.clone());
        self.add_edge(b, a);
    }
    /// Borrowing iterator over neighbors of `n` (empty if `n` not present).
    pub fn neighbors<'a>(&'a self, n: &N) -> impl Iterator<Item = &'a N> + 'a {
        self.adj.get(n).into_iter().flatten()
    }
    /// Iterator over all nodes that have been mentioned on the left-hand side.
    pub fn nodes(&self) -> impl Iterator<Item = &N> {
        self.adj.keys()
    }
    /// Compute indegree counts for every node mentioned either as a key or as a neighbor.
    pub fn indegrees(&self) -> HashMap<N, usize> {
        let mut indeg: HashMap<N, usize> = self.adj.keys().cloned().map(|n| (n, 0)).collect();
        for v in self.adj.values() {
            for u in v {
                *indeg.entry(u.clone()).or_default() += 1;
            }
        }
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
        // nodes() should include all referenced nodes
        let nodes: Vec<_> = g.nodes().cloned().collect();
        assert!(nodes.contains(&1));
    }

    #[test]
    fn add_node_and_undirected() {
        let mut g = Adjacency::new();
        g.add_node(1);
        g.add_undirected(1, 2);
        let neighbors: Vec<_> = g.neighbors(&1).cloned().collect();
        assert!(neighbors.contains(&2));
        let neighbors2: Vec<_> = g.neighbors(&2).cloned().collect();
        assert!(neighbors2.contains(&1));
    }
}
