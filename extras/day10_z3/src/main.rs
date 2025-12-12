//! Day 10 Part 2 solver that uses the `z3` crate.
//!
//! Run with `cargo run --release -- <path>` from this directory. If no path is supplied,
//! the solver defaults to `../../inputs/day10.txt`. On macOS with Homebrew Z3, for example:
//! `C_INCLUDE_PATH=/opt/homebrew/include LIBRARY_PATH=/opt/homebrew/lib cargo run --release -- ../../inputs/day10.txt`

use std::{env, error::Error, fs, path::PathBuf};

use z3::{
    ast::{Ast, Int},
    Config, Context, Optimize, SatResult,
};

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../inputs/day10.txt")
            .canonicalize()
            .unwrap_or_else(|_| PathBuf::from("../../inputs/day10.txt"))
    });
    let input = fs::read_to_string(&path)?;
    let machines = parse_machines(&input)
        .map_err(|err| format!("Failed to parse {}: {err}", path.display()))?;
    if machines.is_empty() {
        println!("No machines with joltage requirements were found.");
        return Ok(());
    }

    let mut total = 0i64;
    for (idx, machine) in machines.iter().enumerate() {
        let presses = solve_machine(machine)
            .map_err(|err| format!("Machine {}: {err}", idx + 1))?;
        total += presses;
        println!("Machine {:>3}: {presses} button presses", idx + 1);
    }
    println!("Total presses: {total}");
    Ok(())
}

#[derive(Debug, Clone)]
struct Machine {
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}

fn parse_machines(input: &str) -> Result<Vec<Machine>, String> {
    let mut machines = Vec::new();
    for (line_idx, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        machines.push(
            parse_machine(line).map_err(|err| format!("line {}: {err}", line_idx + 1))?,
        );
    }
    Ok(machines)
}

fn parse_machine(line: &str) -> Result<Machine, String> {
    let pattern_start = line
        .find('[')
        .ok_or_else(|| "missing '[' in machine description".to_string())?
        + 1;
    let pattern_end = line[pattern_start..]
        .find(']')
        .ok_or_else(|| "missing ']' in machine description".to_string())?
        + pattern_start;
    let pattern = &line[pattern_start..pattern_end];
    let num_lights = pattern.chars().count();
    if num_lights == 0 {
        return Err("indicator diagram must not be empty".into());
    }

    let mut rest = line[pattern_end + 1..].trim_start();
    let mut buttons = Vec::new();
    while !rest.is_empty() {
        if rest.starts_with('{') {
            break;
        }
        if !rest.starts_with('(') {
            return Err(format!("expected '(' while parsing buttons near '{rest}'"));
        }
        let after_open = &rest[1..];
        let close = after_open
            .find(')')
            .ok_or_else(|| "missing ')' in button definition".to_string())?;
        let inside = &after_open[..close];
        let parsed = parse_button(inside, num_lights)?;
        if !parsed.is_empty() {
            buttons.push(parsed);
        }
        rest = after_open[close + 1..].trim_start();
    }

    if !rest.contains('{') {
        return Err("machine is missing joltage requirements".into());
    }
    let brace_start = rest.find('{').unwrap();
    let inner = &rest[brace_start + 1..];
    let brace_end = inner
        .find('}')
        .ok_or_else(|| "missing closing '}' in joltage spec".to_string())?;
    let joltage = parse_joltage(&inner[..brace_end])?;
    if joltage.len() != num_lights {
        return Err(format!(
            "joltage requirement count ({}) must match indicator lights ({num_lights})",
            joltage.len()
        ));
    }

    buttons.sort();
    buttons.dedup();

    Ok(Machine { buttons, joltage })
}

fn parse_button(spec: &str, lights: usize) -> Result<Vec<usize>, String> {
    let mut indices = Vec::new();
    for entry in spec.split(',') {
        let trimmed = entry.trim();
        if trimmed.is_empty() {
            continue;
        }
        let idx = trimmed
            .parse::<usize>()
            .map_err(|_| format!("invalid button index '{trimmed}'"))?;
        if idx >= lights {
            return Err(format!(
                "button index {idx} exceeds number of lights ({lights})"
            ));
        }
        indices.push(idx);
    }
    indices.sort_unstable();
    indices.dedup();
    Ok(indices)
}

fn parse_joltage(spec: &str) -> Result<Vec<i64>, String> {
    let mut values = Vec::new();
    for entry in spec.split(',') {
        let trimmed = entry.trim();
        if trimmed.is_empty() {
            continue;
        }
        let value = trimmed
            .parse::<i64>()
            .map_err(|_| format!("invalid joltage value '{trimmed}'"))?;
        values.push(value);
    }
    Ok(values)
}

fn solve_machine(machine: &Machine) -> Result<i64, String> {
    if machine.joltage.is_empty() {
        return Ok(0);
    }
    if machine.buttons.is_empty() {
        if machine.joltage.iter().all(|&val| val == 0) {
            return Ok(0);
        }
        return Err("no buttons available to satisfy non-zero requirements".into());
    }

    let mut cfg = Config::new();
    cfg.set_timeout_msec(10_000);
    let ctx = Context::new(&cfg);
    let optimizer = Optimize::new(&ctx);
    let zero = Int::from_i64(&ctx, 0);
    let vars: Vec<Int> = machine
        .buttons
        .iter()
        .enumerate()
        .map(|(idx, _)| Int::new_const(&ctx, format!("x_{idx}")))
        .collect();
    for var in &vars {
        optimizer.assert(&var.ge(&zero));
    }

    let involvement = build_involvement(machine);
    for (counter_idx, &target) in machine.joltage.iter().enumerate() {
        let mut expr = Int::from_i64(&ctx, 0);
        for &button_idx in &involvement[counter_idx] {
            expr = expr + vars[button_idx].clone();
        }
        optimizer.assert(&expr._eq(&Int::from_i64(&ctx, target)));
    }

    let objective = vars
        .iter()
        .cloned()
        .fold(Int::from_i64(&ctx, 0), |acc, var| acc + var);
    optimizer.minimize(&objective);

    match optimizer.check(&[]) {
        SatResult::Sat => {
            let model = optimizer
                .get_model()
                .ok_or_else(|| "solver produced no model".to_string())?;
            let value = model
                .eval(&objective, true)
                .ok_or_else(|| "failed to evaluate objective".to_string())?;
            value
                .as_i64()
                .ok_or_else(|| "solution does not fit in i64".to_string())
        }
        SatResult::Unsat => Err("machine is unsatisfiable".into()),
        SatResult::Unknown => Err("solver returned unknown".into()),
    }
}

fn build_involvement(machine: &Machine) -> Vec<Vec<usize>> {
    let mut involvement = vec![Vec::new(); machine.joltage.len()];
    for (button_idx, indices) in machine.buttons.iter().enumerate() {
        for &counter in indices {
            involvement[counter].push(button_idx);
        }
    }
    involvement
}
