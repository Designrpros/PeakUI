use super::state::*;
use crate::prelude::*;
use crate::reference::AppPage;
use crate::views::chat::ChatViewMessage;

#[derive(Debug, Clone)]
pub enum ShellMessage {
    SetTab(AppPage),
    ToggleSearch,
    ToggleInspector,
    ToggleSidebar,
    ToggleUserProfile,
    SetNavigationMode(String),
    ToggleSection(String),
    Search(String),
    SetLanguage(String, Vec<String>),
    OpenUrl(String),
    ResizeSidebar(f32),
    ResizeInspector(f32),
    StartResizingSidebar,
    StopResizingSidebar,
    StartResizingInspector,
    StopResizingInspector,
}

#[derive(Debug, Clone)]
pub enum IntelligenceMessage {
    Chat(ChatViewMessage),
    #[cfg(feature = "intelligence")]
    AIResponse(std::result::Result<String, String>),
    #[cfg(feature = "intelligence")]
    ChatStreamUpdate(std::result::Result<String, String>),
    #[cfg(feature = "intelligence")]
    AIChatComplete,
    #[cfg(feature = "intelligence")]
    SetApiKey(String),
    #[cfg(feature = "intelligence")]
    SetAIProvider(AIProviderChoice),
    #[cfg(feature = "intelligence")]
    ProcessToolResult(String, serde_json::Value), // name, result
}

#[derive(Debug, Clone)]
pub enum LabMessage {
    SetRenderMode(RenderMode),
    // Icon Lab
    UpdateIconLabIcon(String),
    UpdateIconLabSize(f32),
    UpdateIconLabColor(Option<Color>),
    // Emoji Lab
    UpdateEmojiLabEmoji(String),
    UpdateEmojiLabSize(f32),
    // Spacer Lab
    UpdateSpacerWidth(f32),
    UpdateSpacerHeight(f32),
    // Button Lab
    UpdateButtonLabel(String),
    UpdateButtonIcon(Option<String>),
    UpdateButtonSize(crate::engine::modifiers::ControlSize),
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
    UpdateLayoutAlignment(Alignment),
    UpdateLayoutItemSizing(SizingType),
    // Sizing Lab
    UpdateSizingWidthType(SizingType),
    UpdateSizingHeightType(SizingType),
    UpdateSizingFixedWidth(f32),
    UpdateSizingFixedHeight(f32),
    // Accessibility Lab
    UpdateAccessibilityComponent(AccessibilityComponent),
    LoadMoreIcons,
}

#[derive(Debug, Clone)]
pub enum InteractionMessage {
    SetTheme(peak_theme::ThemeTone),
    SetThemeKind(peak_theme::PeakTheme),
    SetScaling(f32),
    CopyCode(String),
    ClearCopiedFeedback,
    SetInspectorTab(InspectorTab),
    SetExposure(bool),
    OpenContextMenu(Point),
    CloseContextMenu,
    ContextMenuAction(String),
    UpdateCursorPos(Point),
    SudoRequest(SudoAction),
    SudoApprove,
    SudoDeny,
}

#[derive(Debug, Clone)]
pub enum Message {
    Shell(ShellMessage),
    #[cfg(feature = "intelligence")]
    Intelligence(IntelligenceMessage),
    Lab(LabMessage),
    Interaction(InteractionMessage),

    // System / Global
    EnterApp,
    WindowResized(Size),
    FontLoaded(std::result::Result<(), crate::prelude::font::Error>),
    CmdBackspacePressed,
    Heartbeat,
    ExecuteShell(String),
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
    // Shell
    SetTab(AppPage),
    ToggleSearch,
    ToggleInspector,
    ToggleSidebar,
    ToggleUserProfile,
    SetNavigationMode(String),
    ToggleSection(String),
    Search(String),
    // Interaction
    SetTheme(peak_theme::ThemeTone),
    SetThemeKind(peak_theme::PeakTheme),
    SetInspectorTab(InspectorTab),
    #[cfg(feature = "intelligence")]
    SetApiKey(String),
    #[cfg(feature = "intelligence")]
    SetAIProvider(AIProviderChoice),
    SetExposure(bool),
    // Lab
    SetRenderMode(RenderMode),
    UpdateButtonLabel(String),
    UpdateButtonVariant(crate::style::Variant),
    UpdateButtonIntent(crate::style::Intent),
    ToggleButtonFullWidth(bool),
    ToggleButtonDisabled(bool),
    ToggleButtonFocused(bool),
    UpdateTypographyText(String),
    UpdateTypographySize(f32),
    ToggleTypographyBold(bool),
    ToggleTypographyItalic(bool),
    UpdateLayoutOuterSpacing(f32),
    UpdateLayoutInnerSpacing(f32),
    UpdateLayoutChildCount(usize),
    UpdateLayoutAlignment(String),
    UpdateLayoutItemSizing(SizingType),
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
            Command::SetTab(page) => Message::Shell(ShellMessage::SetTab(page)),
            Command::ToggleSearch => Message::Shell(ShellMessage::ToggleSearch),
            Command::ToggleInspector => Message::Shell(ShellMessage::ToggleInspector),
            Command::ToggleSidebar => Message::Shell(ShellMessage::ToggleSidebar),
            Command::ToggleUserProfile => Message::Shell(ShellMessage::ToggleUserProfile),
            Command::SetNavigationMode(mode) => {
                Message::Shell(ShellMessage::SetNavigationMode(mode))
            }
            Command::ToggleSection(section) => Message::Shell(ShellMessage::ToggleSection(section)),
            Command::Search(query) => Message::Shell(ShellMessage::Search(query)),

            Command::SetTheme(tone) => Message::Interaction(InteractionMessage::SetTheme(tone)),
            Command::SetThemeKind(theme) => {
                Message::Interaction(InteractionMessage::SetThemeKind(theme))
            }
            Command::SetInspectorTab(tab) => {
                Message::Interaction(InteractionMessage::SetInspectorTab(tab))
            }
            #[cfg(feature = "intelligence")]
            Command::SetApiKey(key) => Message::Intelligence(IntelligenceMessage::SetApiKey(key)),
            #[cfg(feature = "intelligence")]
            Command::SetAIProvider(provider) => {
                Message::Intelligence(IntelligenceMessage::SetAIProvider(provider))
            }
            Command::SetExposure(enable) => {
                Message::Interaction(InteractionMessage::SetExposure(enable))
            }

            Command::SetRenderMode(mode) => Message::Lab(LabMessage::SetRenderMode(mode)),
            Command::UpdateButtonLabel(label) => Message::Lab(LabMessage::UpdateButtonLabel(label)),
            Command::UpdateButtonVariant(variant) => {
                Message::Lab(LabMessage::UpdateButtonVariant(variant))
            }
            Command::UpdateButtonIntent(intent) => {
                Message::Lab(LabMessage::UpdateButtonIntent(intent))
            }
            Command::ToggleButtonFullWidth(full) => {
                Message::Lab(LabMessage::ToggleButtonFullWidth(full))
            }
            Command::ToggleButtonDisabled(disabled) => {
                Message::Lab(LabMessage::ToggleButtonDisabled(disabled))
            }
            Command::ToggleButtonFocused(focused) => {
                Message::Lab(LabMessage::ToggleButtonFocused(focused))
            }
            Command::UpdateTypographyText(text) => {
                Message::Lab(LabMessage::UpdateTypographyText(text))
            }
            Command::UpdateTypographySize(size) => {
                Message::Lab(LabMessage::UpdateTypographySize(size))
            }
            Command::ToggleTypographyBold(bold) => {
                Message::Lab(LabMessage::ToggleTypographyBold(bold))
            }
            Command::ToggleTypographyItalic(italic) => {
                Message::Lab(LabMessage::ToggleTypographyItalic(italic))
            }
            Command::UpdateLayoutOuterSpacing(s) => {
                Message::Lab(LabMessage::UpdateLayoutOuterSpacing(s))
            }
            Command::UpdateLayoutInnerSpacing(s) => {
                Message::Lab(LabMessage::UpdateLayoutInnerSpacing(s))
            }
            Command::UpdateLayoutChildCount(c) => {
                Message::Lab(LabMessage::UpdateLayoutChildCount(c))
            }
            Command::UpdateLayoutAlignment(align) => {
                let alignment = match align.to_lowercase().as_str() {
                    "start" | "left" | "top" => Alignment::Start,
                    "center" => Alignment::Center,
                    "end" | "right" | "bottom" => Alignment::End,
                    _ => Alignment::Center,
                };
                Message::Lab(LabMessage::UpdateLayoutAlignment(alignment))
            }
            Command::UpdateLayoutItemSizing(sizing) => {
                Message::Lab(LabMessage::UpdateLayoutItemSizing(sizing))
            }
            Command::UpdateSizingWidthType(t) => Message::Lab(LabMessage::UpdateSizingWidthType(t)),
            Command::UpdateSizingHeightType(t) => {
                Message::Lab(LabMessage::UpdateSizingHeightType(t))
            }
            Command::UpdateSizingFixedWidth(w) => {
                Message::Lab(LabMessage::UpdateSizingFixedWidth(w))
            }
            Command::UpdateSizingFixedHeight(h) => {
                Message::Lab(LabMessage::UpdateSizingFixedHeight(h))
            }

            Command::ApplyNativeVibrancy => Message::ApplyNativeVibrancy,
            Command::None => Message::None,
        }
    }
}
