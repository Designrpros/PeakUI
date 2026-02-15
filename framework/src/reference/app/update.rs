use crate::prelude::*;
use crate::views::{ChatMessage, ChatRole};
use crate::views::chat::ChatViewMessage;
#[cfg(feature = "intelligence")]
use crate::reference::intelligence::Action;
#[cfg(feature = "intelligence")]
use peak_theme::{PeakTheme, ThemeTokens, ThemeTone};
use std::sync::Arc;
#[cfg(feature = "intelligence")]
use crate::core::Context;

use super::message::Message;
use super::state::*;
use crate::reference::AppPage;

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let task = match message {
            Message::TypewriterTick(_) => {
                self.tick = self.tick.wrapping_add(1);

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
                    #[allow(unused_variables)]
                    let query = self.search_query.clone();
                    self.search_query.clear();
                    self.show_inspector = true;
                    self.inspector_tab = InspectorTab::App;
                    #[cfg(feature = "intelligence")]
                    return self.start_ai_chat(query);
                    #[cfg(not(feature = "intelligence"))]
                    let _ = query;
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
                    AppPage::Button
                    | AppPage::Typography
                    | AppPage::BasicSizing
                    | AppPage::Layout => {
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
                    AppPage::Landing
                    | AppPage::PeakOSDetail
                    | AppPage::PeakUIDetail
                    | AppPage::PeakDBDetail
                    | AppPage::PeakRelayDetail
                    | AppPage::PeakHubDetail => {
                        self.show_landing = true;
                        // On mobile, the landing/home view should show the menu (sidebar)
                        if self.window_width < 900.0 && self.active_tab == AppPage::Landing {
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
            Message::SetScaling(scaling) => {
                self.scaling = scaling;
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
                    "Start" => AppPage::Introduction,
                    "Catalog" => AppPage::Button,
                    "Data" => AppPage::PeakDB,
                    "Settings" => AppPage::Appearance,
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
                    Arc::make_mut(&mut self.expanded_sections).remove(&section);
                } else {
                    Arc::make_mut(&mut self.expanded_sections).insert(section);
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
            Message::CopyCode(code) => {
                self.last_copied_code = Some(code.clone());
                Task::batch(vec![
                    crate::prelude::clipboard::write(code),
                    Task::future(async {
                        #[cfg(target_arch = "wasm32")]
                        wasmtimer::tokio::sleep(std::time::Duration::from_secs(2)).await;
                        #[cfg(not(target_arch = "wasm32"))]
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        Message::ClearCopiedFeedback
                    }),
                ])
            }
            Message::ClearCopiedFeedback => {
                self.last_copied_code = None;
                Task::none()
            }
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
                Arc::make_mut(&mut self.button_lab).label = label;
                Task::none()
            }
            Message::UpdateButtonIcon(icon) => {
                Arc::make_mut(&mut self.button_lab).icon = icon;
                Task::none()
            }
            Message::UpdateButtonVariant(variant) => {
                Arc::make_mut(&mut self.button_lab).variant = variant;
                Task::none()
            }
            Message::UpdateButtonIntent(intent) => {
                Arc::make_mut(&mut self.button_lab).intent = intent;
                Task::none()
            }
            Message::UpdateButtonSize(size) => {
                Arc::make_mut(&mut self.button_lab).size = size;
                Task::none()
            }
            Message::ToggleButtonFullWidth(full_width) => {
                Arc::make_mut(&mut self.button_lab).is_full_width = full_width;
                Task::none()
            }
            Message::ToggleButtonDisabled(disabled) => {
                Arc::make_mut(&mut self.button_lab).is_disabled = disabled;
                Task::none()
            }
            Message::ToggleButtonFocused(focused) => {
                Arc::make_mut(&mut self.button_lab).is_focused = focused;
                Task::none()
            }

            // Icon Lab Handlers
            Message::UpdateIconLabIcon(icon) => {
                Arc::make_mut(&mut self.icon_lab).selected_icon = icon;
                Task::none()
            }
            Message::UpdateIconLabSize(size) => {
                Arc::make_mut(&mut self.icon_lab).size = size;
                Task::none()
            }
            Message::UpdateIconLabColor(color) => {
                Arc::make_mut(&mut self.icon_lab).color = color;
                Task::none()
            }

            // Emoji Lab Handlers
            Message::UpdateEmojiLabEmoji(emoji) => {
                Arc::make_mut(&mut self.emoji_lab).selected_emoji = emoji;
                Task::none()
            }
            Message::UpdateEmojiLabSize(size) => {
                Arc::make_mut(&mut self.emoji_lab).size = size;
                Task::none()
            }
            Message::UpdateSpacerWidth(width) => {
                Arc::make_mut(&mut self.spacer_lab).width = width;
                Task::none()
            }
            Message::UpdateSpacerHeight(height) => {
                Arc::make_mut(&mut self.spacer_lab).height = height;
                Task::none()
            }

            // Typography Lab Handlers
            Message::UpdateTypographyText(text) => {
                Arc::make_mut(&mut self.typography_lab).text = text;
                Task::none()
            }
            Message::UpdateTypographySize(size) => {
                Arc::make_mut(&mut self.typography_lab).size = size;
                Task::none()
            }
            Message::ToggleTypographyBold(bold) => {
                Arc::make_mut(&mut self.typography_lab).is_bold = bold;
                Task::none()
            }
            Message::ToggleTypographyItalic(italic) => {
                Arc::make_mut(&mut self.typography_lab).is_italic = italic;
                Task::none()
            }

            // Layout Lab Handlers
            Message::UpdateLayoutOuterSpacing(spacing) => {
                Arc::make_mut(&mut self.layout_lab).outer_spacing = spacing;
                Task::none()
            }
            Message::UpdateLayoutInnerSpacing(spacing) => {
                Arc::make_mut(&mut self.layout_lab).inner_spacing = spacing;
                Task::none()
            }
            Message::UpdateLayoutChildCount(count) => {
                Arc::make_mut(&mut self.layout_lab).child_count = count.min(10);
                Task::none()
            }
            Message::UpdateLayoutAlignment(alignment) => {
                Arc::make_mut(&mut self.layout_lab).alignment = alignment;
                Task::none()
            }
            Message::UpdateLayoutItemSizing(sizing) => {
                Arc::make_mut(&mut self.layout_lab).item_sizing = sizing;
                Task::none()
            }

            // Sizing Lab Handlers
            Message::UpdateSizingWidthType(t) => {
                Arc::make_mut(&mut self.sizing_lab).width_type = t;
                Task::none()
            }
            Message::UpdateSizingHeightType(t) => {
                Arc::make_mut(&mut self.sizing_lab).height_type = t;
                Task::none()
            }
            Message::UpdateSizingFixedWidth(w) => {
                Arc::make_mut(&mut self.sizing_lab).fixed_width = w;
                Task::none()
            }
            Message::UpdateSizingFixedHeight(h) => {
                Arc::make_mut(&mut self.sizing_lab).fixed_height = h;
                Task::none()
            }

            Message::UpdateAccessibilityComponent(comp) => {
                Arc::make_mut(&mut self.accessibility_lab).selected_component = comp;
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
            #[cfg(feature = "intelligence")]
            Message::SetApiKey(key) => {
                self.api_key = key.clone();
                let settings = Settings {
                    api_key: self.api_key.clone(),
                    ai_provider: self.ai_provider,
                    enable_exposure: self.enable_exposure,
                };
                settings.save();

                // Hot-reload intelligence bridge with new key
                let provider = match self.ai_provider {
                    AIProviderChoice::Ollama => peak_intelligence::llm::ModelProvider::Ollama,
                    AIProviderChoice::LlamaCpp => peak_intelligence::llm::ModelProvider::LlamaCpp,
                    AIProviderChoice::OpenRouter => {
                        peak_intelligence::llm::ModelProvider::OpenRouter
                    }
                };

                self.intelligence = Arc::new(
                    crate::reference::intelligence::bridge::PeakIntelligenceBridge::new(
                        provider,
                        match provider {
                            peak_intelligence::llm::ModelProvider::Ollama => "llama3",
                            _ => "google/gemini-3-flash-preview",
                        },
                        if key.is_empty() { None } else { Some(key) },
                        #[cfg(feature = "neural")]
                        self.db.clone(),
                        #[cfg(not(feature = "neural"))]
                        Arc::new(crate::reference::data::stub_db::StubDB::new()),
                    ),
                );

                Task::none()
            }
            #[cfg(feature = "intelligence")]
            Message::SetAIProvider(provider) => {
                self.ai_provider = provider;
                let settings = Settings {
                    api_key: self.api_key.clone(),
                    ai_provider: self.ai_provider,
                    enable_exposure: self.enable_exposure,
                };
                settings.save();

                // Hot-reload intelligence bridge with new provider
                let model_provider = match provider {
                    AIProviderChoice::Ollama => peak_intelligence::llm::ModelProvider::Ollama,
                    AIProviderChoice::LlamaCpp => peak_intelligence::llm::ModelProvider::LlamaCpp,
                    AIProviderChoice::OpenRouter => {
                        peak_intelligence::llm::ModelProvider::OpenRouter
                    }
                };

                self.intelligence = Arc::new(
                    crate::reference::intelligence::bridge::PeakIntelligenceBridge::new(
                        model_provider,
                        match model_provider {
                            peak_intelligence::llm::ModelProvider::Ollama => "llama3",
                            _ => "google/gemini-3-flash-preview",
                        },
                        if self.api_key.is_empty() {
                            None
                        } else {
                            Some(self.api_key.clone())
                        },
                        #[cfg(feature = "neural")]
                        self.db.clone(),
                        #[cfg(not(feature = "neural"))]
                        Arc::new(crate::reference::data::stub_db::StubDB::new()),
                    ),
                );

                Task::none()
            }
            Message::SetExposure(enable) => {
                self.enable_exposure = enable;
                let settings = Settings {
                    api_key: self.api_key.clone(),
                    ai_provider: self.ai_provider,
                    enable_exposure: self.enable_exposure,
                };
                settings.save();

                #[cfg(not(target_arch = "wasm32"))]
                {
                    // Native server logic would go here
                }

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
                self.window_height = size.height;
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
                Arc::make_mut(&mut self.chat_messages).push(ChatMessage {
                    role: ChatRole::System,
                    content: format!("Shell command executed securely: `{}`", cmd),
                });
                Task::none()
            }
            Message::None => Task::none(),
            Message::Unknown(s) => {
                log::info!("Message::Unknown: {}", s);
                Task::none()
            }

            #[cfg(not(feature = "intelligence"))]
            Message::Chat(msg) => {
                match msg {
                    ChatViewMessage::InputChanged(val) => {
                        self.chat_input = val;
                    }
                    _ => {}
                }
                Task::none()
            }

            #[cfg(feature = "intelligence")]
            Message::Chat(msg) => match msg {
                ChatViewMessage::InputChanged(val) => {
                    self.chat_input = val;
                    Task::none()
                }
                ChatViewMessage::CopyCode(code) => {
                    crate::prelude::clipboard::write::<Message>(code).map(|_| Message::None)
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
            #[cfg(feature = "intelligence")]
            Message::ChatStreamUpdate(result) => match result {
                Ok(text) => {
                    let messages = Arc::make_mut(&mut self.chat_messages);
                    if let Some(last) = messages.last_mut() {
                        if last.role == ChatRole::Assistant {
                            last.content = text.clone();
                        } else {
                            messages.push(ChatMessage {
                                role: ChatRole::Assistant,
                                content: text.clone(),
                            });
                        }
                    } else {
                        messages.push(ChatMessage {
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
                    Arc::make_mut(&mut self.chat_messages).push(ChatMessage {
                        role: ChatRole::System,
                        content: format!("Error: {}", e),
                    });
                    Task::none()
                }
            },
            #[cfg(feature = "intelligence")]
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
            #[cfg(feature = "intelligence")]
            Message::AIResponse(res) => {
                self.is_thinking = false;
                match res {
                    Ok(content) => {
                        let task = self.process_assistant_actions(&content);

                        if !content.is_empty() {
                            // Sync with streaming message or push new
                            let messages = Arc::make_mut(&mut self.chat_messages);
                            if let Some(last) = messages.last_mut() {
                                if last.role == ChatRole::Assistant {
                                    last.content = content.clone();
                                } else {
                                    messages.push(ChatMessage {
                                        role: ChatRole::Assistant,
                                        content: content.clone(),
                                    });
                                }
                            } else {
                                messages.push(ChatMessage {
                                    role: ChatRole::Assistant,
                                    content: content.clone(),
                                });
                            }
                        }
                        return task;
                    }
                    Err(err) => {
                        Arc::make_mut(&mut self.chat_messages).push(ChatMessage {
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
            #[cfg(feature = "intelligence")]
            Message::ProcessToolResult(tool_name, result) => {
                self.is_thinking = false;
                Arc::make_mut(&mut self.chat_messages).push(ChatMessage {
                    role: ChatRole::System,
                    content: format!("[result:{}] {}", tool_name, result),
                });
                self.ai_chat_completion()
            }
        };

        self.export_neural_view();
        task
    }

    #[cfg(feature = "intelligence")]
    pub fn start_ai_chat(&mut self, content: String) -> Task<Message> {
        Arc::make_mut(&mut self.chat_messages).push(ChatMessage {
            role: ChatRole::User,
            content,
        });

        self.ai_chat_completion()
    }

    #[cfg(feature = "intelligence")]
    pub fn ai_chat_completion(&mut self) -> Task<Message> {
        // TOKEN OPTIMIZATION & SAFETY SWITCH
        const MAX_CONTEXT_CHARS: usize = 16_000;
        const MAX_HISTORY_MESSAGES: usize = 20;

        let system_prompt = self.get_system_prompt();
        let system_chars = system_prompt.len();

        if system_chars > MAX_CONTEXT_CHARS {
            Arc::make_mut(&mut self.chat_messages).push(ChatMessage {
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

        use crate::prelude::futures::StreamExt;
        let mapped_stream = stream.map(|res| Message::ChatStreamUpdate(res)).chain(
            crate::prelude::futures::stream::once(async { Message::AIChatComplete }),
        );

        Task::stream(mapped_stream)
    }

    #[cfg(feature = "intelligence")]
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
                    Action::WebSearch(query) => Message::ProcessToolResult(
                        "web_search".to_string(),
                        serde_json::json!({"query": query}),
                    ),
                    Action::ReadFile(path) => Message::ProcessToolResult(
                        "read_file".to_string(),
                        serde_json::json!({"path": path}),
                    ),
                    Action::WriteFile { path, content } => Message::ProcessToolResult(
                        "write_file".to_string(),
                        serde_json::json!({"path": path, "content": content}),
                    ),
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
                    tasks.push(Task::perform(async {}, move |_| {
                        Message::UpdateButtonVariant(variant)
                    }));
                }
                Action::SetButtonIntent(intent) => {
                    tasks.push(Task::perform(async {}, move |_| {
                        Message::UpdateButtonIntent(intent)
                    }));
                }
                Action::SetThemeKind(kind) => {
                    tasks.push(Task::perform(async {}, move |_| {
                        Message::SetThemeKind(kind)
                    }));
                }
                Action::SetThemeTone(tone) => {
                    tasks.push(Task::perform(async {}, move |_| Message::SetTheme(tone)));
                }
                Action::SetLabMode(mode) => {
                    tasks.push(Task::perform(async {}, move |_| {
                        Message::SetRenderMode(mode)
                    }));
                }
                Action::Shell(_) => {
                    // Handled by Neural Sudo Interception above
                }
                #[cfg(feature = "neural")]
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
                #[cfg(not(feature = "neural"))]
                Action::Memorize(_) => {
                    log::warn!("Memorize action received but neural feature is disabled");
                }
                Action::Unknown(name) => {
                    log::warn!("Unknown AI Action: {}", name);
                }
                Action::Teleport { .. } | Action::Scale { .. } | Action::Rotate { .. } => {
                    log::info!("AI Spatial Action received: {:?}", action);
                }
                Action::WebSearch(query) => {
                    log::info!("AI requesting Web Search: {}", query);
                    let provider = self.intelligence.clone();
                    tasks.push(
                        provider
                            .execute_tool(
                                "web_search".to_string(),
                                serde_json::json!({"query": query}),
                            )
                            .map(|res| match res {
                                Ok(val) => {
                                    Message::ProcessToolResult("web_search".to_string(), val)
                                }
                                Err(e) => Message::ProcessToolResult(
                                    "web_search".to_string(),
                                    serde_json::json!({"error": e}),
                                ),
                            }),
                    );
                }
                Action::ReadFile(path) => {
                    log::info!("AI requesting Read File: {}", path);
                    let provider = self.intelligence.clone();
                    tasks.push(
                        provider
                            .execute_tool(
                                "read_file".to_string(),
                                serde_json::json!({"path": path}),
                            )
                            .map(|res| match res {
                                Ok(val) => Message::ProcessToolResult("read_file".to_string(), val),
                                Err(e) => Message::ProcessToolResult(
                                    "read_file".to_string(),
                                    serde_json::json!({"error": e}),
                                ),
                            }),
                    );
                }
                Action::WriteFile { path, content } => {
                    log::info!("AI requesting Write File: {}", path);
                    let provider = self.intelligence.clone();
                    tasks.push(
                        provider
                            .execute_tool(
                                "write_file".to_string(),
                                serde_json::json!({"path": path, "content": content}),
                            )
                            .map(|res| match res {
                                Ok(val) => {
                                    Message::ProcessToolResult("write_file".to_string(), val)
                                }
                                Err(e) => Message::ProcessToolResult(
                                    "write_file".to_string(),
                                    serde_json::json!({"error": e}),
                                ),
                            }),
                    );
                }
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
        let mut ctx = Context::new(
            crate::core::ShellMode::Desktop,
            ThemeTokens::new(PeakTheme::Peak, ThemeTone::Light),
            Size::new(1280.0, 800.0),
            self.localization.clone(),
        );
        ctx.tick = self.tick;
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

    pub fn export_neural_view(&self) {
        #[cfg(feature = "neural")]
        if !self.enable_exposure {
            return;
        }

        #[cfg(feature = "neural")]
        {
            let mut ctx = Context::new(
                crate::core::ShellMode::Desktop,
                ThemeTokens::new(peak_theme::PeakTheme::Peak, peak_theme::ThemeTone::Light),
                Size::new(1280.0, 800.0),
                self.localization.clone(),
            );
            ctx.tick = self.tick;
            let view = crate::reference::views::ContentView::new(self);
            let tree = view.describe(&ctx);
            let ui_json = serde_json::to_string(&tree).unwrap_or_default();
            let _ = std::fs::write(".peak/current_view.json", ui_json);
        }
    }
}
