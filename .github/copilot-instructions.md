# GitHub Copilot Instructions — LinuxTeasing

## Project Purpose
A Rust CLI binary that runs at shell startup and renders a pixel art penguin ("Senior Tux") when the system hardware clock is configured to `LOCAL` time — the fingerprint of a dual-boot Windows/Linux setup. **Silent when hardware clock is UTC. Silent if already judged for the same configuration. Loud only on a new `LOCAL` detection.**

## Architecture — 5 Modules, One Pass

```
main.rs  →  detector::judge()  →  Judgment::None | Judgment::Guilty
                                       ↓ Guilty only
                                   renderer::render()   (pixel art)
                                   message::get_message()  (random quote)
                                   detector::update_state_after_judgment()
```

| File | Role |
|---|---|
| `src/detector.rs` | Core logic: read `/etc/adjtime` line 3 → hwclock mode comparison → state load → emit `Judgment` enum |
| `src/state.rs` | `AppState` JSON persistence via `directories` crate (XDG on Linux, Known Folders on Windows) |
| `src/renderer.rs` | Pixel art engine: color-key string array → truecolor terminal via `crossterm` |
| `src/message.rs` | Random snarky quote via `rand::SliceRandom` |
| `src/ascii.rs` | Legacy simple ASCII art — **not used in the active execution path** |

## Pixel Art Rendering Convention
`SENIOR_TUX` in `renderer.rs` is a `&[&str]` color-key grid. Each character maps to an RGB color:

| Key | Color |
|---|---|
| `B` | Black body (`#222222`) |
| `W` | White belly (`#F0F0F0`) |
| `Y` | Yellow beak/feet (`#FFAE00`) |
| `G` | Gray mug (`#5D5D5D`) |
| `C` | Coffee brown (`#6F4E37`) |
| `Z` | Light gray eyes (`#AAAAAA`) |
| ` ` | Transparent (terminal background) |

Rendering pairs **two rows at a time**: FG = top pixel color, BG = bottom pixel color, prints `▀` (U+2580 Upper Half Block). This halves terminal lines needed for the same resolution.

## State Schema
`~/.config/linux-teasing/state.json` (Linux) / `%APPDATA%\LinuxTeasing\config\state.json` (Windows):
```json
{ "last_known_timezone": "LOCAL", "last_judgment_timestamp": 1234567890 }
```
`last_known_timezone` stores the uppercased, trimmed value from `/etc/adjtime` line 3 — either `"UTC"` or `"LOCAL"`. Defaults to `"UTC"` on any read failure. Reset judgment by deleting this file.

## Build & Install
```bash
cargo build --release          # optimized: strip, opt-level="z", lto=true, codegen-units=1
./install.sh                   # build + install to /usr/local/bin + append to .bashrc/.zshrc/.config/fish
```
Expected binary: **<5MB**, startup: **<10ms**. No network calls, no telemetry.

## Testing
```bash
# Run all 10 unit tests
cargo test

# Manual: trigger the penguin locally (debug build reads ./mock_adjtime)
printf "0.0 0 0.0\n0\nLOCAL\n" > mock_adjtime
cargo run                         # → penguin appears
cargo run                         # → silent (state.json now records "LOCAL")

# Reset and test fail-safe
rm mock_adjtime && cargo run      # → silent, no panic
rm ~/.config/linux-teasing/state.json  # reset state between runs
```
Unit tests live in `src/detector.rs` `#[cfg(test)]` module — 5 pure-logic tests (zero I/O) + 5 file-parsing tests via `tempfile`. `tempfile = "3"` is the only dev-dependency.

## Key Design Decisions
- **Detection via `/etc/adjtime` line 3**: Windows sets the hardware clock to local time (`LOCAL`); Linux defaults to `UTC`. Reading this file is a zero-dependency, sub-microsecond syscall — no `chrono`, no shell commands.
- **Fail-safe to silent**: Any read failure (file missing, permission denied, malformed, < 3 lines, empty line) returns `Judgment::None`. Zero `unwrap()` / `panic!()` in the detection path.
- **One-time judgment per hwclock mode**: The tool is silent once the state file records a mode. It only re-triggers on a *change*.
- **Testable seam via `#[cfg(debug_assertions)]`**: Debug builds read `./mock_adjtime`; release builds read `/etc/adjtime`. Create a 3-line mock file to trigger the penguin locally without touching system files.
- **State saved unconditionally** via `update_state_after_judgment()` called in both `Judgment::None` and `Judgment::Guilty` branches in `main.rs`.
- **Release profile for size**: `Cargo.toml` uses `strip = true` + `opt-level = "z"` — avoid changing these for a shell startup tool.

## Doc vs. Code Divergence
`docs/rust_cli_architecture.md` describes an "Uptime > 24h" cooldown check and a `sysinfo` dependency — **neither exists in the current implementation**. The actual logic is: read `/etc/adjtime` → parse hwclock mode → state compare → judge. Treat the source code as the ground truth.

## Adding New Features
- **New messages**: add strings to the `vec![]` in `message::get_message()`.
- **New pixel art variant**: add a new `&[&str]` constant to `renderer.rs` using the same color-key convention; extend `get_color()` if new keys are needed.
- **New judgment conditions**: modify `judge_with_mode()` in `detector.rs` — it accepts `Option<String>` + `&AppState`, keeping it a pure, testable function. Keep the `Judgment` enum as the output contract.
