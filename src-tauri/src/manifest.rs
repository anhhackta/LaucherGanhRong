use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewsItem {
    pub title: String,
    pub image: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameManifest {
    pub game_name: String,
    pub latest_version: String,
    pub game_zip: String,
    pub checksum: String,
    pub backgrounds: Vec<String>,
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
    let path = PathBuf::from("cache/manifest.json");
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
    let path = PathBuf::from("cache/manifest.json");
    // ensure cache dir exists
    let _ = fs::create_dir_all("cache");
    if let Ok(content) = serde_json::to_string_pretty(manifest) {
        let _ = fs::write(path, content);
    }
}
