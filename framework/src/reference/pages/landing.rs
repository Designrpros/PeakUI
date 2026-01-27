use super::super::app::Message;
use crate::prelude::*;
// No direct iced imports! Rely on PeakUI abstractions.

pub fn view(context: &Context) -> Element<'static, Message, Theme, Renderer> {
    let t = context.theme;
    let is_mobile = context.is_slim();

    // --- Helpers ---
    let section_title = |title: &'static str| {
        Text::new(title)
            .title2()
            .bold()
            .color(t.colors.text_primary)
            .align_center()
    };

    let section_desc = |text: &'static str| {
        Text::new(text)
            .body()
            .color(t.colors.text_secondary)
            .align_center()
            .width(if is_mobile {
                Length::Fill
            } else {
                Length::Fixed(600.0)
            })
    };

    // --- Hero Section ---
    let hero = VStack::<Message>::new()
        .spacing(32.0)
        .align_x(Alignment::Center)
        .push(
            VStack::new()
                .spacing(12.0)
                .align_x(Alignment::Center)
                .push(
                    Text::new("PeakUI")
                        .size(if is_mobile { 48.0 } else { 96.0 })
                        .bold()
                        .color(t.colors.text_primary)
                        .align_center()
                )
                .push(
                    Text::new("The Operating System for your User Interface")
                        .size(if is_mobile { 24.0 } else { 32.0 })
                        .align_center()
                        .color(t.colors.text_secondary)
                )
        )
        .push(
            Text::new("PeakUI is a cross-platform design system engine built for performance, type-safety, and absolute developer control across GUI, Terminal, and Neural interfaces.")
                .body()
                .secondary()
                .align_center()
                .width(if is_mobile { Length::Fill } else { Length::Fixed(700.0) })
        )
        .push(
            if is_mobile {
                // Mobile: Vertical Stack
                Box::new(VStack::new()
                    .spacing(16.0)
                    .width(Length::Fill)
                    .padding(Padding { left: 24.0, right: 24.0, ..Default::default() }) // Add side padding
                    .align_x(Alignment::Center)
                    .push(
                        Button::label("Quick Start")
                            .on_press(Message::EnterApp)
                            .size(ControlSize::Large)
                            .variant(Variant::Outline)
                            .intent(Intent::Primary)
                            .width(Length::Fixed(280.0)) // Fixed width for scroll gutters
                    )
                    .push(
                        Button::label("Browse Catalog")
                            .variant(Variant::Outline)
                            .on_press(Message::EnterApp)
                            .size(ControlSize::Large)
                            .width(Length::Fixed(280.0)) // Fixed width for scroll gutters
                    )) as Box<dyn View<Message, IcedBackend>>
            } else {
                // Desktop: Horizontal Stack
                Box::new(HStack::new()
                    .spacing(20.0)
                    .width(Length::Shrink)
                    .align_y(Alignment::Center)
                    .push(
                        Button::label("Quick Start")
                            .on_press(Message::EnterApp)
                            .size(ControlSize::Large)
                            .variant(Variant::Outline)
                            .intent(Intent::Primary)
                            .width(Length::Fixed(180.0))
                    )
                    .push(
                        Button::label("Browse Catalog")
                            .variant(Variant::Outline)
                            .on_press(Message::EnterApp)
                            .size(ControlSize::Large)
                            .width(Length::Fixed(180.0))
                    )) as Box<dyn View<Message, IcedBackend>>
            }
        );

    // --- Core Values (Text Focused) ---
    // Instead of heavy cards, we'll use clean columns
    let feature_item = |icon: &'static str, title: &'static str, desc: &'static str| {
        VStack::new()
            .spacing(16.0)
            .align_x(Alignment::Center) // Center items
            .push(Icon::new(icon).size(32.0).color(t.colors.primary))
            .push(
                Text::new(title)
                    .title3()
                    .bold()
                    .color(t.colors.text_primary)
                    .align_center(),
            )
            .push(
                VStack::new()
                    // Remove fixed height to allow wrapping on mobile without clipping
                    // .height(Length::Fixed(80.0))
                    .align_x(Alignment::Center) // Fix centering of description
                    .push(
                        Text::new(desc)
                            .body()
                            .color(t.colors.text_secondary)
                            .align_center(),
                    ),
            )
    };

    let features_grid =
        VStack::<Message>::new()
            .spacing(64.0) // More breathing room
            .align_x(Alignment::Center)
            .push(if is_mobile {
                // Stack vertically
                Box::new(VStack::new()
                    .spacing(48.0)
                    .width(Length::Fill)
                    .push(feature_item(
                        "boxes",
                        "Modular Architecture",
                        "Composed of independent atoms and molecules for maximum reusability.",
                    ))
                    .push(feature_item(
                        "zap",
                        "High Performance",
                        "Built on Iced and WGPU for fluid, hardware-accelerated 120fps rendering.",
                    ))
                    .push(feature_item(
                        "shield-check",
                        "Type Safe",
                        "Leveraging Rust's ownership and type system for guaranteed reliability.",
                    ))) as Box<dyn View<Message, IcedBackend>>
            } else {
                // Horizontal Grid
                Box::new(HStack::new()
                    .spacing(48.0)
                    .width(Length::Fill)
                    .push(feature_item(
                        "boxes",
                        "Modular Architecture",
                        "Composed of independent atoms and molecules for maximum reusability.",
                    ))
                    .push(feature_item(
                        "zap",
                        "High Performance",
                        "Built on Iced and WGPU for fluid, hardware-accelerated 120fps rendering.",
                    ))
                    .push(feature_item(
                        "shield-check",
                        "Type Safe",
                        "Leveraging Rust's ownership and type system for guaranteed reliability.",
                    ))) as Box<dyn View<Message, IcedBackend>>
            });

    // --- Section: Green AI (Storytelling) ---
    let green_ai = VStack::<Message>::new()
        .spacing(40.0)
        .align_x(Alignment::Center)
        .push(section_title("Turning Heavy AI into Green AI"))
        .push(section_desc("Traditional Computer Vision processes millions of pixels per frame. This approach is computationally expensive, energy-intensive, and prone to latency."))
        .push(section_desc("PeakUI exposes the Semantic Tree directly to AI agents. This means your AI doesn't need to 'see' the screen—it understands the structure immediately. This reduces energy consumption by 99% and enables instant, error-free agent interaction."))
        .push(
            HStack::new()
                .spacing(12.0)
                .width(Length::Shrink) // Shrink to allow centering
                .align_y(Alignment::Center)
                .push(Icon::new("fuel").size(24.0).color(t.colors.success))
                .push(Text::new("99% Energy Reduction").body().bold().color(t.colors.success).align_start()) // Align start next to icon
        );

    // --- Section: Robot OS ---
    let robot_os = VStack::<Message>::new()
        .spacing(40.0)
        .align_x(Alignment::Center) // Centered!
        .push(Text::new("Every UI is an API").title2().bold().color(t.colors.text_primary).align_center())
        .push(
            Text::new("An industrial robot checking a 'Pressure Gauge' doesn't need a camera to 'see' the screen. With PeakUI's semantic state, the UI itself becomes a structured API that machines can query reliably.")
                .body()
                .color(t.colors.text_secondary)
                .width(if is_mobile { Length::Fill } else { Length::Fixed(700.0) }) // Responsive Width
                .align_center()
        )
        .push(
            // Constrain width to nicely align with text and provide "outer padding" visually
            VStack::new()
                .width(if is_mobile { Length::Fill } else { Length::Fixed(700.0) }) // Responsive Width
                .push(CodeBlock::rust("let pressure = framework.get_state(\"pressure_gauge\");"))
        );

    // --- Footer ---
    let footer = VStack::<Message>::new()
        .spacing(32.0)
        .align_x(Alignment::Center)
        .padding(Padding {
            top: 120.0,
            ..Default::default()
        }) // More spacing before footer
        .width(Length::Fill)
        .push(Divider::new())
        .push(
            VStack::new()
                .spacing(24.0)
                .align_x(Alignment::Center) // Center footer content
                .push(
                    VStack::new()
                        .spacing(8.0)
                        .align_x(Alignment::Center)
                        .push(Text::new("PeakUI Framework").bold().align_center())
                        .push(
                            Text::new("v2.4.0 (Alpha)")
                                .caption1()
                                .secondary()
                                .align_center(),
                        ),
                )
                .push(
                    HStack::new()
                        .spacing(32.0) // Wider spacing for centered links
                        .width(Length::Shrink) // Shrink to allow centering by parent
                        .push(Text::new("Docs").body().secondary().align_center())
                        .push(Text::new("GitHub").body().secondary().align_center())
                        .push(Text::new("Twitter").body().secondary().align_center()),
                ),
        );

    // --- Assembly ---
    let main_content = VStack::<Message>::new()
        .width(Length::Fill)
        .spacing(if is_mobile { 100.0 } else { 160.0 }) // Reduce spacing on mobile
        .padding(Padding {
            top: 140.0,
            right: if is_mobile { 24.0 } else { 0.0 }, // Add horizontal padding on mobile
            bottom: 140.0,
            left: if is_mobile { 24.0 } else { 0.0 }, // Add horizontal padding on mobile
        })
        .push(hero)
        .push(features_grid)
        .push(green_ai)
        .push(robot_os)
        .push(footer);

    // Wrap in a centered container with max width
    // ScrollView generates the scrolling area.
    ScrollView::new(
        VStack::new()
            .width(Length::Fill)
            .align_x(Alignment::Center) // Center the column
            .push(main_content),
    )
    .view(context)
}
