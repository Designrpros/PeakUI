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
    on_action: Arc<dyn Fn(ChatViewMessage) -> Message + Send + Sync>,
}

impl<Message: Clone + 'static> AIChatView<Message> {
    pub fn new(
        messages: Vec<ChatMessage>,
        input_value: String,
        on_action: impl Fn(ChatViewMessage) -> Message + Send + Sync + 'static,
    ) -> Self {
        Self {
            messages,
            input_value,
            on_action: Arc::new(on_action),
        }
    }
}

// Implement View for AIChatView (IcedBackend)
impl<Message: Clone + 'static> View<Message, crate::core::IcedBackend> for AIChatView<Message> {
    fn view(
        &self,
        context: &Context,
    ) -> <crate::core::IcedBackend as crate::core::Backend>::AnyView<Message> {
        let on_action = &self.on_action;

        // 1. Create Bubble Views
        let bubbles: Vec<Box<dyn View<Message, crate::core::IcedBackend>>> = self
            .messages
            .iter()
            .map(|msg| {
                Box::new(ChatBubble {
                    message: msg.clone(),
                    on_action: self.on_action.clone(),
                }) as Box<dyn View<Message, crate::core::IcedBackend>>
            })
            .collect();

        // 2. Message List (ScrollView -> VStack)
        let message_stack = VStack::new()
            .spacing(16.0)
            .padding(iced::Padding {
                top: 20.0,
                ..Default::default()
            })
            .width(Length::Fill)
            .extend(bubbles);

        let scroll = ScrollView::new(message_stack).height(Length::Fill);

        // 3. Input Area
        let input_area = crate::atoms::Container::new(
            HStack::new()
                .spacing(8.0)
                .align_y(iced::Alignment::Center)
                .push(
                    TextInput::new(&self.input_value, "Ask anything...", {
                        let act = self.on_action.clone();
                        move |s| (act)(ChatViewMessage::InputChanged(s))
                    })
                    .variant(Variant::Ghost)
                    .width(Length::Fill)
                    .on_submit((on_action)(ChatViewMessage::SendPressed)),
                )
                .push(
                    Button::label("")
                        .icon("arrow-up")
                        .variant(Variant::Ghost)
                        .on_press((on_action)(ChatViewMessage::SendPressed)),
                ),
        )
        .padding(Padding::new(4.0));

        // 4. Combine
        let input_container_view = ProxyView::new(move |ctx| {
            let inner = input_area.view(ctx);

            iced::widget::container(inner)
                .padding(Padding::new(16.0))
                .style(move |t: &Theme| {
                    let palette = t.extended_palette();
                    iced::widget::container::Style {
                        border: iced::Border {
                            radius: 12.0.into(),
                            width: 1.0,
                            color: palette.background.strong.color.scale_alpha(0.15),
                        },
                        ..Default::default()
                    }
                })
                .into()
        });

        VStack::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(scroll)
            .push(Divider::new())
            .push(input_container_view)
            .view(context)
    }

    fn describe(&self, _context: &Context) -> SemanticNode {
        SemanticNode {
            role: "ai_chat".to_string(),
            label: Some("AI Assistant".to_string()),
            content: None,
            children: vec![], // In a real semantic tree, we'd describe children too
            neural_tag: None,
            documentation: None,
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

impl<Message: Clone + 'static> View<Message, crate::core::IcedBackend> for ChatBubble<Message> {
    fn view(
        &self,
        context: &Context,
    ) -> <crate::core::IcedBackend as crate::core::Backend>::AnyView<Message> {
        let t = context.theme;
        let is_user = self.message.role == ChatRole::User;

        if is_user {
            // User Bubble: Keep it clean but scoped
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

            iced::widget::row![iced::widget::horizontal_space(), bubble]
                .padding(Padding {
                    top: 0.0,
                    right: 20.0,
                    bottom: 0.0,
                    left: 0.0,
                })
                .width(Length::Fill)
                .into()
        } else {
            // AI Message: No bubble, Caption size, Secondary text, Markdown supported
            let on_act = self.on_action.clone();
            let content = MarkdownView::new(self.message.content.clone())
                .size(12.0)
                .padding(Padding::ZERO) // Minimal internal padding
                .on_copy(move |code| (on_act)(ChatViewMessage::CopyCode(code)))
                .view(context);

            iced::widget::container(content)
                .padding(Padding {
                    top: 12.0,
                    right: 20.0,
                    bottom: 12.0,
                    left: 20.0,
                })
                .width(Length::Fill)
                .into()
        }
    }

    fn describe(&self, _context: &Context) -> SemanticNode {
        SemanticNode {
            role: "chat_bubble".to_string(),
            content: Some(self.message.content.clone()),
            label: None,
            children: vec![],
            neural_tag: None,
            documentation: None,
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
