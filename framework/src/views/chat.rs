use crate::core::{Backend, Context, ScrollDirection, SemanticNode, View};
use crate::modifiers::{Intent, Variant};
use crate::reference::intelligence::{Action, ActionParser, ContentPart};
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
            ScrollDirection::Vertical,
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
                B::space(Length::Fill, Length::Shrink, context),
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
            Length::Shrink,
            true, // is_compact = true
            context,
        );

        let input_row_2_fixed = B::hstack(
            vec![B::space(Length::Fill, Length::Shrink, context), send_btn],
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
        SemanticNode::new("ai_chat").with_label("AI Assistant")
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
                vec![B::space(Length::Fill, Length::Shrink, context), bubble],
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
            // AI Message: Parse for Actions using the new Protocol
            let parts = ActionParser::split_text_and_actions(&self.message.content);
            let mut assistant_children = Vec::new();
            let on_act = self.on_action.clone();

            for part in parts {
                match part {
                    ContentPart::Text(text) => {
                        let on_act_inner = on_act.clone();
                        let content = MarkdownView::new(text)
                            .size(12.0)
                            .padding(Padding::ZERO)
                            .on_copy(move |code| (on_act_inner)(ChatViewMessage::CopyCode(code)));
                        assistant_children.push(View::<Message, B>::view(&content, context));
                    }
                    ContentPart::Action(action) => {
                        assistant_children
                            .push(View::<Message, B>::view(&ToolCard::new(action), context));
                    }
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
        SemanticNode::new("chat_bubble").with_content(self.message.content.clone())
    }
}

pub struct ToolCard {
    action: Action,
}

impl ToolCard {
    pub fn new(action: Action) -> Self {
        Self { action }
    }
}

impl<Message: Clone + 'static, B: Backend> View<Message, B> for ToolCard {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let t = context.theme;

        let icon_name = match &self.action {
            Action::Navigate(_) => "navigation",
            Action::SetThemeKind(_) | Action::SetThemeTone(_) => "palette",
            Action::SetButtonVariant(_) | Action::SetButtonIntent(_) => "mouse-pointer",
            Action::SetLabMode(_) => "flask-conical",
            Action::Shell(_) => "terminal",
            Action::Memorize(_) => "database",
            Action::Teleport { .. } => "move",
            Action::Scale { .. } => "maximize",
            Action::Rotate { .. } => "rotate-cw",
            Action::Unknown(_) => "help-circle",
        };

        let (display_name, params) = match &self.action {
            Action::Navigate(page) => ("Navigate", format!("{:?}", page)),
            Action::SetThemeKind(kind) => ("Change Theme", format!("{:?}", kind)),
            Action::SetThemeTone(tone) => ("Switch Tone", format!("{:?}", tone)),
            Action::SetButtonVariant(v) => ("Update Button Style", format!("{:?}", v)),
            Action::SetButtonIntent(i) => ("Update Button Intent", format!("{:?}", i)),
            Action::SetLabMode(m) => ("Switch Mode", format!("{:?}", m)),
            Action::Shell(cmd) => ("Execute Shell", cmd.clone()),
            Action::Memorize(content) => ("Memorize", content.clone()),
            Action::Teleport { target, x, y, z } => (
                "Teleport",
                format!("{} to [{:.0}, {:.0}, {:.0}]", target, x, y, z),
            ),
            Action::Scale { target, factor } => ("Scale", format!("{} by {:.2}x", target, factor)),
            Action::Rotate { target, x, y, z } => (
                "Rotate",
                format!("{} to [{:.0}, {:.0}, {:.0}]", target, x, y, z),
            ),
            Action::Unknown(raw) => ("Action", raw.clone()),
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
                            params,
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
        SemanticNode::new("tool_card").with_label(format!("{:?}", self.action))
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
        SemanticNode::new("thinking_indicator").with_content("AI is thinking...")
    }
}
