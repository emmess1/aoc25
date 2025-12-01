//! Day 01: Safe Cracker Dial

pub mod extras;

use std::time::Duration;

use super::util;

const START_POS: i64 = 50;
const DIAL_SIZE: i64 = 100;

pub fn part1(input: &str) -> String {
    let rotations = parse_rotations(input);
    let (zero_hits, _, _) = simulate(&rotations);
    zero_hits.to_string()
}

pub fn part2(input: &str) -> String {
    let rotations = parse_rotations(input);
    let (_, _, zero_clicks) = simulate(&rotations);
    zero_clicks.to_string()
}

fn parse_rotations(input: &str) -> Vec<(char, i64)> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let (dir_ch, rest) = line.split_at(1);
            let dir = dir_ch
                .chars()
                .next()
                .map(|c| c.to_ascii_uppercase())
                .expect("direction char");
            let steps: i64 = rest.trim().parse().unwrap_or_else(|_| {
                panic!("invalid rotation distance: {line}");
            });
            if steps < 0 {
                panic!("rotation distance must be non-negative: {line}");
            }
            Some((dir, steps))
        })
        .collect()
}

fn simulate(rotations: &[(char, i64)]) -> (usize, i64, usize) {
    let mut pos = START_POS;
    let mut zero_hits = 0usize;
    let mut zero_clicks = 0usize;
    for &(dir, steps) in rotations {
        zero_clicks += zero_hits_during_rotation(pos, dir, steps);
        pos = match dir {
            'L' => (pos - steps).rem_euclid(DIAL_SIZE),
            'R' => (pos + steps).rem_euclid(DIAL_SIZE),
            other => panic!("unknown rotation direction: {other}"),
        };
        if pos == 0 {
            zero_hits += 1;
        }
    }
    (zero_hits, pos, zero_clicks)
}

fn zero_hits_during_rotation(start_pos: i64, dir: char, steps: i64) -> usize {
    if steps == 0 {
        return 0;
    }
    let offset = match dir {
        'L' => start_pos.rem_euclid(DIAL_SIZE),
        'R' => (DIAL_SIZE - start_pos.rem_euclid(DIAL_SIZE)).rem_euclid(DIAL_SIZE),
        other => panic!("unknown rotation direction: {other}"),
    };
    let first_hit = if offset == 0 { DIAL_SIZE } else { offset };
    if steps < first_hit {
        0
    } else {
        let remaining = steps - first_hit;
        1 + (remaining / DIAL_SIZE) as usize
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = util::read_input("day01")?;
    if let Some(options) = animation_options_from_env() {
        extras::animation::animate_from_input(&input, options)?;
    }
    if let Some(path) = web_animation_path_from_env() {
        extras::web::write_animation_html(&path, &input)?;
        eprintln!("Day01 web animation written to {}", path);
    }
    println!("Day 01\nPart 1: {}\nPart 2: {}", part1(&input), part2(&input));
    Ok(())
}

/// Inspect `DAY01_ANIMATE` (and optional tuning vars) to decide whether to
/// render the dial animation before printing puzzle answers.
fn animation_options_from_env() -> Option<extras::animation::AnimationOptions> {
    let flag = std::env::var("DAY01_ANIMATE").ok()?;
    if matches!(
        flag.trim().to_ascii_lowercase().as_str(),
        "" | "0" | "false" | "off" | "no"
    ) {
        return None;
    }
    let mut opts = extras::animation::AnimationOptions::default();
    if let Some(delay_ms) = std::env::var("DAY01_ANIMATE_DELAY_MS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
    {
        opts.frame_delay = Duration::from_millis(delay_ms);
    }
    if let Some(max_frames) = std::env::var("DAY01_ANIMATE_MAX_FRAMES")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
    {
        opts.max_frames = Some(max_frames);
    }
    if let Some(clear) = std::env::var("DAY01_ANIMATE_CLEAR").ok() {
        let toggle = !matches!(
            clear.trim().to_ascii_lowercase().as_str(),
            "" | "0" | "false" | "off" | "no"
        );
        opts.clear_screen = toggle;
    }
    Some(opts)
}

fn web_animation_path_from_env() -> Option<String> {
    std::env::var("DAY01_ANIMATE_WEB")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fill these when you know the example answers to enable assertions.
    const EXPECTED_PART1: Option<&str> = Some("3");
    const EXPECTED_PART2: Option<&str> = Some("6");

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
