use crate::core::{Backend, SpatialBackend};
use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::{ButtonLabState, Message, RenderMode};
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

fn create_button<B: Backend>(lab: &ButtonLabState) -> impl View<Message, B> {
    Button::<Message, B>::label(lab.label.clone())
        .variant(lab.variant)
        .intent(lab.intent)
        .size(lab.size)
        .width(if lab.is_full_width {
            Length::Fill
        } else {
            Length::Shrink
        })
        .on_press_maybe(if lab.is_disabled {
            None
        } else {
            Some(Message::CopyCode("Clicked!".to_string()))
        })
        .neural("lab-primary-action")
        .document("Primary action for the lab experiment")
}

pub fn view(
    base_context: &Context,
    lab: &ButtonLabState,
    render_mode: RenderMode,
) -> PageResult<Message> {
    let context = if lab.is_focused {
        base_context.clone().with_focus("button")
    } else {
        base_context.clone()
    };
    let context = &context;

    let code_snippet = generate_code(lab);

    // 1. Canvas View (Standard GUI)
    let button = create_button::<IcedBackend>(lab);
    let canvas_preview = vstack![button].width(if lab.is_full_width {
        Length::Fixed(400.0)
    } else {
        Length::Shrink
    });

    // 2. Terminal View (ANSI Text)
    let terminal_preview = create_button::<TermBackend>(lab).view(context);

    // 3. Neural View (Semantic JSON)
    let neural_preview = create_button::<AIBackend>(lab).view(context);

    // 4. Spatial View (3D transforms)
    let spatial_preview = create_button::<SpatialBackend>(lab).view(context);

    let doc = ComponentDoc::new(
        "Button",
        "A versatile button component with support for multiple variants, icons, and reactive states.",
        code_snippet,
        Arc::new(canvas_preview),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
        r#"
### Evolution of the Button
Buttons are the primary vehicle for user intent. In **PeakUI**, buttons follow a semantic model where **Intent** (Success, Danger, etc.) is decoupled from **Variant** (Solid, Outline, etc.).

- **Solid:** High emphasis. Used for primary actions.
- **Soft:** Medium emphasis. Subtle background for secondary actions.
- **Outline:** Low emphasis. Used for tertiary actions or within dense UI.
- **Ghost:** Minimal emphasis. Disappears until hover.
"#,
    )
    .props_table(
        r#"
| Modifier | Type | Description |
| :--- | :--- | :--- |
| `.label(text)` | `String` | Sets the button text. |
| `.variant(v)` | `Variant` | Solid, Soft, Outline, or Ghost. |
| `.intent(i)` | `Intent` | Primary, Success, Danger, etc. |
| `.size(s)` | `ControlSize` | Small, Medium, Large, or XLarge. |
| `.on_press(m)` | `Message` | Message to emit when clicked. |
| `.neural(tag)` | `&str` | Sets a stable AI-native semantic tag. |
"#,
    );

    PageResult::new(doc).inspector(ButtonInspector::new(lab))
}

fn generate_code(lab: &ButtonLabState) -> String {
    let mut code = format!("Button::label(\"{}\")", lab.label);

    if lab.variant != Variant::Solid {
        code.push_str(&format!("\n    .variant(Variant::{:?})", lab.variant));
    }

    if lab.intent != Intent::Primary {
        code.push_str(&format!("\n    .intent(Intent::{:?})", lab.intent));
    }

    if lab.size != ControlSize::Medium {
        code.push_str(&format!("\n    .size(ControlSize::{:?})", lab.size));
    }

    if lab.is_full_width {
        code.push_str("\n    .width(Length::Fill)");
    }

    if lab.is_disabled {
        code.push_str("\n    // No .on_press() means disabled");
    } else {
        code.push_str("\n    .on_press(Message::Action)");
    }

    code.push_str("\n    .neural(\"lab-primary-action\")");

    code
}

struct ButtonInspector {
    lab: ButtonLabState,
}

impl ButtonInspector {
    fn new(lab: &ButtonLabState) -> Self {
        Self { lab: lab.clone() }
    }
}

impl View<Message, IcedBackend> for ButtonInspector {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;

        ScrollView::new(
            VStack::new_generic()
                .spacing(24.0)
                .padding(Padding {
                    top: context.safe_area.top,
                    right: 20.0,
                    bottom: context.safe_area.bottom,
                    left: 20.0,
                })
                .push(
                    vstack![
                        text("Label").caption2().bold().secondary(),
                        TextInput::<Message>::new(self.lab.label.clone(), "Button label", |s| {
                            Message::UpdateButtonLabel(s)
                        }),
                    ]
                    .spacing(8.0),
                )
                .push(
                    vstack![
                        text("Variant").caption2().bold().secondary(),
                        SegmentedPicker::<Message, Theme>::new(
                            vec![
                                (
                                    "Solid".to_string(),
                                    Message::UpdateButtonVariant(Variant::Solid),
                                ),
                                (
                                    "Soft".to_string(),
                                    Message::UpdateButtonVariant(Variant::Soft),
                                ),
                                (
                                    "Outl".to_string(),
                                    Message::UpdateButtonVariant(Variant::Outline),
                                ),
                                (
                                    "Ghst".to_string(),
                                    Message::UpdateButtonVariant(Variant::Ghost),
                                ),
                                (
                                    "Cmpct".to_string(),
                                    Message::UpdateButtonVariant(Variant::Compact),
                                ),
                            ],
                            match self.lab.variant {
                                Variant::Solid => 0,
                                Variant::Soft => 1,
                                Variant::Outline => 2,
                                Variant::Ghost => 3,
                                Variant::Compact => 4,
                                Variant::Plain => 4,
                            },
                        )
                        .background_color(theme.colors.surface_variant)
                        .active_bg_color(theme.colors.primary.scale_alpha(0.8)),
                    ]
                    .spacing(12.0),
                )
                .push(
                    vstack![
                        text("Intent").caption2().bold().secondary(),
                        SegmentedPicker::<Message, Theme>::new(
                            vec![
                                (
                                    "Pri".to_string(),
                                    Message::UpdateButtonIntent(Intent::Primary),
                                ),
                                (
                                    "Suc".to_string(),
                                    Message::UpdateButtonIntent(Intent::Success),
                                ),
                                (
                                    "Wrn".to_string(),
                                    Message::UpdateButtonIntent(Intent::Warning),
                                ),
                                (
                                    "Dng".to_string(),
                                    Message::UpdateButtonIntent(Intent::Danger),
                                ),
                            ],
                            match self.lab.intent {
                                Intent::Primary => 0,
                                Intent::Success => 1,
                                Intent::Warning => 2,
                                Intent::Danger => 3,
                                _ => 0,
                            },
                        )
                        .background_color(theme.colors.surface_variant)
                        .active_bg_color(theme.colors.primary.scale_alpha(0.8)),
                    ]
                    .spacing(12.0),
                )
                .push(Divider::new())
                .push(
                    vstack![
                        Toggle::new("Full Width", self.lab.is_full_width, |b| {
                            Message::ToggleButtonFullWidth(b)
                        }),
                        Toggle::new("Disabled", self.lab.is_disabled, |b| {
                            Message::ToggleButtonDisabled(b)
                        }),
                        Toggle::new("Simulate Focus", self.lab.is_focused, |b| {
                            Message::ToggleButtonFocused(b)
                        })
                    ]
                    .spacing(16.0),
                ),
        )
        .view(context)
    }
}
