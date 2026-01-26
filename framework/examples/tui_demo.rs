use iced::Size;
use peak_core::registry::ShellMode;
use peak_theme::ThemeTokens;
use peak_ui::catalog::Catalog;
use peak_ui::core::{Context, TermBackend};
use peak_ui::prelude::*;

fn main() {
    let tokens = ThemeTokens::get(ShellMode::Desktop, peak_theme::ThemeTone::Dark);
    let context = Context::new(ShellMode::Desktop, tokens, Size::new(800.0, 600.0));

    // Define a UI using the SAME components as the Desktop app
    // We use the full Catalog here to prove parity!
    let mut catalog = Catalog::<TermBackend>::new();

    // Simulate some navigation
    catalog.selected_id = Some("typography");

    // Render it!
    let output = catalog.view(&context);

    println!("{}", output);
}
