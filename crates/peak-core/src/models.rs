#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum MediaKind {
    #[default]
    Game,
    Movie,
    Music,
    App,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MediaItem {
    pub id: String,
    pub title: String,
    pub cover_image: String,    // Path or Resource ID
    pub launch_command: String, // e.g. "steam steam://run/1091500"
    pub kind: MediaKind,
    pub status: MediaStatus,
    pub image_handle: Option<iced::widget::image::Handle>,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum MediaStatus {
    Ready,
    #[allow(dead_code)]
    Running,
    Updating(f32), // Progress 0.0 - 1.0
}

use crate::integrations::steam::SteamScanner;

impl MediaItem {
    pub fn scan_system() -> Vec<Self> {
        let mut library = Vec::new();

        // 1. Scan Steam
        let steam_games = SteamScanner::scan();
        for s in steam_games {
            library.push(MediaItem {
                id: s.app_id.clone(),
                title: s.name,
                // Official Steam CDN URL format
                cover_image: format!(
                    "https://steamcdn-a.akamaihd.net/steam/apps/{}/library_600x900.jpg",
                    s.app_id
                ),
                launch_command: s.app_id,
                kind: MediaKind::Game,
                status: MediaStatus::Ready,
                image_handle: None,
            });
        }

        // 2. Scan Apps (Stremio/VLC)
        #[cfg(not(target_arch = "wasm32"))]
        library.extend(crate::systems::scanner::AppScanner::scan());

        // 3. Scan Music
        #[cfg(not(target_arch = "wasm32"))]
        library.extend(crate::systems::scanner::MusicScanner::scan());

        // If empty, maybe fallback to mock? Or just return empty.
        if library.is_empty() {
            // Mock fallback only for games? Or keep empty?
            // Let's keep empty or minimal mock if needed
            // return Self::mock_library();
        }

        library
    }

    #[allow(dead_code)]
    pub fn mock_library() -> Vec<Self> {
        vec![
            MediaItem {
                id: "cp2077".into(),
                title: "Cyberpunk 2077".into(),
                cover_image: "cp2077.jpg".into(),
                launch_command: "1091500".into(),
                kind: MediaKind::Game,
                status: MediaStatus::Ready,
                image_handle: None,
            },
            MediaItem {
                id: "elden".into(),
                title: "Elden Ring".into(),
                cover_image: "eldenring.jpg".into(),
                launch_command: "1245620".into(),
                kind: MediaKind::Game,
                status: MediaStatus::Ready,
                image_handle: None,
            },
            MediaItem {
                id: "stardew".into(),
                title: "Stardew Valley".into(),
                cover_image: "stardew.jpg".into(),
                launch_command: "413150".into(),
                kind: MediaKind::Game,
                status: MediaStatus::Updating(0.45),
                image_handle: None,
            },
            MediaItem {
                id: "apex".into(),
                title: "Apex Legends".into(),
                cover_image: "apex.jpg".into(),
                launch_command: "1172470".into(),
                kind: MediaKind::Game,
                status: MediaStatus::Ready,
                image_handle: None,
            },
        ]
    }
}
