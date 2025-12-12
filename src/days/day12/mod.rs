//! AoC Day 12 — Christmas Tree Farm
use std::collections::HashSet;

use super::util;

pub fn part1(input: &str) -> String {
    let puzzle = parse_input(input);
    let solver = Solver::new(&puzzle.shapes);
    let mut count = 0u64;
    for region in &puzzle.regions {
        if solver.can_fit(region) {
            count += 1;
        }
    }
    count.to_string()
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = util::read_input("day12")?;
    println!("Day 12\nPart 1: {}", part1(&input));
    Ok(())
}

#[derive(Debug, Clone)]
struct Shape {
    id: usize,
    cells: Vec<(i32, i32)>,
    variants: Vec<Variant>,
}

#[derive(Debug, Clone)]
struct Variant {
    width: i32,
    height: i32,
    cells: Vec<(i32, i32)>,
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    requirements: Vec<u16>,
}

#[derive(Debug)]
struct PuzzleInput {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

fn parse_input(input: &str) -> PuzzleInput {
    let mut shapes = Vec::new();
    let mut regions = Vec::new();
    let mut reading_shapes = true;
    let mut current_shape_id = None;
    let mut current_rows: Vec<String> = Vec::new();

    for raw_line in input.lines() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if reading_shapes {
            if let Some(id_str) = trimmed.strip_suffix(':') {
                if let Some(id) = current_shape_id.take() {
                    shapes.push(build_shape(id, &current_rows));
                    current_rows.clear();
                }
                let id = id_str
                    .trim()
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("invalid shape index: {trimmed}"));
                current_shape_id = Some(id);
                continue;
            }
            if trimmed.contains('x') {
                if let Some(id) = current_shape_id.take() {
                    shapes.push(build_shape(id, &current_rows));
                    current_rows.clear();
                }
                reading_shapes = false;
                regions.push(parse_region(trimmed));
                continue;
            }
            current_rows.push(trimmed.to_string());
        } else {
            regions.push(parse_region(trimmed));
        }
    }
    if let Some(id) = current_shape_id {
        shapes.push(build_shape(id, &current_rows));
    }

    shapes.sort_by_key(|s| s.id);
    for (expected, shape) in shapes.iter().enumerate() {
        assert_eq!(
            shape.id, expected,
            "shape indices must be contiguous starting at 0"
        );
    }
    let shape_count = shapes.len();
    for region in &mut regions {
        if region.requirements.len() < shape_count {
            region.requirements.resize(shape_count, 0);
        } else if region.requirements.len() > shape_count {
            panic!("region requirement list longer than number of shapes");
        }
    }

    PuzzleInput { shapes, regions }
}

fn build_shape(id: usize, rows: &[String]) -> Shape {
    if rows.is_empty() {
        panic!("shape {id} has no rows");
    }
    let mut cells = Vec::new();
    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == '#' {
                cells.push((x as i32, y as i32));
            }
        }
    }
    if cells.is_empty() {
        panic!("shape {id} must have at least one cell");
    }
    let mut variants = generate_variants(&cells);
    variants.sort_by(|a, b| {
        a.height
            .cmp(&b.height)
            .then_with(|| a.width.cmp(&b.width))
            .then_with(|| a.cells.cmp(&b.cells))
    });
    variants.dedup_by(|a, b| a.cells == b.cells);
    Shape {
        id,
        cells,
        variants,
    }
}

fn normalize(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let min_x = cells.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = cells.iter().map(|(_, y)| *y).min().unwrap();
    let mut normed = cells
        .iter()
        .map(|(x, y)| (x - min_x, y - min_y))
        .collect::<Vec<_>>();
    normed.sort_unstable();
    normed
}

fn rotate(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    cells.iter().map(|(x, y)| (-y, *x)).collect()
}

fn reflect(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    cells.iter().map(|(x, y)| (-x, *y)).collect()
}

fn generate_variants(base: &[(i32, i32)]) -> Vec<Variant> {
    let mut variants = Vec::new();
    let mut seen = HashSet::new();
    let mut current = base.to_vec();
    for rotation in 0..4 {
        if rotation > 0 {
            current = rotate(&current);
        }
        for flip in 0..2 {
            let transformed = if flip == 0 {
                current.clone()
            } else {
                reflect(&current)
            };
            let normalized = normalize(&transformed);
            if seen.insert(normalized.clone()) {
                let width = normalized.iter().map(|(x, _)| *x).max().unwrap() + 1;
                let height = normalized.iter().map(|(_, y)| *y).max().unwrap() + 1;
                variants.push(Variant {
                    width,
                    height,
                    cells: normalized.clone(),
                });
            }
        }
    }
    variants
}

fn parse_region(line: &str) -> Region {
    let (size, rest) = line
        .split_once(':')
        .unwrap_or_else(|| panic!("invalid region line: {line}"));
    let (width, height) = size
        .split_once('x')
        .unwrap_or_else(|| panic!("invalid region size: {size}"));
    let width: usize = width.trim().parse().expect("invalid width");
    let height: usize = height.trim().parse().expect("invalid height");
    let requirements = rest
        .split_whitespace()
        .map(|v| v.parse::<u16>().expect("invalid requirement"))
        .collect();
    Region {
        width,
        height,
        requirements,
    }
}

struct Solver<'a> {
    shapes: &'a [Shape],
}

impl<'a> Solver<'a> {
    fn new(shapes: &'a [Shape]) -> Self {
        Solver { shapes }
    }

    fn can_fit(&self, region: &Region) -> bool {
        let mut pieces = Vec::new();
        for (idx, &count) in region.requirements.iter().enumerate() {
            for _ in 0..count {
                pieces.push(idx);
            }
        }
        if pieces.is_empty() {
            return true;
        }
        let total_cells: usize = pieces.iter().map(|&idx| self.shapes[idx].cells.len()).sum();
        if total_cells > region.width * region.height {
            return false;
        }
        let placements = self.compute_placements(region);
        for &idx in &pieces {
            if placements[idx].is_empty() {
                return false;
            }
        }
        pieces.sort_by_key(|&idx| placements[idx].len());
        let mut board = BitBoard::new(region.width * region.height);
        self.search(&pieces, 0, &placements, &mut board)
    }

    fn compute_placements(&self, region: &Region) -> Vec<Vec<Vec<u64>>> {
        let bit_len = BitBoard::bits_len(region.width * region.height);
        let mut placements = vec![Vec::new(); self.shapes.len()];
        for (idx, shape) in self.shapes.iter().enumerate() {
            if region
                .requirements
                .get(idx)
                .copied()
                .unwrap_or(0) == 0
            {
                continue;
            }
            let mut seen = HashSet::new();
            for variant in &shape.variants {
                if variant.width as usize > region.width
                    || variant.height as usize > region.height
                {
                    continue;
                }
                let max_x = region.width as i32 - variant.width;
                let max_y = region.height as i32 - variant.height;
                for oy in 0..=max_y {
                    for ox in 0..=max_x {
                        let mut bits = vec![0u64; bit_len];
                        for &(dx, dy) in &variant.cells {
                            let x = (ox + dx) as usize;
                            let y = (oy + dy) as usize;
                            let idx_bit = y * region.width + x;
                            bits[idx_bit / 64] |= 1u64 << (idx_bit % 64);
                        }
                        if seen.insert(bits.clone()) {
                            placements[idx].push(bits);
                        }
                    }
                }
            }
        }
        placements
    }

    fn search(
        &self,
        pieces: &[usize],
        piece_idx: usize,
        placements: &[Vec<Vec<u64>>],
        board: &mut BitBoard,
    ) -> bool {
        if piece_idx == pieces.len() {
            return true;
        }
        let shape_idx = pieces[piece_idx];
        for placement in &placements[shape_idx] {
            if board.can_place(placement) {
                board.apply(placement);
                if self.search(pieces, piece_idx + 1, placements, board) {
                    board.remove(placement);
                    return true;
                }
                board.remove(placement);
            }
        }
        false
    }
}

#[derive(Clone)]
struct BitBoard {
    bits: Vec<u64>,
}

impl BitBoard {
    fn new(cells: usize) -> Self {
        Self {
            bits: vec![0u64; Self::bits_len(cells)],
        }
    }

    fn bits_len(cells: usize) -> usize {
        (cells + 63) / 64
    }

    fn can_place(&self, placement: &[u64]) -> bool {
        self.bits
            .iter()
            .zip(placement.iter())
            .all(|(a, b)| (a & b) == 0)
    }

    fn apply(&mut self, placement: &[u64]) {
        for (a, b) in self.bits.iter_mut().zip(placement.iter()) {
            *a |= *b;
        }
    }

    fn remove(&mut self, placement: &[u64]) {
        for (a, b) in self.bits.iter_mut().zip(placement.iter()) {
            *a &= !*b;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_PART1: Option<&str> = Some("2");
    const EXAMPLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/inputs/day12_example.txt"
    ));

    #[test]
    fn example_part1() {
        let got = part1(EXAMPLE);
        if let Some(exp) = EXPECTED_PART1 {
            assert_eq!(got, exp);
        }
    }
}
