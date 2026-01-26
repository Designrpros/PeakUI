#![allow(dead_code)]
use crate::models::{MediaItem, MediaKind, MediaStatus};
use std::path::Path;

pub struct AppScanner;

impl AppScanner {
    pub fn scan() -> Vec<MediaItem> {
        let mut apps = Vec::new();

        // Scan for system binaries (Alpine/Linux apps)
        apps.extend(Self::scan_system_binaries());

        // Scan for macOS apps (development environment)
        apps.extend(Self::scan_macos_apps());

        apps
    }

    fn scan_macos_apps() -> Vec<MediaItem> {
        #[cfg_attr(target_arch = "wasm32", allow(unused_mut))]
        let mut apps = Vec::new();

        #[cfg(feature = "native")]
        {
            // Check if /Applications exists (macOS only)
            let app_dir = Path::new("/Applications");
            if !app_dir.exists() {
                return apps;
            }

            // Scan all .app bundles in /Applications
            if let Ok(entries) = std::fs::read_dir(app_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(ext) = path.extension() {
                        if ext == "app" {
                            if let Some(app_name) = path.file_stem() {
                                let name = app_name.to_string_lossy().to_string();
                                let id = name.to_lowercase().replace(' ', "_");

                                apps.push(MediaItem {
                                    id,
                                    title: name.clone(),
                                    cover_image: format!("{}_icon.png", name.to_lowercase()),
                                    launch_command: format!("open \"{}\"", path.display()),
                                    kind: MediaKind::App,
                                    status: MediaStatus::Ready,
                                    image_handle: None,
                                });
                            }
                        }
                    }
                }
            }
        }

        apps
    }

    fn scan_system_binaries() -> Vec<MediaItem> {
        #[cfg_attr(target_arch = "wasm32", allow(unused_mut))]
        let mut apps = Vec::new();

        #[cfg(feature = "native")]
        {
            // 1. Scan for standard Linux .desktop files
            let desktop_paths = [
                "/usr/share/applications",
                "/usr/local/share/applications",
                "/var/lib/flatpak/exports/share/applications", // Flatpak system
            ];

            for dir in desktop_paths {
                let path = Path::new(dir);
                if !path.exists() {
                    continue;
                }

                if let Ok(entries) = std::fs::read_dir(path) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().map_or(false, |ext| ext == "desktop") {
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                let mut name = None;
                                let mut exec = None;
                                let mut icon = None;
                                let mut no_display = false;

                                for line in content.lines() {
                                    if line.starts_with("Name=") && name.is_none() {
                                        name = Some(line.trim_start_matches("Name=").to_string());
                                    } else if line.starts_with("Exec=") && exec.is_none() {
                                        // Remove special params like %u, %f
                                        let full_exec = line.trim_start_matches("Exec=");
                                        let clean_exec =
                                            full_exec.split(' ').next().unwrap_or(full_exec);
                                        exec = Some(clean_exec.to_string());
                                    } else if line.starts_with("Icon=") && icon.is_none() {
                                        icon = Some(line.trim_start_matches("Icon=").to_string());
                                    } else if line == "NoDisplay=true" {
                                        no_display = true;
                                    }
                                }

                                if let (Some(n), Some(e)) = (name, exec) {
                                    if !no_display {
                                        apps.push(MediaItem {
                                            id: e.clone(),
                                            title: n,
                                            cover_image: icon
                                                .unwrap_or_else(|| "app_icon".to_string()),
                                            launch_command: e,
                                            kind: MediaKind::App,
                                            status: MediaStatus::Ready,
                                            image_handle: None,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 2. Fallback to known apps if grid is still very sparse (debug/mock)
            if apps.is_empty() {
                let known_apps = [
                    ("chromium", "Chromium", "Open source web browser."),
                    ("firefox", "Firefox", "Privacy-focused browser."),
                    ("gimp", "GIMP", "GNU Image Manipulation Program."),
                ];

                for (binary, name, _description) in known_apps {
                    if Self::check_binary_installed(binary) {
                        apps.push(MediaItem {
                            id: binary.to_string(),
                            title: name.to_string(),
                            cover_image: format!("{}_icon.png", binary),
                            launch_command: binary.to_string(),
                            kind: MediaKind::App,
                            status: MediaStatus::Ready,
                            image_handle: None,
                        });
                    }
                }
            }
        }

        apps
    }

    fn check_binary_installed(binary: &str) -> bool {
        #[cfg(feature = "native")]
        {
            // Check if binary exists in PATH
            std::process::Command::new("which")
                .arg(binary)
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }

        #[cfg(not(feature = "native"))]
        {
            let _ = binary;
            false
        }
    }
}

pub struct MusicScanner;

impl MusicScanner {
    pub fn scan() -> Vec<MediaItem> {
        #[cfg_attr(target_arch = "wasm32", allow(unused_mut))]
        let mut tracks = Vec::new();
        #[cfg(feature = "native")]
        {
            if let Some(user_dirs) = directories::UserDirs::new() {
                if let Some(audio_dir) = user_dirs.audio_dir() {
                    // Initial simple scan: just list files
                    // Real impl: Use `walkdir`
                    if let Ok(entries) = std::fs::read_dir(audio_dir) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if let Some(ext) = path.extension() {
                                if ext == "mp3" || ext == "flac" || ext == "wav" {
                                    let filename = path.file_stem().unwrap().to_string_lossy();
                                    // Basic cover art assumption? or default icon
                                    tracks.push(MediaItem {
                                        id: filename.to_string(),
                                        title: filename.to_string(),
                                        cover_image: "music_icon.png".into(),
                                        launch_command: format!("play \"{}\"", path.display()), // Needs specialized logic
                                        kind: MediaKind::Music,
                                        status: MediaStatus::Ready,
                                        image_handle: None,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        tracks
    }
}
