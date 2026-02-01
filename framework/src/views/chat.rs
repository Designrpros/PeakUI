use crate::core::{Backend, Context, SemanticNode, View};
use crate::modifiers::{Intent, Variant};
use crate::views::MarkdownView;
use iced::{Length, Padding};
use std::sync::Arc;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ChatRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone)]
pub enum ChatViewMessage {
    InputChanged(String),
    SendPressed,
    CopyCode(String),
}

pub struct AIChatView<Message: Clone + 'static> {
    messages: Vec<ChatMessage>,
    input_value: String,
    is_thinking: bool,
    on_action: Arc<dyn Fn(ChatViewMessage) -> Message + Send + Sync>,
}

impl<Message: Clone + 'static> AIChatView<Message> {
    pub fn new(
        messages: Vec<ChatMessage>,
        input_value: String,
        is_thinking: bool,
        on_action: impl Fn(ChatViewMessage) -> Message + Send + Sync + 'static,
    ) -> Self {
        Self {
            messages,
            input_value,
            is_thinking,
            on_action: Arc::new(on_action),
        }
    }
}

// Consolidate backend implementations into one generic implementation
impl<Message: Clone + 'static, B: Backend> View<Message, B> for AIChatView<Message> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let on_action = &self.on_action;

        // 1. Create message list
        let mut message_list = Vec::new();

        for msg in &self.messages {
            message_list.push(View::<Message, B>::view(
                &ChatBubble::new(msg.clone(), self.on_action.clone()),
                context,
            ));
        }

        if self.is_thinking {
            message_list.push(View::<Message, B>::view(&ThinkingBubble {}, context));
        }

        // Wrap message list in VStack
        let message_stack = B::vstack(
            message_list,
            12.0,
            Padding {
                top: 20.0,
                bottom: 20.0,
                left: 12.0,
                right: 12.0,
            },
            Length::Fill,
            Length::Shrink, // Only as tall as content? No, vstack for content list usually shrinks.
            // But ScrollView handles the scrolling.
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        );

        let scroll = B::scroll_view(
            message_stack,
            Length::Fill,
            Length::Fill,
            Some("chat_scroll"),
            true,
            context,
        );

        // 2. Input Area
        let input_row_1 = B::text_input(
            self.input_value.clone(),
            "Ask anything...".to_string(),
            {
                let act = self.on_action.clone();
                move |s| (act)(ChatViewMessage::InputChanged(s))
            },
            Some((on_action)(ChatViewMessage::SendPressed)),
            None,
            false,
            Variant::Ghost,
            Some(iced::widget::Id::new("chat_input")),
            context,
        );

        let _input_row_2 = B::hstack(
            vec![
                B::space(Length::Fill, Length::Shrink),
                B::button(
                    B::text(
                        "".to_string(),
                        14.0,
                        None,
                        false,
                        false,
                        None,
                        None,
                        Length::Shrink,
                        iced::Alignment::Center,
                        context,
                    ), // Empty label, icon handled by modifier or separate setup?
                    // Button helper in Backend trait currently takes 'content' View.
                    // But here we want Icon button logic.
                    // Let's create content with Icon.
                    Some((on_action)(ChatViewMessage::SendPressed)),
                    Variant::Compact,
                    Intent::Neutral,
                    Length::Shrink,
                    false, // is_compact
                    context,
                ), // Wait, original code was:
                   // Button::label("").icon("arrow-up").variant(Variant::Compact)
                   // The generic B::button takes already built content.
                   // So I wrap Icon in it.
            ],
            0.0,
            Padding::ZERO,
            Length::Fill, // HStack Width
            Length::Shrink,
            iced::Alignment::Start,
            iced::Alignment::Center,
            context,
        );

        // Fix input_row_2 button content:
        let send_icon = crate::atoms::Icon::<B>::new("arrow-up").view(context);
        let send_btn = B::button(
            send_icon,
            Some((on_action)(ChatViewMessage::SendPressed)),
            Variant::Compact,
            Intent::Neutral,
            Length::Shrink,
            true, // is_compact = true
            context,
        );

        let input_row_2_fixed = B::hstack(
            vec![B::space(Length::Fill, Length::Shrink), send_btn],
            0.0,
            Padding::ZERO,
            Length::Fill,
            Length::Shrink,
            iced::Alignment::Start,
            iced::Alignment::Center,
            context,
        );

        let divider = B::divider(context);

        let input_area = B::vstack(
            vec![
                divider,
                B::vstack(
                    vec![input_row_1, input_row_2_fixed],
                    8.0,
                    Padding::from(12),
                    Length::Fill,
                    Length::Shrink,
                    iced::Alignment::Start,
                    iced::Alignment::Start,
                    context,
                ),
            ],
            8.0,
            Padding::ZERO,
            Length::Fill,
            Length::Shrink,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        );

        // 3. Combine
        B::vstack(
            vec![scroll, input_area],
            0.0,
            Padding::ZERO,
            Length::Fill,
            Length::Fill,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        )
    }

    fn describe(&self, _context: &Context) -> SemanticNode {
        SemanticNode {
            accessibility: None,
            role: "ai_chat".to_string(),
            label: Some("AI Assistant".to_string()),
            content: None,
            children: vec![],
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }
}

// ChatBubble View
struct ChatBubble<Message> {
    message: ChatMessage,
    on_action: Arc<dyn Fn(ChatViewMessage) -> Message + Send + Sync>,
}

impl<Message> ChatBubble<Message> {
    pub fn new(
        message: ChatMessage,
        on_action: Arc<dyn Fn(ChatViewMessage) -> Message + Send + Sync>,
    ) -> Self {
        Self { message, on_action }
    }
}

impl<Message: Clone + 'static, B: Backend> View<Message, B> for ChatBubble<Message> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let t = context.theme;
        let is_user = self.message.role == ChatRole::User;

        if is_user {
            let bubble_content = B::text(
                self.message.content.clone(),
                14.0, // Caption1 size approx
                Some(t.colors.on_primary),
                false,
                false,
                None,
                None,
                Length::Shrink, // Text width
                iced::Alignment::Start,
                context,
            );

            let bubble = B::container(
                bubble_content,
                Padding::from(12),
                Length::Shrink,
                Length::Shrink,
                Some(t.colors.primary),
                12.0,
                0.0,
                None,
                None,
                iced::Alignment::Start,
                iced::Alignment::Start,
                context,
            );

            B::hstack(
                vec![B::space(Length::Fill, Length::Shrink), bubble],
                0.0,
                Padding {
                    top: 0.0,
                    right: 12.0,
                    bottom: 0.0,
                    left: 0.0,
                },
                Length::Fill,
                Length::Shrink,
                iced::Alignment::Start,
                iced::Alignment::Start,
                context,
            )
        } else {
            // AI Message: Parse for Actions
            let raw_content = self.message.content.clone();
            let mut text_parts = Vec::new();
            let mut actions = Vec::new();

            let mut last_idx = 0;
            let action_pattern = "[Action: ";
            let action_end = ")]";

            let mut search_str = &raw_content[..];
            let mut current_pos = 0;

            while let Some(start_rel) = search_str.find(action_pattern) {
                let start = current_pos + start_rel;
                if let Some(end_rel) = search_str[start_rel..].find(action_end) {
                    let end = start + end_rel + action_end.len();
                    if start > last_idx {
                        text_parts.push(raw_content[last_idx..start].to_string());
                    }
                    actions.push(raw_content[start..end].to_string());
                    last_idx = end;
                    search_str = &raw_content[last_idx..];
                    current_pos = last_idx;
                } else {
                    break;
                }
            }

            if last_idx < raw_content.len() {
                text_parts.push(raw_content[last_idx..].to_string());
            }

            let mut assistant_children = Vec::new();
            let on_act = self.on_action.clone();

            if actions.is_empty() {
                let on_act_inner = on_act.clone();
                let content = MarkdownView::new(raw_content)
                    .size(12.0)
                    .padding(Padding::ZERO)
                    .on_copy(move |code| (on_act_inner)(ChatViewMessage::CopyCode(code)));
                assistant_children.push(View::<Message, B>::view(&content, context));
            } else {
                for text in text_parts {
                    if !text.trim().is_empty() {
                        let on_act_inner = on_act.clone();
                        let content = MarkdownView::new(text)
                            .size(12.0)
                            .padding(Padding::ZERO)
                            .on_copy(move |code| (on_act_inner)(ChatViewMessage::CopyCode(code)));
                        assistant_children.push(View::<Message, B>::view(&content, context));
                    }
                }

                for action in actions {
                    assistant_children
                        .push(View::<Message, B>::view(&ToolCard::new(action), context));
                }
            }

            let assistant_col = B::vstack(
                assistant_children,
                12.0,
                Padding::ZERO,
                Length::Fill,
                Length::Shrink,
                iced::Alignment::Start,
                iced::Alignment::Start,
                context,
            );

            B::container(
                assistant_col,
                Padding::from(12),
                Length::Fill,
                Length::Shrink, // Container height
                None,           // Transparent
                0.0,
                0.0,
                None,
                None,
                iced::Alignment::Start,
                iced::Alignment::Start,
                context,
            )
        }
    }

    fn describe(&self, _context: &Context) -> SemanticNode {
        SemanticNode {
            accessibility: None,
            role: "chat_bubble".to_string(),
            content: Some(self.message.content.clone()),
            label: None,
            children: vec![],
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }
}

pub struct ToolCard {
    action_tag: String,
}

impl ToolCard {
    pub fn new(action_tag: impl Into<String>) -> Self {
        Self {
            action_tag: action_tag.into(),
        }
    }
}

impl<Message: Clone + 'static, B: Backend> View<Message, B> for ToolCard {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let t = context.theme;

        // Parse: [Action: Name(Params)]
        let inner = self
            .action_tag
            .trim_start_matches("[Action: ")
            .trim_end_matches(")]");
        let (name, params) = if let Some(idx) = inner.find('(') {
            (&inner[..idx], &inner[idx + 1..])
        } else {
            (inner, "")
        };

        let icon_name = match name.to_lowercase().as_str() {
            "navigate" => "navigation",
            "setthemekind" | "setthemetone" => "palette",
            "setbuttonvariant" | "setbuttonintent" => "mouse-pointer",
            "setlabmode" => "flask-conical",
            _ => "terminal",
        };

        let display_name = match name.to_lowercase().as_str() {
            "navigate" => "Navigate",
            "setthemekind" => "Change Theme",
            "setthemetone" => "Switch Tone",
            "setbuttonvariant" => "Update Button Style",
            "setbuttonintent" => "Update Button Intent",
            "setlabmode" => "Switch Mode",
            "setspacing" => "Adjust Spacing",
            "setalignment" => "Set Alignment",
            _ => name,
        };

        let card_content = B::hstack(
            vec![
                B::icon(icon_name.to_string(), 16.0, Some(t.colors.primary), context),
                B::vstack(
                    vec![
                        B::text(
                            display_name.to_string(),
                            12.0,
                            None,
                            true,
                            false,
                            None,
                            None,
                            Length::Shrink,
                            iced::Alignment::Start,
                            context,
                        ),
                        B::text(
                            params.to_string(),
                            10.0,
                            Some(t.colors.text_secondary),
                            false,
                            true,
                            None,
                            None,
                            Length::Shrink,
                            iced::Alignment::Start,
                            context,
                        ),
                    ],
                    2.0,
                    Padding::ZERO,
                    Length::Shrink,
                    Length::Shrink,
                    iced::Alignment::Start,
                    iced::Alignment::Start,
                    context,
                ),
            ],
            12.0,
            Padding::ZERO,
            Length::Shrink,
            Length::Shrink,
            iced::Alignment::Start,
            iced::Alignment::Center,
            context,
        );

        B::container(
            card_content,
            Padding::from(8),
            Length::Shrink,
            Length::Shrink,
            Some(t.colors.surface.scale_alpha(0.3)), // extended_palette replacement
            8.0,
            1.0,
            Some(t.colors.border.scale_alpha(0.5)), // extended_palette replacement
            None,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        )
    }

    fn describe(&self, _context: &Context) -> SemanticNode {
        SemanticNode {
            role: "tool_card".to_string(),
            label: Some(self.action_tag.clone()),
            ..Default::default()
        }
    }
}

// ThinkingBubble View
struct ThinkingBubble {}

impl<Message: Clone + 'static, B: Backend> View<Message, B> for ThinkingBubble {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let content = B::hstack(
            vec![
                B::text(
                    "•".to_string(),
                    16.0,
                    None,
                    false,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    iced::Alignment::Start,
                    context,
                ),
                B::text(
                    "•".to_string(),
                    16.0,
                    None,
                    false,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    iced::Alignment::Start,
                    context,
                ),
                B::text(
                    "•".to_string(),
                    16.0,
                    None,
                    false,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    iced::Alignment::Start,
                    context,
                ),
            ],
            4.0,
            Padding::ZERO,
            Length::Shrink,
            Length::Shrink,
            iced::Alignment::Start,
            iced::Alignment::Center,
            context,
        );

        B::container(
            content,
            Padding::new(12.0).right(20.0).left(20.0),
            Length::Fill,
            Length::Shrink,
            None,
            0.0,
            0.0,
            None,
            None,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        )
    }

    fn describe(&self, _context: &Context) -> SemanticNode {
        SemanticNode {
            role: "thinking_indicator".to_string(),
            content: Some("AI is thinking...".to_string()),
            ..Default::default()
        }
    }
}
