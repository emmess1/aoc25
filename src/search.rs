//! Graph search utilities: BFS, DFS, Dijkstra, and A* over indexed graphs.
//!
//! These helpers assume nodes are `usize` in [0, n), which matches many AoC
//! tasks and integrates with `IndexedMinHeap` for decrease-key operations.

use crate::indexed_heap::IndexedMinHeap;
use std::collections::VecDeque;

/// BFS distances from `start` in an unweighted directed graph.
/// Returns a vector `dist` of length n with -1 for unreachable.
pub fn bfs_distances(n: usize, adj: &Vec<Vec<usize>>, start: usize) -> Vec<i64> {
    let mut dist = vec![-1; n];
    let mut q = VecDeque::new();
    dist[start] = 0; q.push_back(start);
    while let Some(u) = q.pop_front() {
        let du = dist[u];
        for &v in &adj[u] {
            if dist[v] == -1 { dist[v] = du + 1; q.push_back(v); }
        }
    }
    dist
}

/// DFS preorder traversal from `start`.
pub fn dfs_preorder(n: usize, adj: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut st: Vec<(usize, usize)> = vec![(start, 0)]; // (node, next edge index)
    let mut seen = vec![false; n];
    seen[start] = true; out.push(start);
    while let Some((u, i)) = st.pop() {
        if i < adj[u].len() {
            st.push((u, i+1));
            let v = adj[u][i];
            if !seen[v] { seen[v] = true; out.push(v); st.push((v, 0)); }
        }
    }
    out
}

/// Dijkstra on an indexed weighted graph.
/// - `adj_w[u]` contains (v, w) edges.
/// Returns (dist, prev) with i64 distances, i64::MAX meaning unreachable.
pub fn dijkstra_indexed(n: usize, adj_w: &Vec<Vec<(usize, i64)>>, start: usize) -> (Vec<i64>, Vec<Option<usize>>) {
    let mut dist = vec![i64::MAX; n];
    let mut prev = vec![None; n];
    dist[start] = 0;
    let mut pq = IndexedMinHeap::with_items(n);
    pq.set(start, 0);
    while let Some((u, _)) = pq.pop_min() {
        let du = dist[u];
        for &(v, w) in &adj_w[u] {
            if du != i64::MAX && du + w < dist[v] {
                dist[v] = du + w; prev[v] = Some(u); pq.set(v, dist[v]);
            }
        }
    }
    (dist, prev)
}

/// A* on an indexed weighted graph using a non-negative heuristic `h`.
/// `h` maps node index -> estimated remaining cost.
pub fn astar_indexed(n: usize, adj_w: &Vec<Vec<(usize, i64)>>, start: usize, goal: usize, h: &dyn Fn(usize) -> i64) -> Option<(i64, Vec<usize>)> {
    let mut g = vec![i64::MAX; n];
    let mut prev = vec![None; n];
    g[start] = 0;
    let mut pq = IndexedMinHeap::with_items(n);
    pq.set(start, h(start));
    while let Some((u, _)) = pq.pop_min() {
        if u == goal { break; }
        let gu = g[u];
        for &(v, w) in &adj_w[u] {
            if gu != i64::MAX && gu + w < g[v] {
                g[v] = gu + w; prev[v] = Some(u);
                pq.set(v, g[v] + h(v));
            }
        }
    }
    if g[goal] == i64::MAX { return None; }
    // reconstruct path
    let mut path = Vec::new();
    let mut cur = Some(goal);
    while let Some(u) = cur { path.push(u); cur = prev[u]; }
    path.reverse();
    Some((g[goal], path))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bfs_and_dfs() {
        let n=5; let mut adj=vec![vec![];n];
        adj[0]=vec![1,2]; adj[1]=vec![3]; adj[2]=vec![3]; adj[3]=vec![4];
        let dist = bfs_distances(n,&adj,0);
        assert_eq!(dist, vec![0,1,1,2,3]);
        let order = dfs_preorder(n,&adj,0);
        assert_eq!(order[0], 0); // deterministic enough
        assert!(order.contains(&3) && order.contains(&4));
    }
    #[test]
    fn dijkstra_and_astar() {
        // Weighted graph: 0->1(2), 0->2(5), 1->2(1), 2->3(2)
        let n=4; let mut adj=vec![vec![];n];
        adj[0].push((1,2)); adj[0].push((2,5)); adj[1].push((2,1)); adj[2].push((3,2));
        let (dist, prev) = dijkstra_indexed(n,&adj,0);
        assert_eq!(dist, vec![0,2,3,5]);
        // A* with zero heuristic equals Dijkstra
        let h = |_u:usize| 0;
        let res = astar_indexed(n,&adj,0,3,&h).unwrap();
        assert_eq!(res.0, 5);
        assert_eq!(res.1, vec![0,1,2,3]);
    }
}

