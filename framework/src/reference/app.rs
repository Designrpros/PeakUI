use super::model::Page;
use crate::prelude::*;
use peak_core::registry::ShellMode;
use peak_theme::{PeakTheme, ThemeTokens, ThemeTone};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum RenderMode {
    #[default]
    Canvas,
    Terminal,
    Neural,
    Spatial,
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
    pub is_resizing_sidebar: bool,
    pub is_resizing_inspector: bool,
    pub context_menu_pos: Option<iced::Point>,
    pub last_cursor_pos: iced::Point,
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
    FontLoaded(std::result::Result<(), iced::font::Error>),
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

            Command::None => Message::None,
        }
    }
}

impl Default for App {
    fn default() -> Self {
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
            is_resizing_sidebar: false,
            is_resizing_inspector: false,
            context_menu_pos: None,
            last_cursor_pos: iced::Point::ORIGIN,
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
            Message::UpdateCursorPos(pos) => {
                self.last_cursor_pos = pos;
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
            Message::FontLoaded(_) => Task::none(),
            Message::None => Task::none(),
        }
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

            // Overlay Context Menu
            if let Some(pos) = context_menu_pos {
                let menu = crate::views::ContextMenu::new()
                    .item(
                        "Reload",
                        "refresh",
                        Message::ContextMenuAction("Reload".to_string()),
                    )
                    .item(
                        "Inspect",
                        "search",
                        Message::ContextMenuAction("Inspect".to_string()),
                    )
                    .item("Close", "x", Message::CloseContextMenu);

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

            stack.into()
        })
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        let mouse_events = iced::event::listen().map(|event| match event {
            iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
                Message::UpdateCursorPos(position)
            }
            iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Right)) => {
                Message::OpenContextMenu(iced::Point::ORIGIN)
            }
            iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => {
                Message::CloseContextMenu
            }
            _ => Message::None,
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

            iced::Subscription::batch(vec![mouse_events, hash_sub])
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

            iced::Subscription::batch(vec![mouse_events, command_sub])
        }
    }
}
