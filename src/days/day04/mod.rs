//! AoC Day 04 scaffold

use std::collections::VecDeque;

use super::util;

fn parse_grid(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

fn count_neighbors(grid: &[Vec<bool>], y: usize, x: usize) -> u8 {
    let mut neighbors = 0;
    for dy in -1isize..=1 {
        for dx in -1isize..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if ny < 0 || nx < 0 {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;
            if ny >= grid.len() || nx >= grid[ny].len() {
                continue;
            }
            if grid[ny][nx] {
                neighbors += 1;
            }
        }
    }
    neighbors
}

pub fn part1(input: &str) -> String {
    let grid = parse_grid(input);
    if grid.is_empty() {
        return "0".into();
    }

    let mut accessible = 0usize;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] && count_neighbors(&grid, y, x) < 4 {
                accessible += 1;
            }
        }
    }

    accessible.to_string()
}

pub fn part2(input: &str) -> String {
    let mut grid = parse_grid(input);
    if grid.is_empty() {
        return "0".into();
    }

    let mut neighbor_counts: Vec<Vec<u8>> = (0..grid.len())
        .map(|y| {
            (0..grid[y].len())
                .map(|x| {
                    if grid[y][x] {
                        count_neighbors(&grid, y, x)
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect();

    let mut queue = VecDeque::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] && neighbor_counts[y][x] < 4 {
                queue.push_back((y, x));
            }
        }
    }

    let mut removed = 0usize;
    while let Some((y, x)) = queue.pop_front() {
        if !grid[y][x] || neighbor_counts[y][x] >= 4 {
            continue;
        }

        grid[y][x] = false;
        removed += 1;

        for dy in -1isize..=1 {
            for dx in -1isize..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let ny = y as isize + dy;
                let nx = x as isize + dx;
                if ny < 0 || nx < 0 {
                    continue;
                }
                let ny = ny as usize;
                let nx = nx as usize;
                if ny >= grid.len() || nx >= grid[ny].len() {
                    continue;
                }
                if grid[ny][nx] {
                    // Safe because neighbor counts only track cells with rolls.
                    neighbor_counts[ny][nx] -= 1;
                    if neighbor_counts[ny][nx] < 4 {
                        queue.push_back((ny, nx));
                    }
                }
            }
        }
    }

    removed.to_string()
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = util::read_input("day04")?;
    println!(
        "Day 04\nPart 1: {}\nPart 2: {}",
        part1(&input),
        part2(&input)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXPECTED_PART1: Option<&str> = Some("13");
    const EXPECTED_PART2: Option<&str> = Some("43");
    const EXAMPLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/inputs/day04_example.txt"
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
