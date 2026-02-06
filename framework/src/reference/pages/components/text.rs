use crate::core::{AIBackend, Backend, IcedBackend, SpatialBackend, TermBackend};
use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::{Message, RenderMode, TypographyLabState};
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(
    ctx: &Context,
    lab: &TypographyLabState,
    render_mode: RenderMode,
) -> PageResult<Message> {
    // --- 1. Preview Construction ---
    // We create a preview that can be rendered by any backend
    let preview_view = create_preview::<IcedBackend>(lab);
    let terminal_preview = create_preview::<TermBackend>(lab).view(ctx);
    let neural_preview = create_preview::<AIBackend>(lab).view(ctx);
    let spatial_preview = create_preview::<SpatialBackend>(lab).view(ctx);

    // --- 2. Code Snippet ---
    let code_snippet = generate_code(lab);

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "Text",
        "A fundamental typography component for rendering text with various styles, sizes, and intents.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
        r#"
### Semantic Typography
In **PeakUI**, typography is semantic. Instead of choosing raw font sizes for every element, you use **Roles** (like `.large_title()` or `.body()`) that adapt to the platform and device context.

- **Scale Agnostic:** The internal size of a 'Body' component might change between a Mobile and Desktop context, but your code remains the same.
- **Backend Portability:** When rendered in a Terminal, 'Title1' might be mapped to BOLD/UNDERLINE, while on Canvas it uses a high-density font weight.
- **Accessibility Integration:** Semantic roles provide essential metadata to screen readers and AI agents.
"#,
    )
    .props_table(
        r#"
| Modifier | Description |
| :--- | :--- |
| `.large_title()` | Largest header, used for main page titles. |
| `.title1()`, `.title2()`, `.title3()` | Standard hierarchy headers. |
| `.headline()` | Used for emphasized section headers. |
| `.body()` | Standard body text. |
| `.caption1()`, `.caption2()` | Smaller metadata or label text. |
| `.bold()` | Highly emphasized text weight. |
| `.italic()` | Emphasized text style. |
| `.primary()`, `.secondary()` | Standard color intents. |
| `.intent(Intent::Danger)` | Semantic coloring for alerts or errors. |
"#,
    );

    PageResult::new(doc).inspector(TypographyInspector::new(lab))
}

fn create_preview<B: Backend>(lab: &TypographyLabState) -> VStack<Message, B> {
    let mut text_item = Text::<B>::new(lab.text.clone());

    if lab.is_bold {
        text_item = text_item.bold();
    }
    if lab.is_italic {
        text_item = text_item.italic();
    }

    // Applying the manual size if it's not the default
    if (lab.size - 16.0).abs() > 0.1 {
        text_item = text_item.size(lab.size);
    }

    VStack::new_generic()
        .spacing(24.0)
        .push(
            vstack![
                Text::<B>::new("Sample Render").caption2().secondary(),
                text_item
            ]
            .spacing(8.0),
        )
        .push(Divider::new())
        .push(
            vstack![
                Text::<B>::new("Available Roles").caption2().secondary(),
                vstack![
                    Text::<B>::new("Large Title").large_title(),
                    Text::<B>::new("Title 1").title1(),
                    Text::<B>::new("Headline").headline(),
                    Text::<B>::new("Body text with default styling.").body(),
                    Text::<B>::new("Caption text is smaller and dimmer.").caption1(),
                    hstack![
                        Text::<B>::new("Primary").body().primary(),
                        Text::<B>::new("Secondary").body().secondary(),
                        Text::<B>::new("Danger")
                            .body()
                            .intent(crate::modifiers::Intent::Danger)
                    ]
                    .spacing(16.0)
                ]
                .spacing(12.0)
            ]
            .spacing(16.0),
        )
}

fn generate_code(lab: &TypographyLabState) -> String {
    let mut code = format!("Text::new(\"{}\")", lab.text);

    if (lab.size - 16.0).abs() > 0.1 {
        code.push_str(&format!("\n    .size({:.1})", lab.size));
    }
    if lab.is_bold {
        code.push_str("\n    .bold()");
    }
    if lab.is_italic {
        code.push_str("\n    .italic()");
    }

    code.push_str("\n    .body()");
    code
}

struct TypographyInspector {
    lab: TypographyLabState,
}

impl TypographyInspector {
    fn new(lab: &TypographyLabState) -> Self {
        Self { lab: lab.clone() }
    }
}

impl View<Message, IcedBackend> for TypographyInspector {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        ScrollView::new(
            VStack::new_generic()
                .spacing(24.0)
                .padding(Padding {
                    top: context.safe_area.top,
                    right: 20.0,
                    bottom: context.safe_area.bottom,
                    left: 20.0,
                })
                .push(
                    vstack![
                        Text::<IcedBackend>::new("Sample Text")
                            .caption2()
                            .bold()
                            .secondary(),
                        crate::controls::TextInput::<Message>::new(
                            self.lab.text.clone(),
                            "Enter text...",
                            |s| Message::UpdateTypographyText(s),
                        ),
                    ]
                    .spacing(8.0),
                )
                .push(
                    vstack![
                        Text::<IcedBackend>::new("Base Size")
                            .caption2()
                            .bold()
                            .secondary(),
                        hstack![
                            crate::controls::Slider::<Message, IcedBackend>::new(
                                12.0..=72.0,
                                self.lab.size,
                                |v| Message::UpdateTypographySize(v),
                            )
                            .width(Length::Fill),
                            Text::<IcedBackend>::new(format!("{:.0}", self.lab.size))
                                .caption2()
                                .secondary(),
                        ]
                        .spacing(12.0),
                    ]
                    .spacing(8.0),
                )
                .push(Divider::new())
                .push(
                    vstack![
                        crate::controls::Toggle::new("Bold", self.lab.is_bold, |b| {
                            Message::ToggleTypographyBold(b)
                        }),
                        crate::controls::Toggle::new("Italic", self.lab.is_italic, |b| {
                            Message::ToggleTypographyItalic(b)
                        }),
                    ]
                    .spacing(16.0),
                ),
        )
        .view(context)
    }
}
