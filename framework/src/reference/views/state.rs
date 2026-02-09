use super::super::app::{
    AIProviderChoice, AccessibilityLabState, App, ButtonLabState, EmojiLabState, IconLabState,
    InspectorTab, LayoutLabState, RenderMode, SizingLabState, SpacerLabState, SudoAction,
    TypographyLabState,
};
use crate::reference::AppPage;
use crate::views::chat::ChatMessage;
use std::sync::Arc;

/// ViewState is the single source of truth for all UI state in the PeakUI reference implementation.
///
/// It acts as the "Digital Nervous System" of the application, enabling:
/// 1. **State Optimization**: All lab and app states are consolidated here.
/// 2. **Performance**: Large collections (chat messages, DB records) are wrapped in `Arc` for pointer-speed cloning.
/// 3. **AI Introspection**: By deriving `Serialize`, the entire state can be dumped to JSON for AI agents to understand the app's internal "thought process."
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ViewState {
    pub active_tab: AppPage,
    pub show_search: bool,
    pub show_inspector: bool,
    pub show_sidebar: bool,
    pub show_user_profile: bool,
    pub navigation_mode: String,
    pub search_query: String,
    pub expanded_sections: Arc<std::collections::HashSet<String>>,

    // Lab States
    pub button_lab: Arc<ButtonLabState>,
    pub typography_lab: Arc<TypographyLabState>,
    pub layout_lab: Arc<LayoutLabState>,
    pub sizing_lab: Arc<SizingLabState>,
    pub accessibility_lab: Arc<AccessibilityLabState>,
    pub icon_lab: Arc<IconLabState>,
    pub emoji_lab: Arc<EmojiLabState>,
    pub spacer_lab: Arc<SpacerLabState>,

    pub render_mode: RenderMode,
    pub is_thinking: bool,
    pub chat_messages: Arc<Vec<ChatMessage>>, // Now Arc!
    pub chat_input: String,
    pub sidebar_width: f32,
    pub inspector_width: f32,
    pub is_resizing_sidebar: bool,
    pub is_resizing_inspector: bool,
    pub inspector_tab: InspectorTab,

    pub api_key: String,
    pub ai_provider: AIProviderChoice,
    pub icon_limit: usize,
    pub pending_sudo_action: Option<SudoAction>,
    pub db_records: Arc<Vec<crate::core::SemanticRecord>>,
    pub enable_exposure: bool,
}

impl ViewState {
    pub fn new(app: &App) -> Self {
        Self {
            active_tab: app.active_tab.clone(),
            show_search: app.show_search,
            show_inspector: app.show_inspector,
            show_sidebar: app.show_sidebar,
            show_user_profile: app.show_user_profile,
            navigation_mode: app.navigation_mode.clone(),
            search_query: app.search_query.clone(),
            expanded_sections: app.expanded_sections.clone(),
            button_lab: app.button_lab.clone(),
            typography_lab: app.typography_lab.clone(),
            layout_lab: app.layout_lab.clone(),
            sizing_lab: app.sizing_lab.clone(),
            accessibility_lab: app.accessibility_lab.clone(),
            icon_lab: app.icon_lab.clone(),
            emoji_lab: app.emoji_lab.clone(),
            spacer_lab: app.spacer_lab.clone(),
            render_mode: app.render_mode,
            is_thinking: app.is_thinking,
            chat_messages: app.chat_messages.clone(),
            chat_input: app.chat_input.clone(),
            sidebar_width: app.sidebar_width,
            inspector_width: app.inspector_width,
            is_resizing_sidebar: app.is_resizing_sidebar,
            is_resizing_inspector: app.is_resizing_inspector,
            inspector_tab: app.inspector_tab,
            api_key: app.api_key.clone(),
            ai_provider: app.ai_provider,
            icon_limit: app.icon_limit,
            pending_sudo_action: app.pending_sudo_action.clone(),
            db_records: app.db.get_all(),
            enable_exposure: app.enable_exposure,
        }
    }
}
