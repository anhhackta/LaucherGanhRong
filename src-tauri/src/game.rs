use std::fs;
use std::process::Command;
use crate::paths;

pub fn get_local_version() -> String {
    let path = paths::get_game_dir().join("version.txt");
    if path.exists() {
        if let Ok(content) = fs::read_to_string(path) {
            return content.trim().to_string();
        }
    }
    "0.0.0".to_string()
}

pub fn launch_game(exe_name: &str) -> Result<(), String> {
    let game_dir = paths::get_game_dir();
    let exe_path = game_dir.join(exe_name);
    
    if !exe_path.exists() {
        return Err(format!("Game executable not found at {:?}", exe_path));
    }

    // Launch detached
    Command::new(&exe_path)
        .current_dir(&game_dir)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}
