use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::config::LauncherConfig;
use crate::manifest::GameManifest;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum GameStatus {
    ReadyToPlay,
    UpdateAvailable,
    Downloading(f32), // 0.0 to 100.0
    Updating,
    Checking,
    Offline,
    Missing,
}

pub struct AppState {
    pub config: Mutex<LauncherConfig>,
    pub manifest: Mutex<Option<GameManifest>>,
    pub status: Mutex<GameStatus>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: Mutex::new(LauncherConfig::default()),
            manifest: Mutex::new(None),
            status: Mutex::new(GameStatus::Checking),
        }
    }
}
