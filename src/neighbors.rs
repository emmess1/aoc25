//! Neighbor deltas for 4- and 8-directional movement.

use crate::coords::Point;

pub const DELTAS4: [Point; 4] = [
    Point { x: 1, y: 0 }, Point { x: -1, y: 0 }, Point { x: 0, y: 1 }, Point { x: 0, y: -1 },
];
pub const DELTAS8: [Point; 8] = [
    Point { x: 1, y: 0 }, Point { x: -1, y: 0 }, Point { x: 0, y: 1 }, Point { x: 0, y: -1 },
    Point { x: 1, y: 1 }, Point { x: 1, y: -1 }, Point { x: -1, y: 1 }, Point { x: -1, y: -1 },
];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deltas() {
        assert!(DELTAS4.contains(&Point { x: 1, y: 0 }));
        assert!(DELTAS8.contains(&Point { x: -1, y: -1 }));
    }
}

