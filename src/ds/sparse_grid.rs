//! Sparse grid representation keyed by coordinates.

use std::collections::HashMap;
use crate::ds::coords::Point;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SparseGrid<T> {
    cells: HashMap<Point, T>,
}

impl<T> SparseGrid<T> {
    pub fn new() -> Self { Self { cells: HashMap::new() } }
    pub fn is_empty(&self) -> bool { self.cells.is_empty() }
    pub fn len(&self) -> usize { self.cells.len() }
    pub fn insert(&mut self, p: Point, v: T) -> Option<T> { self.cells.insert(p, v) }
    pub fn get(&self, p: &Point) -> Option<&T> { self.cells.get(p) }
    pub fn get_mut(&mut self, p: &Point) -> Option<&mut T> { self.cells.get_mut(p) }
    pub fn remove(&mut self, p: &Point) -> Option<T> { self.cells.remove(p) }
    pub fn iter(&self) -> impl Iterator<Item=(&Point,&T)> { self.cells.iter() }
    /// Bounding rectangle (min_x..=max_x, min_y..=max_y), or None if empty.
    pub fn bounds(&self) -> Option<(i64,i64,i64,i64)> {
        let mut it = self.cells.keys();
        let first = it.next()?;
        let (mut minx, mut maxx, mut miny, mut maxy) = (first.x, first.x, first.y, first.y);
        for p in it { minx=minx.min(p.x); maxx=maxx.max(p.x); miny=miny.min(p.y); maxy=maxy.max(p.y);} 
        Some((minx,maxx,miny,maxy))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut g = SparseGrid::new();
        assert!(g.is_empty());
        g.insert(Point::new(2,3), 9);
        g.insert(Point::new(-1,0), 7);
        assert_eq!(g.len(), 2);
        // iterate cells
        let mut seen = 0;
        for (_p, _v) in g.iter() { seen+=1; }
        assert_eq!(seen, 2);
        assert_eq!(g.get(&Point::new(2,3)), Some(&9));
        if let Some(v) = g.get_mut(&Point::new(2,3)) { *v += 1; }
        assert_eq!(g.get(&Point::new(2,3)), Some(&10));
        let b = g.bounds().unwrap();
        assert_eq!(b, (-1,2,0,3));
        assert_eq!(g.remove(&Point::new(2,3)), Some(10));
        assert!(g.get(&Point::new(2,3)).is_none());
        // Removing missing key returns None
        assert_eq!(g.remove(&Point::new(99,99)), None);
        // Bounds None when empty
        let mut g2: SparseGrid<i32> = SparseGrid::new();
        assert!(g2.bounds().is_none());
    }
}
