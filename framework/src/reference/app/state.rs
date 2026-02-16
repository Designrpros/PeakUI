use super::message::Message;
use crate::engine::modifiers::ControlSize;
use crate::prelude::*;
use crate::reference::AppPage;
use crate::style::{Intent, Variant};
use crate::views::{ChatMessage, ChatRole};
use peak_core::registry::ShellMode;
use peak_theme::{PeakTheme, ThemeTokens, ThemeTone};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum ShellNavigationMode {
    #[default]
    Sidebar,
    MenuBar,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum InspectorTab {
    #[default]
    Properties,
    Documentation,
    Theory,
    Telemetry,
    App,
    Feature,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub enum RenderMode {
    #[default]
    Canvas,
    Terminal,
    Neural,
    Spatial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum AccessibilityComponent {
    #[default]
    Button,
    Slider,
    Toggle,
    Container,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AIProviderChoice {
    Ollama,
    LlamaCpp,
    OpenRouter,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub api_key: String,
    pub ai_provider: AIProviderChoice,
    pub enable_exposure: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            ai_provider: AIProviderChoice::Ollama,
            enable_exposure: false,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        Self::load_or_default()
    }

    pub fn load_or_default() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(content) = std::fs::read_to_string(".peak/settings.json") {
                if let Ok(settings) = serde_json::from_str::<Settings>(&content) {
                    return settings;
                }
            }
        }

        // Fallback or default
        let api_key = option_env!("OPENROUTER_API_KEY")
            .map(|s| s.to_string())
            .unwrap_or_default();

        let ai_provider = if cfg!(target_arch = "wasm32") {
            AIProviderChoice::OpenRouter
        } else if !api_key.is_empty() {
            AIProviderChoice::OpenRouter
        } else {
            AIProviderChoice::Ollama
        };

        Self {
            api_key,
            ai_provider,
            enable_exposure: false,
        }
    }

    pub fn save(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = std::fs::create_dir_all(".peak");
            if let Ok(content) = serde_json::to_string_pretty(self) {
                let _ = std::fs::write(".peak/settings.json", content);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShellState {
    pub active_tab: AppPage,
    pub navigation_mode: String,
    pub show_sidebar: bool,
    pub show_inspector: bool,
    pub show_search: bool,
    pub show_user_profile: bool,
    pub search_query: String,
    pub expanded_sections: Arc<std::collections::HashSet<String>>,
    pub window_width: f32,
    pub window_height: f32,
    pub localization: Localization,
}

#[derive(Debug, Clone)]
pub struct IntelligenceState {
    pub chat_messages: Arc<Vec<ChatMessage>>,
    pub chat_input: String,
    pub is_thinking: bool,
    pub api_key: String,
    pub ai_provider: AIProviderChoice,
    pub show_chat_overlay: bool,
    // Typewriter effect
    pub typewriter_text: String,
    pub typewriter_index: usize,
    pub typewriter_phrase_index: usize,
    pub is_deleting: bool,
    pub is_typing: bool,
    #[cfg(feature = "intelligence")]
    pub bridge: Arc<crate::reference::intelligence::bridge::PeakIntelligenceBridge>,
}

#[derive(Debug, Clone)]
pub struct LabState {
    pub render_mode: RenderMode,
    pub button: Arc<ButtonLabState>,
    pub typography: Arc<TypographyLabState>,
    pub layout: Arc<LayoutLabState>,
    pub spacer: Arc<SpacerLabState>,
    pub sizing: Arc<SizingLabState>,
    pub accessibility: Arc<AccessibilityLabState>,
    pub icon: Arc<IconLabState>,
    pub emoji: Arc<EmojiLabState>,
}

#[derive(Debug, Clone)]
pub struct InteractionState {
    pub show_landing: bool,
    pub sidebar_width: f32,
    pub inspector_width: f32,
    pub inspector_tab: InspectorTab,
    pub is_resizing_sidebar: bool,
    pub is_resizing_inspector: bool,
    pub context_menu_pos: Option<Point>,
    pub last_cursor_pos: Point,
    pub last_copied_code: Option<String>,
    pub pending_sudo_action: Option<SudoAction>,
    pub theme_tone: ThemeTone,
    pub theme: PeakTheme,
    pub scaling: f32,
    pub tick: u64,
    pub enable_exposure: bool,
}

#[derive(Debug, Clone)]
pub struct App {
    pub shell: ShellState,
    pub intelligence: IntelligenceState,
    pub labs: LabState,
    pub interaction: InteractionState,

    // Core Services
    #[cfg(feature = "neural")]
    pub db: Arc<crate::reference::data::db::PeakDBBridge>,
    pub a11y: Arc<crate::engine::accessibility::AccessibilityBridge>,
    pub peak_id: String,
    pub icon_limit: usize,
}

impl App {
    pub fn save_settings(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let settings = Settings {
                api_key: self.intelligence.api_key.clone(),
                ai_provider: self.intelligence.ai_provider,
                enable_exposure: self.interaction.enable_exposure,
            };
            settings.save();
        }
    }
    pub fn shell_mode(&self) -> ShellMode {
        if self.shell.window_width < 900.0 {
            ShellMode::Mobile
        } else {
            ShellMode::Desktop
        }
    }

    pub fn context(&self) -> Context {
        let mode = self.shell_mode();
        let mut tokens =
            ThemeTokens::with_theme(self.interaction.theme, self.interaction.theme_tone);
        tokens.scaling = self.interaction.scaling;

        Context::new(
            mode,
            tokens,
            Size::new(self.shell.window_width, self.shell.window_height),
            self.shell.localization.clone(),
        )
        .with_last_copied_code(self.interaction.last_copied_code.as_deref().map(Arc::from))
    }
}

/// A protected action that requires user confirmation or "Sudo" elevation.
///
/// The `message` is skipped during serialization as it contains a dynamic boxed enum
/// that cannot be easily serialized across boundaries.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SudoAction {
    #[serde(skip, default = "dummy_message")]
    pub message: Box<Message>,
    pub reason: String,
}

fn dummy_message() -> Box<Message> {
    Box::new(Message::Interaction(
        crate::reference::app::InteractionMessage::CloseContextMenu,
    ))
}

fn default_alignment() -> Alignment {
    Alignment::Start
}

/// Configuration state for the Button Lab component.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ButtonLabState {
    pub label: String,
    pub icon: Option<String>,
    pub variant: Variant,
    pub intent: Intent,
    pub size: ControlSize,
    pub is_full_width: bool,
    pub is_disabled: bool,
    pub is_focused: bool,
}

/// Configuration state for the Typography Lab component.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TypographyLabState {
    pub text: String,
    pub size: f32,
    pub is_bold: bool,
    pub is_italic: bool,
    #[serde(skip, default)]
    pub color: Option<Color>,
}

impl Default for TypographyLabState {
    fn default() -> Self {
        Self {
            text: "The quick brown fox jumps over the lazy dog.".to_string(),
            size: 16.0,
            is_bold: false,
            is_italic: false,
            color: None,
        }
    }
}

/// Configuration state for the Layout Lab component.
///
/// `alignment` is skipped during serialization as it is an external iced type without
/// built-in serialization support in this framework version.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LayoutLabState {
    pub outer_spacing: f32,
    pub inner_spacing: f32,
    pub child_count: usize,
    #[serde(skip, default = "default_alignment")]
    pub alignment: Alignment,
    pub item_sizing: SizingType,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SizingLabState {
    pub width_type: SizingType,
    pub height_type: SizingType,
    pub fixed_width: f32,
    pub fixed_height: f32,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AccessibilityLabState {
    pub selected_component: AccessibilityComponent,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IconLabState {
    pub selected_icon: String,
    pub size: f32,
    #[serde(skip, default)]
    pub color: Option<Color>,
}

impl Default for IconLabState {
    fn default() -> Self {
        Self {
            selected_icon: "zap".to_string(),
            size: 32.0,
            color: None,
        }
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmojiLabState {
    pub selected_emoji: String,
    pub size: f32,
}

impl Default for EmojiLabState {
    fn default() -> Self {
        Self {
            selected_emoji: "🚀".to_string(),
            size: 48.0,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpacerLabState {
    pub width: f32,
    pub height: f32,
}

impl Default for SpacerLabState {
    fn default() -> Self {
        Self {
            width: 40.0,
            height: 40.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SizingType {
    Fixed,
    Fill,
    Shrink,
}

impl Default for SizingLabState {
    fn default() -> Self {
        Self {
            width_type: SizingType::Fixed,
            height_type: SizingType::Fixed,
            fixed_width: 200.0,
            fixed_height: 40.0,
        }
    }
}

impl Default for LayoutLabState {
    fn default() -> Self {
        Self {
            outer_spacing: 16.0,
            inner_spacing: 24.0,
            child_count: 3,
            alignment: Alignment::Start,
            item_sizing: SizingType::Fixed,
        }
    }
}

impl Default for ButtonLabState {
    fn default() -> Self {
        Self {
            label: "Click Me".to_string(),
            icon: None,
            variant: Variant::Solid,
            intent: Intent::Primary,
            size: ControlSize::Medium,
            is_full_width: false,
            is_disabled: false,
            is_focused: false,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        let settings = Settings::load();
        #[cfg(feature = "intelligence")]
        let provider = match settings.ai_provider {
            AIProviderChoice::Ollama => peak_intelligence::llm::ModelProvider::Ollama,
            AIProviderChoice::LlamaCpp => peak_intelligence::llm::ModelProvider::LlamaCpp,
            AIProviderChoice::OpenRouter => peak_intelligence::llm::ModelProvider::OpenRouter,
        };

        #[cfg(feature = "neural")]
        let db = Arc::new(crate::reference::data::db::PeakDBBridge::new());

        #[cfg(feature = "neural")]
        {
            // Seed some initial data for RAG testing
            let seed_records = vec![
                ("System", "PeakOS is a decentralized, agent-native operating system designed for the next era of computing."),
                ("Architecture", "PeakUI uses a multi-kernel bridge architecture, allowing AI agents to perceive and interact with UI elements semantically."),
                ("Security", "Neural Sudo is a high-security interception layer that ensures no AI action of high privilege is executed without explicit user consent."),
            ];

            for (i, (collection, content)) in seed_records.into_iter().enumerate() {
                let record = crate::core::SemanticRecord {
                    id: format!("seed-{}", i),
                    collection: collection.to_string(),
                    content: content.to_string(),
                    vector: None,
                    metadata: serde_json::json!({}),
                    timestamp: 0,
                };
                let _ = db.save(record);
            }
        }

        #[cfg(feature = "intelligence")]
        let intelligence_bridge = Arc::new(
            crate::reference::intelligence::bridge::PeakIntelligenceBridge::new(
                provider,
                match provider {
                    peak_intelligence::llm::ModelProvider::Ollama => "llama3",
                    _ => "google/gemini-3-flash-preview",
                },
                if settings.api_key.is_empty() {
                    None
                } else {
                    Some(settings.api_key.clone())
                },
                #[cfg(feature = "neural")]
                db.clone(),
                #[cfg(not(feature = "neural"))]
                Arc::new(crate::reference::data::stub_db::StubDB::new()),
            ),
        );

        Self {
            shell: ShellState {
                active_tab: AppPage::Introduction,
                navigation_mode: "App".to_string(),
                show_sidebar: true,
                show_inspector: false,
                show_search: false,
                show_user_profile: false,
                search_query: String::new(),
                expanded_sections: Arc::new(std::collections::HashSet::new()),
                window_width: {
                    #[cfg(target_arch = "wasm32")]
                    {
                        web_sys::window()
                            .and_then(|w| w.document())
                            .and_then(|d| d.body())
                            .map(|b| b.client_width() as f32)
                            .unwrap_or(1200.0)
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    1200.0
                },
                window_height: {
                    #[cfg(target_arch = "wasm32")]
                    {
                        web_sys::window()
                            .and_then(|w| w.document())
                            .and_then(|d| d.body())
                            .map(|b| b.client_height() as f32)
                            .unwrap_or(800.0)
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    800.0
                },
                localization: Localization::default(),
            },
            intelligence: IntelligenceState {
                chat_messages: Arc::new(vec![ChatMessage {
                    role: ChatRole::System,
                    content: "Welcome to PeakUI. I am your autonomous interface agent.".to_string(),
                }]),
                chat_input: String::new(),
                is_thinking: false,
                api_key: settings.api_key,
                ai_provider: settings.ai_provider,
                show_chat_overlay: false,
                typewriter_text: String::new(),
                typewriter_index: 0,
                typewriter_phrase_index: 0,
                is_deleting: false,
                is_typing: false,
                #[cfg(feature = "intelligence")]
                bridge: intelligence_bridge,
            },
            labs: LabState {
                render_mode: RenderMode::Canvas,
                button: Arc::new(ButtonLabState::default()),
                typography: Arc::new(TypographyLabState::default()),
                layout: Arc::new(LayoutLabState::default()),
                spacer: Arc::new(SpacerLabState::default()),
                sizing: Arc::new(SizingLabState::default()),
                accessibility: Arc::new(AccessibilityLabState::default()),
                icon: Arc::new(IconLabState::default()),
                emoji: Arc::new(EmojiLabState::default()),
            },
            interaction: InteractionState {
                show_landing: true,
                sidebar_width: 260.0,
                inspector_width: 320.0,
                inspector_tab: InspectorTab::App,
                is_resizing_sidebar: false,
                is_resizing_inspector: false,
                context_menu_pos: None,
                last_cursor_pos: Point::ORIGIN,
                last_copied_code: None,
                pending_sudo_action: None,
                theme_tone: ThemeTone::Light,
                theme: PeakTheme::Mono,
                scaling: 1.0,
                tick: 0,
                enable_exposure: settings.enable_exposure,
            },
            #[cfg(feature = "neural")]
            db,
            a11y: Arc::new(crate::engine::accessibility::AccessibilityBridge::new()),
            peak_id: String::new(),
            icon_limit: 50,
        }
    }
}
