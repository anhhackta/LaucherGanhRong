use futures_util::StreamExt;
use reqwest::Client;
use std::cmp::min;
use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::path::{Path, PathBuf};
use zip::ZipArchive;
use sha2::{Sha256, Digest};
use tauri::{AppHandle, Emitter};

// Event for frontend progress
#[derive(Clone, serde::Serialize)]
struct DownloadProgressPayload {
    progress: f32, // 0.0 to 100.0
    status: String,
}

pub async fn download_and_install_game<F>(
    url: &str, 
    checksum: &str,
    version: &str,
    app: AppHandle,
    progress_callback: F
) -> Result<(), Box<dyn std::error::Error>> 
where F: Fn(f32, String) + Send + Sync + 'static 
{
    let target_path = PathBuf::from("cache/game.tmp.zip");
    
    // 1. Download
    progress_callback(0.0, "Downloading...".to_string());
    
    let client = Client::new();
    let res = client.get(url).send().await?;
    let total_size = res.content_length().unwrap_or(0);
    
    let mut file = File::create(&target_path)?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        downloaded = min(downloaded + (chunk.len() as u64), total_size);
        
        if total_size > 0 {
            let p = (downloaded as f32 / total_size as f32) * 100.0;
            app.emit("download-progress", DownloadProgressPayload { progress: p, status: "Downloading".to_string() })?;
        }
    }

    // 2. Verify
    progress_callback(100.0, "Verifying...".to_string());
    app.emit("download-progress", DownloadProgressPayload { progress: 100.0, status: "Verifying".to_string() })?;
    
    if !verify_hash(&target_path, checksum)? {
        return Err("Checksum verification failed".into());
    }

    // 3. Extract (Atomic-ish)
    progress_callback(100.0, "Installing...".to_string());
    app.emit("download-progress", DownloadProgressPayload { progress: 100.0, status: "Installing".to_string() })?;
    
    let extract_path = PathBuf::from("cache/extracted_tmp");
    if extract_path.exists() {
        fs::remove_dir_all(&extract_path)?;
    }
    fs::create_dir_all(&extract_path)?;

    extract_zip(&target_path, &extract_path)?;

    // Write version.txt BEFORE moving
    fs::write(extract_path.join("version.txt"), version)?;

    // 4. Move to game/
    let game_path = PathBuf::from("game");
    if game_path.exists() {
        fs::remove_dir_all(&game_path)?;
    }
    
    // Rename/Move
    // Note: rename only works on same filesystem. 
    // Since cache and game are likely on same drive, this is usually atomic or fast.
    fs::rename(extract_path, &game_path)?;
    
    // Write installed version provided by manifest logic elsewhere, 
    // but here we just ensure the files are there.
    // The main logic will write version.txt after this succeeds.

    // Cleanup
    let _ = fs::remove_file(target_path);

    Ok(())
}

fn verify_hash(path: &Path, expected_hash_prefix: &str) -> Result<bool, std::io::Error> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    let hash_str = hex::encode(hash);
    
    // Compare prefix (e.g. "sha256:XXXX" or just "XXXX")
    // If manifest has "sha256:", strip it.
    let expected = if expected_hash_prefix.starts_with("sha256:") {
        &expected_hash_prefix[7..]
    } else {
        expected_hash_prefix
    };

    Ok(hash_str == expected) // Exact match or full hash? Usually full hash.
}

fn extract_zip(zip_path: &Path, target_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(zip_path)?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => target_dir.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}
