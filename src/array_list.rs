//! ArrayList: a thin, well-documented wrapper over `Vec<T>` for AoC-style usage.
//!
//! Provides a small API for sequence building, indexed access, slicing, and iteration.

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ArrayList<T> {
    inner: Vec<T>,
}

impl<T> ArrayList<T> {
    /// Create an empty list.
    pub fn new() -> Self { Self { inner: Vec::new() } }

    /// Create with capacity.
    pub fn with_capacity(n: usize) -> Self { Self { inner: Vec::with_capacity(n) } }

    /// Build from an iterator.
    pub fn from_iter<I: IntoIterator<Item = T>>(it: I) -> Self { Self { inner: it.into_iter().collect() } }

    /// Number of elements.
    pub fn len(&self) -> usize { self.inner.len() }

    /// Is empty.
    pub fn is_empty(&self) -> bool { self.inner.is_empty() }

    /// Push a value to the end.
    pub fn push(&mut self, v: T) { self.inner.push(v) }

    /// Pop a value from the end.
    pub fn pop(&mut self) -> Option<T> { self.inner.pop() }

    /// Get immutable reference by index (returns None if out of bounds).
    pub fn get(&self, idx: usize) -> Option<&T> { self.inner.get(idx) }

    /// Get mutable reference by index (returns None if out of bounds).
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> { self.inner.get_mut(idx) }

    /// Return an immutable slice of the list for the given range.
    pub fn slice(&self, range: std::ops::Range<usize>) -> &[T] { &self.inner[range] }

    /// Return a mutable slice of the list for the given range.
    pub fn slice_mut(&mut self, range: std::ops::Range<usize>) -> &mut [T] { &mut self.inner[range] }

    /// Iterate immutably.
    pub fn iter(&self) -> std::slice::Iter<'_, T> { self.inner.iter() }

    /// Iterate mutably.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> { self.inner.iter_mut() }

    /// Into the inner Vec.
    pub fn into_vec(self) -> Vec<T> { self.inner }
}

impl<T> From<Vec<T>> for ArrayList<T> {
    fn from(v: Vec<T>) -> Self { Self { inner: v } }
}

#[cfg(test)]
mod tests {
    use super::ArrayList;

    #[test]
    fn basic_ops() {
        let mut a = ArrayList::new();
        assert!(a.is_empty());
        a.push(1);
        a.push(2);
        a.push(3);
        assert_eq!(a.len(), 3);
        assert_eq!(a.get(1), Some(&2));
        if let Some(x) = a.get_mut(1) { *x = 20; }
        assert_eq!(a.get(1), Some(&20));
        assert_eq!(a.slice(0..2), &[1,20]);
        a.slice_mut(1..3).iter_mut().for_each(|x| *x += 1);
        assert_eq!(a.into_vec(), vec![1,21,4]);
    }

    #[test]
    fn from_iter_iter() {
        let a = ArrayList::from_iter(0..5);
        let v: Vec<_> = a.iter().cloned().collect();
        assert_eq!(v, vec![0,1,2,3,4]);
    }
}

