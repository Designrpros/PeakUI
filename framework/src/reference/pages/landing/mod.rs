use crate::reference::model::Page;
use super::super::app::Message;
use crate::prelude::*;
use crate::layout::Wrap;
use crate::controls::TextInput;

pub mod peak_os;
pub mod peak_ui;
pub mod peak_db;
pub mod peak_relay;
pub mod peak_hub;

pub fn view(context: &Context, query: &str, placeholder: &str) -> Element<'static, Message, Theme, Renderer> {
    log::info!("RENDER: LANDING PAGE");
    let is_mobile = context.is_slim();
    let _t = context.theme;

    // Asset selection based on target
    let core_asset = if cfg!(target_arch = "wasm32") {
        "/assets/design_marvel/neural_core.png"
    } else {
        "framework/assets/design_marvel/neural_core.png"
    };

    // Root Stack
    let mut root = VStack::new()
        .width(Length::Fill)
        .spacing(if is_mobile { 80.0 } else { 120.0 });

    root = root.push(hero_section(context, core_asset, is_mobile, query, placeholder));
    root = root.push(about_section(context, is_mobile));
    root = root.push(pillars_section(context, is_mobile));
    root = root.push(green_ai_section(context, is_mobile));
    root = root.push(industrial_api_section(context, is_mobile));
    root = root.push(verticals_section(context, is_mobile));
    root = root.push(safety_ledger_section(context, is_mobile));
    root = root.push(footer(context, is_mobile));

    ScrollView::new(root).view(context)
}

fn hero_section(context: &Context, _asset: &str, is_mobile: bool, query: &str, placeholder: &str) -> ZStack<Message, IcedBackend> {
    let t = context.theme;
    let viewport_height = context.size.height.max(600.0);

    ZStack::new()
        .height(Length::Fixed(viewport_height))
        .width(Length::Fill)
        .push(
            Container::new(Space::new(Length::Fill, Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .background(t.colors.background)
        )
        .push(
            VStack::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(Padding {
                    top: context.safe_area.top + 24.0,
                    right: 20.0,
                    bottom: context.safe_area.bottom,
                    left: 20.0,
                })
                .align_x(Alignment::Center)
                .push(
                    Container::new(nav_bar(context, is_mobile))
                        .width(Length::Fill)
                        .center_x(Length::Fill),
                )
                .push(Space::new(Length::Shrink, Length::Fill))
                .push(
                    VStack::new()
                        .spacing(48.0)
                        .align_x(Alignment::Center)
                        .width(Length::Fill)
                        .push(
                            VStack::new()
                                .spacing(24.0)
                                .align_x(Alignment::Center)
                                .push(Text::new("ERA 01").bold().size(12.0).color(t.colors.primary))
                                .push(
                                    Text::new("AUTONOMOUS\nINTELLIGENCE")
                                        .size(if is_mobile { 48.0 } else { 96.0 })
                                        .bold()
                                        .align_center()
                                        .width(Length::Fill)
                                        .color(t.colors.primary)
                                )
                                .push(
                                    Rectangle::new(Length::Fixed(160.0), Length::Fixed(6.0))
                                        .color(t.colors.primary)
                                        .radius(3.0)
                                )
                        )
                        .push(
                            Container::new(
                                HStack::new()
                                    .align_y(Alignment::Center)
                                    .spacing(16.0)
                                    .padding(Padding::from([12, 24]))
                                    .push(Icon::new("sparkles").size(24.0).color(t.colors.primary))
                                    .push(
                                        TextInput::new(
                                            query.to_string(), 
                                            placeholder, 
                                            Message::Search
                                        )
                                        .variant(Variant::Ghost)
                                        .on_submit(Message::EnterApp)
                                        .width(Length::Fill)
                                    )
                                    .push(
                                        Button::new(
                                            Container::new(Icon::new("arrow-right").size(20.0).color(Color::WHITE))
                                                .padding(10.0)
                                                .background(t.colors.primary)
                                                .radius(100.0)
                                        )
                                        .on_press(Message::EnterApp)
                                        .variant(Variant::Compact)
                                        .padding(0.0)
                                    )
                            )
                            .background(Color::TRANSPARENT)
                            .border(1.5, t.colors.primary)
                            .radius(100.0)
                            .width(Length::Fixed(if is_mobile { 320.0 } else { 560.0 }))
                        )
                        .push(
                            Text::new("try \"change theme\", \"modify the button\", or \"how do you experience the framework\"")
                                .caption1()
                                .secondary()
                                .align_center()
                        )
                )
                .push(Space::new(Length::Shrink, Length::Fill))
                .push(
                    VStack::new()
                        .spacing(8.0)
                        .align_x(Alignment::Center)
                        .push(Text::new("SCROLL TO EXPLORE").caption2().secondary())
                        .push(Icon::new("chevron-down").size(24.0).secondary())
                )
        )
}

fn nav_bar(_context: &Context, is_mobile: bool) -> Container<Message, IcedBackend> {
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

    Container::new(
        HStack::new()
            .align_y(Alignment::Center)
            .padding(Padding::from([0, 24]))
            .push(Text::new("PEAKSUITE").bold().size(14.0))
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(nav_links)
            .push(Space::new(if is_mobile { Length::Fixed(16.0) } else { Length::Fixed(32.0) }, Length::Shrink))
            .push(
                Button::label("Launch")
                    .variant(Variant::Compact)
                    .intent(Intent::Primary)
                    .width(Length::Shrink)
                    .on_press(Message::EnterApp),
            ),
    )
    .padding(0.0)
    .center_y(Length::Fixed(44.0))
    .width(if is_mobile { Length::Fill } else { Length::Fixed(900.0) })
    .radius(22.0)
    .border(2.0, _context.theme.colors.border)
}

fn about_section(context: &Context, is_mobile: bool) -> Container<Message, IcedBackend> {
    let t = context.theme;

    let content = VStack::new()
        .spacing(48.0)
        .width(Length::Fill)
        .align_x(Alignment::Start)
        .push(
            VStack::new()
                .spacing(24.0)
                .width(Length::Fill)
                .align_x(Alignment::Start)
                .push(Text::new("PeakUI").size(if is_mobile { 40.0 } else { 64.0 }).bold().color(t.colors.primary))
                .push(Text::new("The Universal Language for Machines").size(if is_mobile { 20.0 } else { 24.0 }).secondary())
        )
        .push(
            Container::new(
                Text::new("PeakUI is not just a framework; it is the first universal interface layer designed for the Intelligence Era. It decouples logic from rendering, allowing a single Rust codebase to deploy natively to Linux, macOS, Windows, Web (WASM), VR, and Terminal (TUI) without modification.")
                    .body()
                    .height(Length::Shrink)
            )
            .width(Length::Fixed(if is_mobile { 340.0 } else { 800.0 }))
        )
        .push(
            Wrap::new()
                .spacing(10.0)
                .run_spacing(10.0)
                .push(platform_badge("LINUX", t))
                .push(platform_badge("MACOS", t))
                .push(platform_badge("WINDOWS", t))
                .push(platform_badge("WASM", t))
                .push(platform_badge("TUI", t))
                .push(platform_badge("VR/AR", t))
        );

    Container::new(content).padding(if is_mobile { 24.0 } else { 80.0 }).width(Length::Fill)
}

fn platform_badge(label: &str, t: ThemeTokens) -> Container<Message, IcedBackend> {
    Container::new(Text::new(label).size(12.0).bold().color(t.colors.primary))
        .padding(Padding::from([8, 16]))
        .border(1.0, t.colors.border)
        .radius(100.0)
}

fn pillars_section(context: &Context, is_mobile: bool) -> Container<Message, IcedBackend> {
    let content = VStack::new()
        .spacing(48.0)
        .width(Length::Fill)
        .push(section_header("The Technical Core", "Unified hierarchy for intelligence swarms.", is_mobile))
        .push(
            ResponsiveGrid::new()
                .spacing(32.0)
                .push(pillar_card("PeakOS", "Real-Time Orchestration", "Deterministic kernel for hardware coordination.", "cpu", context, Page::PeakOSDetail))
                .push(pillar_card("PeakUI", "Semantic Vision", "The world's first AI-native semantic interface.", "eye", context, Page::PeakUIDetail))
                .push(pillar_card("PeakDB", "Neural Memory", "Local-first vector storage for encrypted on-device RAG.", "database", context, Page::PeakDBDetail))
                .push(pillar_card("PeakRelay", "Distributed Spirit", "Peer-to-peer intelligence mesh for universal swarms.", "share-2", context, Page::PeakRelayDetail))
                .push(pillar_card("Peak Hub", "Swarm Command", "The dedicated dashboard for controlling the entire stack.", "activity", context, Page::PeakHubDetail)),
        );

    Container::new(content).padding(if is_mobile { 24.0 } else { 80.0 }).width(Length::Fill)
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
                .push(Text::new("Traditional Computer Vision processes millions of pixels per frame. PeakUI exposes the Semantic Tree directly to AI agents, eliminating expensive pixel-processing.").body().secondary())
        )
        .push(
            HStack::new()
                .spacing(12.0)
                .align_y(Alignment::Center)
                .push(Icon::new("leaf").size(24.0).color(t.colors.success))
                .push(Text::new("Sustainable industrial intelligence.").caption1().bold().color(t.colors.success))
        );

    Container::new(content).padding(if is_mobile { 24.0 } else { 80.0 }).width(Length::Fill)
}

fn industrial_api_section(_context: &Context, is_mobile: bool) -> Container<Message, IcedBackend> {
    let code = r#"// Native Agent Interface
let pressure = framework.get_state("pressure_gauge");
if pressure.value > 80.0 {
    btn_emergency.press();
}"#;

    let content = VStack::new()
        .spacing(32.0)
        .width(Length::Fill)
        .push(section_header("Every UI is an API", "No Cameras Required", is_mobile))
        .push(Text::new("An industrial robot does not need a camera to see the screen. With PeakUI's semantic state, the UI itself becomes a structured API.").body().secondary())
        .push(Container::new(CodeBlock::new(code).language("rust")).width(Length::Fill));

    Container::new(content).padding(if is_mobile { 24.0 } else { 80.0 }).width(Length::Fill)
}

fn verticals_section(context: &Context, is_mobile: bool) -> Container<Message, IcedBackend> {
    let grid = ResponsiveGrid::new()
        .spacing(32.0)
        .push(vertical_card("Energy", "Smart-grid orchestration and autonomous swarms.", "activity", context))
        .push(vertical_card("Defense", "Decentralized zero-trust tactical secure compute.", "shield", context))
        .push(vertical_card("Manufacturing", "Deterministic control for precision robotics.", "target", context));

    Container::new(VStack::new().spacing(48.0).width(Length::Fill).push(section_header("Industrial Verticals", "Universal tech for infrastructure.", is_mobile)).push(grid))
        .padding(if is_mobile { 24.0 } else { 80.0 }).width(Length::Fill)
}

fn safety_ledger_section(context: &Context, is_mobile: bool) -> Container<Message, IcedBackend> {
    let grid = ResponsiveGrid::new()
        .spacing(64.0)
        .push(blue_print_item("SAFE RUST Foundation", "100% memory safety. No zero-day exploits.", context))
        .push(blue_print_item("ZERO-TRUST Protocol", "Hardware root-of-trust encryption.", context));

    Container::new(VStack::new().spacing(48.0).width(Length::Fill).push(section_header("The Safety Ledger", "Verified industrial security.", is_mobile)).push(grid))
        .padding(if is_mobile { 24.0 } else { 80.0 }).width(Length::Fill)
}

fn section_header(title: &str, subtitle: &str, is_mobile: bool) -> VStack<Message, IcedBackend> {
    VStack::new().spacing(12.0).width(Length::Fill)
        .push(Text::new(title).size(if is_mobile { 32.0 } else { 48.0 }).bold())
        .push(Text::new(subtitle).secondary().size(if is_mobile { 16.0 } else { 20.0 }))
}

fn pillar_card(title: &str, sub: &str, desc: &str, icon: &str, context: &Context, page: Page) -> impl View<Message, IcedBackend> + 'static {
    let t = context.theme;
    let is_beta = title == "PeakOS" || title == "PeakRelay" || title == "PeakDB" || title == "Peak Hub";

    let mut header = HStack::new().align_y(Alignment::Center).width(Length::Fill);
    header = header.push(Icon::new(icon).size(24.0).color(t.colors.primary));

    if is_beta {
        header = header.push(Space::new(Length::Fill, Length::Shrink)).push(
            Container::new(Text::new("BETA").size(10.0).bold().color(t.colors.on_primary))
                .padding(Padding::from([2, 8])).background(t.colors.primary).radius(12.0)
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
                .width(Length::Fill)
                .push(Text::new(title).headline().bold())
                .push(Text::new(sub).caption1().secondary())
        )
        .push(Text::new(desc).body().secondary());

    Container::new(card_stack)
        .width(Length::Fill)
        .border(1.0, t.colors.border)
        .radius(if cfg!(target_arch = "wasm32") { 0.0 } else { 4.0 })
        .on_tap_gesture(Message::SetTab(page))
}

fn vertical_card(title: &str, desc: &str, icon: &str, context: &Context) -> Container<Message, IcedBackend> {
    let t = context.theme;
    let card = VStack::new().spacing(16.0).padding(20.0).width(Length::Fill)
        .push(Icon::new(icon).size(24.0).color(t.colors.primary))
        .push(Text::new(title).headline().bold())
        .push(Text::new(desc).body().secondary());

    Container::new(card).width(Length::Fill).border(1.0, t.colors.border)
        .radius(if cfg!(target_arch = "wasm32") { 0.0 } else { 4.0 })
}

fn blue_print_item(label: &str, desc: &str, context: &Context) -> HStack<Message, IcedBackend> {
    let t = context.theme;
    HStack::new().spacing(24.0).align_y(Alignment::Center).width(Length::Fill)
        .push(Rectangle::new(Length::Fixed(4.0), Length::Fixed(40.0)).color(t.colors.primary).radius(2.0))
        .push(VStack::new().push(Text::new(label).caption1().bold().secondary()).push(Text::new(desc).body()))
}

fn footer(_context: &Context, is_mobile: bool) -> Container<Message, IcedBackend> {
    let content = VStack::new().spacing(48.0).align_x(Alignment::Center).width(Length::Fill)
        .push(Divider::new())
        .push(HStack::new().spacing(32.0)
            .push(Text::new("Oslo").caption1().secondary())
            .push(Text::new("ERA 2026").caption1().secondary())
            .push(Text::new("BSL-1.1").caption1().secondary()));

    Container::new(content).padding(if is_mobile { 32.0 } else { 80.0 }).width(Length::Fill)
}
