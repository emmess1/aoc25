mod days;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day = args.get(1).map(|s| s.as_str()).unwrap_or("day01");
    if let Err(e) = days::run_day(day) {
        eprintln!("{}\nUsage: cargo run -- <day>  (e.g., day01 or 1)", e);
        std::process::exit(1);
    }
}
