//! Topological sorting utilities (Kahn's algorithm) for dependency problems.

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

pub fn topo_sort<N: Eq + Hash + Clone>(edges: &[(N, N)]) -> Option<Vec<N>> {
    let mut adj: HashMap<N, Vec<N>> = HashMap::new();
    let mut indeg: HashMap<N, usize> = HashMap::new();
    for (u, v) in edges {
        adj.entry(u.clone()).or_default().push(v.clone());
        indeg.entry(u.clone()).or_default();
        *indeg.entry(v.clone()).or_default() += 1;
    }
    let mut q: VecDeque<N> = indeg
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(n, _)| n.clone())
        .collect();
    let mut out: Vec<N> = Vec::with_capacity(indeg.len());
    while let Some(u) = q.pop_front() {
        out.push(u.clone());
        for v in adj.get(&u).into_iter().flatten() {
            let e = indeg.get_mut(v).unwrap();
            *e -= 1;
            if *e == 0 {
                q.push_back(v.clone());
            }
        }
    }
    if out.len() == indeg.len() {
        Some(out)
    } else {
        None
    }
}

#[derive(Clone, Debug, Default)]
pub struct Topo<N: Eq + Hash + Clone> {
    edges: Vec<(N, N)>,
}

impl<N: Eq + Hash + Clone> Topo<N> {
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }
    pub fn add_edge(&mut self, u: N, v: N) {
        self.edges.push((u, v));
    }
    pub fn solve(&self) -> Option<Vec<N>> {
        topo_sort(&self.edges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_dag() {
        let edges = [(1, 2), (1, 3), (2, 4), (3, 4)];
        let order = topo_sort(&edges).unwrap();
        // 1 must come before 2 and 3; 2 and 3 before 4.
        let pos = |x| order.iter().position(|&y| y == x).unwrap();
        assert!(pos(1) < pos(2) && pos(1) < pos(3));
        assert!(pos(2) < pos(4) && pos(3) < pos(4));
    }
    #[test]
    fn detect_cycle() {
        let edges = [(1, 2), (2, 3), (3, 1)];
        assert!(topo_sort(&edges).is_none());
        let mut t = Topo::new();
        t.add_edge(1, 2);
        t.add_edge(2, 3);
        t.add_edge(3, 1);
        assert!(t.solve().is_none());
    }
}
