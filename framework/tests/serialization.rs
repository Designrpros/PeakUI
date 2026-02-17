use peak_ui::reference::views::state::ViewState;

#[test]
fn test_view_state_serialization() {
    let app = peak_ui::reference::app::App::default();
    let state = ViewState::new(&app);

    // Test serialization to JSON
    let json = serde_json::to_string(&state).expect("Failed to serialize ViewState");

    println!("Serialized ViewState: {}", json);

    // Basic assertions
    assert!(json.contains("shell"));
    assert!(json.contains("intelligence"));
    assert!(json.contains("labs"));
    assert!(json.contains("interaction"));

    // Test deserialization back
    let deserialized: ViewState =
        serde_json::from_str(&json).expect("Failed to deserialize ViewState");

    assert_eq!(state.shell.active_tab, deserialized.shell.active_tab);
    assert_eq!(
        state.intelligence.is_thinking,
        deserialized.intelligence.is_thinking
    );
}
