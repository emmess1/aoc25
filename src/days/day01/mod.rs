//! AoC Day 01 scaffold

use super::util;

pub fn part1(input: &str) -> String {
    // TODO: implement solution for part 1
    // Example placeholder: count lines
    input.lines().count().to_string()
}

pub fn part2(input: &str) -> String {
    // TODO: implement solution for part 2
    // Example placeholder: count non-empty lines
    input.lines().filter(|l| !l.is_empty()).count().to_string()
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = util::read_input("day01")?;
    println!("Day 01\nPart 1: {}\nPart 2: {}", part1(&input), part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fill these when you know the example answers to enable assertions.
    const EXPECTED_PART1: Option<&str> = None;
    const EXPECTED_PART2: Option<&str> = None;

    // Example input embedded at compile time. Place your example in inputs/day01_example.txt
    const EXAMPLE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day01_example.txt"));

    #[test]
    fn example_part1() {
        let got = part1(EXAMPLE);
        if let Some(exp) = EXPECTED_PART1 {
            assert_eq!(got, exp);
        } else {
            // Scaffold mode: assert the function runs
            assert!(!got.is_empty() || got == "0");
        }
    }

    #[test]
    fn example_part2() {
        let got = part2(EXAMPLE);
        if let Some(exp) = EXPECTED_PART2 {
            assert_eq!(got, exp);
        } else {
            assert!(!got.is_empty() || got == "0");
        }
    }
}

