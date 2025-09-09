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

pub mod linked_list;
pub mod hash_map;
pub mod tree_map;
pub mod fcov;
pub mod doubly_linked_list;
pub mod array_list;
pub mod hash_set_ext;
pub mod coords;
pub mod sparse_grid;
pub mod neighbors;
pub mod stack;
pub mod queue;
pub mod heap;
pub mod graph;
pub mod dsu;
pub mod topo;
pub mod intervals;
pub mod bitmask;
pub mod monotonic_queue;
pub mod dense_grid;
pub mod indexed_heap;
pub mod freq_map;
pub mod string_alg;
pub mod scc;
pub mod fenwick;
pub mod search;
pub mod parsing;

pub use linked_list::LinkedList;
pub use hash_map::SimpleHashMap;
pub use tree_map::BstMap;
pub use doubly_linked_list::DoublyLinkedList;
pub use array_list::ArrayList;
pub use hash_set_ext::HashSetExt;
pub use coords::{Point, Point3, ComplexI};
pub use sparse_grid::SparseGrid;
pub use neighbors::{DELTAS4, DELTAS8};
pub use stack::Stack;
pub use queue::{Queue, Deque};
pub use heap::{MinHeap, MaxHeap};
pub use graph::Adjacency;
pub use dsu::DisjointSet;
pub use topo::{Topo, topo_sort};
pub use intervals::{Interval, IntervalSet};
pub use bitmask::BitMask;
pub use monotonic_queue::{MonotonicQueueMin, MonotonicQueueMax};
pub use dense_grid::DenseGrid2D;
pub use indexed_heap::IndexedMinHeap;
pub use freq_map::FreqMap;
pub use string_alg::{kmp_search, z_function, RollingHash};
pub use scc::tarjan_scc;
pub use fenwick::Fenwick;
pub use search::{bfs_distances, dfs_preorder, dijkstra_indexed, astar_indexed};
pub use parsing::{parse_grid_chars, parse_grid_digits, parse_ints_whitespace, parse_lines_i64};
