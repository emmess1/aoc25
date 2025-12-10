//! AoC Day 07 — Laboratories
use std::collections::{HashSet, VecDeque};

use super::util;

pub fn part1(input: &str) -> String {
    let Some((grid, start)) = parse_grid(input) else {
        return "0".into();
    };
    simulate_splits(&grid, start).to_string()
}

pub fn part2(input: &str) -> String {
    let Some((grid, start)) = parse_grid(input) else {
        return "0".into();
    };
    count_timelines(&grid, start).to_string()
}

fn simulate_splits(grid: &[Vec<char>], start: (usize, usize)) -> u64 {
    let height = grid.len();
    let width = grid[0].len();
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut splitters_hit: HashSet<(usize, usize)> = HashSet::new();
    queue.push_back(start);
    seen.insert(start);
    let mut splits = 0u64;

    while let Some((mut row, col)) = queue.pop_front() {
        while row + 1 < height {
            row += 1;
            match grid[row][col] {
                '^' => {
                    if splitters_hit.insert((row, col)) {
                        splits += 1;
                        if col > 0 {
                            let state = (row, col - 1);
                            if seen.insert(state) {
                                queue.push_back(state);
                            }
                        }
                        if col + 1 < width {
                            let state = (row, col + 1);
                            if seen.insert(state) {
                                queue.push_back(state);
                            }
                        }
                    }
                    break;
                }
                '.' | 'S' => continue,
                _ => continue,
            }
        }
    }

    splits
}

fn count_timelines(grid: &[Vec<char>], start: (usize, usize)) -> u128 {
    let height = grid.len();
    let width = grid[0].len();
    let mut pending = vec![vec![0u128; width]; height];
    let mut in_queue = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();
    pending[start.0][start.1] = 1;
    in_queue[start.0][start.1] = true;
    queue.push_back(start);
    let mut timelines = 0u128;

    while let Some((mut row, col)) = queue.pop_front() {
        let count = pending[row][col];
        pending[row][col] = 0;
        in_queue[row][col] = false;
        if count == 0 {
            continue;
        }

        loop {
            if row + 1 >= height {
                timelines += count;
                break;
            }
            row += 1;
            match grid[row][col] {
                '^' => {
                    timelines += branch_out(
                        row,
                        col,
                        count,
                        -1,
                        width,
                        &mut pending,
                        &mut in_queue,
                        &mut queue,
                    );
                    timelines += branch_out(
                        row,
                        col,
                        count,
                        1,
                        width,
                        &mut pending,
                        &mut in_queue,
                        &mut queue,
                    );
                    break;
                }
                '.' | 'S' => continue,
                _ => continue,
            }
        }
    }

    timelines
}

fn branch_out(
    row: usize,
    col: usize,
    count: u128,
    delta: isize,
    width: usize,
    pending: &mut [Vec<u128>],
    in_queue: &mut [Vec<bool>],
    queue: &mut VecDeque<(usize, usize)>,
) -> u128 {
    let next_col = col as isize + delta;
    if next_col < 0 || next_col >= width as isize {
        return count;
    }
    let next_col = next_col as usize;
    pending[row][next_col] += count;
    if !in_queue[row][next_col] {
        in_queue[row][next_col] = true;
        queue.push_back((row, next_col));
    }
    0
}

fn parse_grid(input: &str) -> Option<(Vec<Vec<char>>, (usize, usize))> {
    let rows: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    if rows.is_empty() {
        return None;
    }
    let width = rows[0].len();
    if width == 0 {
        return None;
    }

    let grid = rows;
    for row in &grid {
        assert_eq!(row.len(), width, "grid rows must be equal length");
    }

    let mut start = None;
    for (r, row) in grid.iter().enumerate() {
        if let Some(c) = row.iter().position(|&ch| ch == 'S') {
            start = Some((r, c));
            break;
        }
    }

    start.map(|s| (grid, s))
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = util::read_input("day07")?;
    println!(
        "Day 07\nPart 1: {}\nPart 2: {}",
        part1(&input),
        part2(&input)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_PART1: Option<&str> = Some("21");
    const EXPECTED_PART2: Option<&str> = Some("40");
    const EXAMPLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/inputs/day07_example.txt"
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
