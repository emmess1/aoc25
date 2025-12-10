//! AoC Day 09 — Movie Theater
use std::collections::{BTreeSet, HashMap, VecDeque};

use super::util;

pub fn part1(input: &str) -> String {
    let points = parse_points(input);
    max_rectangle_area(&points).to_string()
}

pub fn part2(input: &str) -> String {
    let points = parse_points(input);
    if points.len() < 2 {
        return "0".into();
    }
    let region = Region::new(&points);
    let mut best = 0i128;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            if region.rectangle_inside(points[i][0], points[i][1], points[j][0], points[j][1]) {
                best = best.max(rect_area(&points[i], &points[j]));
            }
        }
    }
    best.to_string()
}

fn max_rectangle_area(points: &[[i64; 2]]) -> i128 {
    if points.len() < 2 {
        return 0;
    }
    let mut best: i128 = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            best = best.max(rect_area(&points[i], &points[j]));
        }
    }
    best
}

fn rect_area(a: &[i64; 2], b: &[i64; 2]) -> i128 {
    let dx = (a[0] - b[0]).abs() as i128 + 1;
    let dy = (a[1] - b[1]).abs() as i128 + 1;
    dx * dy
}

fn parse_points(input: &str) -> Vec<[i64; 2]> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .unwrap_or_else(|| panic!("invalid coordinate: {line}"));
            let x = x.trim().parse().expect("invalid x coordinate");
            let y = y.trim().parse().expect("invalid y coordinate");
            [x, y]
        })
        .collect()
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = util::read_input("day09")?;
    println!(
        "Day 09\nPart 1: {}\nPart 2: {}",
        part1(&input),
        part2(&input)
    );
    Ok(())
}

struct Region {
    x_index: HashMap<i64, usize>,
    y_index: HashMap<i64, usize>,
    outside_prefix: Vec<Vec<i64>>,
}

impl Region {
    fn new(points: &[[i64; 2]]) -> Self {
        let (xs, x_index) = build_axis(points.iter().map(|p| p[0]));
        let (ys, y_index) = build_axis(points.iter().map(|p| p[1]));
        let w = xs.len() - 1;
        let h = ys.len() - 1;
        let mut diff = vec![vec![0i32; w + 1]; h + 1];
        let n = points.len();
        for i in 0..n {
            let a = points[i];
            let b = points[(i + 1) % n];
            if a[0] == b[0] {
                let sx = a[0] * 2;
                let x_start = sx - 1;
                let x_end = sx + 1;
                let (y_lo, y_hi) = if a[1] <= b[1] {
                    (a[1], b[1])
                } else {
                    (b[1], a[1])
                };
                let y_start = y_lo * 2 - 1;
                let y_end = y_hi * 2 + 1;
                add_rect(
                    &mut diff, &x_index, &y_index, x_start, x_end, y_start, y_end,
                );
            } else {
                let sy = a[1] * 2;
                let y_start = sy - 1;
                let y_end = sy + 1;
                let (x_lo, x_hi) = if a[0] <= b[0] {
                    (a[0], b[0])
                } else {
                    (b[0], a[0])
                };
                let x_start = x_lo * 2 - 1;
                let x_end = x_hi * 2 + 1;
                add_rect(
                    &mut diff, &x_index, &y_index, x_start, x_end, y_start, y_end,
                );
            }
        }
        let blocked = build_blocked(diff);
        let outside = flood_outside(&blocked);
        let outside_prefix = build_outside_prefix(&outside);
        Self {
            x_index,
            y_index,
            outside_prefix,
        }
    }

    fn rectangle_inside(&self, x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
        let sx_lo = x1.min(x2) * 2 - 1;
        let sx_hi = x1.max(x2) * 2 + 1;
        let sy_lo = y1.min(y2) * 2 - 1;
        let sy_hi = y1.max(y2) * 2 + 1;
        let ix_lo = *self
            .x_index
            .get(&sx_lo)
            .expect("rectangle x start should exist");
        let ix_hi = *self
            .x_index
            .get(&sx_hi)
            .expect("rectangle x end should exist");
        let iy_lo = *self
            .y_index
            .get(&sy_lo)
            .expect("rectangle y start should exist");
        let iy_hi = *self
            .y_index
            .get(&sy_hi)
            .expect("rectangle y end should exist");
        self.query_outside(ix_lo, ix_hi, iy_lo, iy_hi) == 0
    }

    fn query_outside(&self, x0: usize, x1: usize, y0: usize, y1: usize) -> i64 {
        self.outside_prefix[y1][x1] - self.outside_prefix[y0][x1] - self.outside_prefix[y1][x0]
            + self.outside_prefix[y0][x0]
    }
}

fn build_axis<'a>(values: impl Iterator<Item = i64>) -> (Vec<i64>, HashMap<i64, usize>) {
    let mut xs = BTreeSet::new();
    let mut min_val = i64::MAX;
    let mut max_val = i64::MIN;
    for v in values {
        let scaled = v * 2;
        xs.insert(scaled - 1);
        xs.insert(scaled + 1);
        min_val = min_val.min(scaled);
        max_val = max_val.max(scaled);
    }
    xs.insert(min_val - 3);
    xs.insert(max_val + 3);
    let coords: Vec<i64> = xs.into_iter().collect();
    let index = coords
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i))
        .collect::<HashMap<_, _>>();
    (coords, index)
}

fn add_rect(
    diff: &mut [Vec<i32>],
    x_index: &HashMap<i64, usize>,
    y_index: &HashMap<i64, usize>,
    x_start: i64,
    x_end: i64,
    y_start: i64,
    y_end: i64,
) {
    let ix0 = *x_index
        .get(&x_start)
        .unwrap_or_else(|| panic!("missing x_start {x_start}"));
    let ix1 = *x_index
        .get(&x_end)
        .unwrap_or_else(|| panic!("missing x_end {x_end}"));
    let iy0 = *y_index
        .get(&y_start)
        .unwrap_or_else(|| panic!("missing y_start {y_start}"));
    let iy1 = *y_index
        .get(&y_end)
        .unwrap_or_else(|| panic!("missing y_end {y_end}"));
    diff[iy0][ix0] += 1;
    diff[iy1][ix0] -= 1;
    diff[iy0][ix1] -= 1;
    diff[iy1][ix1] += 1;
}

fn build_blocked(mut diff: Vec<Vec<i32>>) -> Vec<Vec<bool>> {
    let h = diff.len() - 1;
    let w = diff[0].len() - 1;
    let mut blocked = vec![vec![false; w]; h];
    for y in 0..h {
        for x in 0..w {
            if y > 0 {
                diff[y][x] += diff[y - 1][x];
            }
            if x > 0 {
                diff[y][x] += diff[y][x - 1];
            }
            if y > 0 && x > 0 {
                diff[y][x] -= diff[y - 1][x - 1];
            }
            blocked[y][x] = diff[y][x] > 0;
        }
    }
    blocked
}

fn flood_outside(blocked: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let h = blocked.len();
    let w = blocked[0].len();
    let mut outside = vec![vec![false; w]; h];
    let mut queue = VecDeque::new();
    let mut start = None;
    'outer: for y in 0..h {
        for x in 0..w {
            if (y == 0 || x == 0 || y + 1 == h || x + 1 == w) && !blocked[y][x] {
                start = Some((y, x));
                break 'outer;
            }
        }
    }
    let (sy, sx) = start.expect("should find outside starting cell");
    queue.push_back((sy, sx));
    outside[sy][sx] = true;
    let dirs = [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)];
    while let Some((y, x)) = queue.pop_front() {
        for (dy, dx) in dirs {
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if ny < 0 || nx < 0 || ny as usize >= h || nx as usize >= w {
                continue;
            }
            let (nyu, nxu) = (ny as usize, nx as usize);
            if blocked[nyu][nxu] || outside[nyu][nxu] {
                continue;
            }
            outside[nyu][nxu] = true;
            queue.push_back((nyu, nxu));
        }
    }
    outside
}

fn build_outside_prefix(outside: &[Vec<bool>]) -> Vec<Vec<i64>> {
    let h = outside.len();
    let w = outside[0].len();
    let mut pref = vec![vec![0i64; w + 1]; h + 1];
    for y in 0..h {
        let mut row_sum = 0i64;
        for x in 0..w {
            if outside[y][x] {
                row_sum += 1;
            }
            pref[y + 1][x + 1] = pref[y][x + 1] + row_sum;
        }
    }
    pref
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_PART1: Option<&str> = Some("50");
    const EXPECTED_PART2: Option<&str> = Some("24");
    const EXAMPLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/inputs/day09_example.txt"
    ));

    #[test]
    fn example_part1() {
        let got = part1(EXAMPLE);
        if let Some(exp) = EXPECTED_PART1 {
            assert_eq!(got, exp);
        }
    }

    #[test]
    fn example_part2() {
        let got = part2(EXAMPLE);
        if let Some(exp) = EXPECTED_PART2 {
            assert_eq!(got, exp);
        }
    }
}
