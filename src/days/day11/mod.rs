//! AoC Day 11 — Reactor
use std::collections::{HashMap, HashSet};

use super::util;

pub fn part1(input: &str) -> String {
    let graph = parse_graph(input);
    let mut memo = HashMap::new();
    let mut visiting = HashSet::new();
    count_paths("you", "out", &graph, &mut memo, &mut visiting).to_string()
}

pub fn part2(input: &str) -> String {
    let graph = parse_graph(input);
    let mut memo = HashMap::new();
    let mut visiting = HashSet::new();
    let counts = count_paths_with_requirements(
        "svr",
        "out",
        &graph,
        &mut memo,
        &mut visiting,
        required_mask,
    );
    counts[3].to_string()
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = util::read_input("day11")?;
    println!(
        "Day 11\nPart 1: {}\nPart 2: {}",
        part1(&input),
        part2(&input)
    );
    Ok(())
}

fn parse_graph(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (src, dests) = line
            .split_once(':')
            .unwrap_or_else(|| panic!("invalid line (missing colon): {line}"));
        let src = src.trim().to_string();
        let neighbors = dests
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        graph.insert(src, neighbors);
    }
    graph
}

fn count_paths(
    node: &str,
    target: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, u128>,
    visiting: &mut HashSet<String>,
) -> u128 {
    if node == target {
        return 1;
    }
    if let Some(&cached) = memo.get(node) {
        return cached;
    }
    if !visiting.insert(node.to_string()) {
        panic!("cycle detected involving node '{node}'");
    }
    let mut total = 0u128;
    if let Some(neighbors) = graph.get(node) {
        for dest in neighbors {
            total += count_paths(dest, target, graph, memo, visiting);
        }
    }
    visiting.remove(node);
    memo.insert(node.to_string(), total);
    total
}

fn required_mask(node: &str) -> u8 {
    match node {
        "dac" => 1,
        "fft" => 2,
        _ => 0,
    }
}

fn count_paths_with_requirements(
    node: &str,
    target: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, [u128; 4]>,
    visiting: &mut HashSet<String>,
    mask_fn: fn(&str) -> u8,
) -> [u128; 4] {
    if let Some(result) = memo.get(node) {
        return *result;
    }
    if !visiting.insert(node.to_string()) {
        panic!("cycle detected involving node '{node}'");
    }
    let self_mask = mask_fn(node) as usize;
    let mut totals = [0u128; 4];
    if node == target {
        totals[self_mask] = 1;
    } else if let Some(neighbors) = graph.get(node) {
        for dest in neighbors {
            let child_counts =
                count_paths_with_requirements(dest, target, graph, memo, visiting, mask_fn);
            for mask in 0..4 {
                let combined = mask | self_mask;
                totals[combined] += child_counts[mask];
            }
        }
    }
    visiting.remove(node);
    memo.insert(node.to_string(), totals);
    totals
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_PART1: Option<&str> = Some("5");
    const EXPECTED_PART2_REQUIRED: &str = "2";
    const EXAMPLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/inputs/day11_example.txt"
    ));
    const EXAMPLE_PART2: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/inputs/day11_part2_example.txt"
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
        let got = part2(EXAMPLE_PART2);
        assert_eq!(got, EXPECTED_PART2_REQUIRED);
    }
}
