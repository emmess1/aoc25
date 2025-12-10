//! Visualization helpers for Day 02 Part 2.
//!
//! This renders a human-readable table showing which base lengths / repeat
//! counts contribute invalid IDs per range. Only primitive bases (i.e.,
//! blocks that are not multiples of a smaller repeating unit) are included,
//! mirroring the main solver logic.

use crate::days::day02::{ceil_div, num_digits, primitive_sum_for_len, sum_range};

pub fn render_part2_report(input: &str) -> String {
    let ranges = super::super::parse_ranges(input);
    let max_end = ranges.iter().map(|&(_, hi)| hi).max().unwrap_or(0);
    if max_end == 0 {
        return "No ranges provided.".to_string();
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

    let mut out = String::new();
    for (idx, &(start, end)) in ranges.iter().enumerate() {
        out.push_str(&format!("Range {}: {}-{}\n", idx + 1, start, end));
        let desc = describe_range_part2(start, end, max_digits, &pow10, &divisors);
        if desc.is_empty() {
            out.push_str("  (no repeated-block IDs)\n\n");
        } else {
            out.push_str(&desc);
            out.push('\n');
        }
    }
    out
}

fn describe_range_part2(
    start: u128,
    end: u128,
    max_digits_total: usize,
    pow10: &[u128],
    divisors: &[Vec<usize>],
) -> String {
    if start > end {
        return String::new();
    }
    let mut lines = String::new();
    let digit_max = num_digits(end);
    for len in 1..=digit_max {
        let base_min = if len == 1 { 1 } else { pow10[len - 1] };
        let base_max = pow10[len] - 1;
        for k in 2.. {
            let rep_digits = len * k;
            if rep_digits > max_digits_total || rep_digits > digit_max {
                break;
            }
            let mult = (pow10[rep_digits] - 1) / (pow10[len] - 1);
            let x_lo = base_min.max(ceil_div(start, mult));
            let x_hi = base_max.min(end / mult);
            if x_lo > x_hi {
                continue;
            }
            let primitive = primitive_sum_for_len(len, x_lo, x_hi, pow10, divisors);
            if primitive == 0 {
                continue;
            }
            lines.push_str(&format!(
                "  len={} repeat={} -> {} primitive values, sum={} (total contrib={})\n",
                len,
                k,
                count_range(x_lo, x_hi),
                sum_range(x_lo, x_hi),
                primitive * mult
            ));
        }
    }
    lines
}

fn count_range(lo: u128, hi: u128) -> u128 {
    hi - lo + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_report_mentions_repeat() {
        let input = "95-115";
        let report = render_part2_report(input);
        assert!(report.contains("repeat"));
    }
}
