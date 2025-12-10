//! Strongly Connected Components via Tarjan's algorithm.

/// Returns a vector of components; each component is a vector of node indices.
pub fn tarjan_scc(adj: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = adj.len();
    let mut index = vec![None; n];
    let mut low = vec![0usize; n];
    let mut onstack = vec![false; n];
    let mut st: Vec<usize> = Vec::new();
    let mut next_index = 0usize;
    let mut comps: Vec<Vec<usize>> = Vec::new();
    fn dfs(
        u: usize,
        adj: &Vec<Vec<usize>>,
        index: &mut [Option<usize>],
        low: &mut [usize],
        onstack: &mut [bool],
        st: &mut Vec<usize>,
        next_index: &mut usize,
        comps: &mut Vec<Vec<usize>>,
    ) {
        index[u] = Some(*next_index);
        low[u] = *next_index;
        *next_index += 1;
        st.push(u);
        onstack[u] = true;
        for &v in &adj[u] {
            match index[v] {
                None => {
                    dfs(v, adj, index, low, onstack, st, next_index, comps);
                    low[u] = low[u].min(low[v]);
                }
                Some(iv) if onstack[v] => {
                    low[u] = low[u].min(iv);
                }
                _ => {}
            }
        }
        if low[u] == index[u].unwrap() {
            let mut comp = Vec::new();
            loop {
                let w = st.pop().unwrap();
                onstack[w] = false;
                comp.push(w);
                if w == u {
                    break;
                }
            }
            comps.push(comp);
        }
    }
    for u in 0..n {
        if index[u].is_none() {
            dfs(
                u,
                adj,
                &mut index,
                &mut low,
                &mut onstack,
                &mut st,
                &mut next_index,
                &mut comps,
            );
        }
    }
    comps
}

#[cfg(test)]
mod tests {
    use super::tarjan_scc;
    #[test]
    fn scc_small() {
        // 0->1->2->0 forms one SCC; 3->4; 4 alone (no edge back)
        let mut adj = vec![vec![], vec![], vec![], vec![], vec![]];
        adj[0].push(1);
        adj[1].push(2);
        adj[2].push(0);
        adj[3].push(4);
        let mut comps = tarjan_scc(&adj);
        // sort inner for stable compare
        for c in comps.iter_mut() {
            c.sort_unstable();
        }
        comps.sort_by_key(|c| c[0]);
        assert_eq!(comps, vec![vec![0, 1, 2], vec![3], vec![4]]);
    }

    #[test]
    fn covers_cross_edge_case() {
        // Create a finished SCC {0,1,2} and an extra node 3 with an edge to 0.
        // When exploring 3 after {0,1,2} is completed, the edge 3->0 hits
        // the branch where index[v] is Some and onstack[v] is false.
        let adj = vec![vec![1], vec![2], vec![0], vec![0]];
        let comps = tarjan_scc(&adj);
        assert_eq!(comps.len(), 2);
    }
}
