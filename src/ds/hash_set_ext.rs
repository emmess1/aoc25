//! HashSetExt: thin convenience wrapper around `std::collections::HashSet` for AoC.

use std::collections::HashSet as StdHashSet;
use std::hash::Hash;

#[derive(Clone, Debug, Default)]
pub struct HashSetExt<T: Eq + Hash>(StdHashSet<T>);

impl<T: Eq + Hash> HashSetExt<T> {
    /// Create empty.
    pub fn new() -> Self { Self(StdHashSet::new()) }
    /// Insert value, returns true if not present.
    pub fn insert(&mut self, v: T) -> bool { self.0.insert(v) }
    /// Check membership.
    pub fn contains(&self, v: &T) -> bool { self.0.contains(v) }
    /// Bulk insert from iterator.
    pub fn extend<I: IntoIterator<Item = T>>(&mut self, it: I) { self.0.extend(it) }
    /// Current size.
    pub fn len(&self) -> usize { self.0.len() }
    /// Is empty.
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    /// Clear all.
    pub fn clear(&mut self) { self.0.clear() }
}

impl<T: Eq + Hash> From<StdHashSet<T>> for HashSetExt<T> {
    fn from(s: StdHashSet<T>) -> Self { Self(s) }
}

#[cfg(test)]
mod tests {
    use super::HashSetExt;
    use std::collections::HashSet as StdHashSet;

    #[test]
    fn basic() {
        let mut s = HashSetExt::new();
        assert!(s.insert(1));
        assert!(!s.insert(1));
        assert!(s.contains(&1));
        s.extend([2,3]);
        assert_eq!(s.len(), 3);
        s.clear();
        assert!(s.is_empty());
    }

    #[test]
    fn empty_contains_len() {
        let s: HashSetExt<i32> = HashSetExt::new();
        assert!(!s.contains(&42));
        assert_eq!(s.len(), 0);
        assert!(s.is_empty());
    }

    #[test]
    fn from_std_set() {
        let mut stds = StdHashSet::new();
        stds.insert(1);
        stds.insert(2);
        let s: HashSetExt<i32> = stds.into();
        assert!(s.contains(&1) && s.contains(&2));
        assert_eq!(s.len(), 2);
    }
}
