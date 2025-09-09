//! Frequency map (multiset) over `HashMap<T, usize>` with remove-on-zero.

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FreqMap<T: Eq + Hash> { m: HashMap<T, usize> }

impl<T: Eq + Hash> FreqMap<T> {
    pub fn new() -> Self { Self { m: HashMap::new() } }
    pub fn inc(&mut self, k: T) { *self.m.entry(k).or_default() += 1; }
    pub fn dec(&mut self, k: &T) { if let Some(v)=self.m.get_mut(k){ if *v>1 { *v-=1; } else { self.m.remove(k); } } }
    pub fn get(&self, k: &T) -> usize { self.m.get(k).copied().unwrap_or(0) }
    pub fn len(&self) -> usize { self.m.len() }
    pub fn is_empty(&self) -> bool { self.m.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::FreqMap;
    #[test]
    fn basic() {
        let mut f = FreqMap::new();
        f.inc('a'); f.inc('a'); f.inc('b');
        assert_eq!(f.get(&'a'), 2);
        f.dec(&'a'); assert_eq!(f.get(&'a'), 1);
        f.dec(&'a'); assert_eq!(f.get(&'a'), 0);
        assert_eq!(f.len(), 1); // only 'b'
    }
}

