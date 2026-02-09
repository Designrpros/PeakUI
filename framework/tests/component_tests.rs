use peak_ui::core::{Context, TermBackend};
use peak_ui::prelude::*;

#[test]
fn test_button_atom() {
    let ctx = Context::default();
    let button = Button::<(), TermBackend>::new(Text::new(std::borrow::Cow::Borrowed("Click Me")))
        .on_press(());

    let desc = button.describe(&ctx);
    assert_eq!(desc.role, std::borrow::Cow::Borrowed("button"));
    assert_eq!(desc.children.len(), 1);
    assert_eq!(desc.children[0].role, std::borrow::Cow::Borrowed("text"));
}

#[test]
fn test_toggle_atom() {
    let ctx = Context::default();
    let toggle = Toggle::<(), TermBackend>::new(std::borrow::Cow::Borrowed("Label"), true, |_| ());

    let desc = toggle.describe(&ctx);
    assert_eq!(desc.role, std::borrow::Cow::Borrowed("toggle"));
    assert_eq!(desc.label, Some(std::borrow::Cow::Borrowed("Label")));
    assert_eq!(desc.content, Some(std::borrow::Cow::Borrowed("on")));
}

#[test]
fn test_slider_atom() {
    let ctx = Context::default();
    let slider = Slider::<(), TermBackend>::new(0.0..=100.0, 50.0, |_| ());

    let desc = slider.describe(&ctx);
    assert_eq!(desc.role, std::borrow::Cow::Borrowed("slider"));
    assert_eq!(desc.content, Some(std::borrow::Cow::Borrowed("50")));
}
