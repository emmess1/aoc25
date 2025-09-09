//! AoC Day 08 scaffold
use super::util;
pub fn part1(input: &str) -> String { input.lines().count().to_string() }
pub fn part2(input: &str) -> String { input.split_whitespace().count().to_string() }
pub fn run() -> Result<(), Box<dyn std::error::Error>> { let input = util::read_input("day08")?; println!("Day 08\nPart 1: {}\nPart 2: {}", part1(&input), part2(&input)); Ok(()) }
#[cfg(test)] mod tests { use super::*; const EXPECTED_PART1: Option<&str> = None; const EXPECTED_PART2: Option<&str> = None; const EXAMPLE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day08_example.txt")); #[test] fn example_part1(){ let got=part1(EXAMPLE); if let Some(exp)=EXPECTED_PART1{ assert_eq!(got,exp);} } #[test] fn example_part2(){ let got=part2(EXAMPLE); if let Some(exp)=EXPECTED_PART2{ assert_eq!(got,exp);} } }
