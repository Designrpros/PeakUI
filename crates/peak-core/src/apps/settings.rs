use crate::app_traits::{PeakApp, ShellContext};
use crate::theme::Theme;
use iced::{Element, Task};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl std::fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeMode::Light => write!(f, "Light"),
            ThemeMode::Dark => write!(f, "Dark"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsTab {
    Sidebar,
    General,
    WiFi,
    Bluetooth,
    Battery,
    Appearance,
    Display,
    Sound,
    Focus,
    Privacy,
    Intelligence,
    Modes,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModelInfo {
    pub name: String,
    pub id: String,
    pub description: String,
    pub size_estimate: String,
    pub min_ram_gb: u8,
    pub is_downloaded: bool,
    pub is_active: bool,
    pub download_progress: Option<f32>, // None = not downloading, Some(0.0-1.0) = progress
    pub last_error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ThemeChanged(ThemeMode),
    VolumeChanged(f32),
    TabChanged(SettingsTab),
    SearchChanged(String),
    ToggleWiFi(bool),
    ToggleBluetooth(bool),
    WallpaperChanged(String),
    // Intelligence
    ModelDownload(String),
    ModelDownloadProgress(String, f32),
    ModelDownloadComplete(String),
    ModelDownloadFailed(String, String),
    ModelDownloadCancel(String),
    ModelRemove(String),
    ModelActivate(String),
    AddModelInputChanged(String),
    AddModelPressed,
    ToggleCaptions(bool),
    ToggleVoice(bool),
    ModeChanged(crate::registry::ShellMode),
    ShellStyleChanged(crate::registry::ShellStyle),
}

#[derive(Debug, Clone)]
pub struct SettingsApp {
    pub theme_mode: ThemeMode,
    pub current_tab: SettingsTab,
    pub volume: f32,
    pub search_query: String,
    pub wifi_enabled: bool,
    pub bluetooth_enabled: bool,
    pub wallpapers: Vec<String>,
    pub current_wallpaper: String,
    // Intelligence
    pub recommended_models: Vec<ModelInfo>,
    pub custom_models: Vec<ModelInfo>,
    pub add_model_input: String,
    pub captions_enabled: bool,
    pub voice_enabled: bool,
    pub current_mode: crate::registry::ShellMode,
    pub current_shell_style: crate::registry::ShellStyle,
}

impl Default for SettingsApp {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsApp {
    pub fn new() -> Self {
        Self {
            theme_mode: ThemeMode::Light,
            current_tab: SettingsTab::General,
            volume: 0.8,
            search_query: String::new(),
            wifi_enabled: true,
            bluetooth_enabled: true,
            wallpapers: Vec::new(),
            current_wallpaper: String::from("mountain_sunset_warm.jpg"),
            recommended_models: vec![
                ModelInfo {
                    name: "Llama 3.2 3B".into(),
                    id: "bartowski/Llama-3.2-3B-Instruct-GGUF".into(),
                    description: "Best balance of speed and intelligence.".into(),
                    size_estimate: "~2.4 GB".into(),
                    min_ram_gb: 8,
                    is_downloaded: false,
                    is_active: false,
                    download_progress: None,
                    last_error: None,
                },
                ModelInfo {
                    name: "Gemma 3 4B".into(),
                    id: "bartowski/google_gemma-3-4b-it-GGUF".into(),
                    description: "Multimodal, large context (128k), efficient.".into(),
                    size_estimate: "~3.0 GB".into(),
                    min_ram_gb: 8,
                    is_downloaded: false,
                    is_active: false,
                    download_progress: None,
                    last_error: None,
                },
                ModelInfo {
                    name: "Qwen 2.5 7B".into(),
                    id: "Qwen/Qwen2.5-7B-Instruct-GGUF".into(),
                    description: "Superior coding and reasoning capabilities.".into(),
                    size_estimate: "~4.5 GB".into(),
                    min_ram_gb: 16,
                    is_downloaded: false,
                    is_active: false,
                    download_progress: None,
                    last_error: None,
                },
                ModelInfo {
                    name: "Ministral 3 8B".into(),
                    id: "bartowski/mistralai_Ministral-3-8B-Reasoning-2512-GGUF".into(),
                    description: "Strong edge model with vision support.".into(),
                    size_estimate: "~5.5 GB".into(),
                    min_ram_gb: 16,
                    is_downloaded: false,
                    is_active: false,
                    download_progress: None,
                    last_error: None,
                },
            ],
            custom_models: Vec::new(),
            add_model_input: String::new(),
            captions_enabled: false,
            voice_enabled: false,
            current_mode: crate::registry::ShellMode::Desktop,
            current_shell_style: crate::registry::ShellStyle::default(),
        }
    }
}

impl PeakApp for SettingsApp {
    type Message = SettingsMessage;

    fn title(&self) -> String {
        String::from("Settings")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _context: &dyn ShellContext,
    ) -> Task<Self::Message> {
        match message {
            SettingsMessage::ThemeChanged(mode) => {
                self.theme_mode = mode;
            }
            SettingsMessage::VolumeChanged(v) => {
                self.volume = v;
            }
            SettingsMessage::TabChanged(tab) => {
                self.current_tab = tab;
            }
            SettingsMessage::SearchChanged(q) => {
                self.search_query = q;
            }
            SettingsMessage::ToggleWiFi(enabled) => {
                self.wifi_enabled = enabled;
            }
            SettingsMessage::ToggleBluetooth(enabled) => {
                self.bluetooth_enabled = enabled;
            }
            SettingsMessage::WallpaperChanged(path) => {
                self.current_wallpaper = path;
            }
            SettingsMessage::ModelDownload(_) => {
                // Initiated in PeakNative
            }
            SettingsMessage::ModelDownloadProgress(id, progress) => {
                for model in self
                    .recommended_models
                    .iter_mut()
                    .chain(self.custom_models.iter_mut())
                {
                    if model.id == id {
                        model.download_progress = Some(progress);
                    }
                }
            }
            SettingsMessage::ModelDownloadComplete(id) => {
                for model in self
                    .recommended_models
                    .iter_mut()
                    .chain(self.custom_models.iter_mut())
                {
                    if model.id == id {
                        model.download_progress = None;
                        model.is_downloaded = true;
                    }
                }
            }
            SettingsMessage::ModelDownloadFailed(id, err) => {
                for model in self
                    .recommended_models
                    .iter_mut()
                    .chain(self.custom_models.iter_mut())
                {
                    if model.id == id {
                        model.download_progress = None;
                        model.is_downloaded = false;
                        model.last_error = Some(err.clone());
                    }
                }
            }
            SettingsMessage::ModelDownloadCancel(id) => {
                for model in self
                    .recommended_models
                    .iter_mut()
                    .chain(self.custom_models.iter_mut())
                {
                    if model.id == id {
                        model.download_progress = None;
                        // Keep is_downloaded as false (or whatever it was)
                    }
                }
            }
            SettingsMessage::ModelRemove(id) => {
                self.custom_models.retain(|m| m.id != id);
            }
            SettingsMessage::ModelActivate(id) => {
                // Deactivate all first
                for model in self
                    .recommended_models
                    .iter_mut()
                    .chain(self.custom_models.iter_mut())
                {
                    model.is_active = false;
                }
                // Activate target if downloaded
                for model in self
                    .recommended_models
                    .iter_mut()
                    .chain(self.custom_models.iter_mut())
                {
                    if model.id == id && model.is_downloaded {
                        model.is_active = true;
                    }
                }
            }
            SettingsMessage::AddModelInputChanged(val) => {
                self.add_model_input = val;
            }
            SettingsMessage::AddModelPressed => {
                if !self.add_model_input.trim().is_empty() {
                    let id = self.add_model_input.trim().to_string();
                    if !self.recommended_models.iter().any(|m| m.id == id)
                        && !self.custom_models.iter().any(|m| m.id == id)
                    {
                        self.custom_models.push(ModelInfo {
                            name: id.clone(),
                            id: id.clone(),
                            description: "Custom Model".into(),
                            size_estimate: "Unknown".into(),
                            min_ram_gb: 0,
                            is_downloaded: false,
                            is_active: false,
                            download_progress: None,
                            last_error: None,
                        });
                        self.add_model_input.clear();
                    }
                }
            }

            SettingsMessage::ToggleCaptions(enabled) => {
                self.captions_enabled = enabled;
            }
            SettingsMessage::ToggleVoice(enabled) => {
                self.voice_enabled = enabled;
            }
            SettingsMessage::ModeChanged(mode) => {
                self.current_mode = mode;
            }
            SettingsMessage::ShellStyleChanged(style) => {
                self.current_shell_style = style;
            }
        }
        Task::none()
    }

    fn view(&self, _theme: &Theme) -> Element<'_, Self::Message> {
        iced::widget::text("Settings View (Stub)").into()
    }
}
