# Data Structures (Rust)

This crate provides minimal, readable implementations of three classic data structures:

- LinkedList: A simple generic singly linked list
- DoublyLinkedList: A safe doubly linked list (push/pop at both ends)
- SimpleHashMap: A separate-chaining hash map with resizing
- BstMap: An ordered map backed by an unbalanced binary search tree (BST)

The emphasis is on clarity and maintainability over peak performance or feature completeness. Each module includes unit tests and examples below cover common usage.

## LinkedList

A minimal singly linked list with O(1) front operations.

- Core ops: new, push_front, pop_front, peek, peek_mut, iter
- Complexity: push_front/pop_front/peek O(1); full iteration O(n)

Example
```rust
use data_structures::LinkedList;

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
use data_structures::SimpleHashMap;

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

## BstMap (ordered map)

An ordered map backed by an unbalanced BST. Keys must implement `Ord`. In the worst case (e.g., sorted inserts) operations can degrade to O(n).

- Core ops: new, insert, get, get_mut, contains_key, remove
- Complexity: O(h) with tree height h; typical h ≈ log2(n)
- Remove handles: leaf, single child (left/right), two children (replace with successor)

Example
```rust
use data_structures::BstMap;

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

## Testing and Coverage

- Run tests: `cargo test`
- Source-based coverage (macOS, Xcode tools):
  - `export RUSTFLAGS="-Cinstrument-coverage"`
  - `export LLVM_PROFILE_FILE="prof-%p-%m.profraw"`
  - `cargo test`
  - `xcrun llvm-profdata merge -sparse prof-*.profraw -o coverage.profdata`
  - `xcrun llvm-cov report --instr-profile=coverage.profdata -object target/debug/deps/data_structures-<hash> -object target/debug/deps/integration-<hash> -object target/debug/deps/libdata_structures-*.rlib --summary-only`

## Design Goals

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
## DoublyLinkedList

A safe doubly linked list implemented with `Rc<RefCell<...>>` forward links and `Weak` backward links to avoid reference cycles.

- Core ops: new, push_front, push_back, pop_front_cloned, pop_back_cloned, peek_front, peek_back, clear, into_iter (consuming)
- Complexity: push/pop/peek at either end are O(1); into_iter drains in O(n)
- Notes: peek and pop return values by cloning (T: Clone). Use the consuming iterator to get owned values without Clone.

Example
```rust
use data_structures::DoublyLinkedList;

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
## Arrays/Lists (ArrayList)

Thin wrapper over `Vec<T>` with slicing and iteration helpers.

Example
```rust
use data_structures::ArrayList;
let mut a = ArrayList::from_iter(0..5);
assert_eq!(a.slice(1..3), &[1,2]);
```

## Hash Sets (HashSetExt)

Convenience wrapper over `HashSet` for fast membership tests.

Example
```rust
use data_structures::HashSetExt;
let mut s = HashSetExt::new();
s.insert("state");
assert!(s.contains(&"state"));
```

## Coordinates (Point, Point3, ComplexI)

2D/3D points with neighbor methods; simple complex integer for rotations.

Example
```rust
use data_structures::{Point, Point3};
let p = Point { x: 0, y: 0 };
let n4 = p.neighbors4();
let p3 = Point3::new(0,0,0);
let n6 = p3.neighbors6();
```

## Sparse Grid (SparseGrid)

Dictionary keyed by coordinates for large, mostly empty grids.

Example
```rust
use data_structures::{SparseGrid, Point};
let mut g = SparseGrid::new();
g.insert(Point::new(2,3), 9);
```

## Neighbor Lookups (DELTAS4/DELTAS8)

Predefined 4- and 8-direction deltas for movement on grids.

## Stack / Queue / Deque

Stack<T>, Queue<T> and Deque<T> for parsing, BFS, and sliding windows.

## Priority Queues / Heaps (MinHeap/MaxHeap)

Binary-heap wrappers for taking smallest/largest next item.

## Adjacency Lists/Maps (Adjacency)

Directed adjacency with neighbor/indegree helpers.

## Union-Find (DisjointSet)

Connectivity queries and merges in near-constant amortized time.

## Topological Sorting

`topo_sort` and `Topo` builder for DAG ordering.

## Intervals/Ranges (Interval, IntervalSet)

Inclusive ranges with merging and membership.

## Bitmasks (BitMask)

Compact u128-backed state with bit operations.

## Monotonic Queues

`MonotonicQueueMin/Max` for sliding window min/max in O(N).

## Dense Grid (DenseGrid2D)

Efficient 2D array with bounds and neighbor iteration.

## Indexed Min-Heap (decrease-key)

`IndexedMinHeap` for Dijkstra/A* with updatable priorities.

## Frequency Map (FreqMap)

Multiset of counts with remove-on-zero semantics.

## String Algorithms

KMP search, Z-function, and a simple RollingHash.

## SCC (Tarjan)

Tarjan’s algorithm returning components as node index lists.

## Fenwick Tree (BIT)

Prefix sums with point updates and range queries in O(log N).

## Graph Search Helpers

- `bfs_distances(n, &adj, start)` → Vec<i64> distances
- `dfs_preorder(n, &adj, start)` → Vec<usize> preorder
- `dijkstra_indexed(n, &adj_w, start)` → (dist, prev)
- `astar_indexed(n, &adj_w, start, goal, h)` → (cost, path)

## Parsing Helpers

- `parse_grid_chars(&str)` → Vec<Vec<char>>
- `parse_grid_digits(&str)` → Vec<Vec<i64>>
- `parse_ints_whitespace(&str)` → Vec<i64>
- `parse_lines_i64(&str)` → Vec<i64>
