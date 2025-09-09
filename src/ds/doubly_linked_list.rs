//! A safe doubly linked list using `Rc<RefCell<...>>` for forward links
//! and `Weak` for backward links to avoid reference cycles.
//!
//! - Each node is reference-counted (`Rc`) and interior-mutable (`RefCell`).
//! - Back-pointers use `Weak` so head/tail chains don't form cycles.
//! - Elements are stored as `Option<T>` inside each node so we can move them
//!   out safely even if other `Rc` clones of the same node exist.
//!
//! Invariants (maintained by all mutation operations):
//! - `len` equals the number of nodes reachable from `head` via `next`.
//! - If `head.is_some()` then `head.prev` is `None`; otherwise `tail` is `None`.
//! - If `tail.is_some()` then `tail.next` is `None`; otherwise `head` is `None`.
//! - For any adjacent nodes A -> B, `B.prev` is a `Weak` to A, and
//!   upgrading it yields the same `Rc` as A (while B is in the list).
//! - For nodes currently in the list, `elem` is `Some(T)`; once structurally
//!   removed, we `take()` the element, leaving `None` in that node.
//!
//! Operations like push/pop at both ends are O(1). Peeking returns a clone
//! of the element to avoid borrowing lifetimes through RefCell — this keeps
//! the API simple and safe.
//!
//! Example
//! ```
//! use data_structures::DoublyLinkedList;
//! let mut dl = DoublyLinkedList::new();
//! dl.push_front(2);
//! dl.push_front(1);
//! assert_eq!(dl.peek_front(), Some(1));
//! assert_eq!(dl.pop_front_cloned(), Some(1));
//! assert_eq!(dl.pop_back_cloned(), Some(2));
//! assert!(dl.is_empty());
//! ```

use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Strong link to a node in the list. `None` represents the end.
type Link<T> = Option<Rc<RefCell<Node<T>>>>;
// Back-pointer from a node to its predecessor. Stored as `Weak` to avoid
// creating reference cycles between neighbors.
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

// Internal list node. Not exposed publicly.
struct Node<T> {
    // Element payload; set to None when moved out during a pop.
    elem: Option<T>,
    // Strong pointer to the next node.
    next: Link<T>,
    // Weak pointer to the previous node to prevent reference cycles.
    prev: WeakLink<T>,
}

/// A doubly linked list supporting O(1) push/pop at both ends.
///
/// Cloning of values is required for non-consuming peek/pop helpers to keep
/// the API lifetime-free and ergonomic with `RefCell`.
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    /// Creates an empty list.
    pub fn new() -> Self {
        Self { head: None, tail: None, len: 0 }
    }

    /// Returns `true` if the list contains no elements.
    pub fn is_empty(&self) -> bool { self.len == 0 }

    /// Returns number of elements in the list.
    pub fn len(&self) -> usize { self.len }

    /// Pushes an element to the front (head) of the list. O(1).
    pub fn push_front(&mut self, elem: T) {
        let new = Rc::new(RefCell::new(Node { elem: Some(elem), next: self.head.clone(), prev: None }));
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new));
                self.head = Some(new);
            }
            None => {
                // Empty list: head and tail both point to `new`.
                self.tail = Some(new.clone());
                self.head = Some(new);
            }
        }
        self.len += 1;
    }

    /// Pushes an element to the back (tail) of the list. O(1).
    pub fn push_back(&mut self, elem: T) {
        let new = Rc::new(RefCell::new(Node { elem: Some(elem), next: None, prev: None }));
        match self.tail.take() {
            Some(old_tail) => {
                new.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                old_tail.borrow_mut().next = Some(new.clone());
                self.tail = Some(new);
            }
            None => {
                // Empty list: head and tail both point to `new`.
                self.head = Some(new.clone());
                self.tail = Some(new);
            }
        }
        self.len += 1;
    }

    // Note: public non-cloning pops are intentionally omitted to avoid
    // requiring unsafe extraction from RefCell-managed nodes. Use
    // `pop_front_cloned` / `pop_back_cloned` or the consuming iterator.

    /// Returns a clone of the front element, if any. O(1).
    /// Requires `T: Clone`.
    pub fn peek_front(&self) -> Option<T> where T: Clone {
        self.head.as_ref().and_then(|rc| rc.borrow().elem.as_ref().cloned())
    }

    /// Returns a clone of the back element, if any. O(1).
    /// Requires `T: Clone`.
    pub fn peek_back(&self) -> Option<T> where T: Clone {
        self.tail.as_ref().and_then(|rc| rc.borrow().elem.as_ref().cloned())
    }

    /// Clears the list by popping from the front until empty. O(n).
    pub fn clear(&mut self) {
        while self.pop_front_value().is_some() {}
    }

    // Internal helper that pops the front node and returns owned `T`.
    // Steps:
    // 1) Detach current head from the list and fix neighbor links.
    // 2) Decrement `len` and update `head`/`tail` as needed.
    // 3) Move the element out of the node's `RefCell` (leaving `None`).
    fn pop_front_value(&mut self) -> Option<T> {
        let head = self.head.take()?;
        // Break links first to ensure list invariants hold even if external
        // `Rc` clones of this node exist.
        let mut head_b = head.borrow_mut();
        let next = head_b.next.take();
        if let Some(ref n) = next { n.borrow_mut().prev = None; }
        drop(head_b);
        self.head = next;
        if self.head.is_none() { self.tail = None; }
        self.len -= 1;
        // Move the payload out of the node. We do NOT rely on unique `Rc`
        // ownership here; taking from the `RefCell` is safe even if other
        // `Rc` clones of the same node still exist elsewhere. Use a local
        // binding to ensure the mutable borrow ends before `head` is dropped.
        let mut cell = head.borrow_mut();
        cell.elem.take()
    }

    // Internal helper that pops the back node and returns owned `T`.
    // Mirrors `pop_front_value`, fixing neighbor links and draining payload.
    fn pop_back_value(&mut self) -> Option<T> {
        let tail = self.tail.take()?;
        let mut tail_b = tail.borrow_mut();
        let prev_weak = tail_b.prev.take();
        let prev = prev_weak.and_then(|w| w.upgrade());
        if let Some(ref p) = prev { p.borrow_mut().next = None; }
        drop(tail_b);
        self.tail = prev.clone();
        if self.tail.is_none() { self.head = None; }
        self.len -= 1;
        // Same borrow-shortening pattern as in `pop_front_value`.
        let mut cell = tail.borrow_mut();
        cell.elem.take()
    }

    /// Pops and returns the front element by cloning it out. O(1).
    /// Requires `T: Clone`.
    pub fn pop_front_cloned(&mut self) -> Option<T> where T: Clone {
        let val = self.peek_front()?;
        // remove front node structurally
        let _ = self.pop_front_value();
        Some(val)
    }

    /// Pops and returns the back element by cloning it out. O(1).
    /// Requires `T: Clone`.
    pub fn pop_back_cloned(&mut self) -> Option<T> where T: Clone {
        let val = self.peek_back()?;
        let _ = self.pop_back_value();
        Some(val)
    }

    /// Consuming iterator that drains the list by popping from the front.
    /// Each `next()` is O(1). After consumption the list is empty.
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}

/// An owning iterator that drains the list from front to back.
/// Holds the list by value and repeatedly removes the head node.
pub struct IntoIter<T> {
    list: DoublyLinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> { self.list.pop_front_value() }
}

#[cfg(test)]
mod tests {
    use super::DoublyLinkedList;

    #[test]
    fn new_is_empty() {
        let dl: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert!(dl.is_empty());
        assert_eq!(dl.len(), 0);
    }

    #[test]
    fn push_front_and_peek() {
        let mut dl = DoublyLinkedList::new();
        dl.push_front(2);
        dl.push_front(1);
        assert_eq!(dl.len(), 2);
        assert_eq!(dl.peek_front(), Some(1));
        assert_eq!(dl.peek_back(), Some(2));
    }

    #[test]
    fn push_back_and_peek() {
        let mut dl = DoublyLinkedList::new();
        dl.push_back(1);
        dl.push_back(2);
        assert_eq!(dl.len(), 2);
        assert_eq!(dl.peek_front(), Some(1));
        assert_eq!(dl.peek_back(), Some(2));
    }

    #[test]
    fn pop_front_cloned_sequence() {
        let mut dl = DoublyLinkedList::new();
        for i in 1..=3 { dl.push_back(i); }
        assert_eq!(dl.pop_front_cloned(), Some(1));
        assert_eq!(dl.pop_front_cloned(), Some(2));
        assert_eq!(dl.pop_front_cloned(), Some(3));
        assert_eq!(dl.pop_front_cloned(), None);
        assert!(dl.is_empty());
    }

    #[test]
    fn pop_back_cloned_sequence() {
        let mut dl = DoublyLinkedList::new();
        for i in 1..=3 { dl.push_back(i); }
        assert_eq!(dl.pop_back_cloned(), Some(3));
        assert_eq!(dl.pop_back_cloned(), Some(2));
        assert_eq!(dl.pop_back_cloned(), Some(1));
        assert_eq!(dl.pop_back_cloned(), None);
        assert!(dl.is_empty());
    }

    #[test]
    fn mixed_ops() {
        let mut dl = DoublyLinkedList::new();
        dl.push_back(1);
        dl.push_front(0);
        dl.push_back(2);
        assert_eq!(dl.peek_front(), Some(0));
        assert_eq!(dl.peek_back(), Some(2));
        assert_eq!(dl.pop_front_cloned(), Some(0));
        assert_eq!(dl.pop_back_cloned(), Some(2));
        assert_eq!(dl.pop_front_cloned(), Some(1));
        assert!(dl.is_empty());
    }

    #[test]
    fn clear_and_reuse() {
        let mut dl = DoublyLinkedList::new();
        for i in 0..5 { dl.push_back(i); }
        dl.clear();
        assert!(dl.is_empty());
        for i in 5..10 { dl.push_front(i); }
        assert_eq!(dl.peek_front(), Some(9));
        assert_eq!(dl.len(), 5);
    }

    #[test]
    fn into_iter_drains() {
        let mut dl = DoublyLinkedList::new();
        for i in 0..4 { dl.push_back(i); }
        let v: Vec<_> = dl.into_iter().collect();
        assert_eq!(v, vec![0,1,2,3]);
    }

    #[test]
    fn peek_on_empty_and_clear_empty() {
        let mut dl: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert_eq!(dl.peek_front(), None);
        assert_eq!(dl.peek_back(), None);
        dl.clear(); // idempotent
        assert!(dl.is_empty());
    }

    #[test]
    fn single_element_pop_from_back_and_front() {
        let mut dl = DoublyLinkedList::new();
        dl.push_back(42);
        assert_eq!(dl.pop_back_cloned(), Some(42));
        assert!(dl.is_empty());

        dl.push_front(7);
        assert_eq!(dl.pop_front_cloned(), Some(7));
        assert!(dl.is_empty());
    }
}
