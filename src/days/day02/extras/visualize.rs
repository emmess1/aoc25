//! Visualization helpers for Day 02 Part 1.
//!
//! The visualization focuses on how an input range contributes invalid IDs for
//! each half-length `k`. For a textual overview, call `render_part1_report`.

use crate::days::day02::{ceil_div, num_digits};

/// Render a multiline string explaining which `x` values (half-blocks) fall
/// inside each range for every valid `k`.
pub fn render_part1_report(input: &str) -> String {
    let ranges = super::super::parse_ranges(input);
    let mut out = String::new();
    for (idx, (start, end)) in ranges.iter().enumerate() {
        let header = format!("Range {}: {}-{}\n", idx + 1, start, end);
        out.push_str(&header);
        let contrib = describe_range_part1(*start, *end);
        if contrib.is_empty() {
            out.push_str("  (no duplicate-half IDs)\n");
        } else {
            out.push_str(&contrib);
        }
        out.push('\n');
    }
    out
}

fn describe_range_part1(start: u128, end: u128) -> String {
    let mut lines = String::new();
    let max_digits = num_digits(end);
    for k in 1..=max_digits / 2 {
        let pow = 10u128.pow(k as u32);
        let mult = pow + 1;
        let min_x = if k == 1 { 1 } else { 10u128.pow(k as u32 - 1) };
        let max_x = pow - 1;
        let x_lo = min_x.max(ceil_div(start, mult));
        let x_hi = max_x.min(end / mult);
        if x_lo > x_hi {
            continue;
        }
        lines.push_str(&format!(
            "  k={}: x in [{}..={}] => {} values (sum = {})\n",
            k,
            x_lo,
            x_hi,
            x_hi - x_lo + 1,
            (x_lo + x_hi) * (x_hi - x_lo + 1) / 2 * mult
        ));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_mentions_k() {
        let input = "11-22";
        let report = render_part1_report(input);
        assert!(report.contains("k=1"));
    }
}
