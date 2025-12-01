//! Console animation for visualizing the Day 01 dial.
//!
//! The animation replays every click (individual "ticks") in the rotation
//! sequence and renders a simple ASCII dial. It is intentionally lightweight
//! so you can run it from a `cargo test -- --nocapture` or a custom helper.
//!
//! # Example
//! ```no_run
//! use std::time::Duration;
//! use crate::days::day01::extras::animation::{self, AnimationOptions};
//!
//! # fn demo(input: &str) -> std::io::Result<()> {
//! let options = AnimationOptions {
//!     frame_delay: Duration::from_millis(40),
//!    ..Default::default()
//! };
//! animation::animate_from_input(input, options)?;
//! # Ok(())
//! # }
//! ```

use std::f64::consts::{FRAC_PI_2, PI};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use super::super::{parse_rotations, DIAL_SIZE, START_POS};

/// Configuration knobs for the console animation.
#[derive(Clone, Copy, Debug)]
pub struct AnimationOptions {
    /// Delay inserted between frames.
    pub frame_delay: Duration,
    /// Optional hard cap on the total number of frames (including the initial one).
    pub max_frames: Option<usize>,
    /// Whether to clear the terminal (ANSI escape) before drawing each frame.
    pub clear_screen: bool,
}

impl Default for AnimationOptions {
    fn default() -> Self {
        Self {
            frame_delay: Duration::from_millis(35),
            max_frames: None,
            clear_screen: true,
        }
    }
}

/// Animate the dial using raw puzzle input (one rotation per line).
pub fn animate_from_input(input: &str, options: AnimationOptions) -> io::Result<()> {
    let rotations = parse_rotations(input);
    animate(&rotations, options)
}

/// Animate a prepared sequence of rotations.
pub fn animate(rotations: &[(char, i64)], options: AnimationOptions) -> io::Result<()> {
    let mut stdout = io::stdout();
    run_animation(rotations, options, &mut stdout)
}

fn run_animation<W: Write>(
    rotations: &[(char, i64)],
    options: AnimationOptions,
    writer: &mut W,
) -> io::Result<()> {
    let mut pos = START_POS;
    let mut zero_hits = 0usize;
    let mut zero_clicks = 0usize;
    let mut frames = 0usize;

    emit_frame(
        writer,
        frames,
        pos,
        "Start position",
        zero_hits,
        zero_clicks,
        options.clear_screen,
    )?;
    writer.flush()?;

    frames += 1;
    if reached_limit(frames, options.max_frames) {
        return Ok(());
    }

    for (rot_idx, &(dir, steps)) in rotations.iter().enumerate() {
        if steps == 0 {
            continue;
        }
        for step_idx in 0..steps {
            pos = advance(pos, dir);
            if pos == 0 {
                zero_clicks += 1;
            }
            if step_idx + 1 == steps && pos == 0 {
                zero_hits += 1;
            }

            let desc = format!(
                "{}{} • rotation {} / {} • click {} / {}",
                dir,
                steps,
                rot_idx + 1,
                rotations.len(),
                step_idx + 1,
                steps
            );

            emit_frame(
                writer,
                frames,
                pos,
                &desc,
                zero_hits,
                zero_clicks,
                options.clear_screen,
            )?;
            writer.flush()?;
            frames += 1;

            if reached_limit(frames, options.max_frames) {
                return Ok(());
            }
            if !options.frame_delay.is_zero() {
                thread::sleep(options.frame_delay);
            }
        }
    }

    Ok(())
}

fn advance(pos: i64, dir: char) -> i64 {
    match dir {
        'L' => (pos - 1).rem_euclid(DIAL_SIZE),
        'R' => (pos + 1).rem_euclid(DIAL_SIZE),
        other => panic!("unknown direction in animation: {other}"),
    }
}

fn reached_limit(current: usize, limit: Option<usize>) -> bool {
    limit.map_or(false, |max| current >= max)
}

fn emit_frame<W: Write>(
    writer: &mut W,
    frame_idx: usize,
    pos: i64,
    desc: &str,
    zero_hits: usize,
    zero_clicks: usize,
    clear_screen: bool,
) -> io::Result<()> {
    if clear_screen {
        write!(writer, "\x1B[2J\x1B[H")?;
    } else if frame_idx > 0 {
        writeln!(writer)?;
    }

    let dial = render_dial(pos);
    writeln!(writer, "Frame {:04} | position {:02}", frame_idx, pos)?;
    for line in dial {
        writeln!(writer, "{}", line)?;
    }
    writeln!(
        writer,
        "Info: {} | zero-at-rotation: {} | zero-clicks: {}",
        desc, zero_hits, zero_clicks
    )
}

const GRID_SIZE: usize = 21;
const CENTER: f64 = (GRID_SIZE as f64 - 1.0) / 2.0;
const DIAL_RADIUS: f64 = (GRID_SIZE as f64 - 3.0) / 2.0;

fn render_dial(pos: i64) -> Vec<String> {
    let mut grid = vec![vec![' '; GRID_SIZE]; GRID_SIZE];

    // Draw circular outline.
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let dx = col as f64 - CENTER;
            let dy = row as f64 - CENTER;
            let dist = (dx * dx + dy * dy).sqrt();
            if (DIAL_RADIUS - 0.6..=DIAL_RADIUS + 0.6).contains(&dist) {
                grid[row][col] = '.';
            }
        }
    }

    // Mark tick marks every 10 / 5 units.
    for value in (0..DIAL_SIZE).step_by(5) {
        let (row, col) = coords_for_value(value as i64);
        grid[row][col] = if value % 10 == 0 { '+' } else { '.' };
    }

    // Mark zero and pointer.
    let (z_row, z_col) = coords_for_value(0);
    grid[z_row][z_col] = '0';

    let (p_row, p_col) = coords_for_value(pos);
    grid[p_row][p_col] = '^';

    grid.into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect()
}

fn coords_for_value(value: i64) -> (usize, usize) {
    let normalized = value.rem_euclid(DIAL_SIZE) as f64;
    let angle = FRAC_PI_2 - 2.0 * PI * (normalized / DIAL_SIZE as f64);
    let x = CENTER + DIAL_RADIUS * angle.cos();
    let y = CENTER - DIAL_RADIUS * angle.sin();
    let col = x.round().clamp(0.0, (GRID_SIZE - 1) as f64) as usize;
    let row = y.round().clamp(0.0, (GRID_SIZE - 1) as f64) as usize;
    (row, col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_contains_marker() {
        let dial = render_dial(12);
        assert!(dial.iter().any(|line| line.contains('^')));
    }

    #[test]
    fn respect_frame_cap() {
        let input = "R3";
        let rotations = parse_rotations(input);
        let mut buf = Vec::new();
        run_animation(
            &rotations,
            AnimationOptions {
                frame_delay: Duration::from_millis(0),
                max_frames: Some(2),
                clear_screen: false,
            },
            &mut buf,
        )
        .unwrap();
        let text = String::from_utf8(buf).unwrap();
        assert!(text.contains("Frame 0000"));
        assert!(text.contains("Frame 0001"));
    }
}
