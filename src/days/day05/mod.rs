//! AoC Day 05

use super::util;

fn parse_database(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut reading_ranges = true;

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if reading_ranges && !ranges.is_empty() {
                reading_ranges = false;
            }
            continue;
        }

        if reading_ranges {
            let (start, end) = trimmed
                .split_once('-')
                .expect("Range line must contain a '-' separator");
            let start = start.trim().parse::<u64>().expect("Invalid range start");
            let end = end.trim().parse::<u64>().expect("Invalid range end");
            let (start, end) = if start <= end {
                (start, end)
            } else {
                (end, start)
            };
            ranges.push((start, end));
        } else {
            let id = trimmed.parse::<u64>().expect("Invalid ingredient ID");
            ids.push(id);
        }
    }

    (ranges, ids)
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by_key(|r| r.0);
    let mut merged = Vec::with_capacity(ranges.len());
    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1.saturating_add(1) {
            if end > current.1 {
                current.1 = end;
            }
        } else {
            merged.push(current);
            current = (start, end);
        }
    }

    merged.push(current);
    merged
}

pub fn part1(input: &str) -> String {
    let (ranges, mut ids) = parse_database(input);
    if ranges.is_empty() || ids.is_empty() {
        return "0".into();
    }

    let ranges = merge_ranges(ranges);
    ids.sort_unstable();

    let mut fresh = 0usize;
    let mut range_idx = 0usize;

    for id in ids {
        while range_idx < ranges.len() && id > ranges[range_idx].1 {
            range_idx += 1;
        }
        if range_idx == ranges.len() {
            break;
        }
        if id >= ranges[range_idx].0 {
            fresh += 1;
        }
    }

    fresh.to_string()
}

pub fn part2(input: &str) -> String {
    let (ranges, _) = parse_database(input);
    if ranges.is_empty() {
        return "0".into();
    }

    let merged = merge_ranges(ranges);
    let total: u64 = merged
        .into_iter()
        .map(|(start, end)| end.saturating_sub(start).saturating_add(1))
        .sum();
    total.to_string()
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = util::read_input("day05")?;
    println!(
        "Day 05\nPart 1: {}\nPart 2: {}",
        part1(&input),
        part2(&input)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXPECTED_PART1: Option<&str> = Some("3");
    const EXPECTED_PART2: Option<&str> = Some("14");
    const EXAMPLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/inputs/day05_example.txt"
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
