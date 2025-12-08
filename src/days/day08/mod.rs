//! AoC Day 08 — Playground
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use super::util;

const PAIRS_TO_CONNECT: usize = 1000;

pub fn part1(input: &str) -> String {
    let points = parse_points(input);
    if points.is_empty() {
        return "0".into();
    }
    solve(&points, PAIRS_TO_CONNECT).to_string()
}

pub fn part2(input: &str) -> String {
    let points = parse_points(input);
    if points.is_empty() {
        return "0".into();
    }
    last_connection_product(&points).to_string()
}

fn solve(points: &[[i64; 3]], pairs_to_connect: usize) -> u128 {
    let n = points.len();
    if n == 0 {
        return 0;
    }
    let max_edges = n.saturating_sub(1) * n / 2;
    let mut edges = shortest_edges(points, pairs_to_connect.min(max_edges));
    edges.sort_unstable_by(|a, b| a.dist.cmp(&b.dist));

    let mut dsu = DisjointSet::new(n);
    for edge in edges {
        dsu.union(edge.a, edge.b);
    }

    let mut sizes = dsu.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    while sizes.len() < 3 {
        sizes.push(1);
    }
    sizes.iter().take(3).fold(1u128, |acc, &s| acc * s as u128)
}

#[derive(Clone, Copy)]
struct Edge {
    dist: i128,
    a: usize,
    b: usize,
}

fn shortest_edges(points: &[[i64; 3]], limit: usize) -> Vec<Edge> {
    if limit == 0 {
        return Vec::new();
    }
    let mut heap: BinaryHeap<HeapEdge> = BinaryHeap::new();
    let n = points.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance_sq(points[i], points[j]);
            let candidate = HeapEdge { dist, a: i, b: j };
            if heap.len() < limit {
                heap.push(candidate);
            } else if let Some(top) = heap.peek() {
                if candidate.dist < top.dist {
                    heap.pop();
                    heap.push(candidate);
                }
            }
        }
    }
    heap.into_sorted_vec().into_iter().map(|h| Edge {
        dist: h.dist,
        a: h.a,
        b: h.b,
    }).collect()
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct HeapEdge {
    dist: i128,
    a: usize,
    b: usize,
}

impl Ord for HeapEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.dist.cmp(&other.dist) {
            Ordering::Equal => self.a.cmp(&other.a).then(self.b.cmp(&other.b)),
            ord => ord,
        }
    }
}

impl PartialOrd for HeapEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn distance_sq(a: [i64; 3], b: [i64; 3]) -> i128 {
    let dx = (a[0] - b[0]) as i128;
    let dy = (a[1] - b[1]) as i128;
    let dz = (a[2] - b[2]) as i128;
    dx * dx + dy * dy + dz * dz
}

struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        let mut counts: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *counts.entry(root).or_default() += 1;
        }
        counts.into_values().collect()
    }
}

fn parse_points(input: &str) -> Vec<[i64; 3]> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let parts: Vec<_> = line.split(',').map(str::trim).collect();
            assert!(parts.len() == 3, "invalid coordinate: {line}");
            let x = parts[0].parse().expect("invalid x");
            let y = parts[1].parse().expect("invalid y");
            let z = parts[2].parse().expect("invalid z");
            [x, y, z]
        })
        .collect()
}

fn last_connection_product(points: &[[i64; 3]]) -> i128 {
    let mut edges = all_edges(points);
    edges.sort_unstable_by(|a, b| a.dist.cmp(&b.dist));
    let mut dsu = DisjointSet::new(points.len());
    let mut last_edge: Option<&Edge> = None;
    let mut components = points.len();
    for edge in edges.iter() {
        if dsu.union(edge.a, edge.b) {
            last_edge = Some(edge);
            components -= 1;
            if components == 1 {
                break;
            }
        }
    }
    let edge = last_edge.expect("points should form a circuit");
    let product = (points[edge.a][0] as i128) * (points[edge.b][0] as i128);
    product
}

fn all_edges(points: &[[i64; 3]]) -> Vec<Edge> {
    let n = points.len();
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push(Edge {
                dist: distance_sq(points[i], points[j]),
                a: i,
                b: j,
            });
        }
    }
    edges
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = util::read_input("day08")?;
    println!("Day 08\nPart 1: {}\nPart 2: {}", part1(&input), part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_PART1: Option<&str> = Some("40");
    const EXPECTED_PART2: Option<&str> = Some("25272");
    const EXAMPLE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day08_example.txt"));

    #[test]
    fn example_part1() {
        let points = parse_points(EXAMPLE);
        let got = solve(&points, 10);
        if let Some(exp) = EXPECTED_PART1 {
            assert_eq!(got.to_string(), exp);
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
