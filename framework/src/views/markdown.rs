use crate::prelude::*;
use iced::widget::{text, Column, Container, Row};
use iced::{font, Element, Length, Theme};

pub struct MarkdownView {
    content: String,
    size: f32,
    padding: iced::Padding,
}

impl MarkdownView {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            size: 16.0,
            padding: iced::Padding {
                top: 0.0,
                right: 0.0,
                bottom: 48.0,
                left: 0.0,
            },
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn padding(mut self, padding: impl Into<iced::Padding>) -> Self {
        self.padding = padding.into();
        self
    }
}

impl<Message> View<Message, IcedBackend> for MarkdownView
where
    Message: 'static + Clone,
{
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, iced::Renderer> {
        let mut children: Vec<Element<'static, Message, Theme, iced::Renderer>> = Vec::new();

        let mut in_code_block = false;
        let mut current_language: Option<String> = None;
        let mut code_buffer = String::new();

        let mut table_buffer: Vec<String> = Vec::new();

        for line in self.content.lines() {
            let trimmed = line.trim();

            // Table detection
            if trimmed.starts_with('|') {
                table_buffer.push(line.to_string());
                continue;
            } else if !table_buffer.is_empty() {
                // Table ended, flush it
                if table_buffer.len() >= 2 && is_separator_line(&table_buffer[1]) {
                    children.push(render_table(table_buffer.clone(), self.size, context));
                } else {
                    for l in &table_buffer {
                        children.push(
                            container(render_rich_text(l.trim(), self.size, context))
                                .width(Length::Fill)
                                .into(),
                        );
                    }
                }
                table_buffer.clear();
            }

            if trimmed.starts_with("```") {
                if in_code_block {
                    // Render code block
                    children.push(render_code_block(
                        &code_buffer,
                        current_language.as_deref(),
                        context,
                    ));
                    code_buffer.clear();
                    in_code_block = false;
                    current_language = None;
                } else {
                    in_code_block = true;
                    let lang = trimmed.trim_start_matches("```").trim();
                    if !lang.is_empty() {
                        current_language = Some(lang.to_string());
                    }
                }
                continue;
            }

            if in_code_block {
                code_buffer.push_str(line);
                code_buffer.push('\n');
                continue;
            }

            if trimmed.is_empty() {
                continue;
            }

            // Headers
            if trimmed.starts_with("# ") {
                children.push(
                    text(trimmed.trim_start_matches("# ").trim().to_string())
                        .size(self.size * 2.0)
                        .font(font::Font {
                            weight: font::Weight::Bold,
                            ..Default::default()
                        })
                        .width(Length::Fill)
                        .color(context.theme.colors.text_primary)
                        .into(),
                );
            } else if trimmed.starts_with("## ") {
                children.push(
                    text(trimmed.trim_start_matches("## ").trim().to_string())
                        .size(self.size * 1.5)
                        .font(font::Font {
                            weight: font::Weight::Bold,
                            ..Default::default()
                        })
                        .width(Length::Fill)
                        .color(context.theme.colors.text_primary)
                        .into(),
                );
            } else if trimmed.starts_with("### ") {
                children.push(
                    text(trimmed.trim_start_matches("### ").trim().to_string())
                        .size(self.size * 1.25)
                        .font(font::Font {
                            weight: font::Weight::Bold,
                            ..Default::default()
                        })
                        .width(Length::Fill)
                        .color(context.theme.colors.text_primary)
                        .into(),
                );
            }
            // Checkboxes
            else if trimmed.starts_with("- [ ] ") || trimmed.starts_with("- [x] ") {
                let is_checked = trimmed.starts_with("- [x] ");
                let content = if is_checked {
                    trimmed.trim_start_matches("- [x] ").trim().to_string()
                } else {
                    trimmed.trim_start_matches("- [ ] ").trim().to_string()
                };

                let icon_text = if is_checked {
                    text("[x]").size(16).color(context.theme.colors.success)
                } else {
                    text("[ ]")
                        .size(16)
                        .color(context.theme.colors.text_secondary)
                };

                children.push(
                    Row::new()
                        .spacing(12)
                        .width(Length::Fill)
                        .align_y(iced::Alignment::Start)
                        .push(icon_text)
                        .push(render_rich_text(&content, self.size, context))
                        .into(),
                );
            }
            // Bullet points
            else if trimmed.starts_with("- ") {
                let content = trimmed.trim_start_matches("- ").trim().to_string();
                children.push(
                    Row::new()
                        .spacing(8)
                        .width(Length::Fill)
                        .align_y(iced::Alignment::Start)
                        .push(
                            text("•")
                                .size(self.size * 0.875)
                                .color(context.theme.colors.text_secondary),
                        )
                        .push(render_rich_text(&content, self.size, context))
                        .into(),
                );
            }
            // Numbered list (Simple detection)
            else if let Some(rest) = parse_numbered_list(trimmed) {
                let content = rest.to_string();
                children.push(
                    Row::new()
                        .spacing(8)
                        .width(Length::Fill)
                        .align_y(iced::Alignment::Start)
                        .push(
                            text("1.")
                                .size(self.size * 0.875)
                                .color(context.theme.colors.text_secondary)
                                .font(font::Font::MONOSPACE),
                        )
                        .push(render_rich_text(&content, self.size, context))
                        .into(),
                );
            }
            // Paragraph
            else {
                children.push(
                    container(render_rich_text(trimmed, self.size, context))
                        .width(Length::Fill)
                        .into(),
                );
            }
        }

        // Final flushes
        if !table_buffer.is_empty() {
            if table_buffer.len() >= 2 && is_separator_line(&table_buffer[1]) {
                children.push(render_table(table_buffer, self.size, context));
            } else {
                for l in table_buffer {
                    children.push(
                        container(render_rich_text(l.trim(), self.size, context))
                            .width(Length::Fill)
                            .into(),
                    );
                }
            }
        }

        if !code_buffer.is_empty() {
            // Clone buffer for static ownership
            children.push(render_code_block(
                &code_buffer,
                current_language.as_deref(),
                context,
            ));
        }

        // Use standard Column from iced
        Column::with_children(children)
            .spacing(16)
            .width(Length::Fill)
            .padding(self.padding)
            .into()
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode {
            role: "article".to_string(),
            label: Some("Markdown Content".to_string()),
            content: Some(self.content.chars().take(100).collect()),
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
        }
    }
}

fn parse_numbered_list(line: &str) -> Option<&str> {
    let chars: Vec<char> = line.chars().take(10).collect();
    if let Some(dot_idx) = chars.iter().position(|&c| c == '.') {
        if dot_idx > 0 && dot_idx < chars.len() - 1 && chars[dot_idx + 1] == ' ' {
            if chars[0..dot_idx].iter().all(|c| c.is_numeric()) {
                return Some(line[dot_idx + 2..].trim());
            }
        }
    }
    None
}

fn render_rich_text<'a, Message>(
    content: &str,
    size: f32,
    context: &Context,
) -> Element<'a, Message, Theme, iced::Renderer>
where
    Message: 'static + Clone,
{
    let mut spans = Vec::new();
    let theme = context.theme;

    // Split strictly by ** for bold and ` for code
    let mut remaining = content;

    while !remaining.is_empty() {
        if let Some(start) = remaining.find("**") {
            // Text before **
            if start > 0 {
                spans.push(
                    text::Span::new(remaining[..start].to_string())
                        .color(theme.colors.text_secondary),
                );
            }
            if let Some(end) = remaining[start + 2..].find("**") {
                let bold_text = &remaining[start + 2..start + 2 + end];
                spans.push(
                    text::Span::new(bold_text.to_string())
                        .font(font::Font {
                            weight: font::Weight::Bold,
                            ..Default::default()
                        })
                        .color(theme.colors.text_primary), // Bold gets primary color
                );
                remaining = &remaining[start + 2 + end + 2..];
            } else {
                spans.push(
                    text::Span::new(remaining[..].to_string()).color(theme.colors.text_secondary),
                );
                break;
            }
        } else if let Some(start) = remaining.find('`') {
            if start > 0 {
                spans.push(
                    text::Span::new(remaining[..start].to_string())
                        .color(theme.colors.text_secondary),
                );
            }
            if let Some(end) = remaining[start + 1..].find('`') {
                let code_text = &remaining[start + 1..start + 1 + end];
                spans.push(
                    text::Span::new(format!(" {} ", code_text))
                        .font(font::Font::MONOSPACE)
                        .color(theme.colors.primary), // Code gets accent color
                );
                remaining = &remaining[start + 1 + end + 1..];
            } else {
                spans.push(
                    text::Span::new(remaining[..].to_string()).color(theme.colors.text_secondary),
                );
                break;
            }
        } else {
            spans.push(text::Span::new(remaining.to_string()).color(theme.colors.text_secondary));
            break;
        }
    }

    iced::widget::rich_text(spans)
        .size(size)
        .width(Length::Fill)
        .into()
}

fn render_code_block<Message>(
    content: &str,
    language: Option<&str>,
    context: &Context,
) -> Element<'static, Message, Theme, iced::Renderer>
where
    Message: 'static + Clone,
{
    let theme = context.theme;

    // Explicit fixed sizing, bypassing scaling factor
    let code_text = text(content.to_string())
        .font(iced::Font::MONOSPACE)
        .size(14.0)
        .color(theme.colors.text_primary)
        .width(Length::Shrink);

    let scrollable_code = iced::widget::scrollable(code_text)
        .direction(iced::widget::scrollable::Direction::Horizontal(
            iced::widget::scrollable::Scrollbar::new()
                .width(4)
                .scroller_width(4)
                .margin(2),
        ))
        .width(Length::Fill)
        .id(iced::widget::scrollable::Id::new("code_block"));

    let inner: Element<_, _, _> = if let Some(lang) = language {
        Column::new()
            .spacing(8)
            .width(Length::Fill)
            .push(
                Container::new(
                    text(lang.to_uppercase())
                        .size(12)
                        .font(font::Font {
                            weight: font::Weight::Bold,
                            ..Default::default()
                        })
                        .color(theme.colors.text_secondary),
                )
                .padding([4, 8])
                .style(move |_| container::Style {
                    background: Some(theme.colors.surface_variant.scale_alpha(0.3).into()),
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            )
            .push(scrollable_code)
            .into()
    } else {
        scrollable_code.into()
    };

    Container::new(inner)
        .padding(16)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(theme.colors.surface_variant.scale_alpha(0.15).into()),
            border: iced::Border {
                radius: 12.0.into(),
                width: 1.0,                                  // Keeping 1.0 width
                color: theme.colors.border.scale_alpha(0.5), // Back to normal border color
            },
            ..Default::default()
        })
        .into()
}

fn is_separator_line(line: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') {
        return false;
    }
    // Check if it's a separator line (contains only |, -, :, whitespace)
    // and at least one dash
    let mut has_dash = false;
    for c in trimmed.chars() {
        match c {
            '|' | ':' | ' ' | '\t' | '\r' | '\n' => {}
            '-' => has_dash = true,
            _ => return false,
        }
    }
    has_dash
}

fn render_table<'a, Message>(
    lines: Vec<String>,
    size: f32,
    context: &Context,
) -> Element<'a, Message, Theme, iced::Renderer>
where
    Message: 'static + Clone,
{
    // 1. Parse rows
    let mut rows: Vec<Vec<String>> = lines
        .into_iter()
        .map(|line| {
            let line = line.trim();
            let line = if line.starts_with('|') {
                &line[1..]
            } else {
                line
            };
            let line = if line.ends_with('|') {
                &line[..line.len() - 1]
            } else {
                line
            };
            line.split('|').map(|s| s.trim().to_string()).collect()
        })
        .collect();

    if rows.len() < 2 {
        return Column::new().into();
    }

    let header = rows.remove(0);
    let separator = rows.remove(0);
    let data_rows = rows;

    // 2. Parse alignments
    let alignments: Vec<iced::Alignment> = separator
        .iter()
        .map(|s| {
            let s = s.trim();
            let starts = s.starts_with(':');
            let ends = s.ends_with(':');
            if starts && ends {
                iced::Alignment::Center
            } else if ends {
                iced::Alignment::End
            } else {
                iced::Alignment::Start
            }
        })
        .collect();

    // 3. Build the UI
    let mut table_col = Column::new().width(Length::Fill);
    let tokens = context.theme;

    // Header Row
    let mut header_row = Row::new().spacing(12).width(Length::Fill);
    for (i, cell) in header.iter().enumerate() {
        let align = alignments.get(i).cloned().unwrap_or(iced::Alignment::Start);
        header_row = header_row.push(
            container(
                text(cell.clone())
                    .font(font::Font {
                        weight: font::Weight::Bold,
                        ..Default::default()
                    })
                    .color(tokens.colors.text_primary),
            )
            .width(Length::FillPortion(1))
            .align_x(align),
        );
    }

    table_col =
        table_col.push(
            container(header_row)
                .padding([12, 16])
                .style(move |_| container::Style {
                    background: Some(tokens.colors.surface_variant.scale_alpha(0.2).into()),
                    ..Default::default()
                }),
        );

    // Data Rows
    for (r_idx, row_data) in data_rows.iter().enumerate() {
        let mut row = Row::new().spacing(12).width(Length::Fill);
        for (i, cell) in row_data.iter().enumerate() {
            let align = alignments.get(i).cloned().unwrap_or(iced::Alignment::Start);
            row = row.push(
                container(render_rich_text(cell, size, context))
                    .width(Length::FillPortion(1))
                    .align_x(align),
            );
        }

        let row_container = container(row).padding([8, 16]);

        // Optional zebra striping
        if r_idx % 2 == 1 {
            table_col = table_col.push(row_container.style(move |_| container::Style {
                background: Some(tokens.colors.surface_variant.scale_alpha(0.05).into()),
                ..Default::default()
            }));
        } else {
            table_col = table_col.push(row_container);
        }

        // Divider
        if r_idx < data_rows.len() - 1 {
            table_col = table_col.push(
                container(iced::widget::Space::new(Length::Fill, 1.0)).style(move |_| {
                    container::Style {
                        background: Some(tokens.colors.divider.scale_alpha(0.5).into()),
                        ..Default::default()
                    }
                }),
            );
        }
    }

    let border_color = tokens.colors.divider.scale_alpha(0.3);

    // Calculate a reasonable minimum width for the table based on column count
    // This ensures that even on narrow screens, the table remains readable by scrolling
    let min_table_width = (header.len() as f32 * 160.0).max(600.0);

    let table_container = container(table_col)
        .width(Length::Fixed(min_table_width)) // Force a minimum width to enable scrolling
        .style(move |_| container::Style {
            border: iced::Border {
                color: border_color,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        });

    iced::widget::scrollable(table_container)
        .direction(iced::widget::scrollable::Direction::Horizontal(
            iced::widget::scrollable::Scrollbar::new()
                .width(4)
                .scroller_width(4)
                .margin(2),
        ))
        .width(Length::Fill)
        .into()
}
