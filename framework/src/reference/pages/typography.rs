use super::super::app::{Message, RenderMode, TypographyLabState};
use super::super::page::PageResult;
use crate::core::{Backend, SpatialBackend};
use crate::prelude::*;

use super::super::views::ComponentDoc;
use std::sync::Arc;

pub fn view(
    base_context: &Context,
    lab: &TypographyLabState,
    render_mode: RenderMode,
) -> PageResult {
    let context = base_context;

    let code_snippet = generate_code(lab);

    // 1. Canvas View (Standard GUI)
    let canvas_preview = create_preview::<IcedBackend>(lab);

    // 2. Terminal View (ANSI Text)
    let terminal_preview = create_preview::<TermBackend>(lab).view(context);

    // 3. Neural View (Semantic JSON)
    let neural_preview = create_preview::<AIBackend>(lab).view(context);

    // 4. Spatial View (3D transforms)
    let spatial_preview = create_preview::<SpatialBackend>(lab).view(context);

    let doc = ComponentDoc::new(
        "Typography",
        "Use semantic styles and modifiers to ensure consistency and accessibility across different platforms.",
        code_snippet,
        Arc::new(canvas_preview),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
        r#"
### Semantic Typography
In **PeakUI**, typography is semantic. Instead of choosing raw font sizes, you choose a **Role** (e.g., Title1, Body, Caption) that adapts to the platform and device context.

- **Hierarchical:** Styles are designed to create a clear visual hierarchy.
- **Responsive:** Text roles can scale based on the `Context` size and device type.
- **Type-Safe:** Leveraging Rust's type system to ensure consistent style usage.
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
"#,
    );

    PageResult::new(doc).inspector(TypographyInspector::new(lab))
}

fn create_preview<B: Backend>(lab: &TypographyLabState) -> VStack<Message, B> {
    let mut text = Text::<B>::new(lab.text.clone()).size(lab.size);

    if lab.is_bold {
        text = text.bold();
    }

    if lab.is_italic {
        text = text.italic();
    }

    VStack::new_generic()
        .spacing(24.0)
        .width(Length::Shrink)
        .push(
            VStack::new_generic()
                .spacing(12.0)
                .width(Length::Shrink)
                .push(
                    Text::<B>::new("Sample Role: Title 1")
                        .caption2()
                        .secondary(),
                )
                .push(text.clone().title1()),
        )
        .push(
            VStack::new_generic()
                .spacing(12.0)
                .width(Length::Shrink)
                .push(Text::<B>::new("Sample Role: Body").caption2().secondary())
                .push(text.clone().body()),
        )
        .push(
            VStack::new_generic()
                .spacing(12.0)
                .width(Length::Shrink)
                .push(
                    Text::<B>::new("Sample Role: Caption 1")
                        .caption2()
                        .secondary(),
                )
                .push(text.clone().caption1()),
        )
}

fn generate_code(lab: &TypographyLabState) -> String {
    let mut code = format!("Text::new(\"{}\")", lab.text);

    if lab.size != 16.0 {
        code.push_str(&format!("\n    .size({:.1})", lab.size));
    }

    if lab.is_bold {
        code.push_str("\n    .bold()");
    }

    if lab.is_italic {
        code.push_str("\n    .italic()");
    }

    code.push_str("\n    .title1()");

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
                    top: 96.0,
                    right: 20.0,
                    bottom: 24.0,
                    left: 20.0,
                })
                .push(
                    VStack::new_generic()
                        .spacing(8.0)
                        .push(
                            Text::<IcedBackend>::new("Sample Text")
                                .caption2()
                                .bold()
                                .secondary(),
                        )
                        .push(TextInput::<Message>::new(
                            self.lab.text.clone(),
                            "Enter text...",
                            |s| Message::UpdateTypographyText(s),
                        )),
                )
                .push(
                    VStack::new_generic()
                        .spacing(8.0)
                        .push(
                            Text::<IcedBackend>::new("Base Size")
                                .caption2()
                                .bold()
                                .secondary(),
                        )
                        .push(
                            HStack::new_generic()
                                .spacing(12.0)
                                .push(
                                    Slider::<Message, IcedBackend>::new(
                                        12.0..=72.0,
                                        self.lab.size,
                                        |v| Message::UpdateTypographySize(v),
                                    )
                                    .width(Length::Fill),
                                )
                                .push(
                                    Text::<IcedBackend>::new(format!("{:.0}", self.lab.size))
                                        .caption2()
                                        .secondary(),
                                ),
                        ),
                )
                .push(Divider::new())
                .push(
                    VStack::new_generic()
                        .spacing(16.0)
                        .push(Toggle::new("Bold", self.lab.is_bold, |b| {
                            Message::ToggleTypographyBold(b)
                        }))
                        .push(Toggle::new("Italic", self.lab.is_italic, |b| {
                            Message::ToggleTypographyItalic(b)
                        })),
                ),
        )
        .view(context)
    }
}
