//! AoC Day 02 scaffold

use super::util;

pub fn part1(input: &str) -> String {
    // TODO: implement
    input.len().to_string() // placeholder: input length
}

pub fn part2(input: &str) -> String {
    // TODO: implement
    input.split_whitespace().count().to_string() // placeholder: token count
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = util::read_input("day02")?;
    println!("Day 02\nPart 1: {}\nPart 2: {}", part1(&input), part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_PART1: Option<&str> = None;
    const EXPECTED_PART2: Option<&str> = None;
    const EXAMPLE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day02_example.txt"));

    #[test]
    fn example_part1() {
        let got = part1(EXAMPLE);
        if let Some(exp) = EXPECTED_PART1 { assert_eq!(got, exp); }
    }

    #[test]
    fn example_part2() {
        let got = part2(EXAMPLE);
        if let Some(exp) = EXPECTED_PART2 { assert_eq!(got, exp); }
    }
}

