use crate::prelude::*;
use crate::reference::app::{Message, RenderMode, TypographyLabState};
use crate::reference::views::SimulatorView;
use crate::reference::AppPageResult;
use crate::views::CodeBlock;

pub fn view(
    _base_context: &Context,
    lab: &TypographyLabState,
    render_mode: RenderMode,
) -> AppPageResult {
    let lab_state = lab.clone();
    AppPageResult::new(ProxyView::<Message, IcedBackend>::new(move |ctx| {
        let lab = &lab_state;
        let t = ctx.theme;
        let is_narrow = ctx.size.width < 1000.0;

        // --- 1. Hero Section ---
        let hero = VStack::<Message, IcedBackend>::new_generic()
            .spacing(12.0)
            .align_x(iced::Alignment::Start)
            .width(Length::Fill)
            .push(
                Text::<IcedBackend>::new("Typography")
                    .size(if is_narrow { 42.0 } else { 56.0 })
                    .bold()
                    .align_start()
                    .width(Length::Fill)
                    .color(t.colors.text_primary),
            )
            .push(
                Text::<IcedBackend>::new("Use semantic styles and modifiers to ensure consistency and accessibility across different platforms.")
                    .size(20.0)
                    .align_start()
                    .width(Length::Fill)
                    .color(t.colors.text_secondary),
            );

        // --- 2. The Lab (Standardized) ---
        let playground = {
            let scrollable_tabs = move |ctx: &Context| {
                HStack::<Message, IcedBackend>::new_generic()
                    .spacing(12.0)
                    .width(Length::Shrink)
                    .push(
                        Button::<Message, IcedBackend>::label("Canvas")
                            .variant(if render_mode == RenderMode::Canvas { Variant::Solid } else { Variant::Ghost })
                            .on_press(Message::SetRenderMode(RenderMode::Canvas)),
                    )
                    .push(
                        Button::<Message, IcedBackend>::label("Terminal")
                            .variant(if render_mode == RenderMode::Terminal { Variant::Solid } else { Variant::Ghost })
                            .on_press(Message::SetRenderMode(RenderMode::Terminal)),
                    )
                    .push(
                        Button::<Message, IcedBackend>::label("Neural")
                            .variant(if render_mode == RenderMode::Neural { Variant::Solid } else { Variant::Ghost })
                            .on_press(Message::SetRenderMode(RenderMode::Neural)),
                    )
                    .push(
                        Button::<Message, IcedBackend>::label("Spatial")
                            .variant(if render_mode == RenderMode::Spatial { Variant::Solid } else { Variant::Ghost })
                            .on_press(Message::SetRenderMode(RenderMode::Spatial)),
                    )
                    .view(ctx)
            };

            let preview_content = match render_mode {
                RenderMode::Canvas => {
                    let preview = create_preview::<IcedBackend>(lab);
                    crate::layout::containers::Card::new(
                        ScrollView::new(
                            VStack::new_generic()
                                .padding(48)
                                .width(Length::Fill)
                                .align_x(Alignment::Center)
                                .push(preview)
                        )
                    ).height(Length::Shrink).into_box()
                }
                RenderMode::Terminal => {
                    let ansi = create_preview::<TermBackend>(lab).view(ctx);
                    crate::layout::containers::Card::new(
                        CodeBlock::new(ansi).transparent().on_copy(Message::CopyCode)
                    )
                    .background(iced::Color::from_rgb8(30, 30, 30))
                    .padding(0)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .into_box()
                }
                RenderMode::Neural => {
                    let node = create_preview::<AIBackend>(lab).view(ctx);
                    let json = serde_json::to_string_pretty(&node).unwrap_or_default();
                    crate::layout::containers::Card::new(
                        CodeBlock::new(json).transparent().on_copy(Message::CopyCode)
                    )
                    .background(iced::Color::from_rgb8(30, 30, 30))
                    .padding(0)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .into_box()
                }
                RenderMode::Spatial => {
                    let spatial_node = create_preview::<crate::core::SpatialBackend>(lab).view(ctx);
                    let empty_node = spatial_node.to_empty();
                    crate::layout::containers::Card::new(
                        ProxyView::new(move |ctx| {
                            View::<Message, IcedBackend>::view(&SimulatorView::new(empty_node.clone()), ctx)
                        })
                    )
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .into_box()
                }
            };

            VStack::new_generic()
                .spacing(24.0)
                .width(Length::Fill)
                .push(Text::new("The Lab").title2().bold())
                .push(ProxyView::new(scrollable_tabs))
                .push(preview_content)
        };

        // --- 3. Usage Section ---
        let usage = {
            let code = generate_code(lab);
            VStack::new_generic()
                .spacing(24.0)
                .width(Length::Fill)
                .push(Text::new("Usage").title2().bold())
                .push(CodeBlock::rust(code).on_copy(Message::CopyCode))
        };

        // --- 4. Theory Section ---
        let theory = {
            let text = r#"
### Semantic Typography
In **PeakUI**, typography is semantic. Instead of choosing raw font sizes, you choose a **Role** (e.g., Title1, Body, Caption) that adapts to the platform and device context.

- **Hierarchical:** Styles are designed to create a clear visual hierarchy.
- **Responsive:** Text roles can scale based on the `Context` size and device type.
- **Type-Safe:** Leveraging Rust's type system to ensure consistent style usage.
"#;
            VStack::new_generic()
                .spacing(24.0)
                .width(Length::Fill)
                .push(Text::new("Theory").title2().bold())
                .push(crate::views::MarkdownView::new(text))
        };

        // --- 5. Props Section ---
        let props = {
            let text = r#"
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
"#;
            VStack::new_generic()
                .spacing(24.0)
                .width(Length::Fill)
                .push(Text::new("Modifiers").title2().bold())
                .push(crate::views::MarkdownView::new(text))
        };

        // --- Final Assembly ---
        VStack::new_generic()
            .width(Length::Fill)
            .spacing(64.0)
            .padding(Padding {
                top: ctx.safe_area.top,
                right: if is_narrow { 24.0 } else { 48.0 },
                bottom: ctx.safe_area.bottom + 100.0,
                left: if is_narrow { 24.0 } else { 48.0 },
            })
            .align_x(iced::Alignment::Start)
            .push(hero)
            .push(playground)
            .push(usage)
            .push(theory)
            .push(props)
            .view(ctx)
    })).inspector(TypographyInspector::new(lab))
}

fn create_preview<B: Backend>(lab: &TypographyLabState) -> VStack<Message, B> {
    let base_size = lab.size;
    let mut text = Text::<B>::new(lab.text.clone());

    if lab.is_bold {
        text = text.bold();
    }
    if lab.is_italic {
        text = text.italic();
    }

    VStack::new_generic()
        .spacing(32.0)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .push(
            VStack::new_generic()
                .spacing(8.0)
                .align_x(Alignment::Center)
                .push(Text::<B>::new("Title 1").caption2().secondary())
                .push(text.clone().title1().size(base_size * 2.25)),
        )
        .push(
            VStack::new_generic()
                .spacing(8.0)
                .align_x(Alignment::Center)
                .push(Text::<B>::new("Body").caption2().secondary())
                .push(text.clone().body().size(base_size)),
        )
        .push(
            VStack::new_generic()
                .spacing(8.0)
                .align_x(Alignment::Center)
                .push(Text::<B>::new("Caption").caption2().secondary())
                .push(text.clone().caption1().size(base_size * 0.75)),
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
                    top: context.safe_area.top,
                    right: 20.0,
                    bottom: context.safe_area.bottom,
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
