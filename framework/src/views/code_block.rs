use crate::atoms::Icon;
use crate::core::{Backend, Context, ScrollDirection, TextSpan, View};
use crate::modifiers::Variant;
use iced::{Color, Length};

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

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = language.into();
        self
    }
}

impl<Message, B: Backend> View<Message, B> for CodeBlock<Message>
where
    Message: Clone + 'static,
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let theme_mode = context.theme;

        // VS Code Colors
        let bg_color = Color::from_rgb8(30, 30, 30); // #1E1E1E
        let header_bg = Color::from_rgb8(37, 37, 38); // #252526
        let border_color = theme_mode.colors.border.scale_alpha(0.5);

        // 1. Window Chrome (Header)
        let header_items = vec![
            B::text(
                self.language.to_uppercase(),
                10.0,
                Some(Color::from_rgb8(150, 150, 150)),
                false,
                false,
                None,
                Some(iced::Font::MONOSPACE),
                Length::Shrink,
                iced::Alignment::Start,
                context,
            ),
            B::space(Length::Fill, Length::Shrink, context),
            if let Some(on_copy) = &self.on_copy {
                let msg = on_copy(self.code.clone());

                let btn_content = B::hstack(
                    vec![
                        Icon::<B>::new("copy")
                            .size(10.0)
                            .color(Color::from_rgb8(150, 150, 150))
                            .view(context),
                        B::text(
                            "Copy".to_string(),
                            10.0,
                            Some(Color::from_rgb8(150, 150, 150)),
                            false,
                            false,
                            None,
                            Some(iced::Font::MONOSPACE),
                            Length::Shrink,
                            iced::Alignment::Start,
                            context,
                        ),
                    ],
                    6.0,
                    iced::Padding::from([2, 4]),
                    Length::Shrink,
                    Length::Shrink,
                    iced::Alignment::Start,
                    iced::Alignment::Center,
                    context,
                );

                B::button(
                    btn_content,
                    Some(msg),
                    Variant::Ghost,
                    crate::modifiers::Intent::Neutral,
                    Length::Shrink,
                    true,
                    context,
                )
            } else {
                B::space(Length::Fixed(0.0), Length::Shrink, context)
            },
        ];

        let header = B::hstack(
            header_items,
            12.0,
            iced::Padding::from([8, 12]),
            Length::Fill,
            Length::Shrink,
            iced::Alignment::Start,
            iced::Alignment::Center,
            context,
        );

        let header_container = B::container(
            header,
            iced::Padding::ZERO,
            Length::Fill,
            Length::Shrink,
            Some(header_bg),
            0.0, // Top radius handled by outer container? Or 0 here.
            0.0,
            None,
            None,
            iced::Alignment::Start,
            iced::Alignment::Center,
            context,
        );

        // 2. Code Area with Syntax Highlighting
        let raw_code_view = highlight_rust::<Message, B>(&self.code, context);
        let scroll_area = B::scroll_view(
            raw_code_view,
            Length::Fill,
            Length::Shrink,
            None,
            true,
            ScrollDirection::Horizontal,
            context,
        );
        // Note: scroll_view usually enables scrolling if content overflows.

        let code_container = B::container(
            scroll_area,
            iced::Padding::from(16),
            Length::Fill,
            Length::Shrink,
            Some(bg_color),
            0.0,
            0.0,
            None,
            None,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        );

        // 3. Assemble
        let col = B::vstack(
            vec![header_container, code_container],
            0.0,
            iced::Padding::ZERO,
            Length::Fill,
            Length::Shrink,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        );

        B::container(
            col,
            iced::Padding::ZERO,
            Length::Fill,
            Length::Shrink,
            Some(bg_color), // Background of the whole block
            8.0,
            1.0,
            Some(border_color),
            None,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        )
    }
}

// Helper: Heuristic Syntax Highlighter
fn highlight_rust<Message, B: Backend>(content: &str, context: &Context) -> B::AnyView<Message>
where
    Message: 'static + Clone,
{
    let mut spans: Vec<TextSpan> = Vec::new();

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

        spans.push(TextSpan {
            content: token.to_string(),
            color: Some(color),
            font: Some(iced::Font::MONOSPACE),
            size: None, // Default size
        });
    }

    B::rich_text(spans, 13.0, Length::Fill, iced::Alignment::Start, context)
}
