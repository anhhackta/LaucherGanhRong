use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CloseBehavior {
    MinimizeToTray,
    Exit,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LauncherConfig {
    pub language: String,
    pub close_behavior: CloseBehavior,
    pub launch_at_startup: bool,
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            close_behavior: CloseBehavior::MinimizeToTray,
            launch_at_startup: false,
        }
    }
}

impl LauncherConfig {
    pub fn load() -> Self {
        let config_path = PathBuf::from("config.json");
        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(config_path) {
                if let Ok(config) = serde_json::from_str(&content) {
                    return config;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        let config_path = PathBuf::from("config.json");
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = fs::write(config_path, content);
        }
    }
}
