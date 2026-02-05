use crate::navigation::PageResult;
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
                        Text::<IcedBackend>::new("Project Structure")
                            .size(if is_narrow { 32.0 } else { 48.0 })
                            .bold()
                            .align_start()
                            .width(Length::Fill)
                            .color(t.colors.text_primary),
                    )
                    .push(
                        Text::<IcedBackend>::new("PeakUI follows a modular, feature-based architecture. This ensures that the framework and applications remains maintainable and 'Intelligence-Native' at every scale.")
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
                        Button::label("Open GitHub")
                            .variant(Variant::Outline)
                            .on_press(Message::SetTab(crate::reference::model::Page::Introduction)) // Placeholder
                            .size(ControlSize::Large)
                            .width(Length::Fill),
                    )
                    .push(
                        Button::label("Architecture Guide")
                            .variant(Variant::Soft)
                            .on_press(Message::SetTab(crate::reference::model::Page::Architecture))
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

        let core_modules = doc_section(
            "Core Modules",
            vec![
                Box::new(Text::new("The codebase is organized into highly specialized crates, each handling a critical pillar of the interface engine.")),
                Box::new(Text::new("• `crates/peak-ui`: The primary framework library. Contains the renderer, layout engine, and core 'Intelligence' traits.")),
                Box::new(Text::new("• `crates/peak-core`: The stable foundation. Defines the protocols (like `SemanticNode`) that allow other modules to speak to each other.")),
                Box::new(Text::new("• `crates/peak-theme`: The sovereign design system. Manages HSL color tokens, typography scales, and responsive breakpoints.")),
            ],
        );

        let source_organization = doc_section(
            "Source Organization",
            vec![
                Box::new(Text::new("Within `crates/peak-ui`, the source is structured to mirror the logical flow of a modern UI engine.")),
                Box::new(code_block("src/\n├── core.rs         # Traits: View, App, Backend\n├── accessibility.rs # The A11y & Neural Bridge\n├── atoms/          # Basic components (Text, Button)\n├── layout/         # Spatial & Linear engines\n└── reference/      # The Showcase Application")),
                Box::new(Text::new("This separation ensures that as we add support for new platforms (like VR/AR or TUI), the core logic remains untouched.")),
            ],
        );

        let spatial_engines = doc_section(
            "Spatial & Volumetric Engines",
            vec![
                Box::new(Text::new("A unique aspect of the PeakUI structure is its dedicated spatial reasoning layer found in `src/layout/spatial`.")),
                Box::new(Text::new("• `bounding_box.rs`: Handles 3D collision and hit-testing for spatial environments (Vision Pro, Meta Quest).")),
                Box::new(Text::new("• `billboarding.rs`: Logic for components that always face the user in 3D space.")),
                Box::new(Text::new("• `depth.rs`: Managing Z-index and physical physical layering.")),
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
            .push(core_modules)
            .push(source_organization)
            .push(spatial_engines)
            .push(Space::<IcedBackend>::new(
                Length::Fill,
                Length::Fixed(120.0),
            ))
            .view(ctx)
    }))
}
