use crate::core::Backend;
use crate::prelude::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

// Implement View for AIChatView (IcedBackend)
impl<Message: Clone + 'static> View<Message, crate::core::IcedBackend> for AIChatView<Message> {
    fn view(&self, context: &Context) -> iced::Element<'static, Message, Theme, Renderer> {
        let on_action = &self.on_action;

        // 1. Create message list
        let mut message_stack = VStack::<Message, crate::core::IcedBackend>::new()
            .spacing(12.0)
            .padding(iced::Padding {
                top: 20.0,
                bottom: 20.0,
                left: 12.0,
                right: 12.0,
            })
            .width(Length::Fill);

        for msg in &self.messages {
            message_stack =
                message_stack.push(ChatBubble::new(msg.clone(), self.on_action.clone()));
        }

        if self.is_thinking {
            message_stack = message_stack.push(ThinkingBubble {});
        }

        let scroll = ScrollView::<Message, crate::core::IcedBackend>::new(message_stack)
            .height(Length::Fill)
            .id("chat_scroll");

        // 2. Input Area
        let input_row_1 = TextInput::new(&self.input_value, "Ask anything...", {
            let act = self.on_action.clone();
            move |s| (act)(ChatViewMessage::InputChanged(s))
        })
        .variant(Variant::Ghost)
        .width(Length::Fill)
        .on_submit((on_action)(ChatViewMessage::SendPressed));

        let input_row_2 = HStack::<Message, crate::core::IcedBackend>::new()
            .align_y(iced::Alignment::Center)
            .push(crate::atoms::Space::new(Length::Fill, Length::Shrink))
            .push(
                Button::<Message, crate::core::IcedBackend>::label("")
                    .icon("arrow-up")
                    .variant(Variant::Solid)
                    .compact()
                    .on_press((on_action)(ChatViewMessage::SendPressed)),
            );

        let divider = crate::atoms::Divider::new();

        let input_area = VStack::<Message, crate::core::IcedBackend>::new()
            .spacing(8.0)
            .push(divider)
            .push(
                VStack::<Message, crate::core::IcedBackend>::new()
                    .padding(12.0)
                    .push(input_row_1)
                    .push(input_row_2),
            );

        // 3. Combine
        VStack::<Message, crate::core::IcedBackend>::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(scroll)
            .push(input_area)
            .view(context)
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

// Stub for TermBackend
impl<Message: Clone + 'static> View<Message, crate::core::TermBackend> for AIChatView<Message> {
    fn view(&self, _context: &Context) -> String {
        "AI Chat (Terminal Not Supported)".to_string()
    }
    fn describe(&self, _context: &Context) -> SemanticNode {
        Default::default()
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

impl<Message: Clone + 'static> View<Message, crate::core::IcedBackend> for ChatBubble<Message> {
    fn view(&self, context: &Context) -> iced::Element<'static, Message, Theme, Renderer> {
        let t = context.theme;
        let is_user = self.message.role == ChatRole::User;

        if is_user {
            let bubble = iced::widget::container(
                Text::<crate::core::IcedBackend>::new(self.message.content.clone())
                    .caption1()
                    .color(t.colors.on_primary)
                    .view(context),
            )
            .padding(12.0)
            .style(move |_| iced::widget::container::Style {
                background: Some(t.colors.primary.into()),
                border: iced::Border {
                    radius: 12.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            });

            iced::widget::row![iced::widget::Space::new().width(Length::Fill), bubble]
                .padding(Padding {
                    top: 0.0,
                    right: 12.0,
                    bottom: 0.0,
                    left: 0.0,
                })
                .width(Length::Fill)
                .into()
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

            let mut assistant_content =
                VStack::<Message, crate::core::IcedBackend>::new().spacing(12.0);
            let on_act = self.on_action.clone();

            if actions.is_empty() {
                let on_act_inner = on_act.clone();
                let content = MarkdownView::new(raw_content)
                    .size(12.0)
                    .padding(Padding::ZERO)
                    .on_copy(move |code| (on_act_inner)(ChatViewMessage::CopyCode(code)));
                assistant_content = assistant_content.push(content);
            } else {
                for text in text_parts {
                    if !text.trim().is_empty() {
                        let on_act_inner = on_act.clone();
                        assistant_content = assistant_content.push(
                            MarkdownView::new(text)
                                .size(12.0)
                                .padding(Padding::ZERO)
                                .on_copy(move |code| {
                                    (on_act_inner)(ChatViewMessage::CopyCode(code))
                                }),
                        );
                    }
                }

                for action in actions {
                    assistant_content = assistant_content.push(ToolCard::new(action));
                }
            }

            iced::widget::container(assistant_content.view(context))
                .padding(Padding {
                    top: 12.0,
                    right: 12.0,
                    bottom: 12.0,
                    left: 12.0,
                })
                .width(Length::Fill)
                .into()
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

impl<Message: Clone + 'static> View<Message, crate::core::IcedBackend> for ToolCard {
    fn view(&self, context: &Context) -> iced::Element<'static, Message, Theme, Renderer> {
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

        let card_content = HStack::<Message, crate::core::IcedBackend>::new()
            .spacing(12.0)
            .align_y(iced::Alignment::Center)
            .push(
                Icon::<crate::core::IcedBackend>::new(icon_name)
                    .size(16.0)
                    .color(t.colors.primary),
            )
            .push(
                VStack::<Message, crate::core::IcedBackend>::new()
                    .spacing(2.0)
                    .push(
                        Text::<crate::core::IcedBackend>::new(display_name)
                            .caption1()
                            .bold(),
                    )
                    .push(
                        Text::<crate::core::IcedBackend>::new(params)
                            .caption2()
                            .dim(),
                    ),
            );

        iced::widget::container(card_content.view(context))
            .padding(8.0)
            .width(Length::Shrink)
            .style(move |t: &Theme| {
                let palette = t.extended_palette();
                iced::widget::container::Style {
                    background: Some(palette.background.weak.color.scale_alpha(0.05).into()),
                    border: iced::Border {
                        radius: 8.0.into(),
                        width: 1.0,
                        color: palette.background.strong.color.scale_alpha(0.1),
                    },
                    ..Default::default()
                }
            })
            .into()
    }

    fn describe(&self, _context: &Context) -> SemanticNode {
        SemanticNode {
            role: "tool_card".to_string(),
            label: Some(self.action_tag.clone()),
            ..Default::default()
        }
    }
}

impl<Message: Clone + 'static> View<Message, crate::core::TermBackend> for ChatBubble<Message> {
    fn view(&self, _context: &Context) -> String {
        format!(
            "{}: {}",
            if self.message.role == ChatRole::User {
                "User"
            } else {
                "AI"
            },
            self.message.content
        )
    }
    fn describe(&self, _context: &Context) -> SemanticNode {
        Default::default()
    }
}

// ThinkingBubble View
struct ThinkingBubble {}

impl<Message: Clone + 'static> View<Message, crate::core::IcedBackend> for ThinkingBubble {
    fn view(
        &self,
        context: &Context,
    ) -> <crate::core::IcedBackend as crate::core::Backend>::AnyView<Message> {
        let content = HStack::<Message, crate::core::IcedBackend>::new()
            .spacing(4.0)
            .align_y(iced::Alignment::Center)
            .push(Text::<crate::core::IcedBackend>::new("•").size(16.0))
            .push(Text::<crate::core::IcedBackend>::new("•").size(16.0))
            .push(Text::<crate::core::IcedBackend>::new("•").size(16.0))
            .view(context);

        crate::core::IcedBackend::container(
            content,
            Padding::new(12.0).right(20.0).left(20.0),
            Length::Fill,
            Length::Shrink,
            None,
            0.0,
            0.0,
            None,
            None,
            Alignment::Start,
            Alignment::Start,
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
