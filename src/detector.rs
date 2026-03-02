use crate::state::AppState;
use std::time::{SystemTime, UNIX_EPOCH};

pub enum Judgment {
    None,   // Do nothing — silent
    Guilty, // Show the penguin
}

/// Production path (release build): reads the real hardware clock config.
/// Debug path (cargo run / cargo test): reads a local mock file for safe testing.
#[cfg(not(debug_assertions))]
const ADJTIME_PATH: &str = "/etc/adjtime";

#[cfg(debug_assertions)]
const ADJTIME_PATH: &str = "./mock_adjtime";

/// Reads line 3 of `path`, trims whitespace, and uppercases the result.
/// Returns `None` on any I/O failure, missing file, or empty/absent line 3.
/// This is the testable seam — accepts any path, never reads ADJTIME_PATH directly.
pub(crate) fn read_hwclock_mode_from(path: &str) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    let third_line = content.lines().nth(2)?;
    let mode = third_line.trim().to_uppercase();
    if mode.is_empty() {
        return None;
    }
    Some(mode)
}

/// Reads from the configured ADJTIME_PATH (production or debug, via #[cfg]).
fn read_hwclock_mode() -> Option<String> {
    read_hwclock_mode_from(ADJTIME_PATH)
}

/// Pure judgment logic — accepts an already-resolved mode string and a loaded AppState.
/// No file system access. Fully deterministic and testable.
pub(crate) fn judge_with_mode(mode: Option<String>, state: &AppState) -> Judgment {
    let hwclock_mode = match mode {
        Some(m) => m,
        None => return Judgment::None, // Fail-safe: unreadable adjtime → stay silent
    };

    // Hardware clock set to UTC → correct configuration, stay silent
    if hwclock_mode == "UTC" {
        return Judgment::None;
    }

    // Only "LOCAL" triggers judgment. Any other unknown value → stay silent
    if hwclock_mode != "LOCAL" {
        return Judgment::None;
    }

    // One-time judgment contract: already recorded this mode → stay silent
    if state.last_known_timezone == hwclock_mode {
        return Judgment::None;
    }

    Judgment::Guilty
}

/// Public entry point called by main.rs.
pub fn judge() -> Judgment {
    let mode = read_hwclock_mode();
    let state = AppState::load();
    judge_with_mode(mode, &state)
}

/// Persists the current hwclock mode (or "UTC" on read failure) with a Unix timestamp.
/// Called unconditionally by main.rs after both Judgment::None and Judgment::Guilty.
pub fn update_state_after_judgment() {
    let hwclock_mode = read_hwclock_mode().unwrap_or_else(|| "UTC".to_string());

    let timestamp_now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let state = AppState {
        last_known_timezone: hwclock_mode,
        last_judgment_timestamp: timestamp_now,
    };

    let _ = state.save();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AppState;

    fn empty_state() -> AppState {
        AppState::default()
    }

    fn state_with_tz(tz: &str) -> AppState {
        AppState {
            last_known_timezone: tz.to_string(),
            last_judgment_timestamp: 0,
        }
    }

    // --- judge_with_mode: pure logic tests (zero file I/O) ---

    #[test]
    fn utc_mode_is_silent() {
        assert!(matches!(
            judge_with_mode(Some("UTC".to_string()), &empty_state()),
            Judgment::None
        ));
    }

    #[test]
    fn local_mode_first_time_is_guilty() {
        assert!(matches!(
            judge_with_mode(Some("LOCAL".to_string()), &empty_state()),
            Judgment::Guilty
        ));
    }

    #[test]
    fn local_mode_already_recorded_is_silent() {
        assert!(matches!(
            judge_with_mode(Some("LOCAL".to_string()), &state_with_tz("LOCAL")),
            Judgment::None
        ));
    }

    #[test]
    fn missing_adjtime_is_silent() {
        assert!(matches!(judge_with_mode(None, &empty_state()), Judgment::None));
    }

    #[test]
    fn unknown_value_is_silent() {
        assert!(matches!(
            judge_with_mode(Some("WEIRD".to_string()), &empty_state()),
            Judgment::None
        ));
    }

    // --- read_hwclock_mode_from: file parsing tests (uses tempfile) ---

    #[test]
    fn parse_utc_returns_utc() {
        use std::io::Write;
        let mut tmp = tempfile::NamedTempFile::new().expect("tempfile");
        writeln!(tmp, "0.0 0 0.0").unwrap();
        writeln!(tmp, "0").unwrap();
        writeln!(tmp, "UTC").unwrap();
        assert_eq!(
            read_hwclock_mode_from(tmp.path().to_str().unwrap()),
            Some("UTC".to_string())
        );
    }

    #[test]
    fn parse_local_lowercase_and_whitespace() {
        use std::io::Write;
        let mut tmp = tempfile::NamedTempFile::new().expect("tempfile");
        writeln!(tmp, "0.0 0 0.0").unwrap();
        writeln!(tmp, "0").unwrap();
        writeln!(tmp, " local ").unwrap();
        assert_eq!(
            read_hwclock_mode_from(tmp.path().to_str().unwrap()),
            Some("LOCAL".to_string())
        );
    }

    #[test]
    fn missing_file_returns_none() {
        assert_eq!(
            read_hwclock_mode_from("/nonexistent/path/to/adjtime_xyz"),
            None
        );
    }

    #[test]
    fn too_few_lines_returns_none() {
        use std::io::Write;
        let mut tmp = tempfile::NamedTempFile::new().expect("tempfile");
        writeln!(tmp, "0.0 0 0.0").unwrap();
        writeln!(tmp, "0").unwrap();
        // No third line written
        assert_eq!(
            read_hwclock_mode_from(tmp.path().to_str().unwrap()),
            None
        );
    }

    #[test]
    fn empty_third_line_returns_none() {
        use std::io::Write;
        let mut tmp = tempfile::NamedTempFile::new().expect("tempfile");
        writeln!(tmp, "0.0 0 0.0").unwrap();
        writeln!(tmp, "0").unwrap();
        writeln!(tmp, "   ").unwrap(); // whitespace only
        assert_eq!(
            read_hwclock_mode_from(tmp.path().to_str().unwrap()),
            None
        );
    }
}
