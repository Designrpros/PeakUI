use crate::prelude::*;
use iced::widget::{column, container, row, scrollable, text};
use iced::{Background, Border, Color, Element, Length, Theme};

pub struct CodeBlock<Message = ()> {
    code: String,
    language: String,
    on_copy: Option<Box<dyn Fn(String) -> Message>>,
}

impl<Message> CodeBlock<Message> {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            language: "rust".to_string(),
            on_copy: None,
        }
    }

    pub fn rust(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            language: "rust".to_string(),
            on_copy: None,
        }
    }

    pub fn on_copy<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> Message + 'static,
    {
        self.on_copy = Some(Box::new(f));
        self
    }
}

impl<Message> View<Message, IcedBackend> for CodeBlock<Message>
where
    Message: Clone + 'static,
{
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, iced::Renderer> {
        let theme_mode = context.theme; // We use this for outer borders, but code is always dark

        // VS Code Colors
        let bg_color = Color::from_rgb8(30, 30, 30); // #1E1E1E
        let header_bg = Color::from_rgb8(37, 37, 38); // #252526

        // 1. Window Chrome (Header)
        let header = container(
            row![
                // Language Label
                text(self.language.to_uppercase())
                    .size(10)
                    .font(iced::Font::MONOSPACE)
                    .color(Color::from_rgb8(150, 150, 150)),
                iced::widget::Space::with_width(Length::Fill),
                // Copy Button
                if let Some(on_copy) = &self.on_copy {
                    let msg = on_copy(self.code.clone());
                    let btn: Element<'static, Message, Theme, iced::Renderer> =
                        iced::widget::button(
                            iced::widget::container(
                                text("Copy")
                                    .size(10)
                                    .font(iced::Font::MONOSPACE)
                                    .color(Color::from_rgb8(150, 150, 150)),
                            )
                            .padding([4, 8])
                            .style(move |_| container::Style {
                                background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.05).into()),
                                border: Border {
                                    radius: 4.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                        )
                        .on_press(msg)
                        .style(iced::widget::button::secondary)
                        .into();
                    btn
                } else {
                    iced::widget::Space::with_width(0).into()
                },
            ]
            .width(Length::Fill)
            .align_y(iced::Alignment::Center)
            .padding([8, 12])
            .spacing(12),
        )
        .style(move |_| container::Style {
            background: Some(Background::Color(header_bg)),
            ..Default::default()
        });

        // 2. Code Area with Syntax Highlighting
        let code_area = container(scrollable(highlight_rust::<Message>(&self.code)).direction(
            scrollable::Direction::Both {
                vertical: scrollable::Scrollbar::new().width(6).scroller_width(6),
                horizontal: scrollable::Scrollbar::new().width(6).scroller_width(6),
            },
        ))
        .padding(16)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(bg_color)),
            ..Default::default()
        });

        // 3. Assemble
        container(column![header, code_area])
            .clip(true)
            .width(Length::Fill)
            .style(move |_| container::Style {
                border: Border {
                    color: theme_mode.colors.border.scale_alpha(0.5),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                background: Some(Background::Color(bg_color)),
                ..Default::default()
            })
            .into()
    }
}

// Helper: Heuristic Syntax Highlighter
fn highlight_rust<'a, Message>(content: &str) -> Element<'a, Message, Theme, iced::Renderer>
where
    Message: 'static + Clone,
{
    let mut spans = Vec::new();

    // VS Code Dark Theme Palette
    let c_keyword = Color::from_rgb8(197, 134, 192); // Purple
    let c_type = Color::from_rgb8(78, 201, 176); // Teal
    let c_string = Color::from_rgb8(206, 145, 120); // Orange
    let c_comment = Color::from_rgb8(106, 153, 85); // Green
    let c_fn = Color::from_rgb8(220, 220, 170); // Yellow
    let c_plain = Color::from_rgb8(212, 212, 212); // White/Grey

    // Very naive tokenization (split by whitespace but keep delimiters)
    let tokens = content.split_inclusive(|c: char| c.is_whitespace() || "(){}[],.;:".contains(c));

    for token in tokens {
        let trimmed = token.trim();
        let color = if trimmed.starts_with("//") {
            c_comment
        } else if trimmed.starts_with('"') {
            c_string
        } else if [
            "fn", "pub", "struct", "impl", "use", "mod", "let", "mut", "return", "match", "if",
            "else",
        ]
        .contains(&trimmed)
        {
            c_keyword
        } else if [
            "String", "Option", "Vec", "u32", "bool", "Self", "Context", "View",
        ]
        .contains(&trimmed)
            || (trimmed.chars().next().map_or(false, |c| c.is_uppercase()))
        {
            c_type
        } else if trimmed.ends_with('(') {
            c_fn
        } else {
            c_plain
        };

        spans.push(
            iced::widget::text::Span::new(token.to_string())
                .color(color)
                .font(iced::Font::MONOSPACE),
        );
    }

    iced::widget::rich_text(spans)
        .size(13)
        .line_height(1.6)
        .into()
}
