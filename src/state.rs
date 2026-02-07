use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use std::io::{self};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppState {
    pub last_known_timezone: String,
    pub last_judgment_timestamp: i64,
}

impl AppState {
    pub fn load() -> Self {
        let path = Self::get_state_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(state) = serde_json::from_str(&content) {
                    return state;
                }
            }
        }
        AppState::default()
    }

    pub fn save(&self) -> io::Result<()> {
        let path = Self::get_state_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    fn get_state_path() -> PathBuf {
        // Linux: ~/.config/linux-teasing/state.json
        // Windows: AppData/Roaming/LinuxTeasing/config/state.json (or similar)
        if let Some(proj_dirs) = ProjectDirs::from("", "", "linux-teasing") {
            let mut path = proj_dirs.config_dir().to_path_buf();
            path.push("state.json");
            return path;
        }
        
        // Fallback
        PathBuf::from(".config/linux-teasing/state.json")
    }
}
