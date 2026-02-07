use peak_ui::prelude::*;
use peak_ui::reference::app::AIProviderChoice;
use peak_ui::reference::pages::settings::ai;

fn main() {
    let ctx = Context::default();

    // We want to see how the AI settings page describes itself
    let page_result = ai::view(
        &ctx,
        false, // is_mobile
        "sk-test-key".to_string(),
        AIProviderChoice::Ollama,
        false, // enable_exposure
        None,  // state_json
    );

    // PageResult contains a boxed View. We call describe() on it.
    let desc = page_result.view.describe(&ctx);

    // Verify icon loading
    println!("=== ICON LOADING CHECK ===");
    if let Some(svg) = peak_icons::get_icon("brain") {
        println!("Icon 'brain' found! SVG length: {}", svg.len());
    } else {
        println!("Icon 'brain' NOT FOUND!");
    }
    println!("==========================");
}
