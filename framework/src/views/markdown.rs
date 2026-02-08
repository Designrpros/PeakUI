use crate::core::{Backend, Context, ScrollDirection, TextSpan, View};
use crate::views::CodeBlock;
use iced::{font, Length};
use std::sync::Arc;

pub struct MarkdownView<Message: 'static + Send + Sync> {
    content: String,
    size: f32,
    padding: iced::Padding,
    on_copy: Option<Arc<dyn Fn(String) -> Message + Send + Sync>>,
}

impl<Message: 'static + Send + Sync> MarkdownView<Message> {
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
            on_copy: None,
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

    pub fn on_copy<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> Message + Send + Sync + 'static,
    {
        self.on_copy = Some(Arc::new(f));
        self
    }
}

impl<Message, B: Backend> View<Message, B> for MarkdownView<Message>
where
    Message: 'static + Clone + Send + Sync,
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let mut children: Vec<B::AnyView<Message>> = Vec::new();

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
                    children.push(render_table::<Message, B>(
                        table_buffer.clone(),
                        self.size,
                        context,
                    ));
                } else {
                    for l in &table_buffer {
                        children.push(B::container(
                            render_rich_text::<Message, B>(l.trim(), self.size, context),
                            iced::Padding::ZERO,
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
                        ));
                    }
                }
                table_buffer.clear();
            }

            if trimmed.starts_with("```") {
                if in_code_block {
                    // Render code block
                    let mut block = CodeBlock::new(&code_buffer);
                    if let Some(lang) = &current_language {
                        block = block.language(lang);
                    }

                    if let Some(on_copy) = &self.on_copy {
                        let f = on_copy.clone();
                        block = block.on_copy(move |s| f(s));
                    }

                    children.push(View::<Message, B>::view(&block, context));

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
                children.push(B::text(
                    trimmed.trim_start_matches("# ").trim().to_string(),
                    self.size * 2.0,
                    Some(context.theme.colors.text_primary),
                    true,
                    false,
                    None,
                    None,
                    Length::Fill,
                    iced::Alignment::Start,
                    context,
                ));
            } else if trimmed.starts_with("## ") {
                children.push(B::text(
                    trimmed.trim_start_matches("## ").trim().to_string(),
                    self.size * 1.5,
                    Some(context.theme.colors.text_primary),
                    true,
                    false,
                    None,
                    None,
                    Length::Fill,
                    iced::Alignment::Start,
                    context,
                ));
            } else if trimmed.starts_with("### ") {
                children.push(B::text(
                    trimmed.trim_start_matches("### ").trim().to_string(),
                    self.size * 1.25,
                    Some(context.theme.colors.text_primary),
                    true,
                    false,
                    None,
                    None,
                    Length::Fill,
                    iced::Alignment::Start,
                    context,
                ));
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
                    B::text(
                        "[x]".to_string(),
                        16.0,
                        Some(context.theme.colors.success),
                        false,
                        false,
                        None,
                        None,
                        Length::Shrink,
                        iced::Alignment::Start,
                        context,
                    )
                } else {
                    B::text(
                        "[ ]".to_string(),
                        16.0,
                        Some(context.theme.colors.text_secondary),
                        false,
                        false,
                        None,
                        None,
                        Length::Shrink,
                        iced::Alignment::Start,
                        context,
                    )
                };

                children.push(B::hstack(
                    vec![
                        icon_text,
                        render_rich_text::<Message, B>(&content, self.size, context),
                    ],
                    12.0,
                    iced::Padding::ZERO,
                    Length::Fill,
                    Length::Shrink,
                    iced::Alignment::Start,
                    iced::Alignment::Start,
                    context,
                ));
            }
            // Bullet points
            else if trimmed.starts_with("- ") {
                let content = trimmed.trim_start_matches("- ").trim().to_string();
                children.push(B::hstack(
                    vec![
                        B::text(
                            "•".to_string(),
                            self.size * 0.875,
                            Some(context.theme.colors.text_secondary),
                            false,
                            false,
                            None,
                            None,
                            Length::Shrink,
                            iced::Alignment::Start,
                            context,
                        ),
                        render_rich_text::<Message, B>(&content, self.size, context),
                    ],
                    8.0,
                    iced::Padding::ZERO,
                    Length::Fill,
                    Length::Shrink,
                    iced::Alignment::Start,
                    iced::Alignment::Start,
                    context,
                ));
            }
            // Numbered list (Simple detection)
            else if let Some(rest) = parse_numbered_list(trimmed) {
                let content = rest.to_string();
                children.push(B::hstack(
                    vec![
                        B::text(
                            "1.".to_string(),
                            self.size * 0.875,
                            Some(context.theme.colors.text_secondary),
                            false,
                            false,
                            None,
                            Some(iced::Font::MONOSPACE),
                            Length::Shrink,
                            iced::Alignment::Start,
                            context,
                        ),
                        render_rich_text::<Message, B>(&content, self.size, context),
                    ],
                    8.0,
                    iced::Padding::ZERO,
                    Length::Fill,
                    Length::Shrink,
                    iced::Alignment::Start,
                    iced::Alignment::Start,
                    context,
                ));
            }
            // Paragraph
            else {
                children.push(B::container(
                    render_rich_text::<Message, B>(trimmed, self.size, context),
                    iced::Padding::ZERO,
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
                ));
            }
        }

        // Final flushes
        if !table_buffer.is_empty() {
            if table_buffer.len() >= 2 && is_separator_line(&table_buffer[1]) {
                children.push(render_table::<Message, B>(table_buffer, self.size, context));
            } else {
                for l in table_buffer {
                    children.push(B::container(
                        render_rich_text::<Message, B>(l.trim(), self.size, context),
                        iced::Padding::ZERO,
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
                    ));
                }
            }
        }

        if !code_buffer.is_empty() {
            let mut block = CodeBlock::new(&code_buffer);
            if let Some(on_copy) = &self.on_copy {
                let f = on_copy.clone();
                block = block.on_copy(move |s| f(s));
            }
            children.push(View::<Message, B>::view(&block, context));
        }

        B::vstack(
            children,
            16.0,
            self.padding,
            Length::Fill,
            Length::Shrink,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        )
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("article")
            .with_label("Markdown Content")
            .with_content(self.content.chars().take(100).collect::<String>())
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

fn render_rich_text<Message: Clone + Send + Sync + 'static, B: Backend>(
    content: &str,
    size: f32,
    context: &Context,
) -> B::AnyView<Message> {
    let mut spans: Vec<TextSpan> = Vec::new();
    let theme = context.theme;

    // Split strictly by ** for bold and ` for code
    let mut remaining = content;

    while !remaining.is_empty() {
        if let Some(start) = remaining.find("**") {
            // Text before **
            if start > 0 {
                spans.push(TextSpan {
                    content: remaining[..start].into(),
                    color: Some(theme.colors.text_secondary),
                    font: None,
                    size: None,
                    is_bold: false,
                    is_dim: false,
                });
            }
            if let Some(end) = remaining[start + 2..].find("**") {
                let bold_text = &remaining[start + 2..start + 2 + end];
                spans.push(TextSpan {
                    content: bold_text.into(),
                    color: Some(theme.colors.text_primary),
                    font: Some(font::Font {
                        weight: font::Weight::Bold,
                        ..Default::default()
                    }),
                    size: None,
                    is_bold: true,
                    is_dim: false,
                });
                remaining = &remaining[start + 2 + end + 2..];
            } else {
                spans.push(TextSpan {
                    content: remaining[..].into(),
                    color: Some(theme.colors.text_secondary),
                    font: None,
                    size: None,
                    is_bold: false,
                    is_dim: false,
                });
                break;
            }
        } else if let Some(start) = remaining.find('`') {
            if start > 0 {
                spans.push(TextSpan {
                    content: remaining[..start].into(),
                    color: Some(theme.colors.text_secondary),
                    font: None,
                    size: None,
                    is_bold: false,
                    is_dim: false,
                });
            }
            if let Some(end) = remaining[start + 1..].find('`') {
                let code_text = &remaining[start + 1..start + 1 + end];
                spans.push(TextSpan {
                    content: format!(" {} ", code_text).into(),
                    color: Some(theme.colors.primary),
                    font: Some(font::Font::MONOSPACE),
                    size: None,
                    is_bold: false,
                    is_dim: false,
                });
                remaining = &remaining[start + 1 + end + 1..];
            } else {
                spans.push(TextSpan {
                    content: remaining[..].into(),
                    color: Some(theme.colors.text_secondary),
                    font: None,
                    size: None,
                    is_bold: false,
                    is_dim: false,
                });
                break;
            }
        } else {
            spans.push(TextSpan {
                content: remaining.into(),
                color: Some(theme.colors.text_secondary),
                font: None,
                size: None,
                is_bold: false,
                is_dim: false,
            });
            break;
        }
    }

    B::rich_text(spans, size, Length::Fill, iced::Alignment::Start, context)
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

fn render_table<Message: Clone + Send + Sync + 'static, B: Backend>(
    lines: Vec<String>,
    size: f32,
    context: &Context,
) -> B::AnyView<Message> {
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
        return B::vstack(
            vec![],
            0.0,
            iced::Padding::ZERO,
            Length::Shrink,
            Length::Shrink,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        );
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
    let tokens = context.theme;

    // Header Row
    let mut header_cells = Vec::new();
    for (i, cell) in header.iter().enumerate() {
        let align = alignments.get(i).cloned().unwrap_or(iced::Alignment::Start);
        header_cells.push(B::container(
            B::text(
                cell.clone(),
                16.0,
                Some(tokens.colors.text_primary),
                true,
                false,
                None,
                None,
                Length::Fill,
                iced::Alignment::Start,
                context,
            ), // Text alignment? Use B::text params carefully.
            // B::text alignment param is relative to width. if width Fill, it works.
            iced::Padding::ZERO,
            Length::FillPortion(1),
            Length::Shrink,
            None,
            0.0,
            0.0,
            None,
            None,
            align,
            iced::Alignment::Center, // Vertical center
            context,
        ));
    }

    let header_row = B::hstack(
        header_cells,
        12.0,
        iced::Padding::ZERO,
        Length::Fill,
        Length::Shrink,
        iced::Alignment::Start,
        iced::Alignment::Start,
        context,
    );

    let mut table_children = Vec::new();
    table_children.push(B::container(
        header_row,
        iced::Padding::from([12, 16]),
        Length::Fill,
        Length::Shrink,
        Some(tokens.colors.surface_variant.scale_alpha(0.2)),
        0.0,
        0.0,
        None,
        None,
        iced::Alignment::Start,
        iced::Alignment::Start,
        context,
    ));

    // Data Rows
    for (r_idx, row_data) in data_rows.iter().enumerate() {
        let mut row_cells = Vec::new();
        for (i, cell) in row_data.iter().enumerate() {
            let align = alignments.get(i).cloned().unwrap_or(iced::Alignment::Start);
            row_cells.push(B::container(
                render_rich_text::<Message, B>(cell, size, context),
                iced::Padding::ZERO,
                Length::FillPortion(1),
                Length::Shrink,
                None,
                0.0,
                0.0,
                None,
                None,
                align,
                iced::Alignment::Start,
                context,
            ));
        }

        let row = B::hstack(
            row_cells,
            12.0,
            iced::Padding::ZERO,
            Length::Fill,
            Length::Shrink,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        );
        let row_bg = if r_idx % 2 == 1 {
            Some(tokens.colors.surface_variant.scale_alpha(0.05))
        } else {
            None
        };

        table_children.push(B::container(
            row,
            iced::Padding::from([8, 16]),
            Length::Fill,
            Length::Shrink,
            row_bg,
            0.0,
            0.0,
            None,
            None,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        ));

        // Divider
        if r_idx < data_rows.len() - 1 {
            table_children.push(B::container(
                B::space(Length::Fill, Length::Fixed(1.0), context),
                iced::Padding::ZERO,
                Length::Fill,
                Length::Shrink,
                Some(tokens.colors.divider.scale_alpha(0.5)),
                0.0,
                0.0,
                None,
                None,
                iced::Alignment::Start,
                iced::Alignment::Start,
                context,
            ));
        }
    }

    let min_table_width = (header.len() as f32 * 160.0).max(600.0);
    let table_col = B::vstack(
        table_children,
        0.0,
        iced::Padding::ZERO,
        Length::Fill,
        Length::Shrink,
        iced::Alignment::Start,
        iced::Alignment::Start,
        context,
    );

    let table_container = B::container(
        table_col,
        iced::Padding::ZERO,
        Length::Fixed(min_table_width),
        Length::Shrink,
        None,
        8.0,
        1.0,
        Some(tokens.colors.divider.scale_alpha(0.3)),
        None,
        iced::Alignment::Start,
        iced::Alignment::Start,
        context,
    );

    B::scroll_view(
        table_container,
        Length::Fill,
        Length::Shrink,
        None,
        true,
        ScrollDirection::Horizontal,
        context,
    )
}
