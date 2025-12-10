//! Coordinate systems and helpers for grid-based AoC problems.

use std::ops::{Add, Sub};

/// 2D integer point.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    /// 4-neighbors around this point.
    pub fn neighbors4(self) -> [Point; 4] {
        [
            self + Point::new(1, 0),
            self + Point::new(-1, 0),
            self + Point::new(0, 1),
            self + Point::new(0, -1),
        ]
    }
    /// 8-neighbors around this point.
    pub fn neighbors8(self) -> [Point; 8] {
        [
            self + Point::new(1, 0),
            self + Point::new(-1, 0),
            self + Point::new(0, 1),
            self + Point::new(0, -1),
            self + Point::new(1, 1),
            self + Point::new(1, -1),
            self + Point::new(-1, 1),
            self + Point::new(-1, -1),
        ]
    }
}

impl From<(i64, i64)> for Point {
    fn from(t: (i64, i64)) -> Self {
        Point::new(t.0, t.1)
    }
}
impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

/// Simple integer complex-like type for grid math: (re, im)
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ComplexI {
    pub re: i64,
    pub im: i64,
}

impl ComplexI {
    pub fn new(re: i64, im: i64) -> Self {
        Self { re, im }
    }
    /// Multiply by i (rotate 90 degrees CCW).
    pub fn mul_i(self) -> Self {
        Self {
            re: -self.im,
            im: self.re,
        }
    }
    /// Multiply by -i (rotate 90 degrees CW).
    pub fn mul_neg_i(self) -> Self {
        Self {
            re: self.im,
            im: -self.re,
        }
    }
}

impl Add for ComplexI {
    type Output = ComplexI;
    fn add(self, rhs: ComplexI) -> ComplexI {
        ComplexI::new(self.re + rhs.re, self.im + rhs.im)
    }
}
impl Sub for ComplexI {
    type Output = ComplexI;
    fn sub(self, rhs: ComplexI) -> ComplexI {
        ComplexI::new(self.re - rhs.re, self.im - rhs.im)
    }
}

/// 3D integer point.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Point3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3 {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
    /// 6-neighbors (axis-aligned).
    pub fn neighbors6(self) -> [Point3; 6] {
        [
            Point3::new(self.x + 1, self.y, self.z),
            Point3::new(self.x - 1, self.y, self.z),
            Point3::new(self.x, self.y + 1, self.z),
            Point3::new(self.x, self.y - 1, self.z),
            Point3::new(self.x, self.y, self.z + 1),
            Point3::new(self.x, self.y, self.z - 1),
        ]
    }
    /// 26-neighbors (including diagonals) in 3D.
    pub fn neighbors26(self) -> Vec<Point3> {
        let mut v = Vec::with_capacity(26);
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
                    v.push(Point3::new(self.x + dx, self.y + dy, self.z + dz));
                }
            }
        }
        v
    }
}

impl Add for Point3 {
    type Output = Point3;
    fn add(self, rhs: Point3) -> Point3 {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Sub for Point3 {
    type Output = Point3;
    fn sub(self, rhs: Point3) -> Point3 {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_neighbors() {
        let p = Point::new(0, 0);
        let n4 = p.neighbors4();
        assert!(n4.contains(&Point::new(1, 0)));
        assert!(n4.contains(&Point::new(0, -1)));
        let n8 = p.neighbors8();
        assert!(n8.contains(&Point::new(1, 1)));
        assert!(n8.contains(&Point::new(-1, -1)));
    }
    #[test]
    fn complex_rotations() {
        let v = ComplexI::new(1, 0);
        assert_eq!(v.mul_i(), ComplexI::new(0, 1));
        assert_eq!(v.mul_neg_i(), ComplexI::new(0, -1));
    }
    #[test]
    fn point_ops_and_from() {
        let p: Point = (1, 2).into();
        assert_eq!(p, Point::new(1, 2));
        assert_eq!(Point::new(1, 2) + Point::new(3, 4), Point::new(4, 6));
        assert_eq!(Point::new(5, 5) - Point::new(2, 3), Point::new(3, 2));
    }
    #[test]
    fn complex_add_sub() {
        let a = ComplexI::new(2, 3);
        let b = ComplexI::new(1, 1);
        assert_eq!(a + b, ComplexI::new(3, 4));
        assert_eq!(a - b, ComplexI::new(1, 2));
    }
    #[test]
    fn point3_neighbors() {
        let p = Point3::new(0, 0, 0);
        let n6 = p.neighbors6();
        assert!(n6.contains(&Point3::new(1, 0, 0)) && n6.contains(&Point3::new(0, 0, -1)));
        let n26 = p.neighbors26();
        assert_eq!(n26.len(), 26);
        assert!(n26.contains(&Point3::new(1, 1, 1)));
    }
    #[test]
    fn point_add_sub() {
        let a = Point::new(1, 1);
        let b = Point::new(0, 2);
        assert_eq!(a + b, Point::new(1, 3));
        assert_eq!(a - b, Point::new(1, -1));
    }

    #[test]
    fn point3_add_sub() {
        let a = Point3::new(1, 2, 3);
        let b = Point3::new(-1, 0, 2);
        assert_eq!(a + b, Point3::new(0, 2, 5));
        assert_eq!(a - b, Point3::new(2, 2, 1));
    }
}
