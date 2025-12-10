//! AoC Day 10 — Factory
use std::collections::{HashSet, VecDeque};
use std::convert::TryFrom;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, ToPrimitive, Zero};

use super::util;

type Rational = BigRational;

/// Computes the total number of button presses needed for part 1.
///
/// Each machine is solved independently via BFS (see [`min_button_presses`]).
pub fn part1(input: &str) -> String {
    let machines = parse_machines(input);
    let total: u64 = machines
        .iter()
        .map(|machine| {
            min_button_presses(machine)
                .unwrap_or_else(|| panic!("machine has no valid configuration: {:?}", machine))
                as u64
        })
        .sum();
    total.to_string()
}

/// Computes the total number of button presses needed for part 2.
///
/// Part 2 is trickier than part 1 because buttons *add* to counters instead of toggling;
/// therefore we rely on a custom solver that combines reductions and an exact search.
pub fn part2(input: &str) -> String {
    let machines = parse_machines(input);
    let total: u64 = machines
        .iter()
        .map(|machine| {
            min_joltage_button_presses(machine).unwrap_or_else(|| {
                panic!("machine has no valid joltage configuration: {:?}", machine)
            })
        })
        .sum();
    total.to_string()
}

/// Wires everything up to the CLI so `cargo run -- day10` works.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = util::read_input("day10")?;
    println!(
        "Day 10\nPart 1: {}\nPart 2: {}",
        part1(&input),
        part2(&input)
    );
    Ok(())
}

/// Parsed representation of a single machine instruction line.
///
/// * `target` encodes the on/off pattern for the indicator lights (part 1) as a bitmask.
/// * `buttons` holds the toggle mask for each button.
/// * `joltage` lists the required additive counter values for part 2.
#[derive(Debug)]
struct Machine {
    target: u128,
    buttons: Vec<u128>,
    joltage: Vec<u64>,
}

/// Parses every non-empty line into a [`Machine`].
fn parse_machines(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter_map(|line| parse_machine(line))
        .collect()
}

/// Parses indicator diagram, button list, and joltage targets from a single line.
fn parse_machine(line: &str) -> Option<Machine> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let start = line
        .find('[')
        .unwrap_or_else(|| panic!("missing [ in line: {line}"));
    let pattern_start = start + 1;
    let end_rel = line[pattern_start..]
        .find(']')
        .unwrap_or_else(|| panic!("missing ] in line: {line}"));
    let end = pattern_start + end_rel;
    let pattern = &line[pattern_start..end];
    if pattern.is_empty() {
        panic!("indicator diagram must not be empty");
    }
    if pattern.len() > 128 {
        panic!("indicator diagram too large (max 128 lights)");
    }
    let num_lights = pattern.len();
    let mut target = 0u128;
    for (idx, ch) in pattern.chars().enumerate() {
        match ch {
            '#' => target |= 1u128 << idx,
            '.' => {}
            other => panic!("invalid character {other} in indicator diagram"),
        }
    }

    let mut rest = line[end + 1..].trim_start();
    let mut buttons = Vec::new();
    while !rest.is_empty() {
        if rest.starts_with('{') {
            break;
        }
        if !rest.starts_with('(') {
            panic!("expected button definition starting with '(' in line: {line}");
        }
        let after_open = &rest[1..];
        let close = after_open
            .find(')')
            .unwrap_or_else(|| panic!("missing ')' in line: {line}"));
        let inside = &after_open[..close];
        let mask = parse_button(inside, num_lights);
        if mask != 0 {
            buttons.push(mask);
        }
        rest = after_open[close + 1..].trim_start();
    }
    buttons.sort_unstable();
    buttons.dedup();

    let joltage = match rest.find('{') {
        Some(start_brace) => {
            let inner = &rest[start_brace + 1..];
            let end_brace = inner
                .find('}')
                .unwrap_or_else(|| panic!("missing closing '}}' in joltage spec: {line}"));
            parse_joltage(&inner[..end_brace])
        }
        None => Vec::new(),
    };

    if !joltage.is_empty() && joltage.len() != num_lights {
        panic!(
            "joltage requirement count ({}) must match number of indicator lights ({num_lights})",
            joltage.len()
        );
    }

    Some(Machine {
        target,
        buttons,
        joltage,
    })
}

/// Converts a comma-separated list of light indices into a bitmask.
fn parse_button(spec: &str, lights: usize) -> u128 {
    let mut mask = 0u128;
    for entry in spec.split(',') {
        let trimmed = entry.trim();
        if trimmed.is_empty() {
            continue;
        }
        let idx: usize = trimmed
            .parse()
            .unwrap_or_else(|_| panic!("invalid index '{trimmed}' in button definition"));
        if idx >= lights {
            panic!("button index {idx} exceeds number of lights {lights}");
        }
        mask |= 1u128 << idx;
    }
    mask
}

/// Reads the `{a,b,c}` portion into integers for the additive counter targets.
fn parse_joltage(spec: &str) -> Vec<u64> {
    spec.split(',')
        .filter_map(|entry| {
            let trimmed = entry.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(
                    trimmed
                        .parse::<u64>()
                        .unwrap_or_else(|_| panic!("invalid joltage value '{trimmed}'")),
                )
            }
        })
        .collect()
}

/// Classic BFS over indicator-light states.
///
/// Each machine is modeled as an unweighted graph where nodes are indicator bitmasks and edges
/// are button presses (`state ^ mask`). Starting from the all-off mask, the first time BFS reaches
/// the target mask is guaranteed to be the minimal number of presses, so we just sum those values
/// across machines for part 1.
fn min_button_presses(machine: &Machine) -> Option<u32> {
    if machine.target == 0 {
        return Some(0);
    }
    if machine.buttons.is_empty() {
        return None;
    }

    let mut visited: HashSet<u128> = HashSet::new();
    let mut queue = VecDeque::new();
    // Start from the all-off state.
    visited.insert(0);
    queue.push_back((0u128, 0u32));

    while let Some((state, dist)) = queue.pop_front() {
        for &button in &machine.buttons {
            // Toggling a button is just XOR in bitmask space.
            let next = state ^ button;
            let next_dist = dist + 1;
            if next == machine.target {
                return Some(next_dist);
            }
            if visited.insert(next) {
                // First time we see this configuration, so enqueue it with the known distance.
                queue.push_back((next, next_dist));
            }
        }
    }

    None
}

/// Entry point for part 2: reduce the problem as much as possible, then solve the remainder exactly.
///
/// Conceptually we treat each machine as `A * presses = target`, where `A[row][col]` is 1 when the
/// button increments that counter. `reduce_machine` removes zero rows and forced assignments so the
/// remaining matrix is smaller. `solve_linear_system` then does an exact search: it computes the
/// rational RREF to express pivot buttons in terms of free ones, enumerates feasible free counts,
/// and keeps the minimal total. The final answer is `forced_total + optimal_remaining`, and summing
/// that across machines yields Part 2's total.
fn min_joltage_button_presses(machine: &Machine) -> Option<u64> {
    let (forced_total, matrix, target) = reduce_machine(machine)?;
    if matrix.is_empty() {
        return Some(forced_total);
    }
    let extra = solve_linear_system(&matrix, &target)?;
    Some(forced_total + extra)
}

/// Applies cheap logical deductions before the expensive solve.
///
/// Steps:
/// 1. Drop any counters whose target is already zero (and buttons that only touched those rows).
/// 2. Repeatedly locate rows that only have a single active button left; whatever demand remains on
///    that row must come entirely from that button. We subtract the forced amount from every row the
///    button touches and mark the button as fixed.
/// 3. Remove buttons that now only touch satisfied rows.
/// The remaining matrix still captures every feasible solution, but is usually much smaller.
fn reduce_machine(machine: &Machine) -> Option<(u64, Vec<Vec<u8>>, Vec<u64>)> {
    if machine.joltage.is_empty() {
        return Some((0, Vec::new(), Vec::new()));
    }
    if machine.buttons.is_empty() {
        return machine
            .joltage
            .iter()
            .all(|&v| v == 0)
            .then_some((0, Vec::new(), Vec::new()));
    }

    let rows = machine.joltage.len();
    let mut buttons = machine.buttons.clone();
    let targets: Vec<u32> = machine
        .joltage
        .iter()
        .map(|&v| u32::try_from(v).unwrap_or_else(|_| panic!("joltage target too large: {v}")))
        .collect();

    // Quick prune: rows with zero target can be ignored entirely (no button needs to touch them).
    let mut zero_mask = 0u128;
    for (idx, &value) in targets.iter().enumerate() {
        if value == 0 {
            zero_mask |= 1u128 << idx;
        }
    }
    if zero_mask != 0 {
        buttons.retain(|&mask| mask & zero_mask == 0);
    }
    if buttons.is_empty() {
        return targets
            .iter()
            .all(|&v| v == 0)
            .then_some((0, Vec::new(), Vec::new()));
    }

    let mut button_rows: Vec<Vec<usize>> = vec![Vec::new(); buttons.len()];
    let mut row_to_buttons: Vec<Vec<usize>> = vec![Vec::new(); rows];
    for (idx, &mask) in buttons.iter().enumerate() {
        for row in 0..rows {
            if ((mask >> row) & 1) == 1 && targets[row] > 0 {
                button_rows[idx].push(row);
                row_to_buttons[row].push(idx);
            }
        }
    }

    // `active` tracks which buttons are still eligible after pruning.
    let mut active = vec![true; buttons.len()];
    for idx in 0..buttons.len() {
        if button_rows[idx].is_empty() {
            active[idx] = false;
        }
    }

    let mut remaining = targets.clone();
    let mut forced_total = 0u64;
    loop {
        loop {
            let mut progress = false;
            for row in 0..rows {
                let need = remaining[row];
                if need == 0 {
                    continue;
                }
                let covering: Vec<usize> = row_to_buttons[row]
                    .iter()
                    .copied()
                    .filter(|&col| active[col])
                    .collect();
                if covering.is_empty() {
                    return None;
                }
                if covering.len() == 1 {
                    let col = covering[0];
                    let assign = need;
                    for &affected in &button_rows[col] {
                        if remaining[affected] < assign {
                            return None;
                        }
                        remaining[affected] -= assign;
                    }
                    forced_total += assign as u64;
                    active[col] = false;
                    progress = true;
                    break;
                }
            }
            if !progress {
                break;
            }
        }

        let mut removed = false;
        for col in 0..buttons.len() {
            if !active[col] {
                continue;
            }
            if button_rows[col].iter().any(|&row| remaining[row] == 0) {
                active[col] = false;
                removed = true;
            }
        }
        if !removed {
            break;
        }
    }

    // Double-check that the remaining demand is still reachable with the current button set.
    for row in 0..rows {
        if remaining[row] == 0 {
            continue;
        }
        let has_active = row_to_buttons[row]
            .iter()
            .any(|&col| active.get(col).copied().unwrap_or(false));
        if !has_active {
            return None;
        }
    }

    let remaining_rows: Vec<usize> = (0..rows).filter(|&row| remaining[row] > 0).collect();
    if remaining_rows.is_empty() {
        return Some((forced_total, Vec::new(), Vec::new()));
    }

    // The surviving rows and buttons define the residual matrix we pass to the linear solver.
    let kept_cols: Vec<usize> = (0..buttons.len())
        .filter(|&idx| active[idx] && button_rows[idx].iter().any(|&row| remaining[row] > 0))
        .collect();
    if kept_cols.is_empty() {
        return None;
    }

    let mut reduced_matrix = vec![vec![0u8; kept_cols.len()]; remaining_rows.len()];
    for (col_idx, &col) in kept_cols.iter().enumerate() {
        for (row_idx, &row) in remaining_rows.iter().enumerate() {
            if button_rows[col].contains(&row) {
                reduced_matrix[row_idx][col_idx] = 1;
            }
        }
    }
    let reduced_target: Vec<u64> = remaining_rows
        .iter()
        .map(|&row| remaining[row] as u64)
        .collect();

    Some((forced_total, reduced_matrix, reduced_target))
}

/// Solves `matrix * presses = target` over the non-negative integers with minimum 1-norm.
fn solve_linear_system(matrix: &[Vec<u8>], target: &[u64]) -> Option<u64> {
    if matrix.is_empty() {
        if target.iter().all(|&v| v == 0) {
            // Degenerate system: nothing left to solve.
            return Some(0);
        }
        return None;
    }
    let (rref_matrix, rref_rhs, pivot_cols, free_cols) = compute_rref(matrix, target)?;
    let max_press = max_press_counts(matrix, target);
    let button_rows = build_button_rows(matrix);

    if free_cols.is_empty() {
        return evaluate_solution(
            &[],
            &[],
            &rref_matrix,
            &rref_rhs,
            &pivot_cols,
            matrix,
            target,
            &max_press,
        );
    }

    let mut partial_rows = vec![0u64; matrix.len()];
    let mut free_counts = vec![0u64; free_cols.len()];
    let mut best: Option<u64> = None;
    {
        // Because the DFS only knows about the free variables, we pass this closure so it can
        // stitch the partial assignment back into a full solution when needed.
        let mut evaluator = |counts: &[u64]| {
            evaluate_solution(
                &free_cols,
                counts,
                &rref_matrix,
                &rref_rhs,
                &pivot_cols,
                matrix,
                target,
                &max_press,
            )
        };
        search_free_assignments(
            0,
            &free_cols,
            &button_rows,
            &max_press,
            &mut partial_rows,
            &mut free_counts,
            0,
            target,
            &mut best,
            &mut evaluator,
        );
    }
    best
}

/// Pre-computes for each button which rows it touches to accelerate the DFS pruning.
fn build_button_rows(matrix: &[Vec<u8>]) -> Vec<Vec<usize>> {
    if matrix.is_empty() {
        return Vec::new();
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut button_rows = vec![Vec::new(); cols];
    for row in 0..rows {
        for col in 0..cols {
            if matrix[row][col] != 0 {
                button_rows[col].push(row);
            }
        }
    }
    button_rows
}

/// Returns a per-button upper bound: a button cannot be pressed more often than the smallest target it touches.
fn max_press_counts(matrix: &[Vec<u8>], target: &[u64]) -> Vec<u64> {
    if matrix.is_empty() {
        return Vec::new();
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut result = vec![0u64; cols];
    for col in 0..cols {
        let mut min_val = u64::MAX;
        for row in 0..rows {
            if matrix[row][col] != 0 {
                min_val = min_val.min(target[row]);
            }
        }
        if min_val == u64::MAX {
            min_val = 0;
        }
        result[col] = min_val;
    }
    result
}

/// Computes RREF using big rationals so we can reason about exact solutions.
fn compute_rref(
    matrix: &[Vec<u8>],
    rhs: &[u64],
) -> Option<(Vec<Vec<Rational>>, Vec<Rational>, Vec<usize>, Vec<usize>)> {
    if matrix.is_empty() {
        if rhs.iter().all(|&v| v == 0) {
            // Empty but consistent.
            return Some((Vec::new(), Vec::new(), Vec::new(), Vec::new()));
        }
        return None;
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut mat: Vec<Vec<Rational>> = matrix
        .iter()
        .map(|row| row.iter().map(|&v| rational_from_u64(v as u64)).collect())
        .collect();
    let mut vec_rhs: Vec<Rational> = rhs.iter().map(|&v| rational_from_u64(v)).collect();
    let mut pivot_cols = Vec::new();
    let mut current_row = 0usize;

    for col in 0..cols {
        let mut pivot_row = None;
        for row in current_row..rows {
            if !mat[row][col].is_zero() {
                pivot_row = Some(row);
                break;
            }
        }
        let Some(pivot_idx) = pivot_row else {
            continue;
        };
        mat.swap(current_row, pivot_idx);
        vec_rhs.swap(current_row, pivot_idx);
        let pivot_val = mat[current_row][col].clone();
        for c in col..cols {
            mat[current_row][c] /= pivot_val.clone();
        }
        vec_rhs[current_row] /= pivot_val;
        for row in 0..rows {
            if row == current_row {
                continue;
            }
            if mat[row][col].is_zero() {
                continue;
            }
            let factor = mat[row][col].clone();
            for c in col..cols {
                let value = mat[current_row][c].clone() * factor.clone();
                mat[row][c] -= value;
            }
            let rhs_value = vec_rhs[current_row].clone() * factor;
            vec_rhs[row] -= rhs_value;
        }
        pivot_cols.push(col);
        current_row += 1;
        if current_row == rows {
            break;
        }
    }

    for row in current_row..rows {
        let all_zero = mat[row].iter().all(|v| v.is_zero());
        if all_zero && !vec_rhs[row].is_zero() {
            return None;
        }
    }

    let rank = pivot_cols.len();
    let mut truncated_matrix = Vec::with_capacity(rank);
    let mut truncated_rhs = Vec::with_capacity(rank);
    for row in 0..rank {
        truncated_matrix.push(mat[row].clone());
        truncated_rhs.push(vec_rhs[row].clone());
    }
    let mut is_pivot = vec![false; cols];
    for &col in &pivot_cols {
        is_pivot[col] = true;
    }
    let free_cols = (0..cols).filter(|&c| !is_pivot[c]).collect::<Vec<_>>();
    Some((truncated_matrix, truncated_rhs, pivot_cols, free_cols))
}

/// Depth-first search over the free variables, tracking partial feasibility and pruning with bounds.
fn search_free_assignments<F>(
    idx: usize,
    free_cols: &[usize],
    button_rows: &[Vec<usize>],
    max_press: &[u64],
    partial_rows: &mut [u64],
    free_counts: &mut [u64],
    partial_sum: u64,
    target: &[u64],
    best: &mut Option<u64>,
    evaluator: &mut F,
) where
    F: FnMut(&[u64]) -> Option<u64>,
{
    if let Some(best_val) = *best {
        if partial_sum >= best_val {
            // Already worse than the best-known solution: no need to explore deeper.
            return;
        }
    }
    if idx == free_cols.len() {
        if let Some(total) = evaluator(free_counts) {
            if total < best.unwrap_or(u64::MAX) {
                *best = Some(total);
            }
        }
        return;
    }
    let col = free_cols[idx];
    for count in 0..=max_press[col] {
        if let Some(best_val) = *best {
            if partial_sum + count >= best_val {
                if count == 0 {
                    // still allow exploring other buttons
                } else {
                    continue;
                }
            }
        }
        let mut ok = true;
        for &row in &button_rows[col] {
            partial_rows[row] += count;
            if partial_rows[row] > target[row] {
                ok = false;
            }
        }
        if ok {
            free_counts[idx] = count;
            search_free_assignments(
                idx + 1,
                free_cols,
                button_rows,
                max_press,
                partial_rows,
                free_counts,
                partial_sum + count,
                target,
                best,
                evaluator,
            );
        }
        for &row in &button_rows[col] {
            partial_rows[row] -= count;
        }
    }
}

/// Glues the chosen free-variable counts with the RREF solution for the pivot columns.
fn evaluate_solution(
    free_cols: &[usize],
    free_counts: &[u64],
    rref_matrix: &[Vec<Rational>],
    rref_rhs: &[Rational],
    pivot_cols: &[usize],
    matrix: &[Vec<u8>],
    target: &[u64],
    max_press: &[u64],
) -> Option<u64> {
    let cols = if matrix.is_empty() {
        0
    } else {
        matrix[0].len()
    };
    let mut presses = vec![0u64; cols];
    for (idx, &col) in free_cols.iter().enumerate() {
        presses[col] = free_counts[idx];
    }
    for (row_idx, &pivot_col) in pivot_cols.iter().enumerate() {
        let mut value = rref_rhs[row_idx].clone();
        for (free_idx, &col) in free_cols.iter().enumerate() {
            let coeff = rref_matrix[row_idx][col].clone();
            if coeff.is_zero() {
                continue;
            }
            let count_rat = rational_from_u64(free_counts[free_idx]);
            value -= coeff * count_rat;
        }
        if !value.is_integer() {
            return None;
        }
        let integer = value.to_integer();
        if integer.is_negative() {
            return None;
        }
        let count = integer.to_u64()?;
        if count > max_press[pivot_col] {
            return None;
        }
        presses[pivot_col] = count;
    }
    if !verify_press_counts(matrix, target, &presses) {
        return None;
    }
    Some(presses.iter().sum())
}

/// Replays the proposed press vector and ensures it hits every row target exactly.
fn verify_press_counts(matrix: &[Vec<u8>], target: &[u64], presses: &[u64]) -> bool {
    for (row_idx, row) in matrix.iter().enumerate() {
        let mut sum = 0u64;
        for (col_idx, &entry) in row.iter().enumerate() {
            if entry != 0 {
                sum += presses[col_idx];
            }
        }
        if sum != target[row_idx] {
            return false;
        }
    }
    true
}

/// Convenience helper because `BigRational` does not implement `From<u64>` directly.
fn rational_from_u64(value: u64) -> Rational {
    BigRational::from_integer(BigInt::from(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_PART1: Option<&str> = Some("7");
    const EXPECTED_PART2: Option<&str> = Some("33");
    const EXAMPLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/inputs/day10_example.txt"
    ));
    const REAL_INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day10.txt"));

    #[test]
    fn example_part1() {
        let got = part1(EXAMPLE);
        if let Some(exp) = EXPECTED_PART1 {
            assert_eq!(got, exp);
        }
    }

    #[test]
    fn example_part2() {
        let got = part2(EXAMPLE);
        if let Some(exp) = EXPECTED_PART2 {
            assert_eq!(got, exp);
        }
    }

    #[test]
    fn real_input_part1() {
        let got = part1(REAL_INPUT);
        assert_eq!(got, "457");
    }

    #[test]
    fn real_input_part2() {
        let got = part2(REAL_INPUT);
        assert_eq!(got, "17576");
    }

    /// Cross-checks the entire real input against Z3 to prove the solver can't regress silently.
    ///
    /// Running this is intentionally `#[ignore]` because it shells out to the external `z3`
    /// binary and takes several seconds. When it *is* run, it guarantees our custom search finds
    /// the same global minimum as an off-the-shelf SMT optimizer.
    #[test]
    #[ignore]
    fn z3_total_verification() {
        use std::fs;
        let input = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day10.txt"))
            .expect("unable to read day10 input");
        let machines = parse_machines(&input);
        let mut fast_total = 0u64;
        let mut z3_total = 0u64;
        for machine in &machines {
            let fast = min_joltage_button_presses(machine).expect("solver reported no solution");
            fast_total += fast;
            let (forced, matrix, target) = reduce_machine(machine).expect("reduction failed");
            let extra = if matrix.is_empty() {
                0
            } else {
                z3_min_solution(&matrix, &target).expect("z3 failed")
            };
            z3_total += forced + extra;
        }
        eprintln!("fast_total={fast_total}, z3_total={z3_total}");
        assert_eq!(fast_total, z3_total);
    }

    /// Uses the external `z3` binary as a reference solver for the reduced system.
    ///
    /// We encode the problem as a QF_LIA optimization (non-negative integers with a linear
    /// objective). Z3 then returns the minimal 1-norm, letting us double-check our bespoke solver.
    fn z3_min_solution(matrix: &[Vec<u8>], target: &[u64]) -> Option<u64> {
        use std::process::{Command, Stdio};
        if matrix.is_empty() {
            return target.iter().all(|&v| v == 0).then_some(0);
        }
        let cols = matrix[0].len();
        if cols == 0 {
            return target.iter().all(|&v| v == 0).then_some(0);
        }
        let mut smt = String::new();
        smt.push_str("(set-logic QF_LIA)\n");
        for col in 0..cols {
            smt.push_str(&format!("(declare-const x{} Int)\n", col));
            smt.push_str(&format!("(assert (>= x{} 0))\n", col));
        }
        for (row_idx, row) in matrix.iter().enumerate() {
            let mut vars = Vec::new();
            for (col_idx, &entry) in row.iter().enumerate() {
                if entry != 0 {
                    vars.push(format!("x{}", col_idx));
                }
            }
            if vars.is_empty() {
                if target[row_idx] != 0 {
                    return None;
                }
                continue;
            }
            let sum_expr = if vars.len() == 1 {
                vars[0].clone()
            } else {
                format!("(+ {})", vars.join(" "))
            };
            smt.push_str(&format!("(assert (= {} {}))\n", sum_expr, target[row_idx]));
        }
        let objective = if cols == 1 {
            "x0".to_string()
        } else {
            let vars = (0..cols).map(|c| format!("x{}", c)).collect::<Vec<_>>();
            format!("(+ {})", vars.join(" "))
        };
        smt.push_str(&format!("(minimize {})\n", objective));
        smt.push_str("(check-sat)\n(get-objectives)\n");

        let output = Command::new("z3")
            .arg("-in")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                if let Some(stdin) = child.stdin.as_mut() {
                    stdin.write_all(smt.as_bytes())?;
                }
                let output = child.wait_with_output()?;
                Ok(output)
            })
            .ok()?;
        if !output.status.success() {
            return None;
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.trim_start().starts_with("sat") {
            return None;
        }
        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("((") {
                if let Some((_, value_with_paren)) = trimmed.rsplit_once(' ') {
                    let value_str = value_with_paren.trim_end_matches(')').trim();
                    if let Ok(val) = value_str.parse::<u64>() {
                        return Some(val);
                    }
                }
            }
        }
        None
    }
}
