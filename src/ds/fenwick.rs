//! Fenwick tree (Binary Indexed Tree) for prefix sums with point updates.
//!
//! Use this when you need to query prefix sums and apply point updates in
//! O(log N). Range sums are built from two prefix sums.

#[derive(Clone, Debug)]
pub struct Fenwick {
    n: usize,
    bit: Vec<i64>,
}

impl Fenwick {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            bit: vec![0; n + 1],
        }
    }
    /// Add `delta` at index `i` (0-based).
    pub fn add(&mut self, mut i: usize, delta: i64) {
        i += 1;
        while i <= self.n {
            self.bit[i] += delta;
            i += i & (!i + 1);
        }
    }
    /// Prefix sum [0..=i] (0-based). Returns 0 if i is usize::MAX.
    pub fn sum_prefix(&self, mut i: usize) -> i64 {
        let mut s = 0i64;
        i += 1;
        while i > 0 {
            s += self.bit[i];
            i &= i - 1;
        }
        s
    }
    /// Range sum [l..=r].
    pub fn sum_range(&self, l: usize, r: usize) -> i64 {
        if r < l {
            0
        } else {
            self.sum_prefix(r) - if l == 0 { 0 } else { self.sum_prefix(l - 1) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Fenwick;
    #[test]
    fn fenwick_ops() {
        let mut f = Fenwick::new(5);
        f.add(0, 3);
        f.add(2, 5);
        f.add(4, -2);
        assert_eq!(f.sum_prefix(0), 3);
        assert_eq!(f.sum_prefix(2), 8);
        assert_eq!(f.sum_range(1, 3), 5);
        // empty range should be zero
        assert_eq!(f.sum_range(3, 1), 0);
    }
}
