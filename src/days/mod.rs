pub mod util;
pub mod day01;
pub mod day02;

/// Run the selected day by id (e.g., "day01" or "1").
pub fn run_day(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let norm = util::normalize_day(id);
    match norm.as_str() {
        "day01" => day01::run(),
        "day02" => day02::run(),
        _ => Err(format!("Unknown day: {}", id).into()),
    }
}

