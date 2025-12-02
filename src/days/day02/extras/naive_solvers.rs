//! Extremely naive Part 1/Part 2 solvers that operate via brute force string
//! comparisons. These are intentionally slow and suitable only for toy inputs
//! or explanatory purposes.

use super::super::parse_ranges;

/// Sum all Part 1-invalid IDs by manual string splitting.
pub fn sum_with_strings_part1(input: &str) -> u128 {
    let ranges = parse_ranges(input);
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
            let mid = len / 2;
            if &s[..mid] == &s[mid..] {
                total += value;
            }
        }
    }
    total
}

/// Sum all Part 2-invalid IDs by checking every possible block length.
pub fn sum_with_strings_part2(input: &str) -> u128 {
    let ranges = parse_ranges(input);
    let mut total = 0u128;
    for (start, end) in ranges {
        if start > end {
            continue;
        }
        'outer: for value in start..=end {
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
                let block = &s[..block_len];
                if block.repeat(repeats) == s {
                    total += value;
                    continue 'outer;
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/inputs/day02_example.txt"
    ));

    #[test]
    fn strings_solver_matches_example() {
        assert_eq!(sum_with_strings_part1(EXAMPLE), 1_227_775_554u128);
    }

    #[test]
    fn strings_solver_part2_matches_example() {
        assert_eq!(sum_with_strings_part2(EXAMPLE), 4_174_379_265u128);
    }

    #[test]
    #[ignore]
    fn strings_solver_real_input() {
        let input = std::fs::read_to_string("inputs/day02.txt").expect("day02 input");
        let expected: u128 = crate::days::day02::part1(&input)
            .parse()
            .expect("numeric");
        assert_eq!(sum_with_strings_part1(&input), expected);
    }

    #[test]
    #[ignore]
    fn strings_part2_real_input() {
        let input = std::fs::read_to_string("inputs/day02.txt").expect("day02 input");
        let expected: u128 = crate::days::day02::part2(&input)
            .parse()
            .expect("numeric");
        assert_eq!(sum_with_strings_part2(&input), expected);
    }
}
