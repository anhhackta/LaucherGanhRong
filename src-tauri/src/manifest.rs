use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;
use crate::paths;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewsItem {
    pub title: String,
    pub image: String,
    pub date: String,
    #[serde(default)]
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameManifest {
    pub game_name: String,
    #[serde(default)]
    pub game_exe: Option<String>,  // Name of the game executable
    pub latest_version: String,
    pub game_zip: String,
    pub checksum: String,
    #[serde(default)]
    pub server_status: Option<String>,  // "online", "maintenance", "closed"
    #[serde(default)]
    pub maintenance_message: Option<String>,
    pub backgrounds: Vec<String>,
    #[serde(default)]
    pub sidebar_links: Option<HashMap<String, String>>,
    pub news: Vec<NewsItem>,
    pub languages: Vec<String>,
}

pub async fn fetch_manifest(url: &str) -> Result<GameManifest, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client.get(url).send().await?;
    let manifest: GameManifest = resp.json().await?;
    
    // Save to cache
    save_manifest_cache(&manifest);
    
    Ok(manifest)
}

pub fn load_cached_manifest() -> Option<GameManifest> {
    let cache_dir = paths::get_cache_dir();
    let path = cache_dir.join("manifest.json");
    if path.exists() {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(manifest) = serde_json::from_str(&content) {
                return Some(manifest);
            }
        }
    }
    None
}

fn save_manifest_cache(manifest: &GameManifest) {
    let cache_dir = paths::get_cache_dir();
    let _ = fs::create_dir_all(&cache_dir);
    let path = cache_dir.join("manifest.json");
    if let Ok(content) = serde_json::to_string_pretty(manifest) {
        let _ = fs::write(path, content);
    }
}

