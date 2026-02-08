use crate::engine::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::Message;

pub fn view(_context: &Context, is_mobile: bool) -> PageResult<Message> {
    PageResult::new(ProxyView::<Message, IcedBackend>::new(move |ctx| {
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
                        Text::<IcedBackend>::new("Architecture")
                            .size(if is_narrow { 32.0 } else { 48.0 })
                            .bold()
                            .align_start()
                            .width(Length::Fill)
                            .color(t.colors.text_primary),
                    )
                    .push(
                        Text::<IcedBackend>::new("The PeakUI architecture is designed for the 'Intelligence-Native' era, bridging the gap between raw compute and human-centric design.")
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
                        Button::label("Read the pitch")
                            .variant(Variant::Outline)
                            .on_press(Message::SetTab(crate::reference::model::Page::Introduction)) // Placeholder or specific link if exists
                            .size(ControlSize::Large)
                            .width(Length::Fill),
                    )
                    .push(
                        Button::label("Intelligence Guide")
                            .variant(Variant::Soft)
                            .on_press(Message::SetTab(crate::reference::model::Page::Intelligence))
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

        let orchestration = doc_section(
            "Orchestration Layer",
            vec![
                Box::new(Text::new("The ContentView acts as the central hub of a PeakUI application. It doesn't just render pixels; it manages the entire lifecycle of the interface.")),
                Box::new(Text::new("• Dynamic Overlays: Automatic search and inspector injections based on application state.")),
                Box::new(Text::new("• Responsive Adaptability: Unifies mobile and desktop navigation patterns into a single codebase.")),
                Box::new(Text::new("• Contextual Awareness: Propagates safe areas (like notches) and theme tokens down the entire view tree.")),
            ],
        );

        let neural_standard = doc_section(
            "The Neural Standard",
            vec![
                Box::new(Text::new("Unlike traditional frameworks, PeakUI is built for AI agents. This is achieved through two core protocols:")),
                Box::new(Text::new("1. Semantic Serialisation: Every component can describe its purpose in a low-bandwidth format (BSON/JSON), reducing AI data consumption by 99.99%.")),
                Box::new(Text::new("2. Action Bridge: A deterministic API that allows AI agents to trigger UI events with high precision, bypassing unstable vision-based interaction.")),
                Box::new(code_block("impl View for MyComponent {\n    fn describe(&self) -> SemanticNode {\n        SemanticNode::new(\"action_button\")\n          .label(\"Transact\")\n          .sudo(\"Financial Risk\")\n    }\n}")),
            ],
        );

        let visual_systems = doc_section(
            "Visual Systems",
            vec![
                Box::new(Text::new("PeakUI delivers a premium visual experience through advanced rendering techniques:")),
                Box::new(Text::new("• Dynamic Notch: A context-aware area for status indicators and tool selection.")),
                Box::new(Text::new("• Glassmorphism: Real-time background blur and saturation adaptation, providing depth and focus.")),
                Box::new(Text::new("• Spatial Awareness: Components are aware of their Z-depth, enabling consistent behavior in 3D/Volumetric environments.")),
            ],
        );

        let navigation_modes = doc_section(
            "Navigation Modes",
            vec![
                Box::new(Text::new("PeakUI supports distinct navigation contexts tailored to the user's current task:")),
                Box::new(Text::new("• Guide: Linear, narrative-driven documentation (what you are reading now).")),
                Box::new(Text::new("• Catalog: Visual exploration of atomic components with interactive previews.")),
                Box::new(Text::new("• Workspace: Complex, multi-pane application layouts for professional workflows.")),
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
            .push(orchestration)
            .push(neural_standard)
            .push(visual_systems)
            .push(navigation_modes)
            .push(Space::<IcedBackend>::new(
                Length::Fill,
                Length::Fixed(120.0),
            ))
            .view(ctx)
    }))
}
