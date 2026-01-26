use peak_core::registry::ShellMode;
use peak_theme::{ThemeTokens, ThemeTone};
use peak_ui::prelude::*;

fn main() {
    // Setup a basic context
    let mode = ShellMode::Desktop;
    let tokens = ThemeTokens::get(mode, ThemeTone::Dark);
    let context = Context::new(mode, tokens, Size::new(1024.0, 768.0));

    // Create a simple UI
    let ui = VStack::<(), AIBackend>::new_generic()
        .spacing(20.0)
        .padding(40.0)
        .push(Text::new("PeakOS AI Dashboard").large_title())
        .push(
            HStack::new_generic()
                .spacing(10.0)
                .push(Icon::new("settings"))
                .push(Text::new("System Settings").headline()),
        )
        .push(Divider::new())
        .push(
            ZStack::new_generic()
                .push(
                    Rectangle::new(Length::Fill, Length::Fixed(100.0))
                        .color(iced::Color::from_rgb(0.1, 0.1, 0.1)),
                )
                .push(Text::new("Overlay Text").center()),
        )
        .push(
            ResponsiveGrid::new_generic()
                .spacing(10.0)
                .push(
                    VStack::new_generic()
                        .push(Text::new("Metric 1").caption1())
                        .push(Text::new("128").title2()),
                )
                .push(
                    VStack::new_generic()
                        .push(Text::new("Metric 2").caption1())
                        .push(Text::new("512").title2()),
                )
                .push(
                    VStack::new_generic()
                        .push(Text::new("Metric 3").caption1())
                        .push(Text::new("1024").title2()),
                ),
        );

    // Render to semantic data
    let semantic_tree = ui.view(&context);

    // Print as JSON
    println!("--- SEMANTIC UI TREE ---");
    println!("{}", serde_json::to_string_pretty(&semantic_tree).unwrap());
    println!("------------------------");
}
