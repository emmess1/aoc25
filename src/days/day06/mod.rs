//! AoC Day 06 — Trash Compactor
use super::util;

pub fn part1(input: &str) -> String {
    let Some(grid) = build_grid(input) else {
        return "0".into();
    };
    solve_row_major(&grid).to_string()
}

pub fn part2(input: &str) -> String {
    let Some(grid) = build_grid(input) else {
        return "0".into();
    };
    solve_column_major(&grid).to_string()
}

fn solve_row_major(grid: &[Vec<char>]) -> u128 {
    let width = grid[0].len();
    let last_row = grid.len() - 1;
    let mut total: u128 = 0;
    let mut col = 0;
    while col < width {
        while col < width && is_blank_column(grid, col) {
            col += 1;
        }
        if col == width {
            break;
        }

        let start = col;
        while col < width && !is_blank_column(grid, col) {
            col += 1;
        }
        let end = col;

        let mut operands: Vec<u128> = Vec::new();
        for row in 0..last_row {
            let mut slice = String::new();
            for c in start..end {
                slice.push(grid[row][c]);
            }
            let trimmed = slice.trim();
            if trimmed.is_empty() {
                continue;
            }
            let value = trimmed
                .parse::<u128>()
                .expect("invalid number in worksheet");
            operands.push(value);
        }

        assert!(!operands.is_empty(), "problem missing operands");

        let op_char = (start..end)
            .find_map(|c| {
                let ch = grid[last_row][c];
                if ch.is_ascii_whitespace() {
                    None
                } else {
                    Some(ch)
                }
            })
            .expect("missing operator");

        let problem_total = match op_char {
            '+' => operands.iter().sum::<u128>(),
            '*' => operands.iter().product::<u128>(),
            other => panic!("unexpected operator: {other}"),
        };
        total += problem_total;
    }

    total
}

fn solve_column_major(grid: &[Vec<char>]) -> u128 {
    let width = grid[0].len();
    let last_row = grid.len() - 1;
    let mut total: u128 = 0;
    let mut col = width;

    while col > 0 {
        col -= 1;
        if is_blank_column(grid, col) {
            continue;
        }

        let mut start = col;
        while start > 0 {
            let prev = start - 1;
            if is_blank_column(grid, prev) {
                break;
            }
            start = prev;
        }
        let end = col + 1;

        let op_char = (start..end)
            .find_map(|c| {
                let ch = grid[last_row][c];
                if ch.is_ascii_whitespace() {
                    None
                } else {
                    Some(ch)
                }
            })
            .expect("missing operator");

        let mut operands: Vec<u128> = Vec::new();
        for column in (start..end).rev() {
            let mut digits = String::new();
            for row in 0..last_row {
                digits.push(grid[row][column]);
            }
            let trimmed = digits.trim();
            if trimmed.is_empty() {
                continue;
            }
            let value = trimmed
                .parse::<u128>()
                .expect("invalid number in worksheet column");
            operands.push(value);
        }

        assert!(!operands.is_empty(), "problem missing operands");

        let problem_total = match op_char {
            '+' => operands.iter().sum::<u128>(),
            '*' => operands.iter().product::<u128>(),
            other => panic!("unexpected operator: {other}"),
        };
        total += problem_total;

        col = start;
    }

    total
}

fn is_blank_column(rows: &[Vec<char>], col: usize) -> bool {
    rows.iter()
        .all(|row| row.get(col).map_or(true, |ch| ch.is_ascii_whitespace()))
}

fn build_grid(input: &str) -> Option<Vec<Vec<char>>> {
    if input.trim().is_empty() {
        return None;
    }

    let mut rows: Vec<&str> = input.lines().collect();
    while rows
        .last()
        .map(|line| line.trim_end().is_empty())
        .unwrap_or(false)
    {
        rows.pop();
    }

    if rows.is_empty() {
        return None;
    }

    let width = rows.iter().map(|line| line.len()).max().unwrap_or(0);
    if width == 0 {
        return None;
    }

    Some(
        rows.into_iter()
            .map(|line| {
                let mut chars: Vec<char> = line.chars().collect();
                while chars.len() < width {
                    chars.push(' ');
                }
                chars
            })
            .collect(),
    )
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = util::read_input("day06")?;
    println!(
        "Day 06\nPart 1: {}\nPart 2: {}",
        part1(&input),
        part2(&input)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_PART1: Option<&str> = Some("4277556");
    const EXPECTED_PART2: Option<&str> = Some("3263827");
    const EXAMPLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/inputs/day06_example.txt"
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
