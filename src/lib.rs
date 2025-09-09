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

pub use linked_list::LinkedList;
pub use hash_map::SimpleHashMap;
pub use tree_map::BstMap;
pub use doubly_linked_list::DoublyLinkedList;
