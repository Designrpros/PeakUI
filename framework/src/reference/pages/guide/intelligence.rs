
use crate::prelude::*;
use crate::reference::AppPageResult;
use crate::reference::app::Message;

pub fn view(_context: &Context, is_mobile: bool, _api_key: String) -> AppPageResult {
    AppPageResult::new(ProxyView::<Message, IcedBackend>::new(move |ctx| {
        let t = ctx.theme;
        let is_narrow = is_mobile || ctx.size.width < 1000.0;

        // --- 1. Hero Section ---
        let hero = VStack::<Message, IcedBackend>::new_generic()
            .spacing(32.0)
            .align_x(iced::Alignment::Start)
            .width(Length::Fill)
            .push(
                VStack::new_generic()
                    .spacing(12.0)
                    .align_x(iced::Alignment::Start)
                    .push(
                        Text::<IcedBackend>::new("Intelligence")
                            .size(if is_narrow { 32.0 } else { 48.0 })
                            .bold()
                            .align_start()
                            .width(Length::Fill)
                            .color(t.colors.text_primary),
                    )
                    .push(
                        Text::<IcedBackend>::new("PeakUI is not just a UI framework; it's a Semantic Serialization Engine that transforms visual hierarchy into machine-readable state for the AI era.")
                            .size(20.0)
                            .align_start()
                            .width(Length::Fill)
                            .color(t.colors.text_secondary),
                    ),
            )
            .push(
                HStack::new_generic()
                    .spacing(20.0)
                    .align_y(iced::Alignment::Center)
                    .push(
                        Button::label("View Protocol")
                            .variant(Variant::Outline)
                            .on_press(Message::SetTab(crate::reference::AppPage::Introduction)) // Placeholder
                            .size(ControlSize::Large)
                            .width(Length::Fill),
                    )
                    .push(
                        Button::label("Architecture")
                            .variant(Variant::Soft)
                            .on_press(Message::SetTab(crate::reference::AppPage::Architecture))
                            .size(ControlSize::Large)
                            .width(Length::Fill),
                    ),
            );

        // --- Helper: Content Section ---
        let doc_section =
            |title: &'static str, content: Vec<Box<dyn View<Message, IcedBackend>>>| {
                let mut column = VStack::new_generic()
                    .spacing(24.0)
                    .align_x(iced::Alignment::Start)
                    .width(Length::Fill);

                column = column.push(
                    Text::<IcedBackend>::new(title)
                        .title2()
                        .bold()
                        .color(t.colors.text_primary),
                );

                for item in content {
                    column = column.push(item);
                }

                column
            };

        // --- Helper: Code Block ---
        let code_block = |code: &'static str| {
            Box::new(ProxyView::<Message, IcedBackend>::new(move |ctx| {
                let t = ctx.theme;
                iced::widget::container(
                    Text::<IcedBackend>::new(code)
                        .size(14.0)
                        .color(t.colors.text_primary)
                        .view(ctx),
                )
                .padding(24)
                .width(Length::Fill)
                .style(move |_| iced::widget::container::Style {
                    background: Some(t.colors.surface.scale_alpha(0.5).into()),
                    border: iced::Border {
                        radius: 12.0.into(),
                        color: t.colors.border.scale_alpha(0.2),
                        width: 1.0,
                    },
                    ..Default::default()
                })
                .into()
            })) as Box<dyn View<Message, IcedBackend>>
        };

        // --- Sections ---

        let semantic_tree = doc_section(
            "1. The Semantic Tree (Eyes)",
            vec![
                Box::new(Text::new("Every component in PeakUI implements the `describe()` method. This generates a `SemanticNode`—a parallel, sparse tree optimized for LLM consumption.")),
                Box::new(Text::new("• Role: Precise component purpose (e.g., `button`, `text_field`, `neural_sudo`).")),
                Box::new(Text::new("• Content: The semantic value, stripped of styling and visual noise.")),
                Box::new(Text::new("• Neural Tag: A unique key that allows an AI to target and interact with specific elements without visual searching.")),
                Box::new(code_block("// How a component defines its 'Eyes'\nfn describe(&self, _ctx: &Context) -> SemanticNode {\n    SemanticNode::new(\"action_button\")\n        .label(\"Execute Transaction\")\n        .tag(\"main-action-1\")\n}")),
            ],
        );

        let action_bridge = doc_section(
            "2. The Action Bridge (Hands)",
            vec![
                Box::new(Text::new("PeakUI uses a high-precision Action Protocol. The AI emits structured tags in its natural language responses, which the framework translates into internal messages.")),
                Box::new(Text::new("• Deterministic Interaction: No more unstable 'click at pixel X,Y'.")),
                Box::new(Text::new("• Human-in-the-Loop: Critical actions can be wrapped in `NeuralSudo` to require explicit human confirmation.")),
                Box::new(code_block("// The AI emits a protocol tag:\n// \"I will now help you [Action: Navigate(Settings)]\"\n// Framework executes: Message::SetTab(AppPage::Settings)")),
            ],
        );

        let contextual_awareness = doc_section(
            "3. Contextual Awareness",
            vec![
                Box::new(Text::new("By combining the Semantic Tree with the Action Bridge, the AI gains a 'Sense of Self' within the application.")),
                Box::new(Text::new("The assistant doesn't just 'see' the UI; it understands the current application state, available actions, and security constraints, enabling complex multi-step reasoning.")),
            ],
        );

        // --- Final Assembly ---
        VStack::new_generic()
            .width(Length::Fill)
            .spacing(64.0)
            .padding(Padding {
                top: ctx.safe_area.top,
                right: if is_narrow { 24.0 } else { 48.0 },
                bottom: ctx.safe_area.bottom,
                left: if is_narrow { 24.0 } else { 48.0 },
            })
            .align_x(iced::Alignment::Start)
            .push(hero)
            .push(semantic_tree)
            .push(action_bridge)
            .push(contextual_awareness)
            .push(Space::<IcedBackend>::new(
                Length::Fill,
                Length::Fixed(120.0),
            ))
            .view(ctx)
    }))
}
