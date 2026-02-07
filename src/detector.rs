use chrono::Local;
use crate::state::AppState;
use std::time::{SystemTime, UNIX_EPOCH};

pub enum Judgment {
    None,       // Do nothing
    Guilty,     // Show the penguin
}

pub fn judge() -> Judgment {
    let now = Local::now();
    let offset = now.offset();
    let is_utc = offset.local_minus_utc() == 0;
    
    // 1. Golden Rule: IF UTC, do nothing.
    if is_utc {
        return Judgment::None;
    }

    let current_timezone = offset.to_string();
    let mut state = AppState::load();

    // 2. Golden Rule: IF same timezone as last known, do nothing.
    if state.last_known_timezone == current_timezone {
        return Judgment::None;
    }

    // 3. Golden Rule: IF NOT UTC AND CHANGED -> GUILTY.
    
    // Update state immediately to record the judgment (or the new timezone)
    // Actually, prompt says "After Judgment: Update last_known_timezone to current, save state."
    // We should return Guilty, and let main handle the printing and saving?
    // Or save here?
    // "After Judgment: Update last_known_timezone to current, save state." implies we save *after* showing the penguin?
    // But if we fail to show penguin, we might want to retry?
    // Let's safe update until main.
    
    Judgment::Guilty
}

pub fn update_state_after_judgment() {
    let now = Local::now();
    let offset = now.offset();
    let timestamp_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    
    let state = AppState {
        last_known_timezone: offset.to_string(),
        last_judgment_timestamp: timestamp_now,
    };
    
    let _ = state.save();
}
