//! Alternative Part 1 solver that leans entirely on regular expressions.
//!
//! Rust's `regex` crate deliberately omits backreferences, so we pre-build a
//! pattern per even digit length (e.g., `^(\d{3})(\d{3})$`) and compare the
//! two captured halves in user code. This keeps the detection "regex-first"
//! while remaining compatible with the crate's feature set. The solver is
//! intentionally naive (iterating every ID) and best suited for explanation
//! or verification with smaller inputs.

use regex::Regex;

use super::super::{num_digits, parse_ranges};

/// Sum all Part 1-invalid IDs (duplicate halves) using regex matching.
pub fn sum_with_regex(input: &str) -> u128 {
    let ranges = parse_ranges(input);
    let max_end = ranges.iter().map(|&(_, hi)| hi).max().unwrap_or(0);
    if max_end == 0 {
        return 0;
    }
    let max_digits = num_digits(max_end);
    let mut regexes: Vec<Option<Regex>> = vec![None; max_digits + 1];
    for len in (2..=max_digits).step_by(2) {
        let half = len / 2;
        let pattern = format!(r"^(\d{{{}}})(\d{{{}}})$", half, half);
        regexes[len] = Some(Regex::new(&pattern).expect("valid regex"));
    }

    let mut total = 0u128;
    for (start, end) in ranges {
        if start > end {
            continue;
        }
        for value in start..=end {
            let s = value.to_string();
            let len = s.len();
            if len % 2 != 0 {
                continue;
            }
            if let Some(Some(re)) = regexes.get(len) {
                if let Some(caps) = re.captures(&s) {
                    if caps.get(1).unwrap().as_str() == caps.get(2).unwrap().as_str() {
                        total += value;
                    }
                }
            }
        }
    }
    total
}

/// Sum all Part 2-invalid IDs using regexes (block repeated ≥2 times).
///
/// We brute force every ID, build a regex per possible block length, and test
/// whether the entire string is captured by one block repeated two or more
/// times. Use for explanations or small inputs only.
pub fn sum_with_regex_part2(input: &str) -> u128 {
    let ranges = parse_ranges(input);
    let max_end = ranges.iter().map(|&(_, hi)| hi).max().unwrap_or(0);
    if max_end == 0 {
        return 0;
    }
    let max_digits = num_digits(max_end);
    // Map length -> regex `^(\d{len})+$`
    let mut regexes: Vec<Option<Regex>> = vec![None; max_digits + 1];
    for len in 1..=max_digits {
        let pattern = format!(r"^(\d{{{}}})+$", len);
        regexes[len] = Regex::new(&pattern).ok();
    }
    let mut total = 0u128;
    for (start, end) in ranges {
        if start > end {
            continue;
        }
        for value in start..=end {
            let s = value.to_string();
            let len = s.len();
            for block_len in 1..=(len / 2) {
                if len % block_len != 0 {
                    continue;
                }
                let repeats = len / block_len;
                if repeats < 2 {
                    continue;
                }
                if let Some(Some(re)) = regexes.get(block_len) {
                    if let Some(caps) = re.captures(&s) {
                        let block = caps.get(1).unwrap().as_str();
                        if block.repeat(repeats) == s {
                            total += value;
                            break;
                        }
                    }
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::day02::part2;
    use std::fs;

    const EXAMPLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/inputs/day02_example.txt"
    ));

    #[test]
    fn regex_solver_matches_example() {
        assert_eq!(sum_with_regex(EXAMPLE), 1_227_775_554u128);
    }

    #[test]
    fn regex_part2_matches_example() {
        assert_eq!(sum_with_regex_part2(EXAMPLE), 4_174_379_265u128);
    }

    /// Handy hook for running the regex solver against your real puzzle input.
    /// Invoke with `cargo test regex_solver_real_input -- --ignored`.
    #[test]
    #[ignore]
    fn regex_solver_real_input() {
        let input = fs::read_to_string("inputs/day02.txt").expect("day02 input");
        let regex_sum = sum_with_regex(&input);
        let fast_sum: u128 = crate::days::day02::part1(&input)
            .parse()
            .expect("part1 numeric result");
        assert_eq!(regex_sum, fast_sum);
    }

    /// Same as above but checks Part 2 results via regex approach.
    #[test]
    #[ignore]
    fn regex_part2_real_input() {
        let input = fs::read_to_string("inputs/day02.txt").expect("day02 input");
        let regex_sum = sum_with_regex_part2(&input);
        let fast_sum: u128 = part2(&input).parse().expect("part2 numeric result");
        assert_eq!(regex_sum, fast_sum);
    }
}
