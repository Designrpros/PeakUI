use peak_ui::core::{Context, TermBackend};
use peak_ui::prelude::*;
use peak_ui::prelude::{Button, Slider, Text, Toggle};

#[test]
fn test_button_atom() {
    let ctx = Context::default();
    let button = Button::<(), TermBackend>::new(Text::new("Click Me")).on_press(());

    let desc = button.describe(&ctx);
    assert_eq!(desc.role, "button".into());
    assert_eq!(desc.children.len(), 1);
    assert_eq!(desc.children[0].role, "text".into());
}

#[test]
fn test_toggle_atom() {
    let ctx = Context::default();
    let toggle = Toggle::<(), TermBackend>::new("Label", true, |_| ());

    let desc = toggle.describe(&ctx);
    assert_eq!(desc.role, "toggle".into());
    assert_eq!(desc.label, Some("Label".into()));
    assert_eq!(desc.content, Some("true".into()));
}

#[test]
fn test_slider_atom() {
    let ctx = Context::default();
    let slider = Slider::<(), TermBackend>::new(0.0..=100.0, 50.0, |_| ());

    let desc = slider.describe(&ctx);
    assert_eq!(desc.role, "slider".into());
    assert_eq!(desc.content, Some("50".into()));
}
