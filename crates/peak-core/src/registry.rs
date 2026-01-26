#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AppCategoryPrimary {
    Productivity,
    Development,
    Creative,
    WebBrowsers,
    MediaEntertainment,
    Communication,
    Gaming,
    Utilities,
    Education,
    Finance,
    System,
    ScienceTech,
    SecurityPrivacy,
    Lifestyle,
    Business,
}

impl AppCategoryPrimary {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Productivity => "Productivity",
            Self::Development => "Development",
            Self::Creative => "Creative",
            Self::WebBrowsers => "Web & Browsers",
            Self::MediaEntertainment => "Media & Entertainment",
            Self::Communication => "Communication",
            Self::Gaming => "Gaming",
            Self::Utilities => "Utilities",
            Self::Education => "Education",
            Self::Finance => "Finance",
            Self::System => "System",
            Self::ScienceTech => "Science & Tech",
            Self::SecurityPrivacy => "Security & Privacy",
            Self::Lifestyle => "Lifestyle",
            Self::Business => "Business",
        }
    }

    pub fn icon_name(&self) -> &'static str {
        match self {
            Self::Productivity => "productivity",
            Self::Development => "development",
            Self::Creative => "creative",
            Self::WebBrowsers => "web",
            Self::MediaEntertainment => "media",
            Self::Communication => "communication",
            Self::Gaming => "gaming",
            Self::Utilities => "utilities",
            Self::Education => "education",
            Self::Finance => "finance",
            Self::System => "system",
            Self::ScienceTech => "science",
            Self::SecurityPrivacy => "security",
            Self::Lifestyle => "lifestyle",
            Self::Business => "business",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum InstallSource {
    System,
    Apk,
    AppImage,
    Steam,
    Flatpak,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IconConfiguration {
    pub logo: Option<String>,        // PNG logo (brand specific)
    pub brand_icon: Option<String>,  // Simple Icons SVG name
    pub system_icon: Option<String>, // Peak system SVG icon
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppMetadata {
    pub id: AppId,
    pub name: &'static str,
    pub description: &'static str,
    pub category: AppCategoryPrimary,
    pub source: InstallSource,
    pub icons: IconConfiguration,
    pub accent_color: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ShellMode {
    Desktop,
    Auto,
    Console,
    Fireplace,
    Kiosk,
    Mobile,
    Robot,
    Server,
    SmartHome,
    TV,
}

impl std::fmt::Display for ShellMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellMode::Desktop => write!(f, "Desktop"),
            ShellMode::Auto => write!(f, "Auto"),
            ShellMode::Console => write!(f, "Console"),
            ShellMode::Fireplace => write!(f, "Fireplace"),
            ShellMode::Kiosk => write!(f, "Kiosk"),
            ShellMode::Mobile => write!(f, "Mobile"),
            ShellMode::Robot => write!(f, "Robot"),
            ShellMode::Server => write!(f, "Server"),
            ShellMode::SmartHome => write!(f, "Home"),
            ShellMode::TV => write!(f, "TV"),
        }
    }
}

/// Shell visual style - determines the layout of dock, menubar, and window decorations
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize,
)]
pub enum ShellStyle {
    #[default]
    Cupertino, // macOS-style: Top menubar, floating bottom dock
    Redmond, // Windows 10-style: Bottom taskbar with left-aligned Start menu
    AI,      // AI OS-style: Top dock, wrapped wallpaper, bottom AI input
    Console, // PS5-style: Horizontal game carousel, categories at top
    TV,      // Apple TV-style: Large app grid, control center
}

impl std::fmt::Display for ShellStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellStyle::Cupertino => write!(f, "Cupertino"),
            ShellStyle::Redmond => write!(f, "Redmond"),
            ShellStyle::AI => write!(f, "AI"),
            ShellStyle::Console => write!(f, "Console"),
            ShellStyle::TV => write!(f, "TV"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AppId {
    Terminal,
    PeakUI,  // The PeakUI Reference App
    Browser, // The Tauri Guest
    Library, // Internal View
    Cortex,  // Internal View
    Settings,
    FileManager,
    Store,
    AppGrid, // The Launchpad / App Library
    Editor,
    Desktop,
    #[allow(dead_code)]
    Spotify,

    // Media Apps
    Turntable,
}

impl AppId {
    pub fn is_repo(&self) -> bool {
        matches!(self, AppId::Desktop | AppId::FileManager)
    }

    pub fn metadata(&self) -> AppMetadata {
        match self {
            AppId::Terminal => AppMetadata {
                id: *self,
                name: "Terminal",
                description: "Deep integrated command line interface for PeakOS.",
                category: AppCategoryPrimary::Development,
                source: InstallSource::System,
                icons: IconConfiguration {
                    logo: Some("Terminal.png".to_string()),
                    brand_icon: None,
                    system_icon: Some("terminal".to_string()),
                },
                accent_color: "#00FF88",
            },
            AppId::PeakUI => AppMetadata {
                id: *self,
                name: "PeakUI",
                description: "The definitive reference and showcase for the PeakUI design system.",
                category: AppCategoryPrimary::Development,
                source: InstallSource::System,
                icons: IconConfiguration {
                    logo: None,
                    brand_icon: None,
                    system_icon: Some("design".to_string()),
                },
                accent_color: "#FF0055",
            },
            AppId::Browser => AppMetadata {
                id: *self,
                name: "Netscape",
                description: "Web browser designed for privacy and speed.",
                category: AppCategoryPrimary::WebBrowsers,
                source: InstallSource::System,
                icons: IconConfiguration {
                    logo: Some("Browser.png".to_string()),
                    brand_icon: Some("firefox".to_string()),
                    system_icon: Some("web".to_string()),
                },
                accent_color: "#0066FF",
            },
            AppId::Library => AppMetadata {
                id: *self,
                name: "Arcade",
                description: "Your local game and media library.",
                category: AppCategoryPrimary::Gaming,
                source: InstallSource::System,
                icons: IconConfiguration {
                    logo: Some("Development.png".to_string()),
                    brand_icon: Some("steam".to_string()),
                    system_icon: Some("gaming".to_string()),
                },
                accent_color: "#FF0099",
            },
            AppId::Cortex => AppMetadata {
                id: *self,
                name: "Neural Link",
                description: "Local AI intelligence and reasoning engine.",
                category: AppCategoryPrimary::ScienceTech,
                source: InstallSource::System,
                icons: IconConfiguration {
                    logo: Some("cortex_ai_512.png".to_string()),
                    brand_icon: None,
                    system_icon: Some("science".to_string()),
                },
                accent_color: "#00FFFF",
            },
            AppId::Settings => AppMetadata {
                id: *self,
                name: "Settings",
                description: "Configure system preferences and appearance.",
                category: AppCategoryPrimary::System,
                source: InstallSource::System,
                icons: IconConfiguration {
                    logo: Some("Settings.png".to_string()),
                    brand_icon: None,
                    system_icon: Some("system".to_string()),
                },
                accent_color: "#445566",
            },
            AppId::FileManager => AppMetadata {
                id: *self,
                name: "Files",
                description: "The official PeakOS file explorer.",
                category: AppCategoryPrimary::Productivity,
                source: InstallSource::System,
                icons: IconConfiguration {
                    logo: Some("Files.png".to_string()),
                    brand_icon: None,
                    system_icon: Some("productivity".to_string()),
                },
                accent_color: "#0099DD",
            },
            AppId::Store => AppMetadata {
                id: *self,
                name: "Store",
                description: "Discover and install new applications.",
                category: AppCategoryPrimary::Business,
                source: InstallSource::System,
                icons: IconConfiguration {
                    logo: Some("Store.png".to_string()),
                    brand_icon: None,
                    system_icon: Some("business".to_string()),
                },
                accent_color: "#9966FF",
            },
            AppId::AppGrid => AppMetadata {
                id: *self,
                name: "Apps",
                description: "Quickly browse and search your apps.",
                category: AppCategoryPrimary::Utilities,
                source: InstallSource::System,
                icons: IconConfiguration {
                    logo: Some("Apps.png".to_string()),
                    brand_icon: None,
                    system_icon: Some("utilities".to_string()),
                },
                accent_color: "#FF7700",
            },
            AppId::Editor => AppMetadata {
                id: *self,
                name: "Text Editor",
                description: "Clean and powerful code and text editor.",
                category: AppCategoryPrimary::Development,
                source: InstallSource::System,
                icons: IconConfiguration {
                    logo: Some("Editor.png".to_string()),
                    brand_icon: Some("vscode".to_string()),
                    system_icon: Some("development".to_string()),
                },
                accent_color: "#AA66FF",
            },
            AppId::Desktop => AppMetadata {
                id: *self,
                name: "Desktop",
                description: "Manage your workspaces and window layouts.",
                category: AppCategoryPrimary::System,
                source: InstallSource::System,
                icons: IconConfiguration {
                    logo: Some("desktop_workspace_512.png".to_string()),
                    brand_icon: None,
                    system_icon: Some("system".to_string()),
                },
                accent_color: "#8B6F47",
            },
            AppId::Turntable => AppMetadata {
                id: *self,
                name: "Jukebox",
                description: "High-fidelity music playback and management.",
                category: AppCategoryPrimary::MediaEntertainment,
                source: InstallSource::System,
                icons: IconConfiguration {
                    logo: Some("Jukebox.png".to_string()),
                    brand_icon: Some("spotify".to_string()),
                    system_icon: Some("media".to_string()),
                },
                accent_color: "#9966FF",
            },
            AppId::Spotify => AppMetadata {
                id: *self,
                name: "Spotify",
                description: "Streaming music for everyone.",
                category: AppCategoryPrimary::MediaEntertainment,
                source: InstallSource::Steam, // Example, typically Web or specialized
                icons: IconConfiguration {
                    logo: None,
                    brand_icon: Some("spotify".to_string()),
                    system_icon: Some("media".to_string()),
                },
                accent_color: "#1DB954",
            },
        }
    }
}

impl std::fmt::Display for AppId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for AppId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Terminal" => Ok(AppId::Terminal),
            "PeakUI" => Ok(AppId::PeakUI),
            "Browser" => Ok(AppId::Browser),
            "Library" => Ok(AppId::Library),
            "Cortex" => Ok(AppId::Cortex),
            "Settings" => Ok(AppId::Settings),
            "FileManager" => Ok(AppId::FileManager),
            "Store" => Ok(AppId::Store),
            "AppGrid" => Ok(AppId::AppGrid),
            "Editor" => Ok(AppId::Editor),
            "Desktop" => Ok(AppId::Desktop),
            "Spotify" => Ok(AppId::Spotify),
            "Turntable" => Ok(AppId::Turntable),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppInfo {
    pub id: AppId,
    pub name: &'static str,
}

impl AppInfo {
    pub fn get_info(id: AppId) -> Self {
        let name = match id {
            AppId::Terminal => "Terminal",
            AppId::PeakUI => "PeakUI",
            AppId::Browser => "Netscape",
            AppId::Library => "Arcade",
            AppId::Cortex => "Neural Link",
            AppId::Settings => "Settings",
            AppId::FileManager => "Files",
            AppId::Store => "Store",
            AppId::AppGrid => "Apps",
            AppId::Editor => "Text Editor",
            AppId::Desktop => "Desktop",
            AppId::Spotify => "Spotify",
            AppId::Turntable => "Jukebox",
        };
        Self { id, name }
    }

    #[allow(dead_code)]
    pub fn dock() -> Vec<Self> {
        vec![
            AppId::Terminal,
            AppId::PeakUI,
            AppId::Browser,
            AppId::Turntable,
            AppId::Library,
            AppId::FileManager,
            AppId::Store,
            AppId::Settings,
            AppId::AppGrid,
        ]
        .into_iter()
        .map(Self::get_info)
        .collect()
    }

    pub fn all() -> Vec<Self> {
        vec![
            AppId::Terminal,
            AppId::PeakUI,
            AppId::Browser,
            AppId::Library,
            AppId::Cortex,
            AppId::Settings,
            AppId::FileManager,
            AppId::Store,
            AppId::AppGrid,
            AppId::Editor,
            AppId::Desktop,
            AppId::Turntable,
        ]
        .into_iter()
        .map(Self::get_info)
        .collect()
    }

    pub fn all_as_media() -> Vec<crate::models::MediaItem> {
        Self::all()
            .into_iter()
            .map(|info| crate::models::MediaItem {
                id: info.id.to_string(),
                title: info.name.to_string(),
                cover_image: format!("{}_icon.png", info.id.to_string().to_lowercase()),
                launch_command: info.id.to_string(),
                kind: crate::models::MediaKind::App,
                status: crate::models::MediaStatus::Ready,
                image_handle: None,
            })
            .collect()
    }
}
