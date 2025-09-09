//! A simple singly linked list.
//!
//! Scope and intent
//! - Clarity first: a compact, readable implementation of a classic list.
//! - Core operations only: push/pop at the front, peek, and iteration.
//!
//! Complexity overview
//! - `push_front`, `pop_front`, `peek`, `peek_mut`: O(1)
//! - `iter`: O(n) to traverse all elements
//!
//! Example
//! ```
//! use data_structures::LinkedList;
//!
//! let mut ll = LinkedList::new();
//! ll.push_front(2);
//! ll.push_front(1);
//! assert_eq!(ll.peek(), Some(&1));
//! assert_eq!(ll.pop_front(), Some(1));
//! assert_eq!(ll.pop_front(), Some(2));
//! assert!(ll.is_empty());
//! ```

/// A minimal, singly linked list storing elements of type `T`.
///
/// The list holds a pointer to the head node and a running length. Each node
/// stores the element and a link to the next node (if any).
#[derive(Default)]
pub struct LinkedList<T> {
    head: Link<T>,
    len: usize,
}

type Link<T> = Option<Box<Node<T>>>;

/// Internal node type that links an element to the next node.
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> LinkedList<T> {
    /// Creates an empty list.
    ///
    /// Example
    /// ```
    /// use data_structures::LinkedList;
    /// let ll: LinkedList<i32> = LinkedList::new();
    /// assert!(ll.is_empty());
    /// ```
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    /// Returns `true` if the list contains no elements.
    pub fn is_empty(&self) -> bool { self.len == 0 }

    /// Returns number of elements in the list.
    pub fn len(&self) -> usize { self.len }

    /// Pushes an element to the front of the list.
    ///
    /// Example
    /// ```
    /// use data_structures::LinkedList;
    /// let mut ll = LinkedList::new();
    /// ll.push_front(1);
    /// assert_eq!(ll.peek(), Some(&1));
    /// ```
    pub fn push_front(&mut self, elem: T) {
        // Allocate a new node that points to the current head, then update
        // the head to the new node.
        let new = Box::new(Node { elem, next: self.head.take() });
        self.head = Some(new);
        self.len += 1;
    }

    /// Pops an element from the front of the list.
    ///
    /// Returns `None` if the list is empty.
    ///
    /// Example
    /// ```
    /// use data_structures::LinkedList;
    /// let mut ll = LinkedList::new();
    /// ll.push_front(1);
    /// assert_eq!(ll.pop_front(), Some(1));
    /// assert_eq!(ll.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        // Take the head out, relink to its successor if present, and return
        // the element. This avoids recursive drops and runs in O(1).
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            self.len -= 1;
            node.elem
        })
    }

    /// Returns a reference to the front element, if any.
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.elem)
    }

    /// Returns a mutable reference to the front element, if any.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|n| &mut n.elem)
    }

    /// Clears the list, removing all elements.
    pub fn clear(&mut self) {
        // Iteratively pop nodes to free them without recursion.
        while self.pop_front().is_some() {}
    }

    /// Returns an iterator over references to each element.
    ///
    /// Elements are yielded from head to tail.
    pub fn iter(&self) -> Iter<'_, T> { Iter { next: self.head.as_deref() } }

    // A mutable iterator can be added later if needed.
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // Ensure we drop nodes iteratively to avoid stack overflows on long
        // lists.
        self.clear();
    }
}

/// Iterator yielding `&T` from head to tail.
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// Mutable iteration intentionally omitted to keep this simple and safe.

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn new_list_is_empty() {
        let ll: LinkedList<i32> = LinkedList::new();
        assert!(ll.is_empty());
        assert_eq!(ll.len(), 0);
        assert!(ll.peek().is_none());
    }

    #[test]
    fn push_and_pop_front_many() {
        let mut ll = LinkedList::new();
        for i in 0..5 {
            ll.push_front(i);
        }
        assert_eq!(ll.len(), 5);
        assert_eq!(ll.peek(), Some(&4));

        for expected in (0..5).rev() {
            assert_eq!(ll.pop_front(), Some(expected));
        }
        assert!(ll.is_empty());
        assert_eq!(ll.pop_front(), None);
    }

    #[test]
    fn peek_and_peek_mut() {
        let mut ll = LinkedList::new();
        ll.push_front(10);
        assert_eq!(ll.peek(), Some(&10));
        if let Some(x) = ll.peek_mut() {
            *x = 20;
        }
        assert_eq!(ll.peek(), Some(&20));
    }

    #[test]
    fn iterates_in_stack_order() {
        let mut ll = LinkedList::new();
        for i in 1..=4 { ll.push_front(i); }
        let v: Vec<_> = ll.iter().cloned().collect();
        assert_eq!(v, vec![4, 3, 2, 1]);
    }

    #[test]
    fn clear_empties_list() {
        let mut ll = LinkedList::new();
        for i in 0..3 { ll.push_front(i); }
        ll.clear();
        assert!(ll.is_empty());
        assert_eq!(ll.len(), 0);
        assert!(ll.peek().is_none());
    }

    #[test]
    fn interleaved_ops() {
        let mut ll = LinkedList::new();
        ll.push_front(1);
        ll.push_front(2);
        assert_eq!(ll.pop_front(), Some(2));
        ll.push_front(3);
        assert_eq!(ll.peek(), Some(&3));
        assert_eq!(ll.pop_front(), Some(3));
        assert_eq!(ll.pop_front(), Some(1));
        assert_eq!(ll.pop_front(), None);
    }

    #[test]
    fn reuse_after_clear() {
        let mut ll = LinkedList::new();
        for i in 0..10 { ll.push_front(i); }
        ll.clear();
        for i in 10..20 { ll.push_front(i); }
        assert_eq!(ll.len(), 10);
        assert_eq!(ll.peek(), Some(&19));
    }

    #[test]
    fn iter_empty() {
        let ll: LinkedList<i32> = LinkedList::new();
        assert_eq!(ll.iter().next(), None);
    }

    #[test]
    fn stress_push_pop_1000() {
        let mut ll = LinkedList::new();
        for i in 0..1000 { ll.push_front(i); }
        assert_eq!(ll.len(), 1000);
        for i in (0..1000).rev() { assert_eq!(ll.pop_front(), Some(i)); }
        assert!(ll.is_empty());
    }
}
