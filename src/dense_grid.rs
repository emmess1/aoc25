//! Dense 2D grid with bounds checks and neighbor helpers.

use crate::coords::Point;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DenseGrid2D<T> { w: usize, h: usize, data: Vec<T> }

impl<T: Clone> DenseGrid2D<T> {
    /// Create a grid of width w and height h, filled with `fill`.
    pub fn new(w: usize, h: usize, fill: T) -> Self { Self { w, h, data: vec![fill; w*h] } }
}

impl<T> DenseGrid2D<T> {
    pub fn width(&self) -> usize { self.w }
    pub fn height(&self) -> usize { self.h }
    pub fn idx(&self, x: usize, y: usize) -> usize { y*self.w + x }
    pub fn in_bounds(&self, x: i64, y: i64) -> bool { x>=0 && y>=0 && (x as usize) < self.w && (y as usize) < self.h }
    pub fn get(&self, x: usize, y: usize) -> &T { &self.data[self.idx(x,y)] }
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T { let i=self.idx(x,y); &mut self.data[i] }
    pub fn neighbors4(&self, x: usize, y: usize) -> Vec<Point> {
        let (x,y)=(x as i64,y as i64);
        let cand = [Point::new(x+1,y),Point::new(x-1,y),Point::new(x,y+1),Point::new(x,y-1)];
        cand.into_iter().filter(|p| self.in_bounds(p.x,p.y)).collect()
    }
    pub fn neighbors8(&self, x: usize, y: usize) -> Vec<Point> {
        let (x,y)=(x as i64,y as i64); let mut v = Vec::with_capacity(8);
        for dx in -1..=1 { for dy in -1..=1 { if dx!=0 || dy!=0 { let nx=x+dx; let ny=y+dy; if self.in_bounds(nx,ny) { v.push(Point::new(nx,ny)); }}}}
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut g = DenseGrid2D::new(3,2,0);
        *g.get_mut(1,1)=9;
        assert_eq!(*g.get(1,1),9);
        let n4: Vec<_> = g.neighbors4(1,0).into_iter().collect();
        assert!(n4.contains(&Point::new(0,0)) && n4.contains(&Point::new(2,0)) && n4.contains(&Point::new(1,1)));
        let n8: Vec<_> = g.neighbors8(0,0).into_iter().collect();
        assert!(n8.contains(&Point::new(1,1)));
    }
}
