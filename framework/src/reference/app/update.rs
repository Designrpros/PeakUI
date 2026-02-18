use crate::prelude::*;
#[cfg(feature = "intelligence")]
use crate::reference::intelligence::Action;
#[cfg(feature = "intelligence")]
use crate::views::chat::ChatViewMessage;
use crate::views::{ChatMessage, ChatRole};
// #[cfg(feature = "intelligence")]
// use peak_theme::ThemeTone;
use std::sync::Arc;
// #[cfg(feature = "intelligence")]
// use crate::core::Context;

#[cfg(feature = "intelligence")]
use super::message::IntelligenceMessage;
use super::message::InteractionMessage;
use super::message::{LabMessage, Message, ShellMessage};
use super::state::*;
use crate::reference::AppPage;

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let task = self.update_internal(message);

        // Export the live view state for the Neural Exposure API
        self.export_view();

        task
    }

    fn update_internal(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Shell(shell_msg) => match shell_msg {
                ShellMessage::SetTab(tab) => {
                    log::info!("SetTab: {:?}", tab);
                    self.shell.active_tab = tab.clone();
                    self.shell.show_search = false;

                    self.shell.navigation_mode = match tab.navigation_mode().to_lowercase().as_str()
                    {
                        "start" | "guide" => "Start".to_string(),
                        "catalog" | "components" => "Catalog".to_string(),
                        "data" | "ecosystem" => "Data".to_string(),
                        "settings" | "preferences" => "Settings".to_string(),
                        _ => tab.navigation_mode(),
                    };

                    // Auto-enable inspector for lab pages
                    match tab {
                        AppPage::Button
                        | AppPage::Typography
                        | AppPage::BasicSizing
                        | AppPage::Layout => {
                            self.shell.show_inspector = true;
                        }
                        _ => {}
                    }

                    // Mobile Navigation Protocol
                    if self.shell.window_width < 900.0 {
                        self.shell.show_sidebar = false;
                    }

                    // Landing visibility
                    match tab {
                        AppPage::Landing
                        | AppPage::PeakOSDetail
                        | AppPage::PeakUIDetail
                        | AppPage::PeakDBDetail
                        | AppPage::PeakRelayDetail
                        | AppPage::PeakHubDetail => {
                            self.interaction.show_landing = true;
                            if self.shell.window_width < 900.0 && tab == AppPage::Landing {
                                self.shell.show_sidebar = true;
                            }
                        }
                        _ => {
                            self.interaction.show_landing = false;
                            if self.shell.window_width >= 900.0 {
                                self.shell.show_sidebar = true;
                            }
                        }
                    }

                    #[cfg(target_arch = "wasm32")]
                    {
                        let path = tab.to_path();
                        let _ = web_sys::window().and_then(|w| w.location().set_hash(&path).ok());
                    }

                    Task::none()
                }
                ShellMessage::ToggleSearch => {
                    self.shell.show_search = !self.shell.show_search;
                    self.shell.search_query.clear();
                    Task::none()
                }
                ShellMessage::ToggleInspector => {
                    self.shell.show_inspector = !self.shell.show_inspector;
                    Task::none()
                }
                ShellMessage::ToggleSidebar => {
                    self.shell.show_sidebar = !self.shell.show_sidebar;
                    Task::none()
                }
                ShellMessage::ToggleUserProfile => {
                    self.shell.show_user_profile = !self.shell.show_user_profile;
                    Task::none()
                }
                ShellMessage::SetNavigationMode(mode) => {
                    self.shell.navigation_mode = mode.clone();
                    self.shell.active_tab = match mode.as_str() {
                        "Start" => AppPage::Introduction,
                        "Catalog" => AppPage::Button,
                        "Data" => AppPage::PeakDB,
                        "Settings" => AppPage::Appearance,
                        _ => self.shell.active_tab.clone(),
                    };
                    if self.shell.window_width < 900.0 {
                        self.shell.show_sidebar = false;
                    }
                    Task::none()
                }
                ShellMessage::ToggleSection(section) => {
                    let expanded = Arc::make_mut(&mut self.shell.expanded_sections);
                    if expanded.contains(&section) {
                        expanded.remove(&section);
                    } else {
                        expanded.insert(section);
                    }
                    Task::none()
                }
                ShellMessage::Search(query) => {
                    self.shell.search_query = query;
                    Task::none()
                }
                ShellMessage::SetLanguage(lang, resources) => {
                    self.shell.localization.set_language(&lang, resources);
                    Task::none()
                }
                ShellMessage::OpenUrl(url) => {
                    #[cfg(target_arch = "wasm32")]
                    let _ = web_sys::window()
                        .and_then(|w| w.open_with_url_and_target(&url, "_blank").ok());
                    #[cfg(not(target_arch = "wasm32"))]
                    let _ = open::that(url);
                    Task::none()
                }
                ShellMessage::ResizeSidebar(w) => {
                    self.interaction.sidebar_width = w.max(160.0).min(400.0);
                    Task::none()
                }
                ShellMessage::ResizeInspector(w) => {
                    self.interaction.inspector_width = w.max(200.0).min(500.0);
                    Task::none()
                }
                ShellMessage::StartResizingSidebar => {
                    self.interaction.is_resizing_sidebar = true;
                    Task::none()
                }
                ShellMessage::StopResizingSidebar => {
                    self.interaction.is_resizing_sidebar = false;
                    Task::none()
                }
                ShellMessage::StartResizingInspector => {
                    self.interaction.is_resizing_inspector = true;
                    Task::none()
                }
                ShellMessage::StopResizingInspector => {
                    self.interaction.is_resizing_inspector = false;
                    Task::none()
                }
            },
            #[cfg(feature = "intelligence")]
            Message::Intelligence(int_msg) => match int_msg {
                IntelligenceMessage::Chat(msg) => match msg {
                    ChatViewMessage::InputChanged(val) => {
                        self.intelligence.chat_input = val;
                        Task::none()
                    }
                    ChatViewMessage::SendPressed => {
                        let content = self.intelligence.chat_input.trim().to_string();
                        if !content.is_empty() {
                            self.intelligence.chat_input.clear();
                            return self.start_ai_chat(content);
                        }
                        Task::none()
                    }
                    ChatViewMessage::CopyCode(code) => {
                        self.interaction.last_copied_code = Some(code.clone());
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            // use clipboard::{ClipboardContext, ClipboardProvider};
                            // if let Ok(mut ctx) = ClipboardContext::new() {
                            //     let _ = ctx.set_contents(code);
                            // }
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            let _ = web_sys::window()
                                .map(|win| win.navigator().clipboard().write_text(&code));
                        }
                        Task::perform(
                            async {
                                #[cfg(target_arch = "wasm32")]
                                wasmtimer::tokio::sleep(std::time::Duration::from_secs(2)).await;
                                #[cfg(not(target_arch = "wasm32"))]
                                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                            },
                            |_| Message::Interaction(InteractionMessage::ClearCopiedFeedback),
                        )
                    }
                },

                #[cfg(feature = "intelligence")]
                IntelligenceMessage::AIResponse(res) => {
                    self.intelligence.is_thinking = false;
                    match res {
                        Ok(content) => {
                            self.intelligence.append_message(ChatMessage {
                                role: ChatRole::Assistant,
                                content: content.clone(),
                            });
                            return self.process_assistant_actions(&content);
                        }
                        Err(e) => {
                            self.intelligence.append_message(ChatMessage {
                                role: ChatRole::System,
                                content: format!("Error: {}", e),
                            });
                        }
                    }
                    Task::none()
                }
                #[cfg(feature = "intelligence")]
                IntelligenceMessage::ChatStreamUpdate(res) => {
                    match res {
                        Ok(delta) => {
                            let messages = Arc::make_mut(&mut self.intelligence.chat_messages);
                            if let Some(last) = messages.last_mut() {
                                if last.role == ChatRole::Assistant {
                                    last.content.push_str(&delta);
                                } else {
                                    messages.push(ChatMessage {
                                        role: ChatRole::Assistant,
                                        content: delta,
                                    });
                                }
                            } else {
                                messages.push(ChatMessage {
                                    role: ChatRole::Assistant,
                                    content: delta,
                                });
                            }
                        }
                        Err(e) => {
                            self.intelligence.is_thinking = false;
                            self.intelligence.append_message(ChatMessage {
                                role: ChatRole::System,
                                content: format!("Error: {}", e),
                            });
                        }
                    }
                    Task::none()
                }
                #[cfg(feature = "intelligence")]
                IntelligenceMessage::AIChatComplete => {
                    self.intelligence.is_thinking = false;
                    if let Some(last) = self.intelligence.chat_messages.last() {
                        if last.role == ChatRole::Assistant {
                            return self.process_assistant_actions(&last.content.clone());
                        }
                    }
                    Task::none()
                }
                #[cfg(feature = "intelligence")]
                IntelligenceMessage::SetApiKey(key) => {
                    self.intelligence.api_key = key;
                    self.save_settings();
                    self.reload_intelligence_bridge();
                    Task::none()
                }
                #[cfg(feature = "intelligence")]
                IntelligenceMessage::SetAIProvider(p) => {
                    self.intelligence.ai_provider = p;
                    self.save_settings();
                    self.reload_intelligence_bridge();
                    Task::none()
                }
                #[cfg(feature = "intelligence")]
                IntelligenceMessage::ProcessToolResult(name, result) => {
                    log::info!("Tool result: {} -> {:?}", name, result);

                    let result_str = if result.is_object() && result.get("error").is_some() {
                        format!(
                            "Tool '{}' failed: {}",
                            name,
                            result
                                .get("error")
                                .and_then(|e| e.as_str())
                                .unwrap_or("Unknown error")
                        )
                    } else {
                        format!("[result:{}] {}", name, result)
                    };

                    self.intelligence.append_message(ChatMessage {
                        role: ChatRole::System,
                        content: result_str,
                    });
                    return self.ai_chat_completion();
                }
            },
            Message::Lab(lab_msg) => match lab_msg {
                LabMessage::SetRenderMode(mode) => {
                    self.labs.render_mode = mode;
                    Task::none()
                }
                LabMessage::UpdateIconLabIcon(icon) => {
                    Arc::make_mut(&mut self.labs.icon).selected_icon = icon;
                    Task::none()
                }
                LabMessage::UpdateIconLabSize(size) => {
                    Arc::make_mut(&mut self.labs.icon).size = size;
                    Task::none()
                }
                LabMessage::UpdateIconLabColor(color) => {
                    Arc::make_mut(&mut self.labs.icon).color = color;
                    Task::none()
                }
                LabMessage::UpdateEmojiLabEmoji(emoji) => {
                    Arc::make_mut(&mut self.labs.emoji).selected_emoji = emoji;
                    Task::none()
                }
                LabMessage::UpdateEmojiLabSize(size) => {
                    Arc::make_mut(&mut self.labs.emoji).size = size;
                    Task::none()
                }
                LabMessage::UpdateSpacerWidth(w) => {
                    Arc::make_mut(&mut self.labs.spacer).width = w;
                    Task::none()
                }
                LabMessage::UpdateSpacerHeight(h) => {
                    Arc::make_mut(&mut self.labs.spacer).height = h;
                    Task::none()
                }
                LabMessage::UpdateButtonLabel(label) => {
                    Arc::make_mut(&mut self.labs.button).label = label;
                    Task::none()
                }
                LabMessage::UpdateButtonIcon(icon) => {
                    Arc::make_mut(&mut self.labs.button).icon = icon;
                    Task::none()
                }
                LabMessage::UpdateButtonSize(size) => {
                    Arc::make_mut(&mut self.labs.button).size = size;
                    Task::none()
                }
                LabMessage::UpdateButtonVariant(v) => {
                    Arc::make_mut(&mut self.labs.button).variant = v;
                    Task::none()
                }
                LabMessage::UpdateButtonIntent(i) => {
                    Arc::make_mut(&mut self.labs.button).intent = i;
                    Task::none()
                }
                LabMessage::ToggleButtonFullWidth(b) => {
                    Arc::make_mut(&mut self.labs.button).is_full_width = b;
                    Task::none()
                }
                LabMessage::ToggleButtonDisabled(b) => {
                    Arc::make_mut(&mut self.labs.button).is_disabled = b;
                    Task::none()
                }
                LabMessage::ToggleButtonFocused(b) => {
                    Arc::make_mut(&mut self.labs.button).is_focused = b;
                    Task::none()
                }
                LabMessage::UpdateTypographyText(t) => {
                    Arc::make_mut(&mut self.labs.typography).text = t;
                    Task::none()
                }
                LabMessage::UpdateTypographySize(s) => {
                    Arc::make_mut(&mut self.labs.typography).size = s;
                    Task::none()
                }
                LabMessage::ToggleTypographyBold(b) => {
                    Arc::make_mut(&mut self.labs.typography).is_bold = b;
                    Task::none()
                }
                LabMessage::ToggleTypographyItalic(b) => {
                    Arc::make_mut(&mut self.labs.typography).is_italic = b;
                    Task::none()
                }
                LabMessage::UpdateLayoutOuterSpacing(s) => {
                    Arc::make_mut(&mut self.labs.layout).outer_spacing = s;
                    Task::none()
                }
                LabMessage::UpdateLayoutInnerSpacing(s) => {
                    Arc::make_mut(&mut self.labs.layout).inner_spacing = s;
                    Task::none()
                }
                LabMessage::UpdateLayoutChildCount(c) => {
                    Arc::make_mut(&mut self.labs.layout).child_count = c.min(10);
                    Task::none()
                }
                LabMessage::UpdateLayoutAlignment(a) => {
                    Arc::make_mut(&mut self.labs.layout).alignment = a;
                    Task::none()
                }
                LabMessage::UpdateLayoutItemSizing(s) => {
                    Arc::make_mut(&mut self.labs.layout).item_sizing = s;
                    Task::none()
                }
                LabMessage::UpdateSizingWidthType(t) => {
                    Arc::make_mut(&mut self.labs.sizing).width_type = t;
                    Task::none()
                }
                LabMessage::UpdateSizingHeightType(t) => {
                    Arc::make_mut(&mut self.labs.sizing).height_type = t;
                    Task::none()
                }
                LabMessage::UpdateSizingFixedWidth(w) => {
                    Arc::make_mut(&mut self.labs.sizing).fixed_width = w;
                    Task::none()
                }
                LabMessage::UpdateSizingFixedHeight(h) => {
                    Arc::make_mut(&mut self.labs.sizing).fixed_height = h;
                    Task::none()
                }
                LabMessage::UpdateAccessibilityComponent(comp) => {
                    Arc::make_mut(&mut self.labs.accessibility).selected_component = comp;
                    Task::none()
                }
                LabMessage::LoadMoreIcons => {
                    self.icon_limit += 50;
                    Task::none()
                }
            },
            Message::Interaction(int_msg) => match int_msg {
                InteractionMessage::SetTheme(tone) => {
                    self.interaction.theme_tone = tone;
                    Task::none()
                }
                InteractionMessage::SetThemeKind(kind) => {
                    self.interaction.theme = kind;
                    Task::none()
                }
                InteractionMessage::SetScaling(s) => {
                    self.interaction.scaling = s;
                    Task::none()
                }
                InteractionMessage::CopyCode(code) => {
                    self.interaction.last_copied_code = Some(code.clone());
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        // use clipboard::{ClipboardContext, ClipboardProvider};
                        // if let Ok(mut ctx) = ClipboardContext::new() {
                        //     let _ = ctx.set_contents(code);
                        // }
                    }
                    #[cfg(target_arch = "wasm32")]
                    {
                        let _ = web_sys::window()
                            .map(|win| win.navigator().clipboard().write_text(&code));
                    }
                    Task::perform(
                        async {
                            #[cfg(target_arch = "wasm32")]
                            wasmtimer::tokio::sleep(std::time::Duration::from_secs(2)).await;
                            #[cfg(not(target_arch = "wasm32"))]
                            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        },
                        |_| Message::Interaction(InteractionMessage::ClearCopiedFeedback),
                    )
                }
                InteractionMessage::ClearCopiedFeedback => {
                    self.interaction.last_copied_code = None;
                    Task::none()
                }
                InteractionMessage::SetInspectorTab(tab) => {
                    self.interaction.inspector_tab = tab;
                    Task::none()
                }
                InteractionMessage::SetExposure(enable) => {
                    self.interaction.enable_exposure = enable;
                    self.save_settings();
                    Task::none()
                }
                InteractionMessage::OpenContextMenu(pos) => {
                    self.interaction.context_menu_pos = Some(pos);
                    Task::none()
                }
                InteractionMessage::CloseContextMenu => {
                    self.interaction.context_menu_pos = None;
                    Task::none()
                }
                InteractionMessage::ContextMenuAction(action) => {
                    self.interaction.context_menu_pos = None;
                    match action.as_str() {
                        "Reload" => {
                            #[cfg(target_arch = "wasm32")]
                            let _ = web_sys::window().and_then(|w| w.location().reload().ok());
                        }
                        "Inspect" => {
                            self.shell.show_inspector = true;
                        }
                        _ => {}
                    }
                    Task::none()
                }
                InteractionMessage::UpdateCursorPos(pos) => {
                    self.interaction.last_cursor_pos = pos;
                    Task::none()
                }
                InteractionMessage::SudoRequest(action) => {
                    self.interaction.pending_sudo_action = Some(action);
                    Task::none()
                }
                InteractionMessage::SudoApprove => {
                    if let Some(action) = self.interaction.pending_sudo_action.take() {
                        Task::perform(async {}, move |_| *action.message)
                    } else {
                        Task::none()
                    }
                }
                InteractionMessage::SudoDeny => {
                    self.interaction.pending_sudo_action = None;
                    Task::none()
                }
            },
            Message::TypewriterTick(_) => {
                self.interaction.tick = self.interaction.tick.wrapping_add(1);
                if !self.interaction.show_landing {
                    return Task::none();
                }
                let phrases = [
                    "Say hello",
                    "Change tone to dark",
                    "Navigate to button lab",
                    "Set button variant to compact",
                ];
                let current_phrase =
                    phrases[self.intelligence.typewriter_phrase_index % phrases.len()];

                if self.intelligence.is_deleting {
                    if self.intelligence.typewriter_index > 0 {
                        self.intelligence.typewriter_index =
                            self.intelligence.typewriter_index.saturating_sub(1);
                        self.intelligence.typewriter_text = current_phrase
                            .chars()
                            .take(self.intelligence.typewriter_index)
                            .collect();
                    } else {
                        self.intelligence.is_deleting = false;
                        self.intelligence.typewriter_phrase_index =
                            (self.intelligence.typewriter_phrase_index + 1) % phrases.len();
                    }
                } else {
                    let target_len = current_phrase.chars().count();
                    if self.intelligence.typewriter_index < target_len + 15 {
                        self.intelligence.typewriter_index += 1;
                        if self.intelligence.typewriter_index <= target_len {
                            self.intelligence.typewriter_text = current_phrase
                                .chars()
                                .take(self.intelligence.typewriter_index)
                                .collect();
                        }
                    } else {
                        self.intelligence.is_deleting = true;
                        self.intelligence.typewriter_index = target_len;
                    }
                }
                Task::none()
            }
            Message::ApplyNativeVibrancy => {
                #[cfg(target_os = "macos")]
                self.apply_vibrancy();
                Task::none()
            }
            Message::EnterApp => {
                self.interaction.show_landing = false;
                self.shell.show_sidebar = true;
                if !self.shell.search_query.trim().is_empty() {
                    let query = self.shell.search_query.clone();
                    self.shell.search_query.clear();
                    self.shell.show_inspector = true;
                    self.interaction.inspector_tab = InspectorTab::App;
                    #[cfg(feature = "intelligence")]
                    return self.start_ai_chat(query);
                    #[cfg(not(feature = "intelligence"))]
                    let _ = query;
                }
                Task::none()
            }
            Message::WindowResized(size) => {
                let was_desktop = self.shell.window_width >= 900.0;
                let is_desktop = size.width >= 900.0;
                if was_desktop && !is_desktop {
                    self.shell.show_sidebar = false;
                }
                if !was_desktop && is_desktop {
                    self.shell.show_sidebar = true;
                }
                self.shell.window_width = size.width;
                self.shell.window_height = size.height;
                Task::none()
            }
            Message::FontLoaded(_) => Task::none(),
            Message::Heartbeat => Task::none(),
            Message::CmdBackspacePressed => {
                self.shell.search_query.clear();
                self.intelligence.chat_input.clear();
                Task::none()
            }
            Message::ExecuteShell(cmd) => {
                log::info!("ExecuteShell: {}", cmd);
                self.intelligence.append_message(ChatMessage {
                    role: ChatRole::System,
                    content: format!("Shell command executed securely: `{}`", cmd),
                });
                Task::none()
            }
            Message::Unknown(s) => {
                log::info!("Unknown message: {}", s);
                Task::none()
            }
            Message::None => Task::none(),
        }
    }

    fn export_view(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Only export if exposure is enabled or we are in dev/debug mode
            // For now, we always export in native to support the local neural API
            let ctx = self.context();
            let view = crate::reference::views::ContentView::new(self);
            let tree = view.describe(&ctx);
            if let Ok(json) = serde_json::to_string(&tree) {
                // Ensure directory exists
                let _ = std::fs::create_dir_all(".peak");
                if let Err(e) = std::fs::write(".peak/current_view.json", json) {
                    log::error!("Failed to export Neural View: {}", e);
                }
            }
        }
    }

    #[cfg(feature = "intelligence")]
    pub fn start_ai_chat(&mut self, query: String) -> Task<Message> {
        self.intelligence.append_message(ChatMessage {
            role: ChatRole::User,
            content: query,
        });
        self.ai_chat_completion()
    }

    #[cfg(feature = "intelligence")]
    pub fn ai_chat_completion(&mut self) -> Task<Message> {
        self.intelligence.is_thinking = true;
        let system_prompt = self.get_system_prompt();
        let mut history: Vec<crate::core::ChatCompletionMessage> = self
            .intelligence
            .chat_messages
            .iter()
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

        let stream = self.intelligence.bridge.chat_stream(history);
        use crate::prelude::futures::StreamExt;
        let mapped_stream = stream
            .map(|res| Message::Intelligence(IntelligenceMessage::ChatStreamUpdate(res)))
            .chain(crate::prelude::futures::stream::once(async {
                Message::Intelligence(IntelligenceMessage::AIChatComplete)
            }));
        Task::stream(mapped_stream)
    }

    #[cfg(feature = "intelligence")]
    pub fn process_assistant_actions(&mut self, text: &str) -> Task<Message> {
        let actions = crate::reference::intelligence::ActionParser::parse_text(text);
        let mut tasks = Vec::new();
        for action in actions {
            match action {
                Action::Navigate(page) => {
                    tasks.push(Task::perform(async {}, move |_| {
                        Message::Shell(ShellMessage::SetTab(page.clone()))
                    }));
                }
                Action::SetButtonVariant(v) => {
                    tasks.push(Task::perform(async {}, move |_| {
                        Message::Lab(LabMessage::UpdateButtonVariant(v))
                    }));
                }
                Action::SetButtonIntent(i) => {
                    tasks.push(Task::perform(async {}, move |_| {
                        Message::Lab(LabMessage::UpdateButtonIntent(i))
                    }));
                }
                Action::SetThemeKind(k) => {
                    tasks.push(Task::perform(async {}, move |_| {
                        Message::Interaction(InteractionMessage::SetThemeKind(k))
                    }));
                }
                Action::SetThemeTone(t) => {
                    tasks.push(Task::perform(async {}, move |_| {
                        Message::Interaction(InteractionMessage::SetTheme(t))
                    }));
                }
                Action::SetLabMode(m) => {
                    tasks.push(Task::perform(async {}, move |_| {
                        Message::Lab(LabMessage::SetRenderMode(m))
                    }));
                }
                Action::WebSearch(query) => {
                    tasks.push(
                        self.intelligence
                            .bridge
                            .execute_tool(
                                "web_search".to_string(),
                                serde_json::json!({ "query": query }),
                            )
                            .map(|res| {
                                Message::Intelligence(IntelligenceMessage::ProcessToolResult(
                                    "web_search".to_string(),
                                    res.unwrap_or_else(|e| serde_json::json!({"error": e})),
                                ))
                            }),
                    );
                }
                Action::ReadFile(path) => {
                    tasks.push(
                        self.intelligence
                            .bridge
                            .execute_tool(
                                "read_file".to_string(),
                                serde_json::json!({ "path": path }),
                            )
                            .map(move |res| {
                                Message::Intelligence(IntelligenceMessage::ProcessToolResult(
                                    "read_file".to_string(),
                                    res.unwrap_or_else(|e| serde_json::json!({"error": e})),
                                ))
                            }),
                    );
                }
                Action::WriteFile { path, content } => {
                    tasks.push(
                        self.intelligence
                            .bridge
                            .execute_tool(
                                "write_file".to_string(),
                                serde_json::json!({ "path": path, "content": content }),
                            )
                            .map(move |res| {
                                Message::Intelligence(IntelligenceMessage::ProcessToolResult(
                                    "write_file".to_string(),
                                    res.unwrap_or_else(|e| serde_json::json!({"error": e})),
                                ))
                            }),
                    );
                }
                Action::Shell(command) => {
                    tasks.push(
                        self.intelligence
                            .bridge
                            .execute_tool(
                                "shell".to_string(),
                                serde_json::json!({ "command": command }),
                            )
                            .map(move |res| {
                                Message::Intelligence(IntelligenceMessage::ProcessToolResult(
                                    "shell".to_string(),
                                    res.unwrap_or_else(|e| serde_json::json!({"error": e})),
                                ))
                            }),
                    );
                }
                Action::Memorize(content) => {
                    tasks.push(
                        self.intelligence
                            .bridge
                            .execute_tool(
                                "memorize".to_string(),
                                serde_json::json!({ "content": content }),
                            )
                            .map(move |res| {
                                Message::Intelligence(IntelligenceMessage::ProcessToolResult(
                                    "memorize".to_string(),
                                    res.unwrap_or_else(|e| serde_json::json!({"error": e})),
                                ))
                            }),
                    );
                }
                _ => {}
            }
        }
        if tasks.is_empty() {
            Task::none()
        } else {
            Task::batch(tasks)
        }
    }

    #[cfg(feature = "intelligence")]
    fn get_system_prompt(&self) -> String {
        let ctx = self.context();
        let view = crate::reference::views::ContentView::new(self);
        let tree = view.describe(&ctx);
        let ui_json = serde_json::to_string(&tree).unwrap_or_default();
        format!("You are PeakUI AI Assistant. Help the user explore the framework.\n\nUI CONTEXT:\n{}\n", ui_json)
    }

    #[cfg(feature = "intelligence")]
    fn reload_intelligence_bridge(&mut self) {
        let provider = match self.intelligence.ai_provider {
            AIProviderChoice::Ollama => peak_intelligence::llm::ModelProvider::Ollama,
            AIProviderChoice::LlamaCpp => peak_intelligence::llm::ModelProvider::LlamaCpp,
            AIProviderChoice::OpenRouter => peak_intelligence::llm::ModelProvider::OpenRouter,
        };
        self.intelligence.bridge = Arc::new(
            crate::reference::intelligence::bridge::PeakIntelligenceBridge::new(
                provider,
                match provider {
                    peak_intelligence::llm::ModelProvider::Ollama => "llama3",
                    _ => "google/gemini-3-flash-preview",
                },
                if self.intelligence.api_key.is_empty() {
                    None
                } else {
                    Some(self.intelligence.api_key.clone())
                },
                #[cfg(feature = "neural")]
                self.db.clone(),
                #[cfg(not(feature = "neural"))]
                Arc::new(crate::reference::data::stub_db::StubDB::new()),
            ),
        );
    }

    #[cfg(target_os = "macos")]
    fn apply_vibrancy(&self) {
        // Platform specific logic...
    }
}

impl IntelligenceState {
    pub fn append_message(&mut self, msg: ChatMessage) {
        Arc::make_mut(&mut self.chat_messages).push(msg);
    }

    pub fn update_last_message_delta(&mut self, _delta: &str) {
        // Logic for streaming updates already inlined in update
    }
}
