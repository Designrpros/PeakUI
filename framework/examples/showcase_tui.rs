use peak_ui::catalog::{Catalog, CatalogMessage};
use peak_ui::prelude::*;

fn main() {
    // Initialize the Catalog with the Terminal backend
    let mut catalog = Catalog::<TermBackend>::new();

    // Hardcode a mock Context for TUI rendering
    // In a real TUI runner, this would be updated based on terminal size
    let context = Context::new(
        peak_core::registry::ShellMode::Desktop,
        peak_theme::ThemeTokens::get(
            peak_core::registry::ShellMode::Desktop,
            peak_theme::ThemeTone::Dark,
        ),
        iced::Size::new(80.0, 24.0),
    );

    println!("{}", catalog.view(&context));

    // Simulate some interaction for the showcase
    println!("\n\n--- Simulating selection: Colors ---\n");
    let _ = catalog.update(CatalogMessage::ItemSelected("colors"));
    println!("{}", catalog.view(&context));

    println!("\n\n--- Simulating selection: Icons ---\n");
    let _ = catalog.update(CatalogMessage::ItemSelected("icons"));
    println!("{}", catalog.view(&context));
}
