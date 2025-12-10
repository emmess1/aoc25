//! Intervals and interval sets (inclusive ranges), with merging and membership.

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Interval {
    pub start: i64,
    pub end: i64,
} // inclusive

impl Interval {
    pub fn new(start: i64, end: i64) -> Self {
        assert!(start <= end);
        Self { start, end }
    }
    pub fn overlaps(&self, other: &Interval) -> bool {
        self.start <= other.end && other.start <= self.end
    }
    pub fn merge(&self, other: &Interval) -> Interval {
        Interval::new(self.start.min(other.start), self.end.max(other.end))
    }
    pub fn contains(&self, x: i64) -> bool {
        self.start <= x && x <= self.end
    }
    pub fn len(&self) -> i64 {
        self.end - self.start + 1
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IntervalSet {
    v: Vec<Interval>,
}

impl IntervalSet {
    pub fn new() -> Self {
        Self { v: Vec::new() }
    }
    pub fn add(&mut self, mut iv: Interval) {
        let mut res: Vec<Interval> = Vec::with_capacity(self.v.len() + 1);
        let mut inserted = false;
        for cur in self.v.drain(..) {
            if cur.end < iv.start - 1 {
                res.push(cur);
            } else if iv.end < cur.start - 1 {
                if !inserted {
                    res.push(iv);
                    inserted = true;
                }
                res.push(cur);
            } else {
                iv = iv.merge(&cur);
            }
        }
        if !inserted {
            res.push(iv);
        }
        self.v = res;
    }
    pub fn contains(&self, x: i64) -> bool {
        self.v.iter().any(|iv| iv.contains(x))
    }
    pub fn intervals(&self) -> &[Interval] {
        &self.v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interval_ops() {
        let a = Interval::new(1, 3);
        let b = Interval::new(3, 5);
        assert!(a.overlaps(&b));
        assert_eq!(a.merge(&b), Interval::new(1, 5));
        assert!(a.contains(2));
        assert_eq!(a.len(), 3);
    }
    #[test]
    fn set_merge_and_contains() {
        let mut s = IntervalSet::new();
        s.add(Interval::new(5, 7));
        s.add(Interval::new(1, 3));
        s.add(Interval::new(4, 4)); // bridges
        assert_eq!(s.intervals(), &[Interval::new(1, 7)]);
        assert!(s.contains(6));
        assert!(!s.contains(0));
    }
}
