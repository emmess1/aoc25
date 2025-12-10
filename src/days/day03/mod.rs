//! Day 03: Battery Banks
//!
//! Part 1 asks for the maximum two-digit number you can form from each bank
//! by choosing two digits in order (left-to-right, no reordering). For each
//! line, we check every ordered pair, keep the largest `10*a + b`, and sum
//! across banks.
//!
//! Part 2 generalizes to choosing exactly 12 digits in order. This is a
//! "maximum subsequence" problem where we want the lexicographically largest
//! sequence of fixed length. We use a greedy stack: process each digit and
//! pop while the next digit is larger and we still have digits left to drop.

pub mod extras;

use super::util;

pub fn part1(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(max_pair_value)
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(max_subsequence_value)
        .sum::<u64>()
        .to_string()
}

fn max_pair_value(line: &str) -> u64 {
    let digits: Vec<u8> = line
        .chars()
        .filter_map(|ch| ch.to_digit(10).map(|d| d as u8))
        .collect();
    let mut best = 0u64;
    for i in 0..digits.len() {
        for j in i + 1..digits.len() {
            let val = (digits[i] as u64) * 10 + digits[j] as u64;
            if val > best {
                best = val;
            }
        }
    }
    best
}

fn max_subsequence_value(line: &str) -> u64 {
    const PICK: usize = 12;
    let digits: Vec<u8> = line
        .chars()
        .filter_map(|ch| ch.to_digit(10).map(|d| d as u8))
        .collect();
    if digits.len() <= PICK {
        return digits.iter().fold(0u64, |acc, &d| acc * 10 + d as u64);
    }
    let mut keep = Vec::with_capacity(PICK);
    let mut to_remove = digits.len() - PICK;
    for &d in &digits {
        while to_remove > 0 && !keep.is_empty() && *keep.last().unwrap() < d {
            keep.pop();
            to_remove -= 1;
        }
        keep.push(d);
    }
    keep.truncate(PICK);
    keep.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = util::read_input("day03")?;
    println!(
        "Day 03\nPart 1: {}\nPart 2: {}",
        part1(&input),
        part2(&input)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXPECTED_PART1: Option<&str> = Some("357");
    const EXPECTED_PART2: Option<&str> = Some("3121910778619");
    const EXAMPLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/inputs/day03_example.txt"
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
