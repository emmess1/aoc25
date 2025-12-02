//! Day 02: Duplicate Halves (and beyond)
//!
//! Part 1 asks for the sum of every ID inside the provided ranges whose
//! decimal representation consists of a block of digits repeated exactly
//! twice (e.g., 55, 6464, 123123). Such IDs always have an even number of
//! digits and can be written as `x * (10^k + 1)`, where `x` has `k` digits
//! and `10^k + 1` glues `x` to itself. Instead of iterating over every ID in
//! a range, we iterate over the half-length `k`, determine which `x` values
//! (if any) keep `x * (10^k + 1)` inside the range, and then sum the entire
//! arithmetic progression in one shot.
//!
//! Part 2 generalizes the pattern: an ID is invalid if it consists of some
//! block of digits repeated **two or more times**. To avoid double-counting
//! numbers that themselves contain smaller repeating blocks (e.g., `121212`
//! is `12` repeated 3 times but also `1212` repeated 1.5 times), we sum only
//! *primitive* blocks by recursively subtracting contributions from proper
//! divisors of each digit length. The arithmetic progression approach still
//! applies once we know the valid `x` range for each (block length, repeats)
//! pair.

use super::util;

pub fn part1(input: &str) -> String {
    let ranges = parse_ranges(input);
    sum_invalid_ids(&ranges).to_string()
}

pub fn part2(input: &str) -> String {
    let ranges = parse_ranges(input);
    sum_invalid_ids_any_repeat(&ranges).to_string()
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = util::read_input("day02")?;
    println!("Day 02\nPart 1: {}\nPart 2: {}", part1(&input), part2(&input));
    Ok(())
}

fn parse_ranges(input: &str) -> Vec<(u128, u128)> {
    input
        .split(',')
        .filter_map(|chunk| {
            let chunk = chunk.trim();
            if chunk.is_empty() {
                return None;
            }
            let (lo, hi) = chunk
                .split_once('-')
                .unwrap_or_else(|| panic!("invalid range segment: {chunk}"));
            let start: u128 = lo.trim().parse().expect("start id");
            let end: u128 = hi.trim().parse().expect("end id");
            assert!(
                start <= end,
                "range start must be <= end (found {start}-{end})"
            );
            Some((start, end))
        })
        .collect()
}

fn sum_invalid_ids(ranges: &[(u128, u128)]) -> u128 {
    let max_end = ranges.iter().map(|&(_, hi)| hi).max().unwrap_or(0);
    let max_digits = num_digits(max_end);
    let max_k = max_digits / 2;
    if max_k == 0 {
        return 0;
    }

    let mut pow10 = vec![1u128; max_k + 1];
    for i in 1..=max_k {
        pow10[i] = pow10[i - 1] * 10;
    }

    ranges
        .iter()
        .map(|&(start, end)| sum_invalid_in_range(start, end, max_k, &pow10))
        .sum()
}

fn sum_invalid_in_range(start: u128, end: u128, max_k: usize, pow10: &[u128]) -> u128 {
    let mut total = 0u128;
    for k in 1..=max_k {
        let pow = pow10[k];
        let mult = pow + 1;
        let min_x = if k == 1 { 1 } else { pow10[k - 1] };
        let max_x = pow - 1;

        let x_lo = min_x.max(ceil_div(start, mult));
        let x_hi = max_x.min(end / mult);
        if x_lo > x_hi {
            continue;
        }
        let count = x_hi - x_lo + 1;
        let sum_x = (x_lo + x_hi) * count / 2;
        total += sum_x * mult;
    }
    total
}

fn sum_invalid_ids_any_repeat(ranges: &[(u128, u128)]) -> u128 {
    let max_end = ranges.iter().map(|&(_, hi)| hi).max().unwrap_or(0);
    if max_end == 0 {
        return 0;
    }
    let max_digits = num_digits(max_end);
    let mut pow10 = vec![1u128; max_digits + 1];
    for i in 1..pow10.len() {
        pow10[i] = pow10[i - 1] * 10;
    }
    let mut divisors = vec![Vec::new(); max_digits + 1];
    for len in 1..=max_digits {
        for d in 1..len {
            if len % d == 0 {
                divisors[len].push(d);
            }
        }
    }
    ranges
        .iter()
        .map(|&(start, end)| sum_any_repeat_in_range(start, end, max_digits, &pow10, &divisors))
        .sum()
}

fn sum_any_repeat_in_range(
    start: u128,
    end: u128,
    max_digits_total: usize,
    pow10: &[u128],
    divisors: &[Vec<usize>],
) -> u128 {
    if start > end {
        return 0;
    }
    let mut total = 0u128;
    let range_digit_max = num_digits(end);
    for len in 1..=range_digit_max {
        let base_min = if len == 1 { 1 } else { pow10[len - 1] };
        let base_max = pow10[len] - 1;
        let denom = pow10[len] - 1;
        for k in 2.. {
            let rep_digits = len * k;
            if rep_digits > max_digits_total {
                break;
            }
            if rep_digits > range_digit_max {
                break;
            }
            let mult = (pow10[rep_digits] - 1) / denom;
            let x_lo = base_min.max(ceil_div(start, mult));
            let x_hi = base_max.min(end / mult);
            if x_lo > x_hi {
                continue;
            }
            let primitive_sum =
                primitive_sum_for_len(len, x_lo, x_hi, pow10, divisors);
            if primitive_sum == 0 {
                continue;
            }
            total += primitive_sum * mult;
        }
    }
    total
}

fn primitive_sum_for_len(
    len: usize,
    mut lo: u128,
    mut hi: u128,
    pow10: &[u128],
    divisors: &[Vec<usize>],
) -> u128 {
    if lo > hi {
        return 0;
    }
    let min_bound = if len == 1 { 1 } else { pow10[len - 1] };
    let max_bound = pow10[len] - 1;
    lo = lo.max(min_bound);
    hi = hi.min(max_bound);
    if lo > hi {
        return 0;
    }
    let total_sum = sum_range(lo, hi);
    let mut subtract = 0u128;
    for &d in &divisors[len] {
        let factor = (pow10[len] - 1) / (pow10[d] - 1);
        let mut mapped_lo = ceil_div(lo, factor);
        let mut mapped_hi = hi / factor;
        let sub_min = if d == 1 { 1 } else { pow10[d - 1] };
        let sub_max = pow10[d] - 1;
        mapped_lo = mapped_lo.max(sub_min);
        mapped_hi = mapped_hi.min(sub_max);
        if mapped_lo > mapped_hi {
            continue;
        }
        let child_sum =
            primitive_sum_for_len(d, mapped_lo, mapped_hi, pow10, divisors);
        subtract += child_sum * factor;
    }
    total_sum - subtract
}

fn sum_range(lo: u128, hi: u128) -> u128 {
    let count = hi - lo + 1;
    (lo + hi) * count / 2
}

fn ceil_div(a: u128, b: u128) -> u128 {
    if a == 0 {
        0
    } else {
        ((a - 1) / b) + 1
    }
}

fn num_digits(mut n: u128) -> usize {
    if n == 0 {
        return 1;
    }
    let mut digits = 0;
    while n > 0 {
        n /= 10;
        digits += 1;
    }
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_PART1: Option<&str> = Some("1227775554");
    const EXPECTED_PART2: Option<&str> = Some("4174379265");
    const EXAMPLE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day02_example.txt"));

    #[test]
    fn example_part1() {
        let got = part1(EXAMPLE);
        if let Some(exp) = EXPECTED_PART1 {
            assert_eq!(got, exp);
        } else {
            panic!("set EXPECTED_PART1 to Some(_) to enable this test");
        }
    }

    #[test]
    fn example_part2() {
        let got = part2(EXAMPLE);
        if let Some(exp) = EXPECTED_PART2 {
            assert_eq!(got, exp);
        }
    }

    #[test]
    fn ceil_div_handles_zero() {
        assert_eq!(ceil_div(0, 5), 0);
        assert_eq!(ceil_div(1, 5), 1);
        assert_eq!(ceil_div(9, 5), 2);
        assert_eq!(ceil_div(10, 5), 2);
    }
}
