use super::state::*;
use crate::prelude::*;
use crate::reference::AppPage;
use crate::views::chat::ChatViewMessage;

#[derive(Debug, Clone)]
pub enum Message {
    SetTab(AppPage),
    ToggleSearch,
    ToggleInspector,
    ToggleSidebar,
    ToggleUserProfile,
    SetNavigationMode(String),
    ToggleSection(String),
    Search(String),
    SetTheme(peak_theme::ThemeTone),
    SetThemeKind(peak_theme::PeakTheme),
    SetScaling(f32),
    CopyCode(String),
    SetRenderMode(RenderMode),
    OpenContextMenu(Point),
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
    SetExposure(bool),

    // Button Lab Messages
    UpdateButtonLabel(String),
    UpdateButtonIcon(Option<String>),
    UpdateButtonSize(crate::engine::modifiers::ControlSize),
    UpdateButtonVariant(crate::style::Variant),
    UpdateButtonIntent(crate::style::Intent),
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
    UpdateCursorPos(Point),
    WindowResized(Size),
    FontLoaded(std::result::Result<(), crate::prelude::font::Error>),
    CmdBackspacePressed,
    LoadMoreIcons,
    Heartbeat,
    SudoRequest(SudoAction),
    SudoApprove,
    SudoDeny,
    ExecuteShell(String), // New: Shell execution message
    ApplyNativeVibrancy,
    #[cfg(target_arch = "wasm32")]
    TypewriterTick(wasmtimer::std::Instant),
    #[cfg(not(target_arch = "wasm32"))]
    TypewriterTick(std::time::Instant),
    Unknown(String),
    None,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub enum Command {
    EnterApp,
    SetTab(AppPage),
    ToggleSearch,
    ToggleInspector,
    ToggleSidebar,
    ToggleUserProfile,
    SetNavigationMode(String),
    ToggleSection(String),
    Search(String),
    SetTheme(peak_theme::ThemeTone),
    SetThemeKind(peak_theme::PeakTheme),
    SetRenderMode(RenderMode),
    SetInspectorTab(InspectorTab),
    SetApiKey(String),
    SetAIProvider(AIProviderChoice),
    SetExposure(bool),

    // Button Lab
    UpdateButtonLabel(String),
    UpdateButtonVariant(crate::style::Variant),
    UpdateButtonIntent(crate::style::Intent),
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
            Command::SetExposure(enable) => Message::SetExposure(enable),

            Command::ApplyNativeVibrancy => Message::ApplyNativeVibrancy,
            Command::None => Message::None,
        }
    }
}
