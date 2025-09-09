//! Basic data structures library: linked list, hash map, and ordered map.
//!
//! What this crate provides
//! - `linked_list`: A minimal, generic, singly linked list. Focuses on
//!   clarity over performance; intended for educational or light-duty use.
//! - `hash_map`: A simple, separate-chaining hash map that resizes when the
//!   load factor exceeds a threshold. Suitable for understanding basic
//!   hashmap mechanics without the complexity of industrial-grade designs.
//! - `tree_map`: A basic ordered map implemented as an unbalanced binary
//!   search tree (BST). It is easy to follow but not balanced, so operations
//!   can degrade to O(n) in the worst case.
//!
//! Design notes
//! - All implementations prioritize readable, idiomatic Rust over micro-
//!   optimizations or advanced unsafe tricks.
//! - The public APIs are intentionally small and mirror common operations
//!   (`new`, `insert`, `get`, `get_mut`, `remove`, etc.).
//! - Complexity: average cases aim for O(1) for the hash map and O(log n) for
//!   the BST, but worst-case complexities are documented in each module.
//! - DoublyLinkedList uses `Rc<RefCell>` + `Weak` for safe back-links and
//!   stores elements as `Option<T>` internally to support moving values out.
//!
//! See the unit tests and integration tests for usage examples.

pub mod ds;

pub use ds::linked_list::LinkedList;
pub use ds::hash_map::SimpleHashMap;
pub use ds::tree_map::BstMap;
pub use ds::doubly_linked_list::DoublyLinkedList;
pub use ds::array_list::ArrayList;
pub use ds::hash_set_ext::HashSetExt;
pub use ds::coords::{Point, Point3, ComplexI};
pub use ds::sparse_grid::SparseGrid;
pub use ds::neighbors::{DELTAS4, DELTAS8};
pub use ds::stack::Stack;
pub use ds::queue::{Queue, Deque};
pub use ds::heap::{MinHeap, MaxHeap};
pub use ds::graph::Adjacency;
pub use ds::dsu::DisjointSet;
pub use ds::topo::{Topo, topo_sort};
pub use ds::intervals::{Interval, IntervalSet};
pub use ds::bitmask::BitMask;
pub use ds::monotonic_queue::{MonotonicQueueMin, MonotonicQueueMax};
pub use ds::dense_grid::DenseGrid2D;
pub use ds::indexed_heap::IndexedMinHeap;
pub use ds::freq_map::FreqMap;
pub use ds::string_alg::{kmp_search, z_function, RollingHash};
pub use ds::scc::tarjan_scc;
pub use ds::fenwick::Fenwick;
pub use ds::search::{bfs_distances, dfs_preorder, dijkstra_indexed, astar_indexed};
pub use ds::parsing::{parse_grid_chars, parse_grid_digits, parse_ints_whitespace, parse_lines_i64};
pub use ds::fcov;
