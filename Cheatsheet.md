# Advent of Code Cheatsheet

Quick mappings from common AoC problems to the data structures and helpers in this crate. Each item also mentions the typical complexity and a tip or two.

## Parsing & Input
- Grid of chars/digits
  - Use: `parse_grid_chars`, `parse_grid_digits`
  - Tip: Convert to `DenseGrid2D` or keep as `Vec<Vec<_>>` depending on needs.
- Lists of integers
  - Use: `parse_ints_whitespace`, `parse_lines_i64`

## Grids & Geometry
- 2D neighbors
  - Use: `Point::neighbors4/8`, or constants `DELTAS4/DELTAS8`
  - Tip: Add deltas to current `Point` and check `DenseGrid2D::in_bounds`.
- 3D neighbors
  - Use: `Point3::neighbors6/26`
- Large sparse maps
  - Use: `SparseGrid<T>` (HashMap keyed by `Point`)
  - Tip: Track `bounds()` as you insert to simplify printing/iteration.
- Small dense maps
  - Use: `DenseGrid2D<T>` (row‑major, cache‑friendly)

## Sequences & Windows
- Sliding window min/max in O(N)
  - Use: `MonotonicQueueMin/Max`
- Sliding frequency / distinct counts
  - Use: `FreqMap` (inc/dec; remove‑on‑zero)
- Simple stacks/queues/deques
  - Use: `Stack<T>`, `Queue<T>`, `Deque<T>`

## Graphs & Traversal
- Build graph from edges
  - Use: `Adjacency<N>` with `add_edge`/`add_undirected`
- BFS / distances (unweighted)
  - Use: `bfs_distances(n, &adj, start)` → `Vec<i64>`
- DFS preorder / reachability
  - Use: `dfs_preorder(n, &adj, start)`
- Shortest paths (non‑negative weights)
  - Use: `dijkstra_indexed(n, &adj_w, start)` + `IndexedMinHeap`
  - Heuristic search: `astar_indexed(n, &adj_w, start, goal, h)`
- Strongly Connected Components
  - Use: `tarjan_scc(&adj_vec)`
- Topological ordering (DAG)
  - Use: `topo_sort(&edges)` or `Topo` builder

## Connectivity & Union‑Find
- Merge/find components (islands, pipes)
  - Use: `DisjointSet<K>` → `union`, `connected`, `size_of`

## Ranges & Numeric
- Merge overlapping intervals
  - Use: `IntervalSet::add(Interval)`, then `intervals()`
- Prefix sums with point updates
  - Use: `Fenwick` (BIT): `add`, `sum_prefix`, `sum_range`

## Strings
- Substring search / periodicity
  - Use: `kmp_search(text, pat)`, `z_function(s)`
- Rolling hash of substrings
  - Use: `RollingHash::new(s, base, mod).hash(l, r)`

## Bitmasks & State Compression
- Small state sets (<=128 elements)
  - Use: `BitMask` (u128): `set`, `clear`, `toggle`, `test`, `count_ones`
  - Tip: Ideal for subset DP and “visited” bitsets.

## Heaps & Priority Queues
- Min/Max item next
  - Use: `MinHeap<T>`, `MaxHeap<T>`
- Decrease‑key (Dijkstra/A*)
  - Use: `IndexedMinHeap::set(idx, new_priority)`

## Practical Tips
- Bounds checks
  - Prefer `DenseGrid2D::in_bounds` before indexing, or guard with `match` on `SparseGrid`.
- Printing grids
  - Combine `SparseGrid::bounds()` with a default char for empty cells.
- Coordinate arithmetic
  - Use `Point` add/sub and `DELTAS4/8` to keep code readable.
- Performance
  - Favor dense representations when the grid is small; switch to sparse as size grows.
  - For shortest paths with many updates, `IndexedMinHeap` avoids re‑inserting duplicates.

## Tiny Patterns
- BFS skeleton (unweighted)
```rust
use aoc25::bfs_distances;
let n = adj.len();
let dist = bfs_distances(n, &adj, start);
```
- Sliding window min (size k)
```rust
use aoc25::MonotonicQueueMin;
let mut mq = MonotonicQueueMin::new();
let mut out = Vec::new();
for i in 0..a.len() { mq.push(a[i]); if i>=k-1 { out.push(mq.min().unwrap()); mq.pop_if(a[i+1-k]); } }
```
- Dijkstra
```rust
use aoc25::dijkstra_indexed;
let (dist, _prev) = dijkstra_indexed(n, &adj_w, start);
```
