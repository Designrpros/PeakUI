use super::model::Page;
use crate::core::{DataProvider, IntelligenceProvider};
use crate::prelude::*;
use crate::reference::intelligence::Action;
use crate::reference::intelligence_bridge::PeakIntelligenceBridge;
use crate::views::chat::{ChatMessage, ChatRole, ChatViewMessage};
use peak_core::registry::ShellMode;
use peak_theme::{PeakTheme, ThemeTokens, ThemeTone};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum InspectorTab {
    #[default]
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
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            ai_provider: AIProviderChoice::Ollama,
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

pub struct App {
    pub active_tab: Page,
    pub show_search: bool,
    pub show_inspector: bool,
    pub show_sidebar: bool,
    pub show_user_profile: bool,
    pub navigation_mode: String,
    pub search_query: String,
    pub expanded_sections: std::collections::HashSet<String>,
    pub theme_tone: ThemeTone,
    pub theme: PeakTheme,

    // Component Lab States
    pub button_lab: ButtonLabState,
    pub typography_lab: TypographyLabState,
    pub layout_lab: LayoutLabState,
    pub sizing_lab: SizingLabState,
    pub accessibility_lab: AccessibilityLabState,
    pub icon_lab: IconLabState,
    pub render_mode: RenderMode,
    pub show_landing: bool,
    // Layout States
    pub sidebar_width: f32,
    pub inspector_width: f32,
    pub inspector_tab: InspectorTab,
    pub is_resizing_sidebar: bool,
    pub is_resizing_inspector: bool,
    pub context_menu_pos: Option<iced::Point>,
    pub last_cursor_pos: iced::Point,

    // Chat State
    pub show_chat_overlay: bool,
    pub chat_messages: Vec<ChatMessage>,
    pub chat_input: String,

    // AI Integration
    pub api_key: String,
    pub ai_provider: AIProviderChoice,

    // Infinite Scroll / Lazy Load
    pub icon_limit: usize,
    pub window_width: f32,
    pub localization: Localization,
    pub pending_sudo_action: Option<SudoAction>,
    pub is_thinking: bool,
    pub intelligence: Arc<crate::reference::intelligence_bridge::PeakIntelligenceBridge>,
    pub db: Arc<crate::reference::db_bridge::PeakDBBridge>,
    pub peak_id: String,

    // Typewriter Effect
    pub typewriter_text: String,
    pub typewriter_index: usize,
    pub typewriter_phrase_index: usize,
    pub is_deleting: bool,
    pub a11y: crate::accessibility::AccessibilityBridge,
}

#[derive(Debug, Clone)]
pub struct SudoAction {
    pub message: Box<Message>,
    pub reason: String,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct TypographyLabState {
    pub text: String,
    pub size: f32,
    pub is_bold: bool,
    pub is_italic: bool,
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

#[derive(Debug, Clone)]
pub struct LayoutLabState {
    pub outer_spacing: f32,
    pub inner_spacing: f32,
    pub child_count: usize,
    pub alignment: Alignment,
    pub item_sizing: SizingType,
}

#[derive(Debug, Clone)]
pub struct SizingLabState {
    pub width_type: SizingType,
    pub height_type: SizingType,
    pub fixed_width: f32,
    pub fixed_height: f32,
}

#[derive(Debug, Clone, Default)]
pub struct AccessibilityLabState {
    pub selected_component: AccessibilityComponent,
}

#[derive(Debug, Clone)]
pub struct IconLabState {
    pub selected_icon: String,
    pub size: f32,
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

#[derive(Debug, Clone)]
pub enum Message {
    SetTab(Page),
    ToggleSearch,
    ToggleInspector,
    ToggleSidebar,
    ToggleUserProfile,
    SetNavigationMode(String),
    ToggleSection(String),
    Search(String),
    SetTheme(ThemeTone),
    SetThemeKind(PeakTheme),
    CopyCode(String),
    SetRenderMode(RenderMode),
    OpenContextMenu(iced::Point),
    CloseContextMenu,
    ContextMenuAction(String),
    EnterApp,
    SetLanguage(String, Vec<String>),
    OpenUrl(String),

    // Chat
    Chat(ChatViewMessage),
    AIResponse(std::result::Result<String, String>),
    ChatStreamUpdate(std::result::Result<String, String>),
    AIChatComplete,

    // Icon Lab
    UpdateIconLabIcon(String),
    UpdateIconLabSize(f32),
    UpdateIconLabColor(Option<Color>),
    SetInspectorTab(InspectorTab),
    SetApiKey(String),
    SetAIProvider(AIProviderChoice),

    // Button Lab Messages
    UpdateButtonLabel(String),
    UpdateButtonIcon(Option<String>),
    UpdateButtonSize(ControlSize),
    UpdateButtonVariant(Variant),
    UpdateButtonIntent(Intent),
    ToggleButtonFullWidth(bool),
    ToggleButtonDisabled(bool),
    ToggleButtonFocused(bool),
    // Typography Lab Messages
    UpdateTypographyText(String),
    UpdateTypographySize(f32),
    ToggleTypographyBold(bool),
    ToggleTypographyItalic(bool),

    // Layout Lab Messages
    UpdateLayoutOuterSpacing(f32),
    UpdateLayoutInnerSpacing(f32),
    UpdateLayoutChildCount(usize),
    UpdateLayoutAlignment(Alignment),
    UpdateLayoutItemSizing(SizingType),

    // Sizing Lab Messages
    UpdateSizingWidthType(SizingType),
    UpdateSizingHeightType(SizingType),
    UpdateSizingFixedWidth(f32),
    UpdateSizingFixedHeight(f32),

    // Accessibility Lab Messages
    UpdateAccessibilityComponent(AccessibilityComponent),

    ResizeSidebar(f32),
    ResizeInspector(f32),
    StartResizingSidebar,
    StopResizingSidebar,
    StartResizingInspector,
    StopResizingInspector,
    UpdateCursorPos(iced::Point),
    WindowResized(iced::Size),
    FontLoaded(std::result::Result<(), iced::font::Error>),
    CmdBackspacePressed,
    LoadMoreIcons,
    Heartbeat,
    SudoRequest(SudoAction),
    SudoApprove,
    SudoDeny,
    ExecuteShell(String), // New: Shell execution message
    ApplyNativeVibrancy,
    TypewriterTick(std::time::Instant),
    None,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub enum Command {
    EnterApp,
    SetTab(Page),
    ToggleSearch,
    ToggleInspector,
    ToggleSidebar,
    ToggleUserProfile,
    SetNavigationMode(String),
    ToggleSection(String),
    Search(String),
    SetTheme(ThemeTone),
    SetThemeKind(PeakTheme),
    SetRenderMode(RenderMode),
    SetInspectorTab(InspectorTab),
    SetApiKey(String),
    SetAIProvider(AIProviderChoice),

    // Button Lab
    UpdateButtonLabel(String),
    UpdateButtonVariant(Variant),
    UpdateButtonIntent(Intent),
    ToggleButtonFullWidth(bool),
    ToggleButtonDisabled(bool),
    ToggleButtonFocused(bool),

    // Typography Lab
    UpdateTypographyText(String),
    UpdateTypographySize(f32),
    ToggleTypographyBold(bool),
    ToggleTypographyItalic(bool),

    // Layout Lab
    UpdateLayoutOuterSpacing(f32),
    UpdateLayoutInnerSpacing(f32),
    UpdateLayoutChildCount(usize),
    UpdateLayoutAlignment(String),
    UpdateLayoutItemSizing(SizingType),

    // Sizing Lab
    UpdateSizingWidthType(SizingType),
    UpdateSizingHeightType(SizingType),
    UpdateSizingFixedWidth(f32),
    UpdateSizingFixedHeight(f32),

    ApplyNativeVibrancy,
    None,
}

impl Command {
    pub fn into_message(self) -> Message {
        match self {
            Command::EnterApp => Message::EnterApp,
            Command::SetTab(page) => Message::SetTab(page),
            Command::ToggleSearch => Message::ToggleSearch,
            Command::ToggleInspector => Message::ToggleInspector,
            Command::ToggleSidebar => Message::ToggleSidebar,
            Command::ToggleUserProfile => Message::ToggleUserProfile,
            Command::SetNavigationMode(mode) => Message::SetNavigationMode(mode),
            Command::ToggleSection(section) => Message::ToggleSection(section),
            Command::Search(query) => Message::Search(query),
            Command::SetTheme(tone) => Message::SetTheme(tone),
            Command::SetThemeKind(theme) => Message::SetThemeKind(theme),
            Command::SetRenderMode(mode) => Message::SetRenderMode(mode),

            // Button Lab
            Command::UpdateButtonLabel(label) => Message::UpdateButtonLabel(label),
            Command::UpdateButtonVariant(variant) => Message::UpdateButtonVariant(variant),
            Command::UpdateButtonIntent(intent) => Message::UpdateButtonIntent(intent),
            Command::ToggleButtonFullWidth(full) => Message::ToggleButtonFullWidth(full),
            Command::ToggleButtonDisabled(disabled) => Message::ToggleButtonDisabled(disabled),
            Command::ToggleButtonFocused(focused) => Message::ToggleButtonFocused(focused),

            // Typography Lab
            Command::UpdateTypographyText(text) => Message::UpdateTypographyText(text),
            Command::UpdateTypographySize(size) => Message::UpdateTypographySize(size),
            Command::ToggleTypographyBold(bold) => Message::ToggleTypographyBold(bold),
            Command::ToggleTypographyItalic(italic) => Message::ToggleTypographyItalic(italic),

            // Layout Lab
            Command::UpdateLayoutOuterSpacing(s) => Message::UpdateLayoutOuterSpacing(s),
            Command::UpdateLayoutInnerSpacing(s) => Message::UpdateLayoutInnerSpacing(s),
            Command::UpdateLayoutChildCount(c) => Message::UpdateLayoutChildCount(c),
            Command::UpdateLayoutAlignment(align) => {
                let alignment = match align.to_lowercase().as_str() {
                    "start" | "left" | "top" => Alignment::Start,
                    "center" => Alignment::Center,
                    "end" | "right" | "bottom" => Alignment::End,
                    _ => Alignment::Center,
                };
                Message::UpdateLayoutAlignment(alignment)
            }
            Command::UpdateLayoutItemSizing(sizing) => Message::UpdateLayoutItemSizing(sizing),

            // Sizing Lab
            Command::UpdateSizingWidthType(t) => Message::UpdateSizingWidthType(t),
            Command::UpdateSizingHeightType(t) => Message::UpdateSizingHeightType(t),
            Command::UpdateSizingFixedWidth(w) => Message::UpdateSizingFixedWidth(w),
            Command::UpdateSizingFixedHeight(h) => Message::UpdateSizingFixedHeight(h),

            Command::SetInspectorTab(tab) => Message::SetInspectorTab(tab),
            Command::SetApiKey(key) => Message::SetApiKey(key),
            Command::SetAIProvider(provider) => Message::SetAIProvider(provider),

            Command::ApplyNativeVibrancy => Message::ApplyNativeVibrancy,
            Command::None => Message::None,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        let settings = Settings::load();
        let provider = match settings.ai_provider {
            AIProviderChoice::Ollama => peak_os_intelligence::llm::ModelProvider::Ollama,
            AIProviderChoice::LlamaCpp => peak_os_intelligence::llm::ModelProvider::LlamaCpp,
            AIProviderChoice::OpenRouter => peak_os_intelligence::llm::ModelProvider::OpenRouter,
        };

        let db = Arc::new(crate::reference::db_bridge::PeakDBBridge::new());

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

        let intelligence = Arc::new(PeakIntelligenceBridge::new(
            provider,
            match provider {
                peak_os_intelligence::llm::ModelProvider::Ollama => "llama3",
                _ => "google/gemini-3-flash-preview",
            },
            if settings.api_key.is_empty() {
                None
            } else {
                Some(settings.api_key.clone())
            },
            db.clone(),
        ));

        Self {
            active_tab: Page::Introduction,
            show_search: false,
            show_inspector: false,
            show_sidebar: true,
            show_user_profile: false,
            navigation_mode: "App".to_string(),
            search_query: String::new(),
            expanded_sections: std::collections::HashSet::new(),
            theme_tone: ThemeTone::Light,
            theme: PeakTheme::Mono,
            button_lab: ButtonLabState::default(),
            typography_lab: TypographyLabState::default(),
            layout_lab: LayoutLabState::default(),
            sizing_lab: SizingLabState::default(),
            accessibility_lab: AccessibilityLabState::default(),
            icon_lab: IconLabState::default(),
            render_mode: RenderMode::Canvas,
            show_landing: true,
            sidebar_width: 260.0,
            inspector_width: 320.0,
            inspector_tab: InspectorTab::App,
            is_resizing_sidebar: false,
            is_resizing_inspector: false,
            context_menu_pos: None,
            last_cursor_pos: iced::Point::ORIGIN,
            show_chat_overlay: false,
            chat_messages: vec![ChatMessage {
                role: ChatRole::System,
                content: "Welcome to PeakUI. I am your autonomous interface agent.".to_string(),
            }],
            chat_input: String::new(),
            api_key: settings.api_key,
            ai_provider: settings.ai_provider,
            peak_id: String::new(),
            icon_limit: 50,
            window_width: {
                #[cfg(target_arch = "wasm32")]
                {
                    let w = web_sys::window()
                        .and_then(|w| w.document())
                        .and_then(|d| d.body())
                        .map(|b| b.client_width() as f32)
                        .unwrap_or(1200.0);
                    w
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    1200.0
                }
            },
            localization: Localization::default(),
            pending_sudo_action: None,
            is_thinking: false,
            intelligence,
            db,
            typewriter_text: String::new(),
            typewriter_index: 0,
            typewriter_phrase_index: 0,
            is_deleting: false,
            a11y: crate::accessibility::AccessibilityBridge::new(),
        }
    }
}

pub use crate::core::{Context, DeviceType};

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TypewriterTick(_) => {
                if !self.show_landing {
                    return Task::none();
                }

                let phrases = [
                    "Say hello",
                    "Change tone to dark",
                    "Navigate to button lab",
                    "Set button variant to compact",
                ];

                let current_phrase = phrases[self.typewriter_phrase_index % phrases.len()];

                if self.is_deleting {
                    if self.typewriter_index > 0 {
                        self.typewriter_index = self.typewriter_index.saturating_sub(1);
                        self.typewriter_text =
                            current_phrase.chars().take(self.typewriter_index).collect();
                    } else {
                        self.is_deleting = false;
                        self.typewriter_phrase_index =
                            (self.typewriter_phrase_index + 1) % phrases.len();
                    }
                } else {
                    let target_len = current_phrase.chars().count();
                    // Add a pause at the end by counting past length
                    if self.typewriter_index < target_len + 15 {
                        self.typewriter_index += 1;
                        if self.typewriter_index <= target_len {
                            self.typewriter_text =
                                current_phrase.chars().take(self.typewriter_index).collect();
                        }
                    } else {
                        self.is_deleting = true;
                        self.typewriter_index = target_len;
                    }
                }
                Task::none()
            }
            Message::ApplyNativeVibrancy => {
                #[cfg(target_os = "macos")]
                {
                    use objc2_app_kit::{
                        NSApplication, NSAutoresizingMaskOptions, NSVisualEffectBlendingMode,
                        NSVisualEffectMaterial, NSVisualEffectState, NSVisualEffectView,
                        NSWindowOrderingMode,
                    };
                    use objc2_foundation::{MainThreadMarker, NSRect};

                    log::info!("Applying native vibrancy...");
                    if let Some(mtm) = MainThreadMarker::new() {
                        let app = NSApplication::sharedApplication(mtm);
                        if let Some(window) = unsafe { app.windows().lastObject() } {
                            unsafe {
                                let frame =
                                    window
                                        .contentView()
                                        .map(|v| v.frame())
                                        .unwrap_or(NSRect::new(
                                            objc2_foundation::CGPoint::new(0.0, 0.0),
                                            objc2_foundation::CGSize::new(1000.0, 1000.0),
                                        ));
                                let effect_view = NSVisualEffectView::initWithFrame(
                                    mtm.alloc::<NSVisualEffectView>(),
                                    frame,
                                );

                                effect_view.setMaterial(NSVisualEffectMaterial::HUDWindow);
                                effect_view
                                    .setBlendingMode(NSVisualEffectBlendingMode::BehindWindow);
                                effect_view.setState(NSVisualEffectState::Active);
                                effect_view.setAutoresizingMask(
                                    NSAutoresizingMaskOptions::from_bits_truncate(18),
                                );

                                if let Some(content_view) = window.contentView() {
                                    content_view.addSubview_positioned_relativeTo(
                                        &effect_view,
                                        NSWindowOrderingMode::NSWindowBelow,
                                        None,
                                    );
                                    log::info!("Vibrancy effect view added.");
                                }
                            }
                        }
                    }
                }
                Task::none()
            }
            Message::EnterApp => {
                self.show_landing = false;
                self.show_sidebar = true;

                if !self.search_query.trim().is_empty() {
                    let query = self.search_query.clone();
                    self.search_query.clear();
                    self.show_inspector = true;
                    self.inspector_tab = InspectorTab::App;
                    return self.start_ai_chat(query);
                }

                Task::none()
            }
            Message::SetLanguage(lang, resources) => {
                self.localization = Localization::new(&lang, resources);
                Task::none()
            }
            Message::SetTab(tab) => {
                log::info!(
                    "🔥 SetTab RECEIVED: {:?} (Category: {})",
                    tab,
                    tab.navigation_mode()
                );
                self.navigation_mode = match tab.navigation_mode().to_lowercase().as_str() {
                    "start" | "guide" => "Start".to_string(),
                    "catalog" | "components" => "Catalog".to_string(),
                    "data" | "ecosystem" => "Data".to_string(),
                    "settings" | "preferences" => "Settings".to_string(),
                    _ => tab.navigation_mode(),
                };
                self.active_tab = tab.clone();
                self.show_search = false;

                // Auto-enable inspector for lab pages
                match self.active_tab {
                    Page::Button | Page::Typography | Page::BasicSizing | Page::Layout => {
                        self.show_inspector = true;
                    }
                    _ => {}
                }

                #[cfg(target_arch = "wasm32")]
                {
                    let path = tab.to_path();
                    let _ = web_sys::window().and_then(|w| w.location().set_hash(&path).ok());
                }

                // Sync window width from browser on every navigation for reliability
                #[cfg(target_arch = "wasm32")]
                {
                    if let Some(w) = web_sys::window()
                        .and_then(|w| w.inner_width().ok())
                        .and_then(|v| v.as_f64())
                    {
                        self.window_width = w as f32;
                    }
                }

                log::info!(
                    "SetTab: {:?}, window_width: {}, show_sidebar: {}",
                    tab,
                    self.window_width,
                    self.show_sidebar
                );

                // Mobile Navigation Protocol: Auto-close sidebar on navigation if on mobile
                if self.window_width < 900.0 {
                    log::info!(" -> Mobile Mode: Auto-closing sidebar");
                    self.show_sidebar = false;
                }

                // Landing/Details visibility logic
                match tab {
                    Page::Landing
                    | Page::PeakOSDetail
                    | Page::PeakUIDetail
                    | Page::PeakDBDetail
                    | Page::PeakRelayDetail
                    | Page::PeakHubDetail => {
                        self.show_landing = true;
                        // On mobile, the landing/home view should show the menu (sidebar)
                        if self.window_width < 900.0 && self.active_tab == Page::Landing {
                            log::info!(" -> Landing on Mobile: Auto-showing sidebar");
                            self.show_sidebar = true;
                        }
                    }
                    _ => {
                        self.show_landing = false;
                        // Only auto-show sidebar on desktop; on mobile we want it closed after navigation
                        if self.window_width >= 900.0 {
                            self.show_sidebar = true;
                        }
                    }
                }

                Task::none()
            }
            Message::ToggleSearch => {
                self.show_search = !self.show_search;
                self.search_query.clear();
                Task::none()
            }
            Message::ToggleInspector => {
                self.show_inspector = !self.show_inspector;
                Task::none()
            }
            Message::ToggleSidebar => {
                self.show_sidebar = !self.show_sidebar;
                log::info!("ToggleSidebar: show_sidebar is now {}", self.show_sidebar);
                Task::none()
            }
            Message::ToggleUserProfile => {
                self.show_user_profile = !self.show_user_profile;
                Task::none()
            }
            Message::SetNavigationMode(mode) => {
                self.navigation_mode = match mode.to_lowercase().as_str() {
                    "start" | "guide" | "documentation" => "Start".to_string(),
                    "catalog" | "components" => "Catalog".to_string(),
                    "data" | "ecosystem" => "Data".to_string(),
                    "settings" | "preferences" => "Settings".to_string(),
                    _ => mode.clone(),
                };
                self.active_tab = match self.navigation_mode.as_str() {
                    "Start" => Page::Introduction,
                    "Catalog" => Page::Button,
                    "Data" => Page::PeakDB,
                    "Settings" => Page::Appearance,
                    _ => self.active_tab.clone(),
                };

                // Mobile Navigation Protocol: Auto-close sidebar on navigation if on mobile
                if self.window_width < 900.0 {
                    log::info!(" -> Mobile Mode (SetNavigationMode): Auto-closing sidebar");
                    self.show_sidebar = false;
                }

                Task::none()
            }
            Message::ToggleSection(section) => {
                if self.expanded_sections.contains(&section) {
                    self.expanded_sections.remove(&section);
                } else {
                    self.expanded_sections.insert(section);
                }
                Task::none()
            }
            Message::Search(query) => {
                self.search_query = query;
                Task::none()
            }
            Message::SetTheme(tone) => {
                self.theme_tone = tone;
                Task::none()
            }
            Message::SetThemeKind(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::CopyCode(code) => iced::clipboard::write(code),
            Message::OpenUrl(url) => {
                #[cfg(target_arch = "wasm32")]
                {
                    let _ = web_sys::window()
                        .and_then(|w| w.open_with_url_and_target(&url, "_blank").ok());
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    log::info!("Opening URL: {}", url);
                }
                Task::none()
            }

            Message::SetRenderMode(mode) => {
                self.render_mode = mode;
                Task::none()
            }

            // Button Lab Handlers
            Message::UpdateButtonLabel(label) => {
                self.button_lab.label = label;
                Task::none()
            }
            Message::UpdateButtonIcon(icon) => {
                self.button_lab.icon = icon;
                Task::none()
            }
            Message::UpdateButtonVariant(variant) => {
                self.button_lab.variant = variant;
                Task::none()
            }
            Message::UpdateButtonIntent(intent) => {
                self.button_lab.intent = intent;
                Task::none()
            }
            Message::UpdateButtonSize(size) => {
                self.button_lab.size = size;
                Task::none()
            }
            Message::ToggleButtonFullWidth(full_width) => {
                self.button_lab.is_full_width = full_width;
                Task::none()
            }
            Message::ToggleButtonDisabled(disabled) => {
                self.button_lab.is_disabled = disabled;
                Task::none()
            }
            Message::ToggleButtonFocused(focused) => {
                self.button_lab.is_focused = focused;
                Task::none()
            }

            // Icon Lab Handlers
            Message::UpdateIconLabIcon(icon) => {
                self.icon_lab.selected_icon = icon;
                Task::none()
            }
            Message::UpdateIconLabSize(size) => {
                self.icon_lab.size = size;
                Task::none()
            }
            Message::UpdateIconLabColor(color) => {
                self.icon_lab.color = color;
                Task::none()
            }

            // Typography Lab Handlers
            Message::UpdateTypographyText(text) => {
                self.typography_lab.text = text;
                Task::none()
            }
            Message::UpdateTypographySize(size) => {
                self.typography_lab.size = size;
                Task::none()
            }
            Message::ToggleTypographyBold(bold) => {
                self.typography_lab.is_bold = bold;
                Task::none()
            }
            Message::ToggleTypographyItalic(italic) => {
                self.typography_lab.is_italic = italic;
                Task::none()
            }

            // Layout Lab Handlers
            Message::UpdateLayoutOuterSpacing(spacing) => {
                self.layout_lab.outer_spacing = spacing;
                Task::none()
            }
            Message::UpdateLayoutInnerSpacing(spacing) => {
                self.layout_lab.inner_spacing = spacing;
                Task::none()
            }
            Message::UpdateLayoutChildCount(count) => {
                self.layout_lab.child_count = count.min(10);
                Task::none()
            }
            Message::UpdateLayoutAlignment(alignment) => {
                self.layout_lab.alignment = alignment;
                Task::none()
            }
            Message::UpdateLayoutItemSizing(sizing) => {
                self.layout_lab.item_sizing = sizing;
                Task::none()
            }

            // Sizing Lab Handlers
            Message::UpdateSizingWidthType(t) => {
                self.sizing_lab.width_type = t;
                Task::none()
            }
            Message::UpdateSizingHeightType(t) => {
                self.sizing_lab.height_type = t;
                Task::none()
            }
            Message::UpdateSizingFixedWidth(w) => {
                self.sizing_lab.fixed_width = w;
                Task::none()
            }
            Message::UpdateSizingFixedHeight(h) => {
                self.sizing_lab.fixed_height = h;
                Task::none()
            }

            Message::UpdateAccessibilityComponent(comp) => {
                self.accessibility_lab.selected_component = comp;
                Task::none()
            }

            Message::ResizeSidebar(width) => {
                self.sidebar_width = width.max(160.0).min(400.0);
                Task::none()
            }
            Message::ResizeInspector(width) => {
                self.inspector_width = width.max(180.0).min(600.0);
                Task::none()
            }
            Message::StartResizingSidebar => {
                self.is_resizing_sidebar = true;
                Task::none()
            }
            Message::StopResizingSidebar => {
                self.is_resizing_sidebar = false;
                Task::none()
            }
            Message::StartResizingInspector => {
                self.is_resizing_inspector = true;
                Task::none()
            }
            Message::StopResizingInspector => {
                self.is_resizing_inspector = false;
                Task::none()
            }
            Message::CmdBackspacePressed => {
                log::info!("CMD + BACKSPACE: Clearing inputs");
                self.search_query.clear();
                self.chat_input.clear();
                Task::none()
            }
            Message::UpdateCursorPos(pos) => {
                self.last_cursor_pos = pos;
                Task::none()
            }
            Message::SetInspectorTab(tab) => {
                self.inspector_tab = tab;
                Task::none()
            }
            Message::SetApiKey(key) => {
                self.api_key = key.clone();
                let settings = Settings {
                    api_key: self.api_key.clone(),
                    ai_provider: self.ai_provider,
                };
                settings.save();

                // Hot-reload intelligence bridge with new key
                let provider = match self.ai_provider {
                    AIProviderChoice::Ollama => peak_os_intelligence::llm::ModelProvider::Ollama,
                    AIProviderChoice::LlamaCpp => {
                        peak_os_intelligence::llm::ModelProvider::LlamaCpp
                    }
                    AIProviderChoice::OpenRouter => {
                        peak_os_intelligence::llm::ModelProvider::OpenRouter
                    }
                };

                self.intelligence = Arc::new(PeakIntelligenceBridge::new(
                    provider,
                    match provider {
                        peak_os_intelligence::llm::ModelProvider::Ollama => "llama3",
                        _ => "google/gemini-3-flash-preview",
                    },
                    if key.is_empty() { None } else { Some(key) },
                    self.db.clone(),
                ));

                Task::none()
            }
            Message::SetAIProvider(provider) => {
                self.ai_provider = provider;
                let settings = Settings {
                    api_key: self.api_key.clone(),
                    ai_provider: self.ai_provider,
                };
                settings.save();

                // Hot-reload intelligence bridge with new provider
                let model_provider = match provider {
                    AIProviderChoice::Ollama => peak_os_intelligence::llm::ModelProvider::Ollama,
                    AIProviderChoice::LlamaCpp => {
                        peak_os_intelligence::llm::ModelProvider::LlamaCpp
                    }
                    AIProviderChoice::OpenRouter => {
                        peak_os_intelligence::llm::ModelProvider::OpenRouter
                    }
                };

                self.intelligence = Arc::new(PeakIntelligenceBridge::new(
                    model_provider,
                    match model_provider {
                        peak_os_intelligence::llm::ModelProvider::Ollama => "llama3",
                        _ => "google/gemini-3-flash-preview",
                    },
                    if self.api_key.is_empty() {
                        None
                    } else {
                        Some(self.api_key.clone())
                    },
                    self.db.clone(),
                ));

                Task::none()
            }
            Message::OpenContextMenu(pos) => {
                self.context_menu_pos = Some(pos);
                Task::none()
            }
            Message::CloseContextMenu => {
                self.context_menu_pos = None;
                Task::none()
            }
            Message::ContextMenuAction(action) => {
                log::info!("Context Menu Action: {}", action);
                self.context_menu_pos = None;
                match action.as_str() {
                    "Reload" => {
                        #[cfg(target_arch = "wasm32")]
                        let _ = web_sys::window().and_then(|w| w.location().reload().ok());
                    }
                    "Inspect" => {
                        self.show_inspector = true;
                    }
                    _ => {}
                }
                Task::none()
            }
            Message::WindowResized(size) => {
                let was_desktop = self.window_width >= 900.0;
                let is_desktop = size.width >= 900.0;

                log::info!(
                    "WindowResized: {}x{} (was_desktop: {}, is_desktop: {})",
                    size.width,
                    size.height,
                    was_desktop,
                    is_desktop
                );

                // Mobile Navigation Protocol: Auto-close sidebar when resizing from desktop to slim
                if was_desktop && !is_desktop {
                    log::info!(" -> Resized to Slim: Auto-closing sidebar");
                    self.show_sidebar = false;
                }

                // Desktop Navigation Protocol: Auto-show sidebar when resizing from slim to desktop
                // This ensures NavigationSplitView behaves like SwiftUI with proper adaptive layout
                if !was_desktop && is_desktop {
                    log::info!(" -> Resized to Desktop: Auto-showing sidebar");
                    self.show_sidebar = true;
                }

                self.window_width = size.width;
                Task::none()
            }
            Message::FontLoaded(_) => Task::none(),
            Message::Heartbeat => Task::none(),
            Message::SudoRequest(action) => {
                self.pending_sudo_action = Some(action);
                Task::none()
            }
            Message::SudoApprove => {
                if let Some(action) = self.pending_sudo_action.take() {
                    Task::perform(async {}, move |_| *action.message)
                } else {
                    Task::none()
                }
            }
            Message::SudoDeny => {
                self.pending_sudo_action = None;
                Task::none()
            }
            Message::ExecuteShell(cmd) => {
                log::info!("EXECUTING SHELL COMMAND: {}", cmd);
                // For safety in this demo, we just log it.
                // In a real PeakOS context, this would hit the syscall layer.
                self.chat_messages.push(ChatMessage {
                    role: ChatRole::System,
                    content: format!("Shell command executed securely: `{}`", cmd),
                });
                Task::none()
            }
            Message::None => Task::none(),

            Message::Chat(msg) => match msg {
                ChatViewMessage::InputChanged(val) => {
                    self.chat_input = val;
                    Task::none()
                }
                ChatViewMessage::CopyCode(code) => {
                    iced::clipboard::write::<Message>(code).map(|_| Message::None)
                }
                ChatViewMessage::SendPressed => {
                    let content = self.chat_input.trim().to_string();
                    if !content.is_empty() {
                        self.chat_input.clear();
                        return self.start_ai_chat(content);
                    }
                    Task::none()
                }
            },
            Message::ChatStreamUpdate(result) => match result {
                Ok(text) => {
                    if let Some(last) = self.chat_messages.last_mut() {
                        if last.role == ChatRole::Assistant {
                            last.content = text.clone();
                        } else {
                            self.chat_messages.push(ChatMessage {
                                role: ChatRole::Assistant,
                                content: text.clone(),
                            });
                        }
                    } else {
                        self.chat_messages.push(ChatMessage {
                            role: ChatRole::Assistant,
                            content: text.clone(),
                        });
                    }

                    // On Stream Completion (e.g. text is essentially complete or stream finishes)
                    // We check if it looks like the end, or wait for AIResponse logic.
                    // For now, let's process actions in AIResponse or if stream results in Ok("") which signals end
                    Task::none()
                }
                Err(e) => {
                    self.is_thinking = false;
                    self.chat_messages.push(ChatMessage {
                        role: ChatRole::System,
                        content: format!("Error: {}", e),
                    });
                    Task::none()
                }
            },
            Message::AIChatComplete => {
                self.is_thinking = false;
                if let Some(last) = self.chat_messages.last() {
                    if last.role == ChatRole::Assistant {
                        let content = last.content.clone();
                        return self.process_assistant_actions(&content);
                    }
                }
                Task::none()
            }
            Message::AIResponse(res) => {
                self.is_thinking = false;
                match res {
                    Ok(content) => {
                        let task = self.process_assistant_actions(&content);

                        if !content.is_empty() {
                            // Sync with streaming message or push new
                            if let Some(last) = self.chat_messages.last_mut() {
                                if last.role == ChatRole::Assistant {
                                    last.content = content.clone();
                                } else {
                                    self.chat_messages.push(ChatMessage {
                                        role: ChatRole::Assistant,
                                        content: content.clone(),
                                    });
                                }
                            } else {
                                self.chat_messages.push(ChatMessage {
                                    role: ChatRole::Assistant,
                                    content: content.clone(),
                                });
                            }
                        }
                        return task;
                    }
                    Err(err) => {
                        self.chat_messages.push(ChatMessage {
                            role: ChatRole::System,
                            content: format!("Error: {}", err),
                        });
                        Task::none()
                    }
                }
            }
            Message::LoadMoreIcons => {
                self.icon_limit += 100;
                Task::none()
            }
        }
    }

    pub fn start_ai_chat(&mut self, content: String) -> Task<Message> {
        self.chat_messages.push(ChatMessage {
            role: ChatRole::User,
            content: content.clone(),
        });

        // TOKEN OPTIMIZATION & SAFETY SWITCH
        const MAX_CONTEXT_CHARS: usize = 16_000;
        const MAX_HISTORY_MESSAGES: usize = 20;

        let system_prompt = self.get_system_prompt();
        let system_chars = system_prompt.len();

        if system_chars > MAX_CONTEXT_CHARS {
            self.chat_messages.push(ChatMessage {
                role: ChatRole::System,
                content: format!(
                    "Error: Context is too large ({} chars). Please reduce the complexity of the current view.",
                    system_chars
                ),
            });
            return Task::none();
        }

        let mut available_chars = MAX_CONTEXT_CHARS.saturating_sub(system_chars);
        let mut history_messages = Vec::new();

        for msg in self.chat_messages.iter().rev().take(MAX_HISTORY_MESSAGES) {
            let msg_len = msg.content.len();
            if msg_len <= available_chars {
                history_messages.push(msg);
                available_chars -= msg_len;
            } else {
                break;
            }
        }

        let mut history: Vec<crate::core::ChatCompletionMessage> = history_messages
            .into_iter()
            .rev()
            .map(|m| crate::core::ChatCompletionMessage {
                role: match m.role {
                    ChatRole::System => "system".to_string(),
                    ChatRole::User => "user".to_string(),
                    ChatRole::Assistant => "assistant".to_string(),
                },
                content: m.content.clone(),
            })
            .collect();

        history.insert(
            0,
            crate::core::ChatCompletionMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
        );

        self.is_thinking = true;
        let stream = self.intelligence.chat_stream(history);

        use iced::futures::StreamExt;
        let mapped_stream =
            stream
                .map(|res| Message::ChatStreamUpdate(res))
                .chain(iced::futures::stream::once(async {
                    Message::AIChatComplete
                }));

        Task::stream(mapped_stream)
    }

    pub fn process_assistant_actions(&mut self, text: &str) -> Task<Message> {
        let mut tasks = Vec::new();
        let actions = crate::reference::intelligence::ActionParser::parse_text(text);

        for action in actions {
            log::info!("AI Action Detected: {:?}", action);

            // NEURAL SUDO INTERCEPTION
            if action.is_protected() {
                log::warn!("Neural Sudo: Intercepting protected action: {:?}", action);
                let reason = action
                    .protection_reason()
                    .unwrap_or_else(|| "Protected action requested".to_string());

                // Convert action to the message it WOULD have sent
                let target_msg = match action {
                    Action::Shell(cmd) => Message::ExecuteShell(cmd),
                    Action::Navigate(page) => Message::SetTab(page.clone()),
                    _ => Message::None,
                };

                if !matches!(target_msg, Message::None) {
                    tasks.push(Task::perform(async {}, move |_| {
                        Message::SudoRequest(SudoAction {
                            message: Box::new(target_msg.clone()),
                            reason: reason.clone(),
                        })
                    }));
                }
                continue; // Skip normal execution
            }

            match action {
                Action::Navigate(page) => {
                    log::info!(" -> Navigating to: {:?}", page);
                    tasks.push(Task::perform(async {}, move |_| {
                        Message::SetTab(page.clone())
                    }));
                }
                Action::SetButtonVariant(variant) => {
                    self.button_lab.variant = variant;
                }
                Action::SetButtonIntent(intent) => {
                    self.button_lab.intent = intent;
                }
                Action::SetThemeKind(kind) => {
                    self.theme = kind;
                }
                Action::SetThemeTone(tone) => {
                    self.theme_tone = tone;
                }
                Action::SetLabMode(mode) => {
                    self.render_mode = mode;
                }
                Action::Shell(_) => {
                    // Handled by Neural Sudo Interception above
                }
                Action::Memorize(content) => {
                    log::info!("AI MEMORIZING: {}", content);
                    let db = self.db.clone();
                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    let record = crate::core::SemanticRecord {
                        id: format!("mem-{}", timestamp),
                        collection: "AI Memory".to_string(),
                        content: content.clone(),
                        vector: None,
                        metadata: serde_json::json!({}),
                        timestamp,
                    };
                    tasks.push(db.save(record).map(|_| Message::None));
                }
                Action::Unknown(name) => {
                    log::warn!("Unknown AI Action: {}", name);
                }
                Action::Teleport { .. } | Action::Scale { .. } | Action::Rotate { .. } => {
                    log::info!("AI Spatial Action received: {:?}", action);
                }
            }
        }

        if tasks.is_empty() {
            Task::none()
        } else {
            Task::batch(tasks)
        }
    }

    fn get_system_prompt(&self) -> String {
        // ... (existing prompt generation)
        let ctx = Context::new(
            crate::core::ShellMode::Desktop,
            ThemeTokens::new(PeakTheme::Peak, ThemeTone::Light),
            iced::Size::new(1280.0, 800.0),
            self.localization.clone(),
        );
        let view = crate::reference::views::ContentView::new(self);
        let tree = view.describe(&ctx);
        // Call the accessibility bridge to update platform/log data
        self.a11y.update(&tree);
        // MINIFICATION: Use to_string instead of to_string_pretty
        let ui_json = serde_json::to_string(&tree).unwrap_or_default();

        format!(
            "You are the PeakUI AI Assistant. You are a helpful companion for the user.\n\n\
             UI CONTEXT:\n\
             - Provider: {}\n\
             - Viewport: 1280x800 (Desktop)\n\
             - Current UI Structure (Dense JSON):\n{}\n\n\
             GOAL: Help the user explore the PeakUI framework. Respond conversationally and use actions when needed.",
            self.peak_id, ui_json
        )
    }

    pub fn view(&self) -> Element<'_, Message> {
        // Debug logging disabled for performance - was causing console spam
        // log::info!(
        //     "App::view: {} Mode, width: {}, show_sidebar: {}",
        //     mode,
        //     self.window_width,
        //     self.show_sidebar
        // );
        let mode = if self.window_width < 900.0 {
            ShellMode::Mobile
        } else {
            ShellMode::Desktop
        };
        let tone = self.theme_tone;
        let tokens = ThemeTokens::with_theme(self.theme, tone);

        if self.show_landing {
            // Create context directly without responsive wrapper for performance
            let size = iced::Size::new(self.window_width, 800.0); // Height doesn't matter for landing
            let context = Context::new(mode, tokens, size, self.localization.clone());

            // Capture the search query state
            let query = self.search_query.clone();
            let typewriter_text = self.typewriter_text.clone();
            let active_tab = self.active_tab.clone();
            let db_records = self.db.get_all();

            let content: Element<'_, Message> = match &active_tab {
                Page::PeakOSDetail => {
                    crate::reference::pages::landing::peak_os::view(&context, context.is_slim())
                        .view
                        .view(&context)
                        .into()
                }
                Page::PeakUIDetail => {
                    crate::reference::pages::landing::peak_ui::view(&context, context.is_slim())
                        .view
                        .view(&context)
                        .into()
                }
                Page::PeakDBDetail => crate::reference::pages::landing::peak_db::view(
                    &context,
                    context.is_slim(),
                    db_records,
                )
                .view
                .view(&context)
                .into(),
                Page::PeakRelayDetail => {
                    crate::reference::pages::landing::peak_relay::view(&context, context.is_slim())
                        .view
                        .view(&context)
                        .into()
                }
                Page::PeakHubDetail => {
                    crate::reference::pages::landing::peak_hub::view(&context, context.is_slim())
                        .view
                        .view(&context)
                        .into()
                }
                _ => crate::reference::pages::landing::view(&context, &query, &typewriter_text)
                    .into(),
            };

            return iced::widget::container(content)
                .style(move |_| iced::widget::container::Style {
                    background: Some(tokens.colors.background.into()),
                    ..Default::default()
                })
                .into();
        }

        // 1. Prepare Content
        let content = super::views::ContentView::new(self);

        let context_menu_pos = self.context_menu_pos;

        // Neural Export (Exported in update for performance)

        let peak_id = self.peak_id.clone();
        crate::core::responsive(
            mode,
            tokens.clone(),
            self.localization.clone(),
            move |mut context| {
                context.peak_id = peak_id.clone();
                // Main App Content
                let base_content = iced::widget::container(content.view(&context))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(move |_| iced::widget::container::Style {
                        background: Some(tokens.colors.background.into()),
                        ..Default::default()
                    });

                let mut stack = iced::widget::stack![base_content]
                    .width(Length::Fill)
                    .height(Length::Fill);

                // Overlay Context Menu
                if let Some(pos) = context_menu_pos {
                    let menu = crate::views::ContextMenu::new()
                        .item(
                            "Reload",
                            "rotate-cw",
                            Message::ContextMenuAction("Reload".to_string()),
                        )
                        .item(
                            "Inspect",
                            "search-code",
                            Message::ContextMenuAction("Inspect".to_string()),
                        )
                        .item("Close", "circle-x", Message::CloseContextMenu);

                    stack = stack.push(
                        iced::widget::container(menu.view(&context))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .padding(iced::Padding {
                                top: pos.y,
                                left: pos.x,
                                ..Default::default()
                            }),
                    );
                }

                // Overlay Sudo Prompt
                if let Some(sudo) = &content.pending_sudo_action {
                    let prompt = iced::widget::container(
                        iced::widget::column![
                            iced::widget::text("Neural Sudo Permission").size(24),
                            iced::widget::Space::new().height(10),
                            iced::widget::text(format!("AI wants to perform an action:")).size(16),
                            iced::widget::text(format!("{:?}", sudo.message))
                                .size(14)
                                .color(tokens.colors.primary),
                            iced::widget::Space::new().height(5),
                            iced::widget::text(format!("Reason: {}", sudo.reason))
                                .size(14)
                                .size(14),
                            iced::widget::Space::new().height(20),
                            iced::widget::row![
                                iced::widget::button("Deny")
                                    .padding(10)
                                    .on_press(Message::SudoDeny),
                                iced::widget::Space::new().width(10),
                                iced::widget::button("Approve")
                                    .padding(10)
                                    .on_press(Message::SudoApprove),
                            ]
                        ]
                        .align_x(iced::Alignment::Center),
                    )
                    .width(400.0)
                    .padding(30)
                    .style(move |_| iced::widget::container::Style {
                        background: Some(tokens.colors.surface.into()),
                        border: iced::Border {
                            radius: 12.0.into(),
                            width: 1.0,
                            color: tokens.colors.border,
                        },
                        shadow: iced::Shadow {
                            color: Color::BLACK,
                            offset: iced::Vector::new(0.0, 10.0),
                            blur_radius: 30.0,
                        },
                        ..Default::default()
                    });

                    stack = stack.push(
                        iced::widget::container(prompt)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center_x(Length::Fill)
                            .center_y(Length::Fill)
                            .style(move |_| iced::widget::container::Style {
                                background: Some(Background::Color(Color {
                                    a: 0.5,
                                    ..tokens.colors.background
                                })),
                                ..Default::default()
                            }),
                    );
                }

                // Render Window Chrome ON TOP of everything (or wrapping)
                // Since we want the notch button to be clickable, and it's in the chrome.

                let content_view: Element<'_, Message> = stack.into();
                content_view
            },
        )
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        let events = iced::event::listen().map(|event| {
            // Debug logging disabled for performance
            // if let iced::Event::Keyboard(_) = event {
            //     log::info!("RAW EVENT: {:?}", event);
            // }
            match event {
                // Cursor tracking disabled - was causing re-render on every mouse move
                // iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
                //     Message::UpdateCursorPos(position)
                // }
                iced::Event::Mouse(iced::mouse::Event::ButtonPressed(
                    iced::mouse::Button::Right,
                )) => Message::OpenContextMenu(iced::Point::ORIGIN),
                iced::Event::Mouse(iced::mouse::Event::ButtonPressed(
                    iced::mouse::Button::Left,
                )) => Message::CloseContextMenu,
                _ => Message::None,
            }
        });

        let hotkeys = iced::event::listen().map(|event| {
            if let iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key, modifiers, ..
            }) = event
            {
                let _is_cmd = modifiers.command() || modifiers.logo();
                let _is_ctrl = modifiers.control();

                let _is_backspace = matches!(
                    key,
                    iced::keyboard::Key::Named(iced::keyboard::key::Named::Backspace)
                );
                let _is_delete_forward = matches!(
                    key,
                    iced::keyboard::Key::Named(iced::keyboard::key::Named::Delete)
                );
                let _is_d =
                    matches!(key, iced::keyboard::Key::Character(ref c) if c.as_str() == "d");
                let _is_u =
                    matches!(key, iced::keyboard::Key::Character(ref c) if c.as_str() == "u");

                // if is_backspace && is_cmd {
                //    return Message::Back;
                // }

                // Cmd+D -> Toggle Dark Mode
                // if is_d && is_cmd {
                //    return Message::ToggleTheme;
                // }

                // Ctrl+U -> Close Context Menu
                if _is_u && _is_ctrl {
                    return Message::CloseContextMenu;
                }
            }
            Message::None
        });
        let window_events = iced::event::listen_with(|event, _status, _window| match event {
            iced::Event::Window(iced::window::Event::Resized(size)) => {
                Some(Message::WindowResized(size))
            }
            _ => None,
        });

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;

            let hash_sub = iced::Subscription::run(|| {
                let (sender, receiver) = iced::futures::channel::mpsc::channel(1);
                let window = web_sys::window().expect("window not found");

                let on_hash_change = wasm_bindgen::prelude::Closure::wrap(Box::new(move || {
                    let hash = web_sys::window()
                        .and_then(|w| w.location().hash().ok())
                        .unwrap_or_default();

                    let path = if hash.starts_with('#') {
                        &hash[1..]
                    } else {
                        &hash
                    };

                    let page = Page::from_path(path);

                    // Defer the message sending to the next event loop tick
                    // to avoid RefCell borrowing conflicts in winit/iced
                    let mut sender = sender.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let _ = sender.try_send(Message::SetTab(page));
                    });
                })
                    as Box<dyn FnMut()>);

                window.set_onhashchange(Some(on_hash_change.as_ref().unchecked_ref()));
                on_hash_change.forget(); // Keep closure alive

                receiver
            });

            iced::Subscription::batch(vec![
                events,
                hash_sub,
                hotkeys,
                window_events,
                iced::time::every(std::time::Duration::from_millis(100))
                    .map(|_| Message::Heartbeat),
            ])
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let command_sub = iced::Subscription::run(|| {
                let (mut sender, receiver) = iced::futures::channel::mpsc::channel(1);

                tokio::spawn(async move {
                    loop {
                        if let Ok(content) = std::fs::read_to_string(".peak/command.json") {
                            if let Ok(cmd) = serde_json::from_str::<Command>(&content) {
                                let _ = sender.try_send(cmd.into_message());
                                let _ = std::fs::remove_file(".peak/command.json");
                            }
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                    }
                });

                receiver
            });

            iced::Subscription::batch(vec![
                events,
                command_sub,
                hotkeys,
                window_events,
                iced::time::every(std::time::Duration::from_millis(100))
                    .map(|_| Message::Heartbeat),
                iced::time::every(std::time::Duration::from_millis(100))
                    .map(Message::TypewriterTick),
            ])
        }
    }
}
