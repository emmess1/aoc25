//! Browser-based visualization support for Day 01.
//!
//! This module writes a self-contained HTML file with embedded JavaScript that
//! animates the dial on a `<canvas>`. No external tooling is required; open the
//! generated file in any modern browser and press play.

use std::fs;
use std::path::Path;

use super::super::{parse_rotations, DIAL_SIZE, START_POS};

/// Generate an HTML file containing the dial animation.
pub fn write_animation_html<P: AsRef<Path>>(path: P, input: &str) -> std::io::Result<()> {
    let rotations = parse_rotations(input);
    let html = build_html(&rotations);
    let path_ref = path.as_ref();
    if let Some(parent) = path_ref.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(path_ref, html)
}

fn build_html(rotations: &[(char, i64)]) -> String {
    let rotations_js = rotations
        .iter()
        .map(|(dir, steps)| format!(r#"{{dir:"{}",steps:{}}}"#, dir, steps))
        .collect::<Vec<_>>()
        .join(",\n            ");

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8"/>
<meta name="viewport" content="width=device-width, initial-scale=1"/>
<title>Day 01 Dial Animation</title>
<style>
    :root {{
        font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
        background: #0b1a24;
        color: #e9eef2;
    }}
    body {{
        margin: 0;
        display: flex;
        justify-content: center;
        padding: 2rem;
    }}
    main {{
        max-width: 640px;
        width: 100%;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        align-items: center;
        text-align: center;
    }}
    canvas {{
        background: #061019;
        border-radius: 12px;
        box-shadow: 0 8px 30px rgba(0,0,0,0.35);
    }}
    .controls {{
        display: flex;
        gap: 0.75rem;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
    }}
    button {{
        padding: 0.6rem 1.4rem;
        border-radius: 999px;
        border: none;
        background: #1dd3b0;
        color: #041217;
        font-weight: 600;
        cursor: pointer;
    }}
    button:hover {{
        filter: brightness(1.1);
    }}
    input[type=range] {{
        width: 200px;
    }}
    .stats {{
        font-variant-numeric: tabular-nums;
    }}
</style>
</head>
<body>
<main>
    <h1>Day 01 Dial Animation</h1>
    <canvas id="dial" width="420" height="420"></canvas>
    <div class="controls">
        <button id="toggle">Pause</button>
        <label>Speed
            <input id="speed" type="range" min="1" max="200" value="60"/>
        </label>
        <button id="reset">Reset</button>
    </div>
    <div class="stats" id="stats"></div>
</main>
<script>
const startPos = {start};
const dialSize = {dial_size};
const rotations = [
            {rotations_js}
];

let state = {{pos: startPos, rotIdx: 0, clickInRot: 0, zeroHits: 0, zeroClicks: 0}};
let running = true;
let lastTs = 0;

const canvas = document.getElementById("dial");
const ctx = canvas.getContext("2d");
const stats = document.getElementById("stats");
const toggleBtn = document.getElementById("toggle");
const speed = document.getElementById("speed");
const resetBtn = document.getElementById("reset");

toggleBtn.addEventListener("click", () => {{
    running = !running;
    toggleBtn.textContent = running ? "Pause" : "Play";
}});

resetBtn.addEventListener("click", () => {{
    running = false;
    toggleBtn.textContent = "Play";
    state = {{pos: startPos, rotIdx: 0, clickInRot: 0, zeroHits: 0, zeroClicks: 0}};
}});

function nextClick() {{
    if (state.rotIdx >= rotations.length) {{
        running = false;
        return;
    }}
    const rot = rotations[state.rotIdx];
    state.pos = rot.dir === "L"
        ? (((state.pos - 1) % dialSize) + dialSize) % dialSize
        : (state.pos + 1) % dialSize;
    if (state.pos === 0) {{
        state.zeroClicks += 1;
    }}
    state.clickInRot += 1;
    if (state.clickInRot === rot.steps) {{
        if (state.pos === 0) {{
            state.zeroHits += 1;
        }}
        state.rotIdx += 1;
        state.clickInRot = 0;
    }}
}}

function loop(ts) {{
    const delay = Number(speed.value);
    if (running && ts - lastTs >= delay) {{
        lastTs = ts;
        nextClick();
    }}
    drawDial();
    requestAnimationFrame(loop);
}}

function drawDial() {{
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    const center = canvas.width / 2;
    const radius = center * 0.75;

    // Face
    ctx.beginPath();
    ctx.arc(center, center, radius, 0, Math.PI * 2);
    ctx.fillStyle = "#0a2230";
    ctx.fill();
    ctx.strokeStyle = "#23b5d3";
    ctx.lineWidth = 4;
    ctx.stroke();

    // Tick marks
    for (let value = 0; value < dialSize; value += 5) {{
        const angle = Math.PI / 2 - 2 * Math.PI * (value / dialSize);
        const inner = radius * (value % 10 === 0 ? 0.78 : 0.84);
        const outer = radius * 0.9;
        const x1 = center + inner * Math.cos(angle);
        const y1 = center - inner * Math.sin(angle);
        const x2 = center + outer * Math.cos(angle);
        const y2 = center - outer * Math.sin(angle);
        ctx.beginPath();
        ctx.moveTo(x1, y1);
        ctx.lineTo(x2, y2);
        ctx.strokeStyle = value === 0 ? "#f5b700" : "#1dd3b0";
        ctx.lineWidth = value % 10 === 0 ? 3 : 1.5;
        ctx.stroke();
    }}

    // Pointer
    const pointerAngle = Math.PI / 2 - 2 * Math.PI * (state.pos / dialSize);
    ctx.beginPath();
    ctx.moveTo(center, center);
    ctx.lineTo(
        center + radius * 0.95 * Math.cos(pointerAngle),
        center - radius * 0.95 * Math.sin(pointerAngle)
    );
    ctx.strokeStyle = "#f26419";
    ctx.lineWidth = 5;
    ctx.stroke();

    ctx.beginPath();
    ctx.arc(center, center, 8, 0, Math.PI * 2);
    ctx.fillStyle = "#f26419";
    ctx.fill();

    stats.textContent = `Rotation: ${{Math.min(state.rotIdx + 1, rotations.length)}} / ${{rotations.length}} | Position: ${{state.pos.toString().padStart(2,"0")}} | Zero @end: ${{state.zeroHits}} | Zero clicks: ${{state.zeroClicks}}`;
}}

requestAnimationFrame(loop);
</script>
</body>
</html>
"##,
        start = START_POS,
        dial_size = DIAL_SIZE,
        rotations_js = rotations_js
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_html() {
        let html = build_html(&[('R', 2)]);
        assert!(html.contains("<canvas"));
        assert!(html.contains("rotations"));
    }
}
