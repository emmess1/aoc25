# Advent of Code Data Structures and Helpers (Rust)

A focused, well‑documented toolkit of lightweight data structures and algorithms tailored for Advent of Code (AoC) style problems. The design favors clarity and maintainability with predictable complexity and plenty of examples.

- Small, readable implementations (no magic, easy to debug)
- Practical APIs with sane defaults and examples
- Broad coverage of common AoC patterns: grids, graphs, parsing, queues/stacks, heaps, union‑find, ranges, strings, bitmasks, and more

## Table of Contents

- [Quick Start](#quick-start)
- [Design Principles](#design-principles)
- [When To Use](#when-to-use)
- [When Not To Use](#when-not-to-use)
- Data Structures
  - [LinkedList](#linkedlist)
  - [SimpleHashMap](#simplehashmap)
  - [BstMap (ordered map)](#bstmap-ordered-map)
  - [DoublyLinkedList](#doublylinkedlist)
  - [Arrays/Lists (ArrayList)](#arrayslists-arraylist)
  - [Hash Sets (HashSetExt)](#hash-sets-hashsetext)
  - [Coordinates (Point, Point3, ComplexI)](#coordinates-point-point3-complexi)
  - [Sparse Grid (SparseGrid)](#sparse-grid-sparsegrid)
  - [Neighbor Lookups (DELTAS4/DELTAS8)](#neighbor-lookups-deltas4deltas8)
  - [Stack / Queue / Deque](#stack--queue--deque)
  - [Priority Queues / Heaps (MinHeap/MaxHeap)](#priority-queues--heaps-minheapmaxheap)
  - [Adjacency Lists/Maps (Adjacency)](#adjacency-listsmaps-adjacency)
  - [Union-Find (DisjointSet)](#union-find-disjointset)
  - [Topological Sorting](#topological-sorting)
  - [Intervals/Ranges (Interval, IntervalSet)](#intervalsranges-interval-intervalset)
  - [Bitmasks (BitMask)](#bitmasks-bitmask)
  - [Monotonic Queues](#monotonic-queues)
  - [Dense Grid (DenseGrid2D)](#dense-grid-densegrid2d)
  - [Indexed Min-Heap (decrease-key)](#indexed-min-heap-decrease-key)
  - [Frequency Map (FreqMap)](#frequency-map-freqmap)
  - [String Algorithms](#string-algorithms)
  - [SCC (Tarjan)](#scc-tarjan)
  - [Fenwick Tree (BIT)](#fenwick-tree-bit)
  - [Graph Search Helpers](#graph-search-helpers)
  - [Parsing Helpers](#parsing-helpers)
- [Further Reading](#further-reading)
- [Testing and Coverage](#testing-and-coverage)

## Quick Start

- Run tests: `cargo test`
- Explore examples inline in this README and rustdoc for each module
- Typical import:
```rust
use aoc25::*; // or bring specific items you need
```

- See also: [Cheatsheet.md](./Cheatsheet.md) for a quick mapping of problems → tools

## LinkedList

A minimal singly linked list with O(1) front operations.

- Core ops: new, push_front, pop_front, peek, peek_mut, iter
- Complexity: push_front/pop_front/peek O(1); full iteration O(n)

Theory
- Singly linked nodes with a head pointer; operations at the head are O(1).
- Not cache-friendly versus arrays; random access is O(n).

Practical
- Great for building or consuming a stream at the front; prefer arrays for heavy indexing.

Example
```rust
use aoc25::LinkedList;

let mut ll = LinkedList::new();
ll.push_front("c");
ll.push_front("b");
ll.push_front("a");

assert_eq!(ll.peek(), Some(&"a"));
let v: Vec<_> = ll.iter().cloned().collect();
assert_eq!(v, vec!["a", "b", "c"]);

while let Some(x) = ll.pop_front() {
    // process x
}
assert!(ll.is_empty());
```

## SimpleHashMap

A basic hash map using separate chaining (`Vec<Vec<(K, V)>>`) and a power-of-two number of buckets for fast index masking.

- Core ops: new, with_capacity, insert, get, get_mut, contains_key, remove, clear
- Complexity (typical): O(1) average for insert/get/remove; O(n) worst-case per bucket
- Resizing: When load factor exceeds a threshold (0.75), capacity doubles and keys are rehashed

Example
```rust
use aoc25::SimpleHashMap;

let mut map = SimpleHashMap::new();
assert_eq!(map.insert("a", 1), None);     // new key
assert_eq!(map.insert("b", 2), None);
assert_eq!(map.insert("a", 3), Some(1));  // update

assert_eq!(map.get(&"a"), Some(&3));
if let Some(v) = map.get_mut(&"b") { *v += 10; }
assert_eq!(map.get(&"b"), Some(&12));

assert!(map.contains_key(&"a"));
assert_eq!(map.remove(&"a"), Some(3));
assert!(!map.contains_key(&"a"));
```

Theory
- Hashing maps keys to buckets; separate chaining stores collisions in small vectors.
- Expected O(1) for insert/lookup/remove with a good hash; worst-case O(n) if many collisions.

Practical
- Perfect for “have I seen this state?” checks and counting distinct items.

## BstMap (ordered map)

An ordered map backed by an unbalanced BST. Keys must implement `Ord`. In the worst case (e.g., sorted inserts) operations can degrade to O(n).

- Core ops: new, insert, get, get_mut, contains_key, remove
- Complexity: O(h) with tree height h; typical h ≈ log2(n)
- Remove handles: leaf, single child (left/right), two children (replace with successor)

Example
```rust
use aoc25::BstMap;

let mut tm = BstMap::new();
tm.insert(5, "five");
tm.insert(3, "three");
tm.insert(7, "seven");

assert_eq!(tm.get(&3), Some(&"three"));
if let Some(v) = tm.get_mut(&7) { *v = "SEVEN"; }
assert_eq!(tm.get(&7), Some(&"SEVEN"));

assert_eq!(tm.remove(&5), Some("five"));
assert!(tm.contains_key(&3));
assert!(tm.contains_key(&7));
```

Theory
- Binary Search Tree invariant: left < key < right. Unbalanced BST can degrade to O(n).

Practical
- Use when you need sorted iteration or nearest-key lookups; for strict performance, prefer BTreeMap.

## Design Principles

- Favor straightforward, idiomatic Rust over complex optimizations
- Keep public APIs small and familiar
- Document complexity and important trade-offs inline

## When To Use

- Learning/teaching: clear reference implementations
- Small utilities or tests where simplicity matters more than raw performance
- As a base to extend with more features (e.g., iteration variants, balancing)

## When Not To Use

- Production systems needing high performance and robust guarantees
- Situations where worst-case O(n) operations are unacceptable (BST is unbalanced)

## License

This project is provided without an explicit license file; add one if you plan to distribute it.
---

## DoublyLinkedList

A safe doubly linked list implemented with `Rc<RefCell<...>>` forward links and `Weak` backward links to avoid reference cycles.

- Core ops: new, push_front, push_back, pop_front_cloned, pop_back_cloned, peek_front, peek_back, clear, into_iter (consuming)
- Complexity: push/pop/peek at either end are O(1); into_iter drains in O(n)
- Notes: peek and pop return values by cloning (T: Clone). Use the consuming iterator to get owned values without Clone.

Example
```rust
use aoc25::DoublyLinkedList;

let mut dl = DoublyLinkedList::new();
dl.push_front(2);
dl.push_front(1);
dl.push_back(3);

assert_eq!(dl.peek_front(), Some(1));
assert_eq!(dl.peek_back(), Some(3));

assert_eq!(dl.pop_front_cloned(), Some(1));
assert_eq!(dl.pop_back_cloned(), Some(3));

let v: Vec<_> = dl.into_iter().collect();
// remaining elements drained front-to-back
```

Theory
- Doubly linked nodes allow O(1) insertion/removal at both ends.
- RefCount (Rc) + interior mutability (RefCell) + Weak back-links avoids cycles.

Practical
- Use pop_front_cloned/pop_back_cloned for safe value extraction; or into_iter() to drain without cloning.
## Arrays/Lists (ArrayList)

Thin wrapper over `Vec<T>` with slicing and iteration helpers.

Example
```rust
use aoc25::ArrayList;
let mut a = ArrayList::from_iter(0..5);
assert_eq!(a.slice(1..3), &[1,2]);
```

Theory
- Dynamic arrays support amortized O(1) push/pop at the end and O(1) random access.

Practical
- Default choice for most sequence tasks; slice cheaply for windows and subranges.

## Hash Sets (HashSetExt)

Convenience wrapper over `HashSet` for fast membership tests.

Example
```rust
use aoc25::HashSetExt;
let mut s = HashSetExt::new();
s.insert("state");
assert!(s.contains(&"state"));
```

Theory
- Hash sets implement expected O(1) membership and insert/remove.

Practical
- Ideal for seen-state de-duplication and cycle detection.

## Coordinates (Point, Point3, ComplexI)

2D/3D points with neighbor methods; simple complex integer for rotations.

Example
```rust
use aoc25::{Point, Point3};
let p = Point { x: 0, y: 0 };
let n4 = p.neighbors4();
let p3 = Point3::new(0,0,0);
let n6 = p3.neighbors6();
```

Theory
- Represent positions as integers; 2D neighbors (4/8) and 3D neighbors (6/26) are standard patterns.

Practical
- Use Point for grids, Point3 for voxel problems, ComplexI for elegant 2D rotations.

## Sparse Grid (SparseGrid)

Dictionary keyed by coordinates for large, mostly empty grids.

Example
```rust
use aoc25::{SparseGrid, Point};
let mut g = SparseGrid::new();
g.insert(Point::new(2,3), 9);
```

Theory
- Dictionary keyed by coordinates; memory proportional to populated cells, not grid area.

Practical
- Use for large, sparse grids (e.g., infinite caves, sand/rock maps).

## Neighbor Lookups (DELTAS4/DELTAS8)

Predefined 4- and 8-direction deltas for movement on grids.

Theory
- 4-neighbors (von Neumann) and 8-neighbors (Moore) capture moves in cardinal and diagonal directions.

Practical
- Add to a Point or filter with bounds to enumerate valid moves.

## Stack / Queue / Deque

Stack<T>, Queue<T> and Deque<T> for parsing, BFS, and sliding windows.

Theory
- Stack: LIFO (push/pop back). Queue: FIFO (push back, pop front). Deque: double-ended.

Practical
- Use Stack for DFS or expression parsing; Queue for BFS; Deque for sliding window tricks.

## Priority Queues / Heaps (MinHeap/MaxHeap)

Binary-heap wrappers for taking smallest/largest next item.

Theory
- Heap is a complete binary tree with heap-order; push/pop/peek are O(log N)/O(log N)/O(1).

Practical
- Use MinHeap for Dijkstra-ish tasks or processing in ascending order; MaxHeap for descending.

## Adjacency Lists/Maps (Adjacency)

Directed adjacency with neighbor/indegree helpers.

Theory
- Graph as map<Node, Vec<Node>>; indegree is used by Kahn’s topological sort.

Practical
- Build problem graphs directly from input; iterate neighbors for traversal.

## Union-Find (DisjointSet)

Connectivity queries and merges in near-constant amortized time.

Theory
- Union by size/rank + path compression give ~inverse-Ackermann time per op.

Practical
- Great for grouping connected components (islands, pipes, portals).

## Topological Sorting

`topo_sort` and `Topo` builder for DAG ordering.

Theory
- Kahn’s algorithm: repeatedly remove nodes with indegree 0; cycle if nodes remain.

Practical
- Schedule tasks with dependencies; detect impossible constraints.

## Intervals/Ranges (Interval, IntervalSet)

Inclusive ranges with merging and membership.

Theory
- Overlap if a.start <= b.end and b.start <= a.end; merge by taking min start, max end.

Practical
- Merge many ranges efficiently (sensor coverage, blocked segments).

## Bitmasks (BitMask)

Compact u128-backed state with bit operations.

Theory
- Set membership/state encoded as bits; bit ops (AND/OR/XOR/SHIFT) are fast.

Practical
- Use for visited sets of small domains (<=128), or DP over subsets.

## Monotonic Queues

`MonotonicQueueMin/Max` for sliding window min/max in O(N).

Theory
- Maintain a deque in sorted order by pruning from the back; front is min/max of the window.

Practical
- Excellent for rolling minima/maxima (calibration, weather, stock span).

## Dense Grid (DenseGrid2D)

Efficient 2D array with bounds and neighbor iteration.

Theory
- Row-major contiguous storage; O(1) indexing, cache-friendly scans.

Practical
- Prefer for smaller/complete grids; combine with Point for BFS/flood fill.

## Indexed Min-Heap (decrease-key)

`IndexedMinHeap` for Dijkstra/A* with updatable priorities.

Theory
- Adds a position map to support decrease/increase key in O(log N).

Practical
- Use for shortest paths where node priorities are updated frequently.

## Frequency Map (FreqMap)

Multiset of counts with remove-on-zero semantics.

Theory
- HashMap<T, usize> with counts; zero counts are removed to trim memory.

Practical
- Track sliding distinct counts and histogram equality efficiently.

## String Algorithms

KMP search, Z-function, and a simple RollingHash.

Theory
- KMP avoids re-checking by using the prefix function; Z-function computes prefix matches; rolling hash allows O(1) substring hashes.

Practical
- Fast substring search, detecting periodicity, and hashing for comparing windows.

## SCC (Tarjan)

Tarjan’s algorithm returning components as node index lists.

Theory
- Single DFS computes low-link values; pops a component when root is found.

Practical
- Collapse cycles to DAGs; useful before topo sort on directed graphs.

## Fenwick Tree (BIT)

Prefix sums with point updates and range queries in O(log N).

Theory
- Binary lifting over lowbit; prefix sum and add are logarithmic.

Practical
- Range sum queries with frequent point updates (scores, counts).

## Graph Search Helpers

- `bfs_distances(n, &adj, start)` → Vec<i64> distances
- `dfs_preorder(n, &adj, start)` → Vec<usize> preorder
- `dijkstra_indexed(n, &adj_w, start)` → (dist, prev)
- `astar_indexed(n, &adj_w, start, goal, h)` → (cost, path)

Examples
```rust
use aoc25::{bfs_distances, dijkstra_indexed, astar_indexed};
let n=4; let mut adj=vec![vec![];n]; adj[0]=vec![1,2]; adj[1]=vec![3]; adj[2]=vec![3];
let dist = bfs_distances(n, &adj, 0);
assert_eq!(dist, vec![0,1,1,2]);

let mut adj_w=vec![vec![];n]; adj_w[0].push((1,2)); adj_w[0].push((2,5)); adj_w[1].push((2,1)); adj_w[2].push((3,2));
let (d,_p) = dijkstra_indexed(n, &adj_w, 0);
assert_eq!(d[3], 5);
let h = |_u:usize| 0; // zero heuristic
let (cost, path) = astar_indexed(n, &adj_w, 0, 3, &h).unwrap();
assert_eq!((cost, path), (5, vec![0,1,2,3]));
```

## Parsing Helpers

- `parse_grid_chars(&str)` → Vec<Vec<char>>
- `parse_grid_digits(&str)` → Vec<Vec<i64>>
- `parse_ints_whitespace(&str)` → Vec<i64>
- `parse_lines_i64(&str)` → Vec<i64>

Examples
```rust
use aoc25::{parse_grid_chars, parse_ints_whitespace};
let grid = parse_grid_chars("ab\ncd\n");
assert_eq!(grid, vec![vec!['a','b'], vec!['c','d']]);
let nums = parse_ints_whitespace("1 -2 3\n4");
assert_eq!(nums, vec![1,-2,3,4]);
```

## Further Reading

- Overview: https://en.wikipedia.org/wiki/Data_structure
- Linked list: https://en.wikipedia.org/wiki/Linked_list
- Hash table: https://en.wikipedia.org/wiki/Hash_table
- Binary heap: https://en.wikipedia.org/wiki/Binary_heap
- Disjoint-set (Union–Find): https://en.wikipedia.org/wiki/Disjoint-set_data_structure
- Topological sorting: https://en.wikipedia.org/wiki/Topological_sorting
- Tarjan’s SCC: https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm
- Fenwick tree (BIT): https://en.wikipedia.org/wiki/Fenwick_tree
- Dijkstra’s algorithm: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
- A* search: https://en.wikipedia.org/wiki/A*_search_algorithm
- KMP string search: https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm

## Testing and Coverage

- Run tests: `cargo test`
- Source-based coverage (macOS, Xcode tools):
  - `export RUSTFLAGS="-Cinstrument-coverage"`
  - `export LLVM_PROFILE_FILE="prof-%p-%m.profraw"`
  - `cargo test`
  - `xcrun llvm-profdata merge -sparse prof-*.profraw -o coverage.profdata`
  - `xcrun llvm-cov report --instr-profile=coverage.profdata -object target/debug/deps/data_structures-<hash> -object target/debug/deps/integration-<hash> -object target/debug/deps/libdata_structures-*.rlib --summary-only`
