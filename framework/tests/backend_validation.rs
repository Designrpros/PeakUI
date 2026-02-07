use peak_ui::atoms::Text;
use peak_ui::core::{AIBackend, Backend, Context, IcedBackend, SpatialBackend, TermBackend};
use peak_ui::layout::VStack;
use peak_ui::prelude::*;

#[test]
fn test_semantic_consistency() {
    let ctx = Context::default();

    // Create a simple UI
    fn create_ui<B: Backend>() -> VStack<(), B> {
        VStack::new()
            .push(Text::new(std::borrow::Cow::Borrowed("Hello World")))
            .push(Text::new(std::borrow::Cow::Borrowed("Subtext")))
    }

    let iced_view = create_ui::<IcedBackend>();
    let term_view = create_ui::<TermBackend>();
    let ai_view = create_ui::<AIBackend>();
    let spatial_view = create_ui::<SpatialBackend>();

    let iced_desc = iced_view.describe(&ctx);
    let term_desc = term_view.describe(&ctx);
    let ai_desc = ai_view.describe(&ctx);
    let spatial_desc = spatial_view.describe(&ctx);

    // Verify that all backends produce semantically equivalent trees
    assert_eq!(iced_desc.role, std::borrow::Cow::Borrowed("vstack"));
    assert_eq!(iced_desc.children.len(), 2);

    assert_eq!(term_desc.role, std::borrow::Cow::Borrowed("vstack"));
    assert_eq!(term_desc.children.len(), 2);

    assert_eq!(ai_desc.role, std::borrow::Cow::Borrowed("vstack"));
    assert_eq!(ai_desc.children.len(), 2);

    assert_eq!(spatial_desc.role, std::borrow::Cow::Borrowed("vstack"));
    assert_eq!(spatial_desc.children.len(), 2);

    // Verify leaf node consistency
    assert_eq!(
        iced_desc.children[0].role,
        std::borrow::Cow::Borrowed("text")
    );
    assert_eq!(
        term_desc.children[0].role,
        std::borrow::Cow::Borrowed("text")
    );
    assert_eq!(ai_desc.children[0].role, std::borrow::Cow::Borrowed("text"));
    assert_eq!(
        spatial_desc.children[0].role,
        std::borrow::Cow::Borrowed("text")
    );
}

#[test]
fn test_semantic_serialization() {
    let ctx = Context::default();
    let ui = VStack::<(), TermBackend>::new().push(Text::new("PeakUI"));

    let desc = ui.describe(&ctx);
    let json = serde_json::to_string_pretty(&desc).unwrap();

    // Live Semantic Dump for the user
    println!(
        "\n=== LIVE SEMANTIC DUMP (BRAIN VIEW) ===\n{}\n======================================\n",
        json
    );

    // Verify essential structure is in JSON (using optimized short keys)
    assert!(json.contains("\"r\": \"vstack\""));
    assert!(json.contains("\"r\": \"text\""));
    assert!(json.contains("\"c\": \"PeakUI\"")); // Content for Text nodes is "c"
}
