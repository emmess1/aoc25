pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod util;

/// Run the selected day by id (e.g., "day01" or "1").
pub fn run_day(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let norm = util::normalize_day(id);
    match norm.as_str() {
        "day01" => day01::run(),
        "day02" => day02::run(),
        "day03" => day03::run(),
        "day04" => day04::run(),
        "day05" => day05::run(),
        "day06" => day06::run(),
        "day07" => day07::run(),
        "day08" => day08::run(),
        "day09" => day09::run(),
        "day10" => day10::run(),
        "day11" => day11::run(),
        "day12" => day12::run(),
        _ => Err(format!("Unknown day: {}", id).into()),
    }
}
