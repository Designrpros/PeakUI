use super::model::Page;
use crate::prelude::*;
use crate::views::chat::{ChatMessage, ChatRole, ChatViewMessage};
use peak_core::registry::ShellMode;
use peak_theme::{PeakTheme, ThemeTokens, ThemeTone};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum InspectorTab {
    #[default]
    App,
    Feature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum RenderMode {
    #[default]
    Canvas,
    Terminal,
    Neural,
    Spatial,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct Settings {
    pub api_key: String,
}

impl Settings {
    pub fn load() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(content) = std::fs::read_to_string(".peak/settings.json") {
                if let Ok(settings) = serde_json::from_str::<Settings>(&content) {
                    return settings;
                }
            }
        }

        // Fallback to environment variable (useful for production/WASM builds)
        let api_key = option_env!("OPENROUTER_API_KEY")
            .map(|s| s.to_string())
            .unwrap_or_default();

        Self { api_key }
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

    // Infinite Scroll / Lazy Load
    pub icon_limit: usize,
    pub window_width: f32,
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
}

#[derive(Debug, Clone)]
pub struct SizingLabState {
    pub width_type: SizingType,
    pub height_type: SizingType,
    pub fixed_width: f32,
    pub fixed_height: f32,
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

    // Chat
    Chat(ChatViewMessage),
    AIResponse(std::result::Result<String, String>),
    SetInspectorTab(InspectorTab),
    SetApiKey(String),

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

    // Sizing Lab Messages
    UpdateSizingWidthType(SizingType),
    UpdateSizingHeightType(SizingType),
    UpdateSizingFixedWidth(f32),
    UpdateSizingFixedHeight(f32),

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

    // Sizing Lab
    UpdateSizingWidthType(SizingType),
    UpdateSizingHeightType(SizingType),
    UpdateSizingFixedWidth(f32),
    UpdateSizingFixedHeight(f32),

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

            // Sizing Lab
            Command::UpdateSizingWidthType(t) => Message::UpdateSizingWidthType(t),
            Command::UpdateSizingHeightType(t) => Message::UpdateSizingHeightType(t),
            Command::UpdateSizingFixedWidth(w) => Message::UpdateSizingFixedWidth(w),
            Command::UpdateSizingFixedHeight(h) => Message::UpdateSizingFixedHeight(h),

            Command::SetInspectorTab(tab) => Message::SetInspectorTab(tab),
            Command::SetApiKey(key) => Message::SetApiKey(key),
            Command::None => Message::None,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        let settings = Settings::load();
        Self {
            active_tab: Page::Introduction,
            show_search: false,
            show_inspector: false,
            show_sidebar: true,
            show_user_profile: false,
            navigation_mode: "Start".to_string(),
            search_query: "".to_string(),
            expanded_sections: ["COMPONENTS".to_string()].into_iter().collect(),
            theme_tone: ThemeTone::Light,
            theme: PeakTheme::Peak,
            button_lab: ButtonLabState::default(),
            typography_lab: TypographyLabState::default(),
            layout_lab: LayoutLabState::default(),
            sizing_lab: SizingLabState::default(),
            render_mode: RenderMode::Canvas,
            show_landing: true,
            sidebar_width: 240.0,
            inspector_width: 300.0,
            inspector_tab: InspectorTab::default(),
            is_resizing_sidebar: false,
            is_resizing_inspector: false,
            context_menu_pos: None,
            last_cursor_pos: iced::Point::ORIGIN,
            show_chat_overlay: false,
            chat_messages: vec![ChatMessage {
                role: ChatRole::System,
                content:
                    "Hello! I am your AI Assistant. I can see the screen and help you navigate."
                        .to_string(),
            }],
            chat_input: String::new(),
            api_key: settings.api_key,
            icon_limit: 50,
            window_width: 1024.0, // Default assume desktop until resized
        }
    }
}

pub use crate::core::{Context, DeviceType};

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EnterApp => {
                self.show_landing = false;
                self.show_sidebar = true;
                Task::none()
            }
            Message::SetTab(tab) => {
                log::debug!(
                    "Setting Tab: {:?} (Category: {})",
                    tab,
                    tab.navigation_mode()
                );
                self.navigation_mode = tab.navigation_mode();
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

                // Mobile Navigation Protocol: Auto-close sidebar on navigation if on mobile
                if self.window_width < 900.0 {
                    self.show_sidebar = false;
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
                Task::none()
            }
            Message::ToggleUserProfile => {
                self.show_user_profile = !self.show_user_profile;
                Task::none()
            }
            Message::SetNavigationMode(mode) => {
                self.navigation_mode = mode.clone();
                self.active_tab = match mode.as_str() {
                    "Start" => Page::Introduction,
                    "Catalog" => Page::Button,
                    "Data" => Page::PeakDB,
                    "Settings" => Page::Appearance,
                    _ => self.active_tab.clone(),
                };
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
                let settings = Settings { api_key: key };
                settings.save();
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
                self.window_width = size.width;
                Task::none()
            }
            Message::FontLoaded(_) => Task::none(),

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
                        self.chat_messages.push(ChatMessage {
                            role: ChatRole::User,
                            content: content.clone(),
                        });
                        self.chat_input.clear();

                        // TOKEN OPTIMIZATION & SAFETY SWITCH

                        // 8k context window (~32k chars) is a safe upper bound for many models,
                        // but to be safe with rate limits, let's aim for 4k tokens (~16k chars).
                        const MAX_CONTEXT_CHARS: usize = 16_000;
                        const MAX_HISTORY_MESSAGES: usize = 20; // Hard cap on count

                        let system_prompt = self.get_system_prompt();
                        let system_chars = system_prompt.len();

                        // SAFETY SWITCH: If system prompt alone is too huge, abort.
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

                        // Collect messages from newest to oldest, fitting into the budget
                        for msg in self.chat_messages.iter().rev().take(MAX_HISTORY_MESSAGES) {
                            let msg_len = msg.content.len();
                            if msg_len <= available_chars {
                                history_messages.push(msg);
                                available_chars -= msg_len;
                            } else {
                                // Message too long to fit, stop collecting history
                                break;
                            }
                        }

                        // We collected reverse, so reverse back to chronological order
                        let mut history: Vec<crate::reference::ai::ChatCompletionMessage> =
                            history_messages
                                .into_iter()
                                .rev()
                                .map(|m| crate::reference::ai::ChatCompletionMessage {
                                    role: match m.role {
                                        ChatRole::System => "system".to_string(),
                                        ChatRole::User => "user".to_string(),
                                        ChatRole::Assistant => "assistant".to_string(),
                                    },
                                    content: m.content.clone(),
                                })
                                .collect();

                        // Inject REAL system context at the front
                        history.insert(
                            0,
                            crate::reference::ai::ChatCompletionMessage {
                                role: "system".to_string(),
                                content: self.get_system_prompt(),
                            },
                        );

                        let api_key = self.api_key.clone();

                        return Task::perform(
                            async move {
                                let client = crate::reference::ai::OpenRouterClient::new(api_key);
                                client.chat(history).await
                            },
                            Message::AIResponse,
                        );
                    }
                    Task::none()
                }
            },
            Message::AIResponse(res) => {
                match res {
                    Ok(mut content) => {
                        // Action Bridge: Check for structured actions in the response
                        // Format: [Action: Navigate(Introduction)]
                        if let Some(start) = content.find("[Action: Navigate(") {
                            if let Some(open_paren) = content[start..].find('(') {
                                if let Some(close_paren) = content[start + open_paren..].find(")]")
                                {
                                    let action_text = content
                                        [start + open_paren + 1..start + open_paren + close_paren]
                                        .trim();
                                    log::info!("AI Action: Navigate to {}", action_text);

                                    let full_tag =
                                        &content[start..start + open_paren + close_paren + 2];
                                    let clean_content =
                                        content.replace(full_tag, "").trim().to_string();

                                    let page: crate::reference::model::Page =
                                        action_text.to_string().into();
                                    self.active_tab = page.clone();
                                    self.navigation_mode = page.navigation_mode();
                                    self.show_landing = false;
                                    content = clean_content;
                                }
                            }
                        }

                        // Action Bridge: SetButtonVariant
                        if let Some(start) = content.find("[Action: SetButtonVariant(") {
                            if let Some(open_paren) = content[start..].find('(') {
                                if let Some(close_paren) = content[start + open_paren..].find(")]")
                                {
                                    let action_text = content
                                        [start + open_paren + 1..start + open_paren + close_paren]
                                        .trim();
                                    log::info!("AI Action: Set Button Variant: {}", action_text);

                                    let variant = match action_text.to_lowercase().as_str() {
                                        "solid" => crate::prelude::Variant::Solid,
                                        "soft" => crate::prelude::Variant::Soft,
                                        "outline" => crate::prelude::Variant::Outline,
                                        "ghost" => crate::prelude::Variant::Ghost,
                                        _ => crate::prelude::Variant::Solid,
                                    };
                                    self.button_lab.variant = variant;

                                    let full_tag =
                                        &content[start..start + open_paren + close_paren + 2];
                                    content = content.replace(full_tag, "").trim().to_string();
                                }
                            }
                        }

                        // Action Bridge: SetButtonIntent
                        if let Some(start) = content.find("[Action: SetButtonIntent(") {
                            if let Some(open_paren) = content[start..].find('(') {
                                if let Some(close_paren) = content[start + open_paren..].find(")]")
                                {
                                    let action_text = content
                                        [start + open_paren + 1..start + open_paren + close_paren]
                                        .trim();
                                    log::info!("AI Action: Set Button Intent: {}", action_text);

                                    let intent = match action_text.to_lowercase().as_str() {
                                        "primary" => crate::prelude::Intent::Primary,
                                        "secondary" => crate::prelude::Intent::Secondary,
                                        "success" => crate::prelude::Intent::Success,
                                        "warning" => crate::prelude::Intent::Warning,
                                        "danger" => crate::prelude::Intent::Danger,
                                        "info" => crate::prelude::Intent::Info,
                                        "neutral" => crate::prelude::Intent::Neutral,
                                        _ => crate::prelude::Intent::Primary,
                                    };
                                    self.button_lab.intent = intent;

                                    let full_tag =
                                        &content[start..start + open_paren + close_paren + 2];
                                    content = content.replace(full_tag, "").trim().to_string();
                                }
                            }
                        }

                        // Action Bridge: SetThemeKind
                        if let Some(start) = content.find("[Action: SetThemeKind(") {
                            if let Some(open_paren) = content[start..].find('(') {
                                if let Some(close_paren) = content[start + open_paren..].find(")]")
                                {
                                    let action_text = content
                                        [start + open_paren + 1..start + open_paren + close_paren]
                                        .trim();
                                    log::info!("AI Action: Set Theme Kind: {}", action_text);

                                    let theme = match action_text.to_lowercase().as_str() {
                                        "peak" => PeakTheme::Peak,
                                        "mountain" => PeakTheme::Mountain,
                                        "cupertino" => PeakTheme::Cupertino,
                                        "smart" => PeakTheme::Smart,
                                        "material" => PeakTheme::Material,
                                        "fluent" => PeakTheme::Fluent,
                                        "highcontrast" => PeakTheme::HighContrast,
                                        _ => PeakTheme::Peak,
                                    };
                                    self.theme = theme;

                                    let full_tag =
                                        &content[start..start + open_paren + close_paren + 2];
                                    content = content.replace(full_tag, "").trim().to_string();
                                }
                            }
                        }

                        // Action Bridge: SetLabMode
                        if let Some(start) = content.find("[Action: SetLabMode(") {
                            if let Some(open_paren) = content[start..].find('(') {
                                if let Some(close_paren) = content[start + open_paren..].find(")]")
                                {
                                    let action_text = content
                                        [start + open_paren + 1..start + open_paren + close_paren]
                                        .trim();
                                    log::info!("AI Action: Set Lab Mode: {}", action_text);

                                    let mode = match action_text.to_lowercase().as_str() {
                                        "canvas" => Some(RenderMode::Canvas),
                                        "terminal" => Some(RenderMode::Terminal),
                                        "neural" => Some(RenderMode::Neural),
                                        "spatial" => Some(RenderMode::Spatial),
                                        _ => None,
                                    };

                                    if let Some(m) = mode {
                                        self.render_mode = m;
                                    }

                                    let full_tag =
                                        &content[start..start + open_paren + close_paren + 2];
                                    content = content.replace(full_tag, "").trim().to_string();
                                }
                            }
                        }

                        // Action Bridge: SetThemeTone
                        if let Some(start) = content.find("[Action: SetThemeTone(") {
                            if let Some(open_paren) = content[start..].find('(') {
                                if let Some(close_paren) = content[start + open_paren..].find(")]")
                                {
                                    let action_text = content
                                        [start + open_paren + 1..start + open_paren + close_paren]
                                        .trim();
                                    log::info!("AI Action: Set Theme Tone: {}", action_text);

                                    let tone = match action_text.to_lowercase().as_str() {
                                        "light" | "lightmode" => ThemeTone::Light,
                                        "dark" | "darkmode" => ThemeTone::Dark,
                                        _ => ThemeTone::Light,
                                    };
                                    self.theme_tone = tone;

                                    let full_tag =
                                        &content[start..start + open_paren + close_paren + 2];
                                    content = content.replace(full_tag, "").trim().to_string();
                                }
                            }
                        }

                        // Action Bridge: SetChildCount
                        if let Some(start) = content.find("[Action: SetChildCount(") {
                            if let Some(open_paren) = content[start..].find('(') {
                                if let Some(close_paren) = content[start + open_paren..].find(")]")
                                {
                                    let action_text = content
                                        [start + open_paren + 1..start + open_paren + close_paren]
                                        .trim();
                                    if let Ok(count) = action_text.parse::<usize>() {
                                        log::info!("AI Action: Set Child Count: {}", count);
                                        self.layout_lab.child_count = count.min(10);
                                    }

                                    let full_tag =
                                        &content[start..start + open_paren + close_paren + 2];
                                    content = content.replace(full_tag, "").trim().to_string();
                                }
                            }
                        }

                        // Action Bridge: SetSpacing
                        if let Some(start) = content.find("[Action: SetSpacing(") {
                            if let Some(open_paren) = content[start..].find('(') {
                                if let Some(close_paren) = content[start + open_paren..].find(")]")
                                {
                                    let action_text = content
                                        [start + open_paren + 1..start + open_paren + close_paren]
                                        .trim();
                                    if let Ok(spacing) = action_text.parse::<f32>() {
                                        log::info!("AI Action: Set Spacing: {}", spacing);
                                        self.layout_lab.inner_spacing = spacing;
                                        self.layout_lab.outer_spacing = spacing;
                                    }

                                    let full_tag =
                                        &content[start..start + open_paren + close_paren + 2];
                                    content = content.replace(full_tag, "").trim().to_string();
                                }
                            }
                        }

                        // Action Bridge: SetAlignment
                        if let Some(start) = content.find("[Action: SetAlignment(") {
                            if let Some(open_paren) = content[start..].find('(') {
                                if let Some(close_paren) = content[start + open_paren..].find(")]")
                                {
                                    let action_text = content
                                        [start + open_paren + 1..start + open_paren + close_paren]
                                        .trim();
                                    log::info!("AI Action: Set Alignment: {}", action_text);

                                    let alignment = match action_text.to_lowercase().as_str() {
                                        "start" => Alignment::Start,
                                        "center" => Alignment::Center,
                                        "end" => Alignment::End,
                                        _ => Alignment::Start,
                                    };
                                    self.layout_lab.alignment = alignment;

                                    let full_tag =
                                        &content[start..start + open_paren + close_paren + 2];
                                    content = content.replace(full_tag, "").trim().to_string();
                                }
                            }
                        }

                        // Action Bridge: SetTypographySize
                        if let Some(start) = content.find("[Action: SetTypographySize(") {
                            if let Some(open_paren) = content[start..].find('(') {
                                if let Some(close_paren) = content[start + open_paren..].find(")]")
                                {
                                    let action_text = content
                                        [start + open_paren + 1..start + open_paren + close_paren]
                                        .trim();
                                    if let Ok(size) = action_text.parse::<f32>() {
                                        log::info!("AI Action: Set Typography Size: {}", size);
                                        self.typography_lab.size = size;
                                    }

                                    let full_tag =
                                        &content[start..start + open_paren + close_paren + 2];
                                    content = content.replace(full_tag, "").trim().to_string();
                                }
                            }
                        }

                        // Action Bridge: SetTypographyText
                        if let Some(start) = content.find("[Action: SetTypographyText(") {
                            if let Some(open_paren) = content[start..].find('(') {
                                if let Some(close_paren) = content[start + open_paren..].find(")]")
                                {
                                    let action_text = content
                                        [start + open_paren + 1..start + open_paren + close_paren]
                                        .trim();
                                    log::info!("AI Action: Set Typography Text: {}", action_text);
                                    self.typography_lab.text = action_text.to_string();

                                    let full_tag =
                                        &content[start..start + open_paren + close_paren + 2];
                                    content = content.replace(full_tag, "").trim().to_string();
                                }
                            }
                        }

                        // Action Bridge: SetTypographyBold
                        if let Some(start) = content.find("[Action: SetTypographyBold(") {
                            if let Some(open_paren) = content[start..].find('(') {
                                if let Some(close_paren) = content[start + open_paren..].find(")]")
                                {
                                    let action_text = content
                                        [start + open_paren + 1..start + open_paren + close_paren]
                                        .trim()
                                        .to_lowercase();
                                    log::info!("AI Action: Set Typography Bold: {}", action_text);
                                    let bold = action_text == "true"
                                        || action_text == "yes"
                                        || action_text == "on"
                                        || action_text == "bold";
                                    self.typography_lab.is_bold = bold;

                                    let full_tag =
                                        &content[start..start + open_paren + close_paren + 2];
                                    content = content.replace(full_tag, "").trim().to_string();
                                }
                            }
                        }

                        // Action Bridge: SetTypographyWeight (Alias for Bold)
                        if let Some(start) = content.find("[Action: SetTypographyWeight(") {
                            if let Some(open_paren) = content[start..].find('(') {
                                if let Some(close_paren) = content[start + open_paren..].find(")]")
                                {
                                    let action_text = content
                                        [start + open_paren + 1..start + open_paren + close_paren]
                                        .trim()
                                        .to_lowercase();
                                    log::info!("AI Action: Set Typography Weight: {}", action_text);
                                    let bold = action_text == "bold"
                                        || action_text == "heavy"
                                        || action_text == "black";
                                    self.typography_lab.is_bold = bold;

                                    let full_tag =
                                        &content[start..start + open_paren + close_paren + 2];
                                    content = content.replace(full_tag, "").trim().to_string();
                                }
                            }
                        }

                        // Action Bridge: SetTypographyItalic
                        if let Some(start) = content.find("[Action: SetTypographyItalic(") {
                            if let Some(open_paren) = content[start..].find('(') {
                                if let Some(close_paren) = content[start + open_paren..].find(")]")
                                {
                                    let action_text = content
                                        [start + open_paren + 1..start + open_paren + close_paren]
                                        .trim()
                                        .to_lowercase();
                                    log::info!("AI Action: Set Typography Italic: {}", action_text);
                                    let italic = action_text == "true"
                                        || action_text == "yes"
                                        || action_text == "on"
                                        || action_text == "italic";
                                    self.typography_lab.is_italic = italic;

                                    let full_tag =
                                        &content[start..start + open_paren + close_paren + 2];
                                    content = content.replace(full_tag, "").trim().to_string();
                                }
                            }
                        }

                        // Action Bridge: SetSizingWidth
                        if let Some(start) = content.find("[Action: SetSizingWidth(") {
                            if let Some(open_paren) = content[start..].find('(') {
                                if let Some(close_paren) = content[start + open_paren..].find(")]")
                                {
                                    let action_text = content
                                        [start + open_paren + 1..start + open_paren + close_paren]
                                        .trim();
                                    log::info!("AI Action: Set Sizing Width: {}", action_text);

                                    self.sizing_lab.width_type =
                                        match action_text.to_lowercase().as_str() {
                                            "fixed" => crate::reference::app::SizingType::Fixed,
                                            "fill" => crate::reference::app::SizingType::Fill,
                                            "shrink" => crate::reference::app::SizingType::Shrink,
                                            _ => crate::reference::app::SizingType::Fixed,
                                        };

                                    let full_tag =
                                        &content[start..start + open_paren + close_paren + 2];
                                    content = content.replace(full_tag, "").trim().to_string();
                                }
                            }
                        }

                        // Action Bridge: SetSizingHeight
                        if let Some(start) = content.find("[Action: SetSizingHeight(") {
                            if let Some(open_paren) = content[start..].find('(') {
                                if let Some(close_paren) = content[start + open_paren..].find(")]")
                                {
                                    let action_text = content
                                        [start + open_paren + 1..start + open_paren + close_paren]
                                        .trim();
                                    log::info!("AI Action: Set Sizing Height: {}", action_text);

                                    self.sizing_lab.height_type =
                                        match action_text.to_lowercase().as_str() {
                                            "fixed" => crate::reference::app::SizingType::Fixed,
                                            "fill" => crate::reference::app::SizingType::Fill,
                                            "shrink" => crate::reference::app::SizingType::Shrink,
                                            _ => crate::reference::app::SizingType::Fixed,
                                        };

                                    let full_tag =
                                        &content[start..start + open_paren + close_paren + 2];
                                    content = content.replace(full_tag, "").trim().to_string();
                                }
                            }
                        }

                        if !content.is_empty() {
                            self.chat_messages.push(ChatMessage {
                                role: ChatRole::Assistant,
                                content,
                            });
                        }
                    }
                    Err(err) => {
                        self.chat_messages.push(ChatMessage {
                            role: ChatRole::System,
                            content: format!("Error: {}", err),
                        });
                    }
                }
                Task::none()
            }
            Message::LoadMoreIcons => {
                self.icon_limit += 100;
                Task::none()
            }
            Message::None => Task::none(),
        }
    }

    fn get_system_prompt(&self) -> String {
        // ... (existing prompt generation)
        let ctx = Context::new(
            crate::core::ShellMode::Desktop,
            ThemeTokens::new(PeakTheme::Peak, ThemeTone::Light),
            iced::Size::new(1280.0, 800.0),
        );
        let view = crate::reference::views::ContentView::new(self);
        let tree = view.describe(&ctx);
        // MINIFICATION: Use to_string instead of to_string_pretty
        let ui_json = serde_json::to_string(&tree).unwrap_or_default();

        format!(
            "You are the PeakUI AI Assistant. You have eyes and hands. You can see the app and interact with it.\n\n\
             VIEWPORT: 1280x800 (Desktop)\n\
             CURRENT SCREEN (JSON Structure):\n{}\n\n\
             GOAL: Help the user. If they ask to see a page, go there.\n\n\
             PROTOCOL for ACTIONS:\n\
             - To navigate, use: [Action: Navigate(PAGE_NAME)]\n\
             - To change Button variant, use: [Action: SetButtonVariant(VARIANT)]\n\
             - To change Button intent, use: [Action: SetButtonIntent(INTENT)]\n\
             - To change Theme Kind, use: [Action: SetThemeKind(THEME)]\n\
             - To change Theme Tone, use: [Action: SetThemeTone(TONE)]\n\
             - To switch documentation tabs (Canvas/Terminal/Neural/Spatial), use: [Action: SetLabMode(MODE)]\n\
             - To change Layout child count, use: [Action: SetChildCount(NUMBER)]\n\
             - To change Layout spacing, use: [Action: SetSpacing(NUMBER)]\n\
             - To change Layout alignment, use: [Action: SetAlignment(ALIGNMENT)]\n\
             - To change Typography size, use: [Action: SetTypographySize(NUMBER)]\n\
             - To change Typography content, use: [Action: SetTypographyText(TEXT)]\n\
             - To change Typography style, use: [Action: SetTypographyBold(true/false)], [Action: SetTypographyItalic(true/false)] or [Action: SetTypographyWeight(Bold/Regular)]\n\
             - To change Sizing width type, use: [Action: SetSizingWidth(SIZING)]\n\
             - To change Sizing height type, use: [Action: SetSizingHeight(SIZING)]\n\n\
             VALID VALUES:\n\
             - PAGE_NAME: Introduction, Architecture, Roadmap, Intelligence, Typography, Customizations, Sizing, Layout, Text, Icon, Button, Shapes, PeakDB, PeakCloud, Appearance, AI, About.\n\
             - VARIANT: Solid, Soft, Outline, Ghost.\n\
             - INTENT: Primary, Secondary, Success, Warning, Danger, Info, Neutral.\n\
             - THEME: Peak, Gaming, Cupertino, Terminal, Mountain, Smart, Material, Fluent, HighContrast, Media, Ambient, Automotive.\n\
             - TONE: Light, Dark.\n\
             - MODE: Canvas, Terminal, Neural, Spatial.\n\
             - ALIGNMENT: Start, Center, End.\n\
             - SIZING: Fixed, Fill, Shrink.\n\n\
             EXAMPLES:\n\
             - User: 'Set the button to outline style'\n\
             - Assistant: 'Done! [Action: SetButtonVariant(Outline)]'\n\
             - User: 'Show me 5 boxes in the layout'\n\
             - Assistant: 'Updating the layout for you. [Action: SetChildCount(5)]'\n\
             - User: 'Make the text larger, bold and say Hello'\n\
             - Assistant: 'Updating typography. [Action: SetTypographySize(32)] [Action: SetTypographyBold(true)] [Action: SetTypographyText(Hello)]'\n\n\
             Always explain what you are doing. Use the action tag as part of your natural response.",
            ui_json
        )
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mode = ShellMode::Desktop;
        let tone = self.theme_tone;
        let tokens = ThemeTokens::with_theme(self.theme, tone);

        if self.show_landing {
            return crate::core::responsive(mode, tokens.clone(), move |context| {
                iced::widget::container(super::pages::landing::view(&context))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(move |_| iced::widget::container::Style {
                        background: Some(tokens.colors.background.into()),
                        ..Default::default()
                    })
                    .into()
            });
        }

        // 1. Prepare Content
        let content = super::views::ContentView::new(self);

        let context_menu_pos = self.context_menu_pos;

        // Neural Export (Desktop only)
        #[cfg(not(target_arch = "wasm32"))]
        {
            let tokens = ThemeTokens::with_theme(self.theme, self.theme_tone);
            let ctx = Context::new(ShellMode::Desktop, tokens, iced::Size::new(1200.0, 800.0));
            let semantic_tree = content.describe(&ctx);
            let state = serde_json::json!({
                "active_tab": self.active_tab,
                "navigation_mode": self.navigation_mode,
                "tree": semantic_tree,
            });

            let _ = std::fs::create_dir_all(".peak");
            let _ = std::fs::write(
                ".peak/neural_state.json",
                serde_json::to_string_pretty(&state).unwrap_or_default(),
            );
        }

        crate::core::responsive(mode, tokens.clone(), move |context| {
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

            // Wrap in Window Chrome if Desktop (Mocking a windowed environment within the app for demo)
            // In a real app, the shell handles this. For this demo, we apply it here or assume OS frame.
            // BUT user wants notch interaction. So we render our own Chrome.

            // To properly mock the interaction, we should use the chrome.
            // However, verify if 'responsive' already creates a full window.
            // Let's assume we can overlay the "Control Center" / "Notch" on top.

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

            // Render Window Chrome ON TOP of everything (or wrapping)
            // Since we want the notch button to be clickable, and it's in the chrome.

            let content_view: Element<'_, Message> = stack.into();
            content_view
        })
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        let events = iced::event::listen().map(|event| {
            if let iced::Event::Keyboard(_) = event {
                // Keep raw logging for now to compare
                log::info!("RAW EVENT: {:?}", event);
            }
            match event {
                iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
                    Message::UpdateCursorPos(position)
                }
                iced::Event::Mouse(iced::mouse::Event::ButtonPressed(
                    iced::mouse::Button::Right,
                )) => Message::OpenContextMenu(iced::Point::ORIGIN),
                iced::Event::Mouse(iced::mouse::Event::ButtonPressed(
                    iced::mouse::Button::Left,
                )) => Message::CloseContextMenu,
                _ => Message::None,
            }
        });

        let hotkeys = iced::keyboard::on_key_press(|key, modifiers| {
            let is_cmd = modifiers.command() || modifiers.logo();
            let is_ctrl = modifiers.control();

            let is_backspace = matches!(
                key,
                iced::keyboard::Key::Named(iced::keyboard::key::Named::Backspace)
            );
            let is_delete_forward = matches!(
                key,
                iced::keyboard::Key::Named(iced::keyboard::key::Named::Delete)
            );
            let is_d = matches!(key, iced::keyboard::Key::Character(ref c) if c.as_str() == "d");
            let is_u = matches!(key, iced::keyboard::Key::Character(ref c) if c.as_str() == "u");

            // Cmd+Backspace (Ghost), Cmd+Delete, Cmd+D, Ctrl+U
            if (is_cmd && (is_backspace || is_delete_forward || is_d)) || (is_ctrl && is_u) {
                log::info!("SHORTCUT TRIGGERED: {:?} + {:?}", modifiers, key);
                Some(Message::CmdBackspacePressed)
            } else {
                None
            }
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

            iced::Subscription::batch(vec![events, hash_sub, hotkeys, window_events])
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

            iced::Subscription::batch(vec![events, command_sub, hotkeys, window_events])
        }
    }
}
