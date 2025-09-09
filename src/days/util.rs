use std::fs;

/// Normalize input like "1", "01", "day1" to canonical "day01".
pub fn normalize_day(id: &str) -> String {
    let s = id.trim().to_lowercase();
    if let Some(num) = s.strip_prefix("day") {
        format!("day{:02}", num.parse::<u32>().unwrap_or(0))
    } else {
        format!("day{:02}", s.parse::<u32>().unwrap_or(0))
    }
}

/// Read input file from `inputs/dayXX.txt`.
pub fn read_input(day: &str) -> std::io::Result<String> {
    let id = normalize_day(day);
    let path = format!("inputs/{}.txt", id);
    fs::read_to_string(path)
}

