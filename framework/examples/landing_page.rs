use iced::{Alignment, Element, Length, Task, Theme};
use peak_core::registry::ShellMode;
use peak_theme::{ThemeTokens, ThemeTone};
use peak_ui::prelude::*;

pub fn main() -> iced::Result {
    iced::application(
        || (LandingPage::default(), Task::none()),
        LandingPage::update,
        LandingPage::view,
    )
    .title("PeakUI Landing Page")
    .theme(_theme)
    .run()
}

fn _theme(_: &LandingPage) -> Theme {
    Theme::Dark
}

struct LandingPage {
    input_value: f32,
}

#[derive(Debug, Clone)]
enum Message {
    SliderChanged(f32),
    NoOp,
}

impl Default for LandingPage {
    fn default() -> Self {
        Self { input_value: 50.0 }
    }
}

impl LandingPage {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SliderChanged(val) => {
                self.input_value = val;
                Task::none()
            }
            Message::NoOp => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // Enforce Dark, Industrial Vibe as requested
        let mode = ShellMode::Desktop;
        let tone = ThemeTone::Dark;
        let tokens = ThemeTokens::get(mode, tone);

        let input_val = self.input_value;

        responsive(mode, tokens, Localization::default(), move |context| {
            let t = context.theme;

            // --- Helper Text Styles ---
            let headline = |text: &'static str| {
                Text::new(text)
                    .large_title()
                    .bold()
                    .color(t.colors.text_primary)
            };
            let subhead =
                |text: &'static str| Text::new(text).title2().color(t.colors.text_secondary);
            let body = |text: &'static str| Text::new(text).body().color(t.colors.text_secondary);

            // --- Section 1: Hero ---
            let hero = VStack::new()
                .spacing(40.0)
                .align_x(Alignment::Center)
                .push(
                    HStack::new()
                        .spacing(32.0)
                        .push(Text::new("Stack").caption1().secondary())
                        .push(Text::new("Vision").caption1().secondary())
                        .push(Text::new("Architecture").caption1().secondary())
                        .push(Space::new(Length::Fixed(16.0), Length::Shrink))
                        .push(
                            Button::label("Launch")
                                .intent(Intent::Primary)
                                .on_press(Message::NoOp),
                        ),
                )
                .push(headline("The Multi-Kernel OS for Humans and Robots").center())
                .push(
                    subhead("Render to Pixels, Text, and Intelligence from a single codebase.")
                        .center(),
                );

            let metric_item = |label: &'static str, val: &'static str, sub: &'static str| {
                VStack::new()
                    .spacing(4.0)
                    .align_x(Alignment::Center)
                    .push(Text::new(label).size(10.0).bold().secondary())
                    .push(Text::new(val).size(16.0).bold())
                    .push(Text::new(sub).size(10.0).dim())
            };

            let metrics_bar = Container::new(
                VStack::new()
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
                    .push(
                        ResponsiveGrid::new()
                            .columns(4)
                            .spacing(64.0)
                            .push(metric_item("SAFETY", "100%", "Safe Rust"))
                            .push(metric_item("LATENCY", "0.8ms", "GPU Mesh"))
                            .push(metric_item("SOCIETY", "DECENTRALIZED", "Peer Relay"))
                            .push(metric_item("ORIGIN", "OSLO", "Norway 2026")),
                    ),
            )
            .padding(32.0)
            .width(Length::Fill)
            .background(t.colors.text_primary.scale_alpha(0.03));

            // --- Section 2: Technical Pillars (Green Math) ---
            let old_way = VStack::new()
                .spacing(24.0)
                .align_x(Alignment::Center)
                .push(Text::new("Visual AI (The Old Way)").title3().color(t.colors.danger))
                .push(body("Requires Computer Vision to process millions of pixels. High Latency. High Energy.").center())
                .push(Icon::new("cpu").size(64.0).color(t.colors.danger));

            let new_way = VStack::new()
                .spacing(24.0)
                .align_x(Alignment::Center)
                .push(Text::new("Semantic AI (PeakUI)").title3().color(t.colors.success))
                .push(body("Exposes the Semantic Node Tree directly to the Agent. 99% Energy Reduction. Zero Latency.").center())
                .push(Icon::new("leaf").size(64.0).color(t.colors.success));

            let pillars = ResponsiveGrid::new()
                .push(GlassCard::new(old_way).padding(32.0))
                .push(GlassCard::new(new_way).padding(32.0))
                .spacing(32.0);

            let technical_pillars = VStack::new()
                .spacing(48.0)
                .align_x(Alignment::Center)
                .push(headline("The Efficiency Equation"))
                .push(pillars);

            // --- Section 3: Every App is an API ---
            let api_intro = VStack::new()
                .spacing(16.0)
                .push(headline("Every App is an API"))
                .push(body("PeakUI isn't just for human eyes. It's a standardized protocol for machine-to-machine interaction."));

            let api_code = VStack::new()
                .push(CodeBlock::rust("let pressure = app.get_state(\"pressure\");\nif pressure > 100.0 {\n    app.trigger(\"shutdown\");\n}"));

            let api_section = ResponsiveGrid::new()
                .push(api_intro)
                .push(api_code)
                .spacing(48.0);

            // --- Section 4: Industrial Verticals ---
            let vertical_card = |title: &'static str, desc: &'static str, icon: &'static str| {
                GlassCard::new(
                    VStack::new()
                        .spacing(16.0)
                        .push(Icon::new(icon).size(32.0).color(t.colors.accent))
                        .push(Text::new(title).title3())
                        .push(body(desc)),
                )
                .padding(24.0)
            };

            let industrial_grid = ResponsiveGrid::new()
                .push(vertical_card(
                    "Oil & Gas",
                    "Predictive maintenance for remote pipelines via low-latency terminal feeds.",
                    "droplet",
                ))
                .push(vertical_card(
                    "Precision Robotics",
                    "High-frequency state management without visual overhead.",
                    "cpu",
                ))
                .push(vertical_card(
                    "Space Systems",
                    "Radiation-hardened UI logic that runs on minimal CPU cycles.",
                    "zap",
                ))
                .push(vertical_card(
                    "Smart Logistics",
                    "Seamless handoff between human dispatchers and autonomous fleets.",
                    "truck",
                ))
                .spacing(24.0);

            let industrial_verticals = VStack::new()
                .spacing(48.0)
                .align_x(Alignment::Center)
                .push(headline("Industrial Verticals"))
                .push(industrial_grid);

            // --- Section 5: Demo ---
            let badge_view = GlassCard::new(
                HStack::new()
                    .spacing(8.0)
                    .push(Icon::new("zap").size(16.0).color(t.colors.on_accent))
                    .push(
                        Text::new("AI Agent Active")
                            .caption1()
                            .color(t.colors.on_accent),
                    ),
            )
            .padding(8.0);

            let reactor_card = GlassCard::new(
                VStack::new()
                    .spacing(24.0)
                    .push(Text::new("Core Status: Stable").color(t.colors.success))
                    .push(Text::new(format!("Temperature: {:.1}°C", input_val)).title3())
                    .push(Slider::new(0.0..=100.0, input_val, Message::SliderChanged)),
            )
            .padding(32.0);

            let active_demo = VStack::new()
                .spacing(48.0)
                .align_x(Alignment::Center)
                .push(headline("Interactive Control"))
                .push(reactor_card.overlay(badge_view, Alignment::Center));

            // --- Assembly ---
            ScrollView::new(
                VStack::new()
                    .width(Length::Fill)
                    .padding(context.size.width.min(80.0))
                    .spacing(120.0)
                    .align_x(Alignment::Center)
                    .push(hero)
                    .push(metrics_bar)
                    .push(technical_pillars)
                    .push(api_section)
                    .push(industrial_verticals)
                    .push(active_demo),
            )
            .view(&context)
        })
    }
}
