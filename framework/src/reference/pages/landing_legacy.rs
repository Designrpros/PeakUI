use super::super::app::Message;
use crate::prelude::*;
// No direct iced imports! Rely on PeakUI abstractions. Force re-include.

pub fn view(context: &Context) -> Element<'static, Message, Theme, Renderer> {
    let t = context.theme;
    let is_mobile = context.is_slim();

    // --- Helpers ---
    let section_title = |title: String| {
        Text::new(title)
            .title2()
            .bold()
            .color(t.colors.text_primary)
            .align_center()
    };

    let section_desc = |text: String| {
        VStack::new()
            .width(if is_mobile {
                Length::Fill
            } else {
                Length::Fixed(600.0)
            })
            .push(
                Text::new(text)
                    .body()
                    .color(t.colors.text_secondary)
                    .align_center()
                    .width(Length::Fill),
            )
    };

    // --- Hero Section ---
    let hero = VStack::<Message>::new()
        .spacing(40.0)
        .padding(Padding {
            top: 100.0,
            ..Default::default()
        })
        .align_x(Alignment::Center)
        .push(
            Text::new(context.t("hero-title"))
                .title1()
                .bold()
                .color(t.colors.text_primary)
                .align_center(),
        )
        .push(
            Text::new(context.t("hero-subtitle"))
                .title2()
                .color(t.colors.text_primary)
                .align_center(),
        )
        .push(
            VStack::new()
                .width(if is_mobile {
                    Length::Fill
                } else {
                    Length::Fixed(700.0)
                })
                .push(
                    Text::new(context.t("hero-desc"))
                        .body()
                        .color(t.colors.text_secondary)
                        .align_center()
                        .width(Length::Fill),
                ),
        )
        .push(
            HStack::new()
                .spacing(12.0)
                .width(Length::Shrink) // Shrink to enable centering by parent VStack
                .align_y(Alignment::Center)
                .push(
                    Button::label("🇺🇸 English")
                        .variant(Variant::Ghost)
                        .on_press(Message::SetLanguage(
                            "en-US".into(),
                            vec![include_str!(
                                "../../../../apps/showcase/assets/locales/en-US/main.ftl"
                            )
                            .to_string()],
                        )),
                )
                .push(Text::new("|").secondary())
                .push(Button::label("🇳🇴 Norsk").variant(Variant::Ghost).on_press(
                    Message::SetLanguage(
                        "nb-NO".into(),
                        vec![include_str!(
                                "../../../../apps/showcase/assets/locales/nb-NO/main.ftl"
                            )
                            .to_string()],
                    ),
                )),
        )
        .push(
            Button::new(
                Text::new(context.t("catalog-button"))
                    .bold()
                    .align_center()
                    .width(Length::Fill),
            )
            .variant(Variant::Outline)
            .width(Length::Fixed(240.0))
            .on_press(Message::EnterApp),
        );

    // --- Core Values (Text Focused) ---
    // Instead of heavy cards, we'll use clean columns
    let feature_item = |icon: &'static str, title: String, desc: String| {
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
                    .align_x(Alignment::Center) // Fix centering of description
                    .push(
                        Text::new(desc)
                            .body()
                            .color(t.colors.text_secondary)
                            .align_center(),
                    ),
            )
    };

    let features_grid = VStack::<Message>::new()
        .spacing(64.0) // More breathing room
        .align_x(Alignment::Center)
        .push(if is_mobile {
            // Stack vertically
            Box::new(
                VStack::new()
                    .spacing(48.0)
                    .width(Length::Fill)
                    .push(feature_item(
                        "boxes",
                        context.t("feature-modular"),
                        context.t("feature-modular-desc"),
                    ))
                    .push(feature_item(
                        "zap",
                        context.t("feature-performant"),
                        context.t("feature-performant-desc"),
                    ))
                    .push(feature_item(
                        "shield-check",
                        context.t("feature-typesafe"),
                        context.t("feature-typesafe-desc"),
                    )),
            ) as Box<dyn View<Message, IcedBackend>>
        } else {
            // Horizontal Grid
            Box::new(
                HStack::new()
                    .spacing(48.0)
                    .width(Length::Fill)
                    .push(feature_item(
                        "boxes",
                        context.t("feature-modular"),
                        context.t("feature-modular-desc"),
                    ))
                    .push(feature_item(
                        "zap",
                        context.t("feature-performant"),
                        context.t("feature-performant-desc"),
                    ))
                    .push(feature_item(
                        "shield-check",
                        context.t("feature-typesafe"),
                        context.t("feature-typesafe-desc"),
                    )),
            ) as Box<dyn View<Message, IcedBackend>>
        });

    // --- Section: Green AI (Storytelling) ---
    let green_ai = VStack::<Message>::new()
        .spacing(40.0)
        .align_x(Alignment::Center)
        .push(section_title(context.t("green-ai-title")))
        .push(section_desc(context.t("green-ai-desc1")))
        .push(section_desc(context.t("green-ai-desc2")))
        .push(
            HStack::new()
                .spacing(12.0)
                .width(Length::Shrink) // Shrink to allow centering
                .align_y(Alignment::Center)
                .push(Icon::new("fuel").size(24.0).color(t.colors.success))
                .push(
                    Text::new(context.t("green-ai-stat"))
                        .body()
                        .bold()
                        .color(t.colors.success)
                        .align_start(),
                ),
        );

    // --- Section: Robot OS ---
    let robot_os = VStack::<Message>::new()
        .spacing(40.0)
        .align_x(Alignment::Center) // Centered!
        .push(
            Text::new(context.t("every-ui-api-title"))
                .title2()
                .bold()
                .color(t.colors.text_primary)
                .align_center(),
        )
        .push(
            VStack::new()
                .width(if is_mobile {
                    Length::Fill
                } else {
                    Length::Fixed(700.0)
                })
                .push(
                    Text::new(context.t("every-ui-api-desc"))
                        .body()
                        .color(t.colors.text_secondary)
                        .align_center()
                        .width(Length::Fill),
                ),
        )
        .push(
            // Constrain width to nicely align with text and provide "outer padding" visually
            VStack::new()
                .width(if is_mobile {
                    Length::Fill
                } else {
                    Length::Fixed(700.0)
                }) // Responsive Width
                .push(CodeBlock::rust(
                    "let pressure = framework.get_state(\"pressure_gauge\");",
                )),
        );

    // --- Footer ---
    let footer = VStack::<Message>::new()
        .spacing(48.0)
        .align_x(Alignment::Center)
        .padding(Padding {
            top: 120.0,
            ..Default::default()
        })
        .width(Length::Fill)
        .push(Divider::new())
        .push(
            VStack::new()
                .spacing(16.0)
                .align_x(Alignment::Center)
                .push(
                    Text::new(context.t("footer-framework"))
                        .bold()
                        .color(t.colors.text_primary)
                        .align_center(),
                )
                .push(
                    Text::new(context.t("footer-version"))
                        .caption1()
                        .color(t.colors.text_primary)
                        .align_center(),
                ),
        )
        .push(
            HStack::new()
                .spacing(32.0)
                .width(Length::Shrink)
                .push(
                    Button::label(context.t("footer-docs"))
                        .variant(Variant::Ghost)
                        .on_press(Message::EnterApp),
                )
                .push(
                    Button::label(context.t("footer-github"))
                        .variant(Variant::Ghost)
                        .on_press(Message::OpenUrl(
                            "https://github.com/Designrpros/PeakUI".into(),
                        )),
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
