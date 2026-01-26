// use iced::color; // Not used yet but available if needed

#[derive(Debug, Clone, Copy)]
pub enum Asset {
    // Images
    Background,
    MeshBackground,

    // Icons (System)
    Icon(SystemIcon),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemIcon {
    Settings,
    Search,
    Wifi,
    Battery,
    Apps,
    Library,
    Map,
    Smile,
    Folder,
    Palette,
    Document,
    Cpu,
    Cube,
    Terminal,
    Store,
    ChevronRight,
    Circle,
}

impl Asset {
    pub fn path(&self) -> String {
        match self {
            Asset::Icon(icon) => icon.path(),
            Asset::Background => {
                if cfg!(target_arch = "wasm32") {
                    "assets/background.png".to_string()
                } else {
                    "crates/peak-ui/assets/background.png".to_string()
                }
            }
            Asset::MeshBackground => {
                if cfg!(target_arch = "wasm32") {
                    "assets/mesh_bg.png".to_string()
                } else {
                    "crates/peak-ui/assets/mesh_bg.png".to_string()
                }
            }
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        SystemIcon::from_name(name).map(Asset::Icon)
    }
}

impl SystemIcon {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "settings" => Some(Self::Settings),
            "search" => Some(Self::Search),
            "wifi" | "wifi_full" => Some(Self::Wifi),
            "battery" => Some(Self::Battery),
            "sidebar" | "grid" | "apps" => Some(Self::Apps),
            "book" | "library" | "book-open" => Some(Self::Library),
            "map" => Some(Self::Map),
            "users" | "smile" => Some(Self::Smile),
            "layers" | "folder" => Some(Self::Folder),
            "palette" => Some(Self::Palette),
            "maximize" => Some(Self::Settings), // Mapping 'maximize' to settings as per core.rs
            "type" | "document" => Some(Self::Document),
            "monitor" | "cpu" => Some(Self::Cpu),
            "box" | "cube" => Some(Self::Cube),
            "terminal" => Some(Self::Terminal),
            "store" => Some(Self::Store),
            "chevron_right" => Some(Self::ChevronRight),
            _ => None, // Or Some(Self::Circle) as fallback?
        }
    }

    pub fn path(&self) -> String {
        let filename = match self {
            Self::Settings => "settings",
            Self::Search => "search",
            Self::Wifi => "wifi_full",
            Self::Battery => "battery",
            Self::Apps => "apps",
            Self::Library => "library",
            Self::Map => "map",
            Self::Smile => "smile",
            Self::Folder => "folder",
            Self::Palette => "palette",
            Self::Document => "document",
            Self::Cpu => "cpu",
            Self::Cube => "cube",
            Self::Terminal => "terminal",
            Self::Store => "store",
            // Fallback/Special cases that might not have files yet,
            // ensure these exist or map to something generic
            Self::ChevronRight | Self::Circle => "circle",
        };

        if cfg!(target_arch = "wasm32") {
            // Logic from core.rs was: checks luminance to pick white/black folder.
            // We can't easily check luminance here without passing context/color.
            // For now, we might default to one or accept a "variant" (Light/Dark).
            // However, the prompt asked to abstract "assets/".
            // Let's stick to returning the relative path, and maybe the Caller handles color-folder choice?
            // Actually, the prompt example was `Asset::Background => "assets/background.png".into()`.
            // But for icons, `core.rs` had logic to choose "white" or "black" folder based on color.
            // That logic relied on `color` passed to `icon`.
            // `Asset::path()` doesn't take color.

            // Strategy: Return the path *template* or just the logical path,
            // and let the backend resolve the exact file if it needs context?
            // OR, update `path()` to accept `is_dark_mode: bool`.

            // Simplification: Assume "system/ui" for complex icons and "menubar" for others?
            // The core.rs logic distinguished "menubar" icons (search, settings, etc) from "system/ui".

            let is_menubar_icon = matches!(
                self,
                Self::Search
                    | Self::Settings
                    | Self::Wifi
                    | Self::Battery
                    | Self::Library
                    | Self::Store
                    | Self::Terminal
            );

            if is_menubar_icon {
                // Defaulting to "black" folder for simplicity if we lack context,
                // but ideally we'd want to know.
                // Let's return a generic path that the Backend can fix up?
                // Or better: Just match the structure.
                format!("assets/icons/menubar/black/{}.svg", filename)
            } else {
                format!("assets/icons/system/ui/{}.svg", filename)
            }
        } else {
            // Desktop doesn't use these specific paths usually, it uses peak_icons::get_ui_icon
            // But if we want to support file loading on desktop too:
            format!("assets/icons/{}.svg", filename)
        }
    }
}
