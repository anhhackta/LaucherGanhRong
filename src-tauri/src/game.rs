use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn get_local_version() -> String {
    let path = PathBuf::from("game/version.txt");
    if path.exists() {
        if let Ok(content) = fs::read_to_string(path) {
            return content.trim().to_string();
        }
    }
    "0.0.0".to_string()
}

pub fn launch_game(exe_name: &str) -> Result<(), String> {
    let exe_path = PathBuf::from("game").join(exe_name);
    
    if !exe_path.exists() {
        return Err("Game executable not found".to_string());
    }

    // Launch detached
    Command::new(exe_path)
        .current_dir("game")
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}
