use super::super::app::Message;
use super::super::model::Page;
use super::super::page::PageResult;
use crate::prelude::*;

pub fn view(_context: &Context, is_mobile: bool) -> PageResult {
    PageResult::new(ProxyView::<Message, IcedBackend>::new(move |ctx| {
        let t = ctx.theme;
        let is_narrow = is_mobile || ctx.size.width < 1000.0;

        // --- 1. Hero Section ---
        let hero = VStack::<Message, IcedBackend>::new_generic()
            .spacing(32.0)
            .align_x(if is_narrow {
                iced::Alignment::Center
            } else {
                iced::Alignment::Start
            })
            .width(Length::Fill)
            .push(
                VStack::new_generic()
                    .spacing(12.0)
                    .align_x(if is_narrow {
                        iced::Alignment::Center
                    } else {
                        iced::Alignment::Start
                    })
                    .push(
                        Text::<IcedBackend>::new("PeakUI")
                            .size(if is_narrow { 48.0 } else { 84.0 })
                            .bold()
                            .color(t.colors.text_primary),
                    )
                    .push(
                        Text::<IcedBackend>::new("The Operating System for your User Interface")
                            .size(if is_narrow { 24.0 } else { 32.0 })
                            .color(t.colors.text_secondary),
                    ),
            )
            .push(
                Text::<IcedBackend>::new("PeakUI is a cross-platform design system engine built for performance, type-safety, and absolute developer control across GUI, Terminal, and Neural interfaces.")
                    .body()
                    .color(t.colors.text_secondary)
                    .width(if is_narrow { Length::Fill } else { Length::Fixed(600.0) }),
            )
            .push(
                HStack::new_generic()
                    .spacing(20.0)
                    .align_y(iced::Alignment::Center)
                    .push(
                        Button::label("Quick Start")
                            .on_press(Message::SetTab(Page::Architecture))
                            .size(ControlSize::Large)
                            .width(Length::Fixed(180.0)),
                    )
                    .push(
                        Button::label("Browse Catalog")
                            .variant(Variant::Soft)
                            .on_press(Message::SetTab(Page::ShowcaseButtons))
                            .size(ControlSize::Large)
                            .width(Length::Fixed(180.0)),
                    ),
            );

        // --- 2. Features Section ---
        let feature_card = |icon: &'static str, title: &'static str, desc: &'static str| {
            ProxyView::<Message, IcedBackend>::new(move |ctx| {
                let t = ctx.theme;
                let is_internal_narrow = ctx.size.width < 1000.0;

                iced::widget::container(
                    VStack::new_generic()
                        .spacing(20.0)
                        .push(
                            Icon::<IcedBackend>::new(icon)
                                .size(28.0)
                                .color(t.colors.primary),
                        )
                        .push(
                            VStack::new_generic()
                                .spacing(12.0)
                                .push(
                                    Text::<IcedBackend>::new(title)
                                        .title3()
                                        .bold()
                                        .color(t.colors.text_primary),
                                )
                                .push(
                                    Text::<IcedBackend>::new(desc)
                                        .body()
                                        .color(t.colors.text_secondary),
                                ),
                        )
                        .view(ctx),
                )
                .padding(24)
                .width(Length::Fill)
                .height(if is_internal_narrow {
                    Length::Shrink
                } else {
                    Length::Fill
                })
                .style(move |_| iced::widget::container::Style {
                    background: Some(t.colors.surface.into()),
                    border: iced::Border {
                        radius: 20.0.into(),
                        color: t.colors.border.scale_alpha(0.3),
                        width: 1.0,
                    },
                    ..Default::default()
                })
                .into()
            })
        };

        // We wrap the entire feature content logic in its own responsive block
        let features = ProxyView::<Message, IcedBackend>::new(move |ctx| {
            let is_internal_narrow = is_mobile || ctx.size.width < 1000.0;

            if is_internal_narrow {
                VStack::new_generic()
                    .spacing(24.0)
                    .width(Length::Fill)
                    .push(feature_card(
                        "layers",
                        "Modular Architecture",
                        "Composed of independent atoms and molecules for maximum reusability.",
                    ))
                    .push(feature_card(
                        "zap",
                        "High Performance",
                        "Built on Iced and WGPU for fluid, hardware-accelerated 120fps rendering.",
                    ))
                    .push(feature_card(
                        "shield",
                        "Type Safe",
                        "Leveraging Rust's ownership and type system for guaranteed reliability.",
                    ))
                    .view(ctx)
            } else {
                HStack::new_generic()
                    .spacing(24.0)
                    .width(Length::Fill)
                    .height(Length::Fixed(240.0))
                    .push(feature_card(
                        "layers",
                        "Modular Architecture",
                        "Composed of independent atoms and molecules for maximum reusability.",
                    ))
                    .push(feature_card(
                        "zap",
                        "High Performance",
                        "Built on Iced and WGPU for fluid, hardware-accelerated 120fps rendering.",
                    ))
                    .push(feature_card(
                        "shield",
                        "Type Safe",
                        "Leveraging Rust's ownership and type system for guaranteed reliability.",
                    ))
                    .view(ctx)
            }
        });

        // --- 3. Quick Start Section ---
        let quick_start = VStack::new_generic()
            .spacing(24.0)
            .align_x(if is_narrow { iced::Alignment::Center } else { iced::Alignment::Start })
            .width(Length::Fill)
            .push(
                Text::<IcedBackend>::new("Seamless Implementation")
                    .title2()
                    .bold()
                    .color(t.colors.text_primary),
            )
            .push(ProxyView::<Message, IcedBackend>::new(move |ctx| {
                let t = ctx.theme;
                iced::widget::container(
                    Text::<IcedBackend>::new(
                        "VStack::new()\n  .spacing(16.0)\n  .push(Text::new(\"Hello PeakUI\").title1())\n  .push(Button::label(\"Get Started\"))",
                    )
                    .size(14.0)
                    .color(t.colors.text_primary)
                    .view(ctx),
                )
                .padding(24)
                .width(if ctx.size.width < 1000.0 { Length::Fill } else { Length::Fixed(500.0) })
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
            }));

        // --- 4. Footer ---
        let footer = VStack::new_generic()
            .spacing(32.0)
            .align_x(if is_narrow {
                iced::Alignment::Center
            } else {
                iced::Alignment::Start
            })
            .width(Length::Fill)
            .push(
                VStack::new_generic()
                    .spacing(16.0)
                    .align_x(if is_narrow {
                        iced::Alignment::Center
                    } else {
                        iced::Alignment::Start
                    })
                    .push(
                        Text::<IcedBackend>::new("Ready to scale?")
                            .title1()
                            .bold()
                            .color(t.colors.text_primary),
                    )
                    .push(
                        Text::<IcedBackend>::new(
                            "Explore our technical documentation and design patterns.",
                        )
                        .body()
                        .color(t.colors.text_secondary),
                    ),
            )
            .push(
                HStack::new_generic()
                    .spacing(20.0)
                    .align_y(iced::Alignment::Center)
                    .push(
                        Button::<Message, IcedBackend>::label("View Roadmap")
                            .variant(Variant::Ghost)
                            .on_press(Message::SetTab(Page::Roadmap)),
                    )
                    .push(
                        Button::<Message, IcedBackend>::label("Project Structure")
                            .variant(Variant::Ghost)
                            .on_press(Message::SetTab(Page::ProjectStructure)),
                    ),
            );

        // --- Final Assembly ---
        VStack::new_generic()
            .width(Length::Fill)
            .spacing(if is_narrow { 48.0 } else { 80.0 })
            .padding(Padding {
                top: 48.0,
                right: if is_narrow { 24.0 } else { 48.0 },
                bottom: 120.0,
                left: if is_narrow { 24.0 } else { 48.0 },
            })
            .align_x(iced::Alignment::Start)
            .push(hero)
            .push(features)
            .push(quick_start)
            .push(footer)
            .view(ctx)
    }))
}
