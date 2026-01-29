use super::super::app::Message;
use crate::prelude::*;
use iced::{Alignment, Color, Length, Padding, Shadow, Vector};

pub fn view(context: &Context) -> Element<'static, Message, Theme, Renderer> {
    let t = context.theme;
    let is_mobile = context.is_slim();

    // --- Sections ---

    // 1. The Hero: Impactful & Minimal
    let hero = VStack::new()
        .spacing(24.0)
        .padding(Padding {
            top: 120.0,
            bottom: 80.0,
            ..Default::default()
        })
        .align_x(Alignment::Center)
        .push(Text::new("PEAKUI").size(14.0).bold().secondary())
        .push(
            Text::new("The Interface of Intelligence.")
                .size(64.0)
                .bold()
                .width(if is_mobile {
                    Length::Fill
                } else {
                    Length::Fixed(800.0)
                })
                .align_center(),
        )
        .push(
            Text::new("Don't just build apps. \nBuild Eyes and Hands for the AI Era.")
                .size(20.0)
                .secondary()
                .align_center()
                .width(if is_mobile {
                    Length::Fill
                } else {
                    Length::Fixed(600.0)
                }),
        )
        .push(
            HStack::new()
                .spacing(16.0)
                .push(
                    Button::label("Join the Swarm")
                        .variant(Variant::Solid)
                        .on_press(Message::EnterApp),
                )
                .push(
                    Button::label("View Docs")
                        .variant(Variant::Outline)
                        .on_press(Message::EnterApp),
                ),
        );

    // 2. The Semantic Edge: AI Vision Demo
    let semantic_demo = HStack::new()
        .spacing(48.0)
        .width(Length::Fill)
        .padding(64.0)
        .align_y(Alignment::Center)
        .push(
            // Left Side: The Visual UI
            GlassCard::new(
                VStack::new()
                    .spacing(24.0)
                    .push(Text::new("Control Panel").headline().bold())
                    .push(
                        VStack::new()
                            .spacing(12.0)
                            .push(
                                HStack::new()
                                    .spacing(12.0)
                                    .push(Icon::new("zap").size(20.0).color(t.colors.primary))
                                    .push(Text::new("Node Power").body().bold())
                            )
                            .push(
                                Container::new(
                                    Rectangle::new(Length::Fill, Length::Fixed(8.0))
                                        .color(t.colors.primary)
                                        .radius(4.0)
                                )
                                .background(t.colors.surface)
                                .radius(4.0)
                            )
                    )
                    .push(
                        Button::label("Emergency Shutdown")
                            .intent(Intent::Danger)
                            .width(Length::Fill)
                            .on_press(Message::EnterApp)
                    )
            )
            .width(Length::Fixed(400.0))
            .padding(24.0)
        )
        .push(
            // Right Side: The AI's Perspective (Semantic Tree)
            VStack::new()
                .spacing(20.0)
                .width(Length::Fill)
                .push(Text::new("How the AI sees it").title3().bold().secondary())
                .push(
                    CodeBlock::rust(r#"SemanticNode {
    role: "control_panel",
    children: [
        { role: "metric", label: "Node Power", value: "98%" },
        { role: "action", label: "Emergency Shutdown", intent: "Danger" }
    ]
}"#)
                )
                .push(
                    Text::new("Native Semantic Reasoning")
                        .body()
                        .bold()
                )
                .push(
                    Text::new("PeakUI doesn't rely on slow OCR or pixel recognition. Every component is natively aware of its purpose, providing 100% deterministic context to LLMs.")
                        .secondary()
                )
        );

    // --- Assembly ---
    let main_content = VStack::new()
        .width(Length::Fill)
        .spacing(120.0)
        .push(hero)
        .push(semantic_demo)
        .push(suite_stack(&context))
        .push(norway_pitch(&context))
        .push(tech_deep_dive(&context))
        .push(footer(&context));

    ScrollView::new(
        VStack::new()
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .push(main_content),
    )
    .view(context)
}

// --- Specialized Sections ---

fn suite_stack(context: &Context) -> VStack<Message, IcedBackend> {
    let _t = context.theme;
    VStack::new()
        .spacing(64.0)
        .padding(64.0)
        .align_x(Alignment::Center)
        .push(
            VStack::new()
                .spacing(16.0)
                .align_x(Alignment::Center)
                .push(Text::new("A Complete Ecosystem").title2().bold())
                .push(Text::new("Built from the ground up for industrial autonomy.").secondary()),
        )
        .push(
            HStack::new()
                .spacing(32.0)
                .push(stack_card(
                    "PeakOS",
                    "The Core",
                    "Deterministic real-time kernel for device orchestration.",
                    "cpu",
                    context,
                ))
                .push(stack_card(
                    "PeakUI",
                    "The Vision",
                    "Semantic hardware-accelerated interface framework.",
                    "layout",
                    context,
                ))
                .push(stack_card(
                    "PeakDB",
                    "The Memory",
                    "Neural vector store with local-first encryption.",
                    "database",
                    context,
                ))
                .push(stack_card(
                    "PeakRelay",
                    "The Spirit",
                    "Peer-to-peer relay for decentralized swarms.",
                    "share-2",
                    context,
                )),
        )
}

fn norway_pitch(context: &Context) -> Container<Message, IcedBackend> {
    let t = context.theme;
    VStack::new()
        .spacing(48.0)
        .padding(80.0)
        .align_x(Alignment::Center)
        .push(
            VStack::new()
                .spacing(24.0)
                .align_x(Alignment::Center)
                .push(Text::new("Sovereign Intelligence").title1().bold())
                .push(
                    Text::new("A Norwegian technical framework for global industry.")
                        .title3()
                        .secondary()
                )
        )
        .push(
            VStack::new()
                .spacing(32.0)
                .width(Length::Fixed(800.0))
                .push(
                    HStack::new()
                        .spacing(24.0)
                        .push(Icon::new("shield-check").size(32.0).color(t.colors.success))
                        .push(
                            VStack::new()
                                .push(Text::new("Zero-Trust Privacy").headline().bold())
                                .push(Text::new("All data stays within the sovereign boundary. No cloud dependency, no unauthorized leaks.").secondary())
                        )
                )
                .push(
                    HStack::new()
                        .spacing(24.0)
                        .push(Icon::new("leaf").size(32.0).color(t.colors.success))
                        .push(
                            VStack::new()
                                .push(Text::new("Green AI: Efficiency by Design").headline().bold())
                                .push(Text::new("Native performance reduces computational overhead by 70%, drastically lowering the carbon footprint of AI operations.").secondary())
                        )
                )
                .push(
                    HStack::new()
                        .spacing(24.0)
                        .push(Icon::new("flag").size(32.0).color(t.colors.success))
                        .push(
                            VStack::new()
                                .push(Text::new("Norwegian Innovation").headline().bold())
                                .push(Text::new("Built to empower Norwegian startups and industry leaders with world-class tools for the fourth industrial revolution.").secondary())
                        )
                )
        )
        .background(t.colors.text_primary.scale_alpha(0.02))
        .corner_radius(24.0)
}

fn tech_deep_dive(_context: &Context) -> VStack<Message, IcedBackend> {
    VStack::new()
        .spacing(64.0)
        .padding(Padding { top: 120.0, bottom: 200.0, ..Default::default() })
        .align_x(Alignment::Center)
        .push(Text::new("The Architecture of 2026").title2().bold())
        .push(
            VStack::new()
                .spacing(40.0)
                .width(Length::Fixed(800.0))
                .push(
                    VStack::new()
                        .spacing(8.0)
                        .push(Text::new("MEMORY SAFETY").caption1().bold().secondary())
                        .push(Text::new("Built entirely in 100% safe Rust. No zero-day pointer exploits. Total thread safety. Guaranteed reliability for mission-critical deployments.").body())
                )
                .push(
                    VStack::new()
                        .spacing(8.0)
                        .push(Text::new("HARDWARE COMPATIBILITY").caption1().bold().secondary())
                        .push(Text::new("Runs natively on Linux, MacOS, Windows, Android, and iOS. Optimized for ARM64 and x86_64 architectures with SIMD acceleration.").body())
                )
                .push(
                    VStack::new()
                        .spacing(8.0)
                        .push(Text::new("AI INTEROPERABILITY").caption1().bold().secondary())
                        .push(Text::new("Standardized Semantic API allows any LLM (Llama 3, Claude 3, GPT-4) to control the UI with zero fine-tuning required.").body())
                )
        )
}

fn stack_card(
    title: &str,
    subtitle: &str,
    desc: &str,
    icon: &str,
    context: &Context,
) -> Container<Message, IcedBackend> {
    let t = context.theme;
    VStack::new()
        .spacing(16.0)
        .padding(24.0)
        .push(Icon::new(icon).size(24.0).color(t.colors.primary))
        .push(
            VStack::new()
                .spacing(4.0)
                .push(Text::new(title).headline().bold())
                .push(Text::new(subtitle).caption1().secondary()),
        )
        .push(Text::new(desc).caption2().secondary())
        .width(Length::Fixed(240.0))
        .background(t.colors.surface)
        .border(1.0, t.colors.border.scale_alpha(0.1))
        .corner_radius(16.0)
        .shadow(Shadow {
            color: Color {
                a: 0.05,
                ..Color::BLACK
            },
            offset: Vector::new(0.0, 4.0),
            blur_radius: 12.0,
        })
}

fn footer(_context: &Context) -> VStack<Message, IcedBackend> {
    VStack::new()
        .spacing(48.0)
        .padding(Padding {
            top: 80.0,
            bottom: 80.0,
            ..Default::default()
        })
        .align_x(Alignment::Center)
        .push(Divider::new())
        .push(Text::new("PeakSuite © 2026").caption1().secondary())
}
