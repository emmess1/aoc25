//! A simple hash map using separate chaining with vectors and resizing.
//!
//! Design
//! - Buckets are `Vec<(K, V)>`, stored in a `Vec` of fixed capacity.
//! - Capacity is always a power of two, so we can use a fast bitmask instead
//!   of a modulo when mapping a hash to a bucket index.
//! - On insert, when the load factor exceeds `LOAD_FACTOR`, we double the
//!   number of buckets and rehash existing entries.
//!
//! Complexity (average, with a good hash function)
//! - `insert`, `get`, `get_mut`, `remove`: O(1) average; O(n) worst case
//!   if many keys collide into the same bucket.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Default initial number of buckets (rounded to power of two internally).
const INITIAL_CAPACITY: usize = 16;
/// Threshold to trigger a resize: items / buckets > LOAD_FACTOR.
const LOAD_FACTOR: f64 = 0.75;

/// A minimal, generic hash map using separate chaining.
pub struct SimpleHashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K: Eq + Hash, V> SimpleHashMap<K, V> {
    /// Create an empty map.
    ///
    /// Example
    /// ```
    /// use aoc25::SimpleHashMap;
    /// let m: SimpleHashMap<&str, i32> = SimpleHashMap::new();
    /// assert!(m.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::with_capacity(INITIAL_CAPACITY)
    }

    /// Create with at least the requested number of buckets.
    ///
    /// The actual capacity is rounded up to a power of two and at least 1.
    pub fn with_capacity(capacity: usize) -> Self {
        let cap = capacity.max(1).next_power_of_two();
        let mut buckets = Vec::with_capacity(cap);
        buckets.resize_with(cap, Vec::new);
        Self { buckets, items: 0 }
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.items
    }

    /// Returns `true` if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.items == 0
    }

    /// Map a key to a bucket index using the default hasher.
    ///
    /// Because `buckets.len()` is a power of two, we can mask the low bits
    /// instead of performing a modulo, which is typically faster.
    fn bucket_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) & (self.buckets.len() - 1)
    }

    /// Whether we should grow the table based on the current load factor.
    fn needs_resize(&self) -> bool {
        let lf = self.items as f64 / self.buckets.len() as f64;
        lf > LOAD_FACTOR
    }

    /// Resize the table to `new_cap` buckets and rehash all entries.
    fn rehash(&mut self, new_cap: usize) {
        let cap = new_cap.max(1).next_power_of_two();
        let mut new_buckets: Vec<Vec<(K, V)>> = Vec::with_capacity(cap);
        new_buckets.resize_with(cap, Vec::new);

        for bucket in self.buckets.iter_mut() {
            for (k, v) in bucket.drain(..) {
                let mut hasher = DefaultHasher::new();
                k.hash(&mut hasher);
                let idx = (hasher.finish() as usize) & (cap - 1);
                new_buckets[idx].push((k, v));
            }
        }
        self.buckets = new_buckets;
    }

    /// Inserts a key-value pair into the map.
    ///
    /// Returns the old value if the key was present, otherwise `None`.
    /// Triggers a rehash when the load factor threshold is exceeded.
    ///
    /// Example
    /// ```
    /// use aoc25::SimpleHashMap;
    /// let mut m = SimpleHashMap::new();
    /// assert_eq!(m.insert("a", 1), None);
    /// assert_eq!(m.insert("a", 2), Some(1));
    /// assert_eq!(m.get(&"a"), Some(&2));
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.needs_resize() {
            self.rehash(self.buckets.len() * 2);
        }
        let idx = self.bucket_index(&key);
        let bucket = &mut self.buckets[idx];
        for (k, v) in bucket.iter_mut() {
            if k == &key {
                return Some(std::mem::replace(v, value));
            }
        }
        bucket.push((key, value));
        self.items += 1;
        None
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// Example
    /// ```
    /// use aoc25::SimpleHashMap;
    /// let mut m = SimpleHashMap::new();
    /// m.insert("k", 1);
    /// assert_eq!(m.get(&"k"), Some(&1));
    /// assert_eq!(m.get(&"missing"), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        let idx = self.bucket_index(key);
        self.buckets[idx]
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// Example
    /// ```
    /// use aoc25::SimpleHashMap;
    /// let mut m = SimpleHashMap::new();
    /// m.insert("k", 1);
    /// if let Some(v) = m.get_mut(&"k") { *v += 1; }
    /// assert_eq!(m.get(&"k"), Some(&2));
    /// ```
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let idx = self.bucket_index(key);
        self.buckets[idx]
            .iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    /// Returns true if the key exists in the map.
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Removes and returns the value corresponding to the key.
    ///
    /// Uses `swap_remove` to keep deletion O(1) within a bucket, which may
    /// reorder elements inside that bucket (acceptable for a hash table).
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let idx = self.bucket_index(key);
        let bucket = &mut self.buckets[idx];
        if let Some(pos) = bucket.iter().position(|(k, _)| k == key) {
            self.items -= 1;
            let (_, v) = bucket.swap_remove(pos);
            return Some(v);
        }
        None
    }

    /// Clears the map, removing all key-value pairs.
    ///
    /// Note: bucket order is not meaningful and may change after operations.
    pub fn clear(&mut self) {
        for bucket in self.buckets.iter_mut() {
            bucket.clear();
        }
        self.items = 0;
    }

    #[cfg(test)]
    pub(crate) fn bucket_count(&self) -> usize {
        self.buckets.len()
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleHashMap;
    use std::hash::{Hash, Hasher};

    #[test]
    fn empty_map() {
        let m: SimpleHashMap<i32, i32> = SimpleHashMap::new();
        assert!(m.is_empty());
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn insert_get_update_remove() {
        let mut m = SimpleHashMap::new();
        assert_eq!(m.insert("a", 1), None);
        assert_eq!(m.insert("b", 2), None);
        assert_eq!(m.len(), 2);
        assert_eq!(m.get(&"a"), Some(&1));
        assert_eq!(m.get(&"b"), Some(&2));
        assert!(m.contains_key(&"a"));

        // update existing
        assert_eq!(m.insert("a", 10), Some(1));
        assert_eq!(m.get(&"a"), Some(&10));

        // get_mut
        if let Some(v) = m.get_mut(&"b") {
            *v = 20;
        }
        assert_eq!(m.get(&"b"), Some(&20));

        // remove
        assert_eq!(m.remove(&"a"), Some(10));
        assert!(!m.contains_key(&"a"));
        assert_eq!(m.remove(&"missing"), None);
        assert_eq!(m.len(), 1);
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Collide(u64);
    impl Hash for Collide {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // Force all keys to the same bucket.
            0u64.hash(state);
        }
    }

    #[test]
    fn handles_collisions() {
        let mut m = SimpleHashMap::with_capacity(4);
        for i in 0..10 {
            assert_eq!(m.insert(Collide(i), i as i32), None);
        }
        for i in 0..10 {
            assert_eq!(m.get(&Collide(i)), Some(&(i as i32)));
        }
        // Remove a middle one to exercise swap_remove path
        assert_eq!(m.remove(&Collide(5)), Some(5));
        assert!(!m.contains_key(&Collide(5)));
        assert_eq!(m.len(), 9);
    }

    #[test]
    fn clear_empties_map() {
        let mut m = SimpleHashMap::new();
        for i in 0..8 {
            m.insert(i, i);
        }
        assert!(m.len() > 0);
        m.clear();
        assert!(m.is_empty());
        // clearing again should be idempotent
        m.clear();
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn resizes_and_preserves_entries() {
        let mut m = SimpleHashMap::with_capacity(2);
        // Insert more than load factor threshold to trigger resize
        for i in 0..50 {
            m.insert(i, i * 10);
        }
        assert_eq!(m.len(), 50);
        for i in 0..50 {
            assert_eq!(m.get(&i), Some(&(i * 10)));
        }

        // Clear resets
        m.clear();
        assert!(m.is_empty());
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn zero_capacity_is_valid() {
        let mut m: SimpleHashMap<i32, i32> = SimpleHashMap::with_capacity(0);
        assert!(m.is_empty());
        m.insert(1, 2);
        assert_eq!(m.get(&1), Some(&2));
    }

    #[test]
    fn with_capacity_rounds_to_power_of_two() {
        let m: SimpleHashMap<i32, i32> = SimpleHashMap::with_capacity(3);
        // 3 -> rounds up to 4 buckets
        assert_eq!(m.bucket_count(), 4);
        let m2: SimpleHashMap<i32, i32> = SimpleHashMap::with_capacity(16);
        assert_eq!(m2.bucket_count(), 16);
    }

    #[test]
    fn string_keys_and_values() {
        let mut m: SimpleHashMap<String, String> = SimpleHashMap::new();
        assert_eq!(m.insert("key".into(), "val".into()), None);
        assert_eq!(m.get(&"key".to_string()), Some(&"val".to_string()));
        if let Some(v) = m.get_mut(&"key".to_string()) {
            v.push_str("2");
        }
        assert_eq!(m.get(&"key".to_string()), Some(&"val2".to_string()));
    }

    #[test]
    fn remove_nonexistent_does_nothing() {
        let mut m = SimpleHashMap::new();
        m.insert(1, 10);
        let len_before = m.len();
        assert_eq!(m.remove(&2), None);
        assert_eq!(m.len(), len_before);
    }

    #[test]
    fn remove_all_entries() {
        let mut m = SimpleHashMap::new();
        for i in 0..20 {
            m.insert(i, i);
        }
        for i in 0..20 {
            assert_eq!(m.remove(&i), Some(i));
        }
        assert!(m.is_empty());
    }

    #[test]
    fn contains_and_get_on_empty() {
        let m: SimpleHashMap<i32, i32> = SimpleHashMap::new();
        assert!(!m.contains_key(&1));
        assert_eq!(m.get(&1), None);
    }

    #[test]
    fn get_mut_missing_and_clear_empty() {
        let mut m: SimpleHashMap<&str, i32> = SimpleHashMap::new();
        assert!(m.get_mut(&"x").is_none());
        // clear on empty should be a no-op
        m.clear();
        assert!(m.is_empty());
    }

    #[test]
    fn rehash_from_unit_capacity() {
        // Start with capacity=1, trigger rehash on second insert
        let mut m = SimpleHashMap::with_capacity(0);
        m.insert(1, 10); // cap=1, items=1
        m.insert(2, 20); // triggers resize to cap=2
        assert_eq!(m.get(&1), Some(&10));
        assert_eq!(m.get(&2), Some(&20));
    }

    #[test]
    fn missing_key_in_populated_bucket() {
        // Ensure we search a non-empty bucket and still miss.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct Collide(u64);
        impl Hash for Collide {
            fn hash<H: Hasher>(&self, state: &mut H) {
                0u64.hash(state);
            }
        }
        let mut m = SimpleHashMap::with_capacity(4);
        for i in 0..5 {
            m.insert(Collide(i), i as i32);
        }
        // Key not present but hashes to same bucket
        assert_eq!(m.get(&Collide(999)), None);
        assert!(m.get_mut(&Collide(999)).is_none());
        assert_eq!(m.remove(&Collide(999)), None);
    }
}
