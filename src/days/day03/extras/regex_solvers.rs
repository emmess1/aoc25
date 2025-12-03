//! Regex-based solvers for Day 03.
#![allow(dead_code)]
//!
//! These helpers demonstrate how you *could* brute-force the problem using
//! only regular-expression queries to detect subsequences. They're meant for
//! experimentation or validation rather than performance.

use regex::Regex;

/// Sum Part 1 answers by checking every digit pair with a regex.
pub fn sum_with_regex_part1(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(best_pair_regex)
        .sum()
}

fn sanitize(line: &str) -> String {
    line.chars().filter(|c| c.is_ascii_digit()).collect()
}

fn best_pair_regex(line: &str) -> u64 {
    let digits = sanitize(line);
    if digits.len() < 2 {
        return 0;
    }
    for d1 in (0..=9).rev() {
        let c1 = char::from_digit(d1, 10).unwrap();
        for d2 in (0..=9).rev() {
            let c2 = char::from_digit(d2, 10).unwrap();
            let pattern = format!(r"(?s).*{}.*{}.*", c1, c2);
            let re = Regex::new(&pattern).expect("valid pattern");
            if re.is_match(&digits) {
                return (10 * d1 + d2) as u64;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const EXAMPLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/inputs/day03_example.txt"
    ));

    #[test]
    fn regex_part1_matches_example() {
        assert_eq!(sum_with_regex_part1(EXAMPLE), 357);
    }

    #[test]
    #[ignore]
    fn regex_part1_real_input() {
        let input = fs::read_to_string("inputs/day03.txt").expect("day03 input");
        let regex_sum = sum_with_regex_part1(&input);
        let fast: u64 = crate::days::day03::part1(&input).parse().unwrap();
        assert_eq!(regex_sum, fast);
    }
}
