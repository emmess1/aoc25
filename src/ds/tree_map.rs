//! A basic ordered map implemented as an unbalanced binary search tree (BST).
//!
//! Design notes
//! - Keys must implement `Ord`; the invariant is: all keys in the left
//!   subtree are `< node.key`, all in the right are `> node.key`.
//! - The tree is unbalanced (not AVL/Red-Black), so worst-case operations can
//!   degrade to O(n) on degenerate inputs (e.g., inserting sorted keys).
//!
//! Complexity (typical)
//! - `insert`, `get`, `get_mut`, `remove`: O(h), where h is the tree height.
//!   With a balanced distribution h ≈ log2(n).
//!
//! Example
//! ```
//! use aoc25::BstMap;
//! let mut m = BstMap::new();
//! m.insert(2, "two");
//! m.insert(1, "one");
//! assert_eq!(m.get(&1), Some(&"one"));
//! assert_eq!(m.remove(&2), Some("two"));
//! ```

use std::cmp::Ordering;

/// A minimal ordered map using an unbalanced binary search tree.
pub struct BstMap<K, V> {
    root: Link<K, V>,
    len: usize,
}

/// Convenience alias for an optional boxed node.
type Link<K, V> = Option<Box<Node<K, V>>>;

/// Internal tree node carrying a key/value plus left/right links.
struct Node<K, V> {
    key: K,
    val: V,
    left: Link<K, V>,
    right: Link<K, V>,
}

impl<K: Ord, V> BstMap<K, V> {
    /// Creates an empty map.
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    /// Returns `true` if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Inserts a key-value pair into the map, returning the old value if the key existed.
    ///
    /// Traverses the tree to find the insertion point. If a node with the
    /// same key exists, replaces its value and returns the old value.
    ///
    /// Example
    /// ```
    /// use aoc25::BstMap;
    /// let mut m = BstMap::new();
    /// assert_eq!(m.insert(5, "a"), None);
    /// assert_eq!(m.insert(5, "b"), Some("a"));
    /// assert_eq!(m.get(&5), Some(&"b"));
    /// ```
    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        let mut link = &mut self.root;
        loop {
            match link {
                Some(node) => match key.cmp(&node.key) {
                    Ordering::Less => link = &mut node.left,
                    Ordering::Greater => link = &mut node.right,
                    Ordering::Equal => {
                        return Some(std::mem::replace(&mut node.val, val));
                    }
                },
                None => {
                    *link = Some(Box::new(Node {
                        key,
                        val,
                        left: None,
                        right: None,
                    }));
                    self.len += 1;
                    return None;
                }
            }
        }
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// Walks down the tree comparing keys until it finds a match or hits a
    /// `None` link.
    ///
    /// Example
    /// ```
    /// use aoc25::BstMap;
    /// let mut m = BstMap::new();
    /// m.insert(1, "x");
    /// assert_eq!(m.get(&1), Some(&"x"));
    /// assert_eq!(m.get(&9), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut cur = self.root.as_deref();
        while let Some(node) = cur {
            match key.cmp(&node.key) {
                Ordering::Less => cur = node.left.as_deref(),
                Ordering::Greater => cur = node.right.as_deref(),
                Ordering::Equal => return Some(&node.val),
            }
        }
        None
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// Example
    /// ```
    /// use aoc25::BstMap;
    /// let mut m = BstMap::new();
    /// m.insert(1, 10);
    /// if let Some(v) = m.get_mut(&1) { *v += 1; }
    /// assert_eq!(m.get(&1), Some(&11));
    /// ```
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut cur = self.root.as_deref_mut();
        while let Some(node) = cur {
            match key.cmp(&node.key) {
                Ordering::Less => cur = node.left.as_deref_mut(),
                Ordering::Greater => cur = node.right.as_deref_mut(),
                Ordering::Equal => return Some(&mut node.val),
            }
        }
        None
    }

    /// Returns true if the key exists in the map.
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Removes the key from the map, returning the stored value if present.
    ///
    /// This delegates to `remove_node`, which updates links in place and
    /// handles the three structural cases: leaf, single child, two children.
    ///
    /// Example
    /// ```
    /// use aoc25::BstMap;
    /// let mut m = BstMap::new();
    /// m.insert(1, "x");
    /// assert_eq!(m.remove(&1), Some("x"));
    /// assert_eq!(m.remove(&1), None);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let removed = remove_node(&mut self.root, key);
        if removed.is_some() {
            self.len -= 1;
        }
        removed
    }
}

fn remove_node<K: Ord, V>(link: &mut Link<K, V>, key: &K) -> Option<V> {
    let node = link.as_mut()?;
    match key.cmp(&node.key) {
        Ordering::Less => return remove_node(&mut node.left, key),
        Ordering::Greater => return remove_node(&mut node.right, key),
        Ordering::Equal => {}
    }

    // Found the node to remove. Take it out, rebuild the subtree according to
    // the standard BST deletion rules, and return the removed value.
    let boxed = link.take().unwrap();
    let Node {
        key: _,
        val,
        left,
        right,
    } = *boxed;
    *link = match (left, right) {
        // Case 1: leaf
        (None, None) => None,
        // Case 2a: only left child -> promote left
        (Some(l), None) => Some(l),
        // Case 2b: only right child -> promote right
        (None, Some(r)) => Some(r),
        // Case 3: two children -> replace by successor (min of right subtree)
        (Some(l), Some(r)) => {
            let ((min_k, min_v), new_right) = pop_min(r);
            Some(Box::new(Node {
                key: min_k,
                val: min_v,
                left: Some(l),
                right: new_right,
            }))
        }
    };
    Some(val)
}

// Removes and returns the minimum (key, value) from the given subtree,
// along with the remaining subtree with that minimum removed.
//
// The successor (minimum of right subtree) becomes the replacement for a
// removed node with two children. We recursively walk left until we hit the
// leftmost node, then splice it out and return it.
fn pop_min<K: Ord, V>(mut node: Box<Node<K, V>>) -> ((K, V), Link<K, V>) {
    if node.left.is_none() {
        let Node {
            key, val, right, ..
        } = *node;
        return ((key, val), right);
    }
    let left = node.left.take().unwrap();
    let ((k, v), new_left) = pop_min(left);
    node.left = new_left;
    ((k, v), Some(node))
}

#[cfg(test)]
mod tests {
    use super::BstMap;

    #[test]
    fn empty_map() {
        let m: BstMap<i32, i32> = BstMap::new();
        assert!(m.is_empty());
        assert_eq!(m.len(), 0);
        assert_eq!(m.get(&1), None);
        assert!(!m.contains_key(&1));
    }

    #[test]
    fn get_and_get_mut_on_empty() {
        let mut m: BstMap<i32, i32> = BstMap::new();
        assert_eq!(m.get(&42), None);
        assert!(m.get_mut(&42).is_none());
        assert_eq!(m.remove(&42), None);
    }

    #[test]
    fn insert_and_get() {
        let mut m = BstMap::new();
        assert_eq!(m.insert(5, "a"), None);
        assert_eq!(m.insert(3, "b"), None);
        assert_eq!(m.insert(7, "c"), None);
        assert_eq!(m.len(), 3);

        assert_eq!(m.get(&5), Some(&"a"));
        assert_eq!(m.get(&3), Some(&"b"));
        assert_eq!(m.get(&7), Some(&"c"));
        assert_eq!(m.get(&9), None);
        assert!(m.contains_key(&3));
        assert!(!m.is_empty());
    }

    #[test]
    fn get_mut_missing_left_and_right_paths() {
        let mut m = BstMap::new();
        for k in [5, 3, 7] {
            m.insert(k, k * 10);
        }
        // Missing left path: 2 < 5 then None
        assert!(m.get_mut(&2).is_none());
        // Missing right path: 9 > 7 then None
        assert!(m.get_mut(&9).is_none());
    }

    #[test]
    fn update_existing_and_get_mut() {
        let mut m = BstMap::new();
        assert_eq!(m.insert(10, 1), None);
        assert_eq!(m.insert(10, 2), Some(1));
        assert_eq!(m.get(&10), Some(&2));
        if let Some(v) = m.get_mut(&10) {
            *v += 3;
        }
        assert_eq!(m.get(&10), Some(&5));
    }

    #[test]
    fn remove_leaf_node() {
        let mut m = BstMap::new();
        m.insert(2, "x");
        m.insert(1, "y");
        m.insert(3, "z");

        // 1 is a leaf
        assert_eq!(m.remove(&1), Some("y"));
        assert!(!m.contains_key(&1));
        assert_eq!(m.len(), 2);
    }

    #[test]
    fn remove_node_with_one_child() {
        let mut m = BstMap::new();
        m.insert(2, "x");
        m.insert(1, "y");
        m.insert(3, "z");
        m.insert(4, "w"); // 3 has one right child

        assert_eq!(m.remove(&3), Some("z"));
        assert!(!m.contains_key(&3));
        assert_eq!(m.get(&4), Some(&"w"));
        assert_eq!(m.len(), 3);
    }

    #[test]
    fn remove_node_with_two_children() {
        let mut m = BstMap::new();
        // Build a tree where 5 has two children
        for (k, v) in [
            (5, 'a'),
            (3, 'b'),
            (7, 'c'),
            (2, 'd'),
            (4, 'e'),
            (6, 'f'),
            (8, 'g'),
        ] {
            m.insert(k, v);
        }
        assert_eq!(m.len(), 7);
        assert_eq!(m.remove(&5), Some('a'));
        assert!(!m.contains_key(&5));
        assert_eq!(m.len(), 6);
        // Remaining keys should still be present
        for k in [2, 3, 4, 6, 7, 8] {
            assert!(m.contains_key(&k));
        }
    }

    #[test]
    fn remove_two_children_immediate_successor() {
        // Right child is the successor (no left subtree under right)
        let mut m = BstMap::new();
        for k in [5, 3, 6, 2, 4, 7] {
            m.insert(k, k);
        }
        assert_eq!(m.remove(&5), Some(5));
        assert!(!m.contains_key(&5));
        // 6 should be new root, 7 remains in right subtree
        assert!(m.contains_key(&6));
        assert!(m.contains_key(&7));
    }

    #[test]
    fn skewed_insert_and_removes() {
        let mut m = BstMap::new();
        // Insert ascending to form a skewed tree
        for i in 0..20 {
            m.insert(i, i * 2);
        }
        assert_eq!(m.len(), 20);
        for i in 0..20 {
            assert_eq!(m.get(&i), Some(&(i * 2)));
        }
        // Remove evens
        for i in (0..20).step_by(2) {
            assert_eq!(m.remove(&i), Some(i * 2));
        }
        for i in (0..20).step_by(2) {
            assert!(!m.contains_key(&i));
        }
        assert_eq!(m.len(), 10);
        // Remove odds
        for i in (1..20).step_by(2) {
            assert!(m.remove(&i).is_some());
        }
        assert!(m.is_empty());
    }

    #[test]
    fn remove_root_with_one_child() {
        let mut m = BstMap::new();
        m.insert(1, 'a');
        m.insert(2, 'b'); // root has only right child
        assert_eq!(m.remove(&1), Some('a'));
        assert_eq!(m.len(), 1);
        assert!(m.contains_key(&2));
    }

    #[test]
    fn remove_root_with_one_left_child() {
        let mut m = BstMap::new();
        m.insert(2, 'a');
        m.insert(1, 'b'); // root has only left child
        assert_eq!(m.remove(&2), Some('a'));
        assert_eq!(m.len(), 1);
        assert!(m.contains_key(&1));
    }

    #[test]
    fn remove_root_with_two_children() {
        let mut m = BstMap::new();
        m.insert(2, 'a');
        m.insert(1, 'b');
        m.insert(3, 'c');
        assert_eq!(m.remove(&2), Some('a'));
        assert_eq!(m.len(), 2);
        assert!(m.contains_key(&1));
        assert!(m.contains_key(&3));
    }

    #[test]
    fn remove_missing_no_change() {
        let mut m = BstMap::new();
        m.insert(10, 1);
        let len_before = m.len();
        assert_eq!(m.remove(&9), None);
        assert_eq!(m.len(), len_before);
    }

    #[test]
    fn remove_missing_on_right_path() {
        let mut m = BstMap::new();
        for k in [10, 15, 20] {
            m.insert(k, k);
        }
        let len_before = m.len();
        // Traverse right twice then miss
        assert_eq!(m.remove(&25), None);
        assert_eq!(m.len(), len_before);
    }

    #[test]
    fn remove_two_children_successor_has_right_child_strings() {
        // Use strings to allow a successor with a right child (e.g., "6" -> right "6a").
        let mut m: BstMap<String, char> = BstMap::new();
        for (k, v) in [
            ("5".to_string(), 'a'),
            ("3".to_string(), 'b'),
            ("7".to_string(), 'c'),
            ("2".to_string(), 'd'),
            ("4".to_string(), 'e'),
            ("6".to_string(), 'f'),
            ("6a".to_string(), 'h'), // right child of successor
        ] {
            m.insert(k, v);
        }
        assert_eq!(m.remove(&"5".to_string()), Some('a'));
        // Ensure successor's right child was reattached
        assert!(m.contains_key(&"6".to_string()));
        assert!(m.contains_key(&"6a".to_string()));
        assert!(m.contains_key(&"7".to_string()));
    }

    #[test]
    fn remove_node_with_only_left_child_non_root() {
        let mut m = BstMap::new();
        // 5 is root; 3 has only left child 2; 7 is right
        for k in [5, 3, 7, 2] {
            m.insert(k, k * 10);
        }
        assert_eq!(m.remove(&3), Some(30));
        assert!(!m.contains_key(&3));
        assert!(m.contains_key(&2));
        assert!(m.contains_key(&5));
        assert!(m.contains_key(&7));
    }
}
