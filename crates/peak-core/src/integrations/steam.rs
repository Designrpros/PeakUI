#[cfg(feature = "native")]
use regex::Regex;
#[cfg(feature = "native")]
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SteamGame {
    pub app_id: String,
    pub name: String,
    pub install_dir: PathBuf,
}

pub struct SteamScanner;

impl SteamScanner {
    // 1. Locate the Steam Directory based on OS
    #[cfg(feature = "native")]
    fn locate_steam_dir() -> Option<PathBuf> {
        let home = dirs::home_dir()?;

        // This is a rough heuristic. A production app might check registry (Windows) or more paths.
        let paths = vec![
            // MacOS
            home.join("Library/Application Support/Steam/steamapps"),
            // Linux
            home.join(".local/share/Steam/steamapps"),
            home.join(".steam/steam/steamapps"),
            // Windows
            PathBuf::from(r"C:\Program Files (x86)\Steam\steamapps"),
        ];

        paths.into_iter().find(|p| p.exists())
    }

    // 2. Scan for Manifest Files
    pub fn scan() -> Vec<SteamGame> {
        #[allow(unused_mut)]
        let mut games = Vec::new();

        #[cfg(feature = "native")]
        if let Some(steam_apps) = Self::locate_steam_dir() {
            println!("✅ Found Steam Library at: {:?}", steam_apps);

            // Regex to grab "appid" and "name" from the VDF file
            let re_id = Regex::new(r#""appid"\s+"(\d+)""#).unwrap();
            let re_name = Regex::new(r#""name"\s+"([^"]+)""#).unwrap();

            if let Ok(entries) = fs::read_dir(steam_apps) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    // Look for appmanifest_12345.acf
                    if let Some(ext) = path.extension() {
                        if ext == "acf" {
                            if let Ok(content) = fs::read_to_string(&path) {
                                // Extract Data
                                let id = re_id.captures(&content).map(|c| c[1].to_string());
                                let name = re_name.captures(&content).map(|c| c[1].to_string());

                                if let (Some(id), Some(name)) = (id, name) {
                                    // Filter out "Steamworks Common Redistributables" (App ID 228980)
                                    if id != "228980" {
                                        games.push(SteamGame {
                                            app_id: id,
                                            name,
                                            install_dir: path.clone(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            println!("⚠️ Could not locate Steam directory.");
        }

        games
    }

    // 3. Launch Protocol
    #[allow(dead_code)]
    pub fn launch(app_id: &str) {
        #[cfg(feature = "native")]
        {
            let uri = format!("steam://run/{}", app_id);
            let _ = opener::open(&uri);
        }

        #[cfg(not(feature = "native"))]
        let _ = app_id;
    }
}
