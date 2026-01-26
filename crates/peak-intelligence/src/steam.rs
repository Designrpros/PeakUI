use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamGame {
    pub app_id: String,
    pub name: String,
    pub install_dir: PathBuf,
    pub size_gb: f32,
    pub last_played: u64, // Unix Timestamp
}

/// The Master Scan Function
pub fn scan_library() -> Vec<SteamGame> {
    let mut games = Vec::new();

    // 1. Locate Steam Base Directory (Linux Standard)
    #[cfg(not(target_arch = "wasm32"))]
    let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
    #[cfg(target_arch = "wasm32")]
    let home = "/".to_string();

    let steam_base = PathBuf::from(format!("{}/.steam/steam", home));

    // 2. Find Library Folders (Valve Logic)
    // Steam can have games across multiple drives. We check the default first.
    let library_folders = vec![
        steam_base.join("steamapps"),
        // Add logic here to parse libraryfolders.vdf for external drives if needed
    ];

    // 3. Iterate all libraries
    for lib_path in library_folders {
        if let Ok(entries) = fs::read_dir(lib_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                // We are looking for "appmanifest_12345.acf"
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if file_name.starts_with("appmanifest_") && file_name.ends_with(".acf") {
                        if let Some(game) = parse_manifest(&path) {
                            games.push(game);
                        }
                    }
                }
            }
        }
    }

    // Sort by Last Played (Most recent first)
    games.sort_by(|a, b| b.last_played.cmp(&a.last_played));
    games
}

/// The VDF/ACF Parser (Simple Text Parsing)
fn parse_manifest(path: &Path) -> Option<SteamGame> {
    let content = fs::read_to_string(path).ok()?;

    let mut app_id = String::new();
    let mut name = String::new();
    let mut install_dir = String::new();
    let mut last_played = 0;

    // Rough line-by-line parsing (Robust enough for Valve's format)
    for line in content.lines() {
        let clean = line.trim().replace("\"", "");
        let parts: Vec<&str> = clean.split_whitespace().collect();

        if parts.len() >= 2 {
            match parts[0] {
                "appid" => app_id = parts[1].to_string(),
                "name" => name = clean.split_once("name").unwrap().1.trim().to_string(), // Handle spaces in names
                "installdir" => install_dir = parts[1].to_string(),
                "LastPlayed" => last_played = parts[1].parse::<u64>().unwrap_or(0),
                _ => {}
            }
        }
    }

    if !app_id.is_empty() && !name.is_empty() {
        Some(SteamGame {
            app_id,
            name,
            install_dir: PathBuf::from(install_dir),
            size_gb: 0.0, // Calculate directory size later if needed
            last_played,
        })
    } else {
        None
    }
}
