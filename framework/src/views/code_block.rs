use crate::core::{Backend, Context, TextSpan, View};
use crate::elements::atoms::Icon;
use crate::style::{Intent, Radius, ScrollDirection, Variant};
use iced::{Color, Length};

pub struct CodeBlock<Message: 'static + Send + Sync = ()> {
    code: String,
    language: String,
    height: Length,
    is_transparent: bool,
    on_copy: Option<Box<dyn Fn(String) -> Message + Send + Sync>>,
}

impl<Message: 'static + Send + Sync> CodeBlock<Message> {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            language: "rust".to_string(),
            height: Length::Shrink,
            is_transparent: false,
            on_copy: None,
        }
    }

    pub fn rust(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            language: "rust".to_string(),
            height: Length::Shrink,
            is_transparent: false,
            on_copy: None,
        }
    }

    pub fn on_copy<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> Message + Send + Sync + 'static,
    {
        self.on_copy = Some(Box::new(f));
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = language.into();
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn transparent(mut self) -> Self {
        self.is_transparent = true;
        self
    }
}

impl<Message, B: Backend> View<Message, B> for CodeBlock<Message>
where
    Message: Clone + Send + Sync + 'static,
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let bg_color = Color::from_rgb8(20, 20, 20); // Darker code area
        let header_bg = Color::from_rgb8(45, 45, 45); // Distinct header
        let border_color = Color::from_rgb8(64, 64, 64);

        // Feedback Logic
        let is_recently_copied = context
            .last_copied_code
            .as_deref()
            .map(|c| c == self.code)
            .unwrap_or(false);

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
            {
                let msg = if is_recently_copied {
                    None
                } else {
                    self.on_copy.as_ref().map(|f| f(self.code.clone()))
                };

                let btn_content = B::hstack(
                    vec![
                        Icon::<B>::new(if is_recently_copied { "check" } else { "copy" })
                            .size(10.0)
                            .color(if is_recently_copied {
                                Color::from_rgb8(100, 200, 100)
                            } else {
                                Color::from_rgb8(150, 150, 150)
                            })
                            .view(context),
                        B::text(
                            if is_recently_copied {
                                "Copied!".to_string()
                            } else {
                                "Copy".to_string()
                            },
                            10.0,
                            Some(if is_recently_copied {
                                Color::from_rgb8(100, 200, 100)
                            } else {
                                Color::from_rgb8(150, 150, 150)
                            }),
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
                    msg,
                    Variant::Compact,
                    if is_recently_copied {
                        Intent::Success
                    } else {
                        Intent::Secondary
                    },
                    Length::Shrink,
                    Length::Shrink,
                    true,
                    context,
                )
            },
        ];

        let header = B::hstack(
            header_items,
            12.0,
            iced::Padding::from([10, 16]),
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
            Some(if self.is_transparent {
                Color::TRANSPARENT
            } else {
                header_bg
            }),
            Radius::new(8.0, 8.0, 0.0, 0.0), // Round top only
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
            self.height,
            None,
            true,
            ScrollDirection::Both,
            context,
        );

        let code_container = B::container(
            scroll_area,
            iced::Padding::from(16),
            Length::Fill,
            Length::Shrink,
            Some(if self.is_transparent {
                Color::TRANSPARENT
            } else {
                bg_color
            }),
            Radius::new(0.0, 0.0, 8.0, 8.0), // Round bottom only
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
            self.height,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        );

        B::container(
            col,
            iced::Padding::ZERO,
            Length::Fill,
            self.height,
            Some(if self.is_transparent {
                Color::TRANSPARENT
            } else {
                bg_color
            }), // Background of lowest layer
            8.0,
            if self.is_transparent { 0.0 } else { 1.0 },
            Some(if self.is_transparent {
                Color::TRANSPARENT
            } else {
                border_color
            }),
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
    Message: 'static + Clone + Send + Sync,
{
    let trimmed_content = trim_common_indentation(content);
    let mut spans: Vec<TextSpan> = Vec::new();

    // VS Code Dark Theme Palette
    let c_keyword = Color::from_rgb8(197, 134, 192); // Purple
    let c_type = Color::from_rgb8(78, 201, 176); // Teal
    let c_string = Color::from_rgb8(206, 145, 120); // Orange
    let c_comment = Color::from_rgb8(106, 153, 85); // Green
    let c_fn = Color::from_rgb8(220, 220, 170); // Yellow
    let c_plain = Color::from_rgb8(212, 212, 212); // White/Grey

    // Very naive tokenization (split by whitespace but keep delimiters)
    let tokens =
        trimmed_content.split_inclusive(|c: char| c.is_whitespace() || "(){}[],.;:".contains(c));

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
            content: token.into(),
            color: Some(color),
            font: Some(iced::Font::MONOSPACE),
            size: None, // Default size
            is_bold: false,
            is_dim: false,
        });
    }

    B::rich_text(spans, 13.0, Length::Fill, iced::Alignment::Start, context)
}

fn trim_common_indentation(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();
    if lines.is_empty() {
        return s.to_string();
    }

    // Find first non-empty line to determine base indentation
    let first_line = lines.iter().find(|l| !l.trim().is_empty());
    if first_line.is_none() {
        return s.trim().to_string();
    }

    let indent = first_line
        .unwrap()
        .chars()
        .take_while(|c| c.is_whitespace())
        .count();

    lines
        .into_iter()
        .map(|l| {
            if l.len() >= indent && l[..indent].trim().is_empty() {
                &l[indent..]
            } else {
                l.trim_start()
            }
        })
        .collect::<Vec<&str>>()
        .join("\n")
        .trim()
        .to_string()
}
