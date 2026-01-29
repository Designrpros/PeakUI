use super::super::app::Message;
use crate::prelude::*;
use iced::{Alignment, Color, Length, Padding};

pub fn view(context: &Context) -> Element<'static, Message, Theme, Renderer> {
    let is_mobile = context.is_slim();

    // --- Asset Detection ---
    let core_asset = if cfg!(target_arch = "wasm32") {
        "/assets/design_marvel/neural_core.png"
    } else {
        "framework/assets/design_marvel/neural_core.png"
    };

    // --- Main Composition ---
    let mut root = VStack::new()
        .width(Length::Fill)
        .spacing(if is_mobile { 80.0 } else { 120.0 });

    // 1. Hero & Navigation
    root = root.push(hero_section(context, core_asset, is_mobile));

    // 3. Technical Core (Responsive 2x2 Grid)
    root = root.push(pillars_section(context, is_mobile));

    // 4. GREEN AI STORY
    root = root.push(green_ai_section(context, is_mobile));

    // 5. INDUSTRIAL API STORY (Markdown-style Code Block)
    root = root.push(industrial_api_section(context, is_mobile));

    // 6. INDUSTRIAL VERTICALS (Responsive Grid)
    root = root.push(verticals_section(context, is_mobile));

    // 7. SAFETY LEDGER (Responsive Grid)
    root = root.push(safety_ledger_section(context, is_mobile));

    // 8. Footer
    root = root.push(footer(context, is_mobile));

    ScrollView::new(root).view(context)
}

fn hero_section(context: &Context, asset: &str, is_mobile: bool) -> ZStack<Message, IcedBackend> {
    let t = context.theme;
    ZStack::new()
        .height(Length::Fixed(if is_mobile { 400.0 } else { 600.0 }))
        .width(Length::Fill)
        .push(
            ZStack::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .push(Image::new(asset).width(Length::Fill).height(Length::Fill))
                .push(Container::new(Space::new(Length::Fill, Length::Fill)).background(Color::from_rgba(0.0, 0.0, 0.0, 0.7)))
        )
        .push(
            VStack::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(Padding {
                    top: 16.0,
                    right: 40.0,
                    bottom: 0.0,
                    left: 40.0,
                })
                .align_x(Alignment::Center)
                .push(nav_bar(context, is_mobile))
                .push(Space::new(Length::Shrink, Length::Fill))
                .push(
                    VStack::new()
                        .spacing(32.0)
                        .align_x(Alignment::Center)
                        .width(Length::Fill)
                        .push(
                            VStack::new()
                                .spacing(8.0)
                                .align_x(Alignment::Center)
                                .push(Text::new("ERA 01").bold().size(11.0).color(t.colors.primary))
                                .push(Text::new("AUTONOMOUS\nINTELLIGENCE").size(if is_mobile { 32.0 } else { 56.0 }).bold().align_center())
                                .push(Rectangle::new(Length::Fixed(120.0), Length::Fixed(4.0)).color(t.colors.primary).radius(2.0))
                        )
                        .push(Text::new(if is_mobile { "The industrial kernel.\nDeterministic. Sovereign." } else { "The only UI framework built for the eyes of LLMs.\nDeterministic. Semantic. Sovereign." }).size(16.0).secondary().align_center())
                )
                .push(Space::new(Length::Shrink, Length::Fill))
                .push(Icon::new("chevron-down").size(32.0).secondary())
        )
}

fn nav_bar(_context: &Context, is_mobile: bool) -> GlassCard<Message, IcedBackend> {
    let nav_links = if is_mobile {
        Container::new(Space::new(Length::Shrink, Length::Shrink))
    } else {
        Container::new(
            HStack::new()
                .spacing(24.0)
                .width(Length::Shrink)
                .align_y(Alignment::Center)
                .push(Text::new("Stack").caption1().secondary())
                .push(Text::new("Vision").caption1().secondary())
                .push(Text::new("Architecture").caption1().secondary()),
        )
    };

    GlassCard::new(
        HStack::new()
            .align_y(Alignment::Center)
            .padding(Padding::from([0, 24]))
            .push(Text::new("PEAKSUITE").bold().size(14.0))
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(nav_links)
            .push(Space::new(
                if is_mobile {
                    Length::Fixed(16.0)
                } else {
                    Length::Fixed(32.0)
                },
                Length::Shrink,
            ))
            .push(
                Button::label("Launch")
                    .variant(Variant::Solid)
                    .intent(Intent::Primary)
                    .width(Length::Fixed(80.0))
                    .compact()
                    .on_press(Message::EnterApp),
            ),
    )
    .padding(0.0)
    .height(Length::Fixed(44.0))
    .width(if is_mobile {
        Length::Fill
    } else {
        Length::Fixed(900.0)
    })
}

fn pillars_section(context: &Context, is_mobile: bool) -> Container<Message, IcedBackend> {
    let content = VStack::new()
        .spacing(48.0)
        .width(Length::Fill)
        .push(section_header(
            "The Technical Core",
            "Unified hierarchy for intelligence swarms.",
            is_mobile,
        ))
        .push(
            ResponsiveGrid::new()
                .spacing(32.0)
                .push(pillar_card(
                    "PeakOS",
                    "Real-Time Orchestration",
                    "Deterministic kernel for hardware coordination.",
                    "cpu",
                    context,
                ))
                .push(pillar_card(
                    "PeakUI",
                    "Semantic Vision",
                    "The world's first AI-native semantic interface.",
                    "eye",
                    context,
                ))
                .push(pillar_card(
                    "PeakDB",
                    "Neural Memory",
                    "Local-first vector storage for encrypted on-device RAG.",
                    "database",
                    context,
                ))
                .push(pillar_card(
                    "PeakRelay",
                    "Distributed Spirit",
                    "Peer-to-peer intelligence mesh for sovereign swarms.",
                    "share-2",
                    context,
                )),
        );

    Container::new(content)
        .padding(if is_mobile { 24.0 } else { 80.0 })
        .width(Length::Fill)
}

fn green_ai_section(context: &Context, is_mobile: bool) -> Container<Message, IcedBackend> {
    let t = context.theme;
    let content = VStack::new()
        .spacing(32.0)
        .width(Length::Fill)
        .push(section_header("Turning Heavy AI into Green AI", "99% Energy Reduction", is_mobile))
        .push(
            VStack::new()
                .spacing(24.0)
                .push(Text::new("Traditional Computer Vision processes millions of pixels per frame. This approach is computationally expensive and energy-intensive.").body().secondary())
                .push(Text::new("PeakUI exposes the Semantic Tree directly to AI agents. This eliminates the need for expensive pixel-processing, enabling instant interaction with minimal thermal overhead.").body().secondary())
        )
        .push(
            HStack::new()
                .spacing(12.0)
                .align_y(Alignment::Center)
                .push(Icon::new("leaf").size(24.0).color(t.colors.success))
                .push(Text::new("Sustainable industrial intelligence.").caption1().bold().color(t.colors.success))
        );

    Container::new(content)
        .padding(if is_mobile { 24.0 } else { 80.0 })
        .width(Length::Fill)
}

fn industrial_api_section(_context: &Context, is_mobile: bool) -> Container<Message, IcedBackend> {
    let code = r#"// Native Agent Interface
let pressure = framework.get_state("pressure_gauge");
if pressure.value > 80.0 {
    btn_emergency.press();
}"#;

    let code_block = CodeBlock::new(code).language("rust");

    let content = VStack::new()
        .spacing(32.0)
        .width(Length::Fill)
        .push(section_header("Every UI is an API", "No Cameras Required", is_mobile))
        .push(Text::new("An industrial robot does not need a camera to see the screen. With PeakUI's semantic state, the UI itself becomes a structured API.").body().secondary())
        .push(
            Container::new(code_block)
                .width(if is_mobile { Length::Fill } else { Length::Fixed(600.0) })
        );

    Container::new(content)
        .padding(if is_mobile { 24.0 } else { 80.0 })
        .width(Length::Fill)
}

fn verticals_section(context: &Context, is_mobile: bool) -> Container<Message, IcedBackend> {
    let grid = ResponsiveGrid::new()
        .spacing(32.0)
        .push(vertical_card(
            "Energy",
            "Smart-grid orchestration and autonomous swarms.",
            "activity",
            context,
        ))
        .push(vertical_card(
            "Defense",
            "Decentralized zero-trust tactical secure compute.",
            "shield",
            context,
        ))
        .push(vertical_card(
            "Manufacturing",
            "Deterministic control for precision robotics.",
            "target",
            context,
        ));

    Container::new(
        VStack::new()
            .spacing(48.0)
            .width(Length::Fill)
            .push(section_header(
                "Industrial Verticals",
                "Sovereign tech for infrastructure.",
                is_mobile,
            ))
            .push(grid),
    )
    .padding(if is_mobile { 24.0 } else { 80.0 })
    .width(Length::Fill)
}

fn safety_ledger_section(context: &Context, is_mobile: bool) -> Container<Message, IcedBackend> {
    let grid = ResponsiveGrid::new()
        .spacing(64.0)
        .push(blue_print_item(
            "SAFE RUST Foundation",
            "100% memory safety. No zero-day exploits.",
            context,
        ))
        .push(blue_print_item(
            "ZERO-TRUST Protocol",
            "Hardware root-of-trust encryption.",
            context,
        ));

    Container::new(
        VStack::new()
            .spacing(48.0)
            .width(Length::Fill)
            .push(section_header(
                "The Safety Ledger",
                "Verified industrial security.",
                is_mobile,
            ))
            .push(grid),
    )
    .padding(if is_mobile { 24.0 } else { 80.0 })
    .width(Length::Fill)
}

fn section_header(title: &str, subtitle: &str, is_mobile: bool) -> VStack<Message, IcedBackend> {
    VStack::new()
        .spacing(12.0)
        .width(Length::Fill)
        .push(
            Text::new(title)
                .size(if is_mobile { 32.0 } else { 48.0 })
                .bold(),
        )
        .push(
            Text::new(subtitle)
                .secondary()
                .size(if is_mobile { 16.0 } else { 20.0 }),
        )
}

fn pillar_card(
    title: &str,
    sub: &str,
    desc: &str,
    icon: &str,
    context: &Context,
) -> Container<Message, IcedBackend> {
    let t = context.theme;
    let is_beta = title == "PeakOS" || title == "PeakRelay" || title == "PeakDB";

    let mut header = HStack::new().align_y(Alignment::Center).width(Length::Fill);

    header = header.push(Icon::new(icon).size(24.0).color(t.colors.primary));

    if is_beta {
        header = header.push(Space::new(Length::Fill, Length::Shrink)).push(
            Container::new(
                Text::new("BETA")
                    .size(10.0)
                    .bold()
                    .color(t.colors.on_primary),
            )
            .padding(Padding::from([2, 8]))
            .background(t.colors.primary)
            .radius(12.0),
        );
    }

    let card_stack = VStack::new()
        .spacing(16.0)
        .padding(20.0)
        .width(Length::Fill)
        .push(header)
        .push(
            VStack::new()
                .spacing(4.0)
                .push(Text::new(title).headline().bold())
                .push(Text::new(sub).caption1().secondary()),
        )
        .push(Text::new(desc).body().secondary());

    Container::new(card_stack)
        .width(Length::Fill)
        .background(t.colors.surface)
        .border(1.0, t.colors.border.scale_alpha(0.1))
        .corner_radius(24.0)
}

fn vertical_card(
    title: &str,
    desc: &str,
    icon: &str,
    context: &Context,
) -> Container<Message, IcedBackend> {
    let t = context.theme;
    let card = VStack::new()
        .spacing(16.0)
        .padding(20.0)
        .width(Length::Fill)
        .push(Icon::new(icon).size(24.0).color(t.colors.primary))
        .push(Text::new(title).headline().bold())
        .push(Text::new(desc).body().secondary());

    Container::new(card)
        .width(Length::Fill)
        .background(t.colors.surface)
        .border(1.0, t.colors.border.scale_alpha(0.1))
        .corner_radius(24.0)
}

fn blue_print_item(label: &str, desc: &str, context: &Context) -> HStack<Message, IcedBackend> {
    let t = context.theme;
    HStack::new()
        .spacing(24.0)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .push(
            Rectangle::new(Length::Fixed(4.0), Length::Fixed(40.0))
                .color(t.colors.primary)
                .radius(2.0),
        )
        .push(
            VStack::new()
                .push(Text::new(label).caption1().bold().secondary())
                .push(Text::new(desc).body()),
        )
}

fn footer(_context: &Context, is_mobile: bool) -> Container<Message, IcedBackend> {
    let content = VStack::new()
        .spacing(48.0)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .push(Divider::new())
        .push(
            HStack::new()
                .spacing(32.0)
                .push(Text::new("Oslo").caption1().secondary())
                .push(Text::new("ERA 2026").caption1().secondary())
                .push(Text::new("BSL-1.1").caption1().secondary()),
        );

    Container::new(content)
        .padding(if is_mobile { 32.0 } else { 80.0 })
        .width(Length::Fill)
}
