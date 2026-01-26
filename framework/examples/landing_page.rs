use iced::{Alignment, Element, Length, Task, Theme};
use peak_core::registry::ShellMode;
use peak_theme::{ThemeTokens, ThemeTone};
use peak_ui::prelude::*;

pub fn main() -> iced::Result {
    iced::application(
        "PeakUI Landing Page",
        LandingPage::update,
        LandingPage::view,
    )
    .theme(|_| Theme::Dark) // Enforce Dark theme based on requirement
    .run()
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

    fn view(&self) -> Element<'_, Message, Theme, iced::Renderer> {
        // Enforce Dark, Industrial Vibe as requested
        let mode = ShellMode::Desktop;
        let tone = ThemeTone::Dark;
        let tokens = ThemeTokens::get(mode, tone);

        let input_val = self.input_value;

        responsive(mode, tokens, move |context| {
            let t = context.theme;

            // --- Helper Text Styles ---
            let headline = |text: &'static str| {
                Text::new(text)
                    .large_title()
                    .bold()
                    .color(t.colors.text_primary)
                    .center()
            };
            let subhead = |text: &'static str| {
                Text::new(text)
                    .title2()
                    .color(t.colors.text_secondary)
                    .center()
            };
            let body = |text: &'static str| Text::new(text).body().color(t.colors.text_secondary);

            // --- Section 1: Hero ---
            let hero = VStack::new()
                .spacing(24.0)
                .align_x(Alignment::Center)
                .push(headline("The Multi-Kernel OS for Humans and Robots"))
                .push(subhead("Render to Pixels (GPU), Text (TUI), and Intelligence (Neural) from a single codebase."))
                .push(
                    HStack::new()
                        .spacing(16.0)
                        .push(Button::label("Watch GUI Demo").intent(Intent::Primary).on_press(Message::NoOp))
                        .push(Button::label("Connect via SSH").variant(Variant::Soft).on_press(Message::NoOp))
                );

            // --- Section 2: Green Math ---
            // Left (Problem)
            let old_way = VStack::new()
                .spacing(16.0)
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .push(Text::new("Visual AI (The Old Way)").title3().color(t.colors.danger)) // Error/Danger color
                .push(body("Requires Computer Vision to process millions of pixels. High Latency. High Energy.").center())
                .push(Icon::new("cpu").size(64.0).color(t.colors.danger));

            // Right (Solution)
            let new_way = VStack::new()
                .spacing(16.0)
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .push(Text::new("Semantic AI (PeakUI)").title3().color(t.colors.success)) // Success color
                .push(body("Exposes the Semantic Node Tree directly to the Agent. 99% Energy Reduction. Zero Latency.").center())
                .push(Icon::new("leaf").size(64.0).color(t.colors.success));

            let green_math = GlassCard::new(
                HStack::new()
                    .spacing(48.0)
                    .push(old_way)
                    .push(
                        Rectangle::new(Length::Fixed(1.0), Length::Fixed(100.0))
                            .color(t.colors.border),
                    ) // Vertical divider simulation
                    .push(new_way),
            )
            .padding(32.0);

            // --- Section 3: Robot OS ---
            let robot_os = VStack::new()
                .spacing(24.0)
                .align_x(Alignment::Center)
                .push(headline("Every App is an API"))
                .push(body("Industrial robots don't need cameras. They query the PeakUI Multi-Kernel backend directly."))
                .push(
                     VStack::new() // Wrap in VStack to control width since CodeBlock doesn't support .width() directly
                        .width(Length::Fixed(600.0))
                        .push(CodeBlock::rust("let pressure = app.get_semantic_state(\"pressure_gauge\");"))
                );

            // --- Section 4: Demo ---
            // Overlay badge using GlassCard
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
            .width(Length::Fixed(500.0))
            .padding(32.0);

            let active_demo = reactor_card.overlay(badge_view, Alignment::Center);

            // --- Assembly ---
            ScrollView::new(
                VStack::new()
                    .width(Length::Fill)
                    .padding(80.0)
                    .spacing(120.0)
                    .align_x(Alignment::Center)
                    .push(hero)
                    .push(green_math)
                    .push(robot_os)
                    .push(active_demo),
            )
            .view(&context)
        })
    }
}
