use std::path::PathBuf;

/// Get the base directory where the app is running from
/// This returns the directory containing the executable
pub fn get_app_dir() -> PathBuf {
    // Try to get the directory of the current executable
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            return parent.to_path_buf();
        }
    }
    // Fallback to current directory
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Get path to the game directory
pub fn get_game_dir() -> PathBuf {
    get_app_dir().join("game")
}

/// Get path to the cache directory
pub fn get_cache_dir() -> PathBuf {
    get_app_dir().join("cache")
}
