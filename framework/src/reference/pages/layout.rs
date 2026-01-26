use super::super::app::{LayoutLabState, Message, RenderMode};
use super::super::page::PageResult;
use crate::atoms::Container;
use crate::core::{Backend, SpatialBackend};
use crate::prelude::*;

use super::super::views::ComponentDoc;
use std::sync::Arc;

pub fn view(
    base_context: &Context,
    _is_mobile: bool,
    lab: &LayoutLabState,
    render_mode: RenderMode,
) -> PageResult {
    let context = base_context;

    let code_snippet = generate_code(lab);

    // 1. Canvas View (Standard GUI)
    let canvas_preview = create_preview::<IcedBackend>(lab);

    // 2. Terminal View (ANSI Text)
    let terminal_preview = create_preview::<TermBackend>(lab).view(context);

    // 3. Neural View (Semantic JSON)
    let neural_preview = create_preview::<AIBackend>(lab).view(context);

    // 4. Spatial View (3D transforms)
    let spatial_preview = create_preview::<SpatialBackend>(lab).view(context);

    let doc = ComponentDoc::new(
        "Layout Engine",
        "Use VStack, HStack, and ZStack to compose complex, responsive layouts with ease.",
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
### Composition over Layout
**PeakUI** uses a declarative, flex-inspired layout engine. Instead of absolute positioning, you compose smaller components into larger stacks.

- **VStack:** Vertical arrangement of views.
- **HStack:** Horizontal arrangement of views.
- **ZStack:** Depth-based stacking (layering).
- **Alignment:** Control how children are positioned on the cross-axis.
- **Spacing:** Uniform gaps between children.
"#,
    )
    .props_table(
        r#"
| Modifier | Description |
| :--- | :--- |
| `.spacing(f32)` | Uniform gap between all immediate children. |
| `.padding(p)` | Inner margin for the stack. |
| `.align_x(a)` | Horizontal alignment (for VStack). |
| `.align_y(a)` | Vertical alignment (for HStack). |
| `.push(v)` | Adds a child view to the end of the stack. |
"#,
    );

    PageResult::new(doc).inspector(LayoutInspector::new(lab))
}

fn create_preview<B: Backend>(lab: &LayoutLabState) -> VStack<Message, B> {
    let mut children = Vec::new();

    for i in 0..lab.child_count {
        let color = match i % 3 {
            0 => Color::from_rgb(0.2, 0.6, 1.0), // Blue
            1 => Color::from_rgb(0.2, 0.8, 0.4), // Green
            _ => Color::from_rgb(1.0, 0.4, 0.4), // Red
        };

        children.push(
            Container::<Message, B>::new(Text::<B>::new(format!("{}", i + 1)).title3().bold())
                .padding(20)
                .width(Length::Fixed(60.0))
                .height(Length::Fixed(60.0))
                .background(color)
                .radius(12.0),
        );
    }

    VStack::new_generic()
        .spacing(lab.outer_spacing)
        .width(Length::Shrink)
        .push(
            VStack::new_generic()
                .spacing(12.0)
                .width(Length::Shrink)
                .push(Text::<B>::new("HStack (Horizontal)").caption2().secondary())
                .push(
                    HStack::new_generic()
                        .spacing(lab.inner_spacing)
                        .align_y(lab.alignment)
                        .extend(children.clone()),
                ),
        )
        .push(Divider::new())
        .push(
            VStack::new_generic()
                .spacing(12.0)
                .width(Length::Shrink)
                .push(Text::<B>::new("VStack (Vertical)").caption2().secondary())
                .push(
                    VStack::new_generic()
                        .spacing(lab.inner_spacing)
                        .align_x(lab.alignment)
                        .extend(children),
                ),
        )
}

fn generate_code(lab: &LayoutLabState) -> String {
    format!(
        "HStack::new()\n    .spacing({:.1})\n    .align_y(Alignment::{:?})\n    .push(...) // x{}",
        lab.inner_spacing, lab.alignment, lab.child_count
    )
}

struct LayoutInspector {
    lab: LayoutLabState,
}

impl LayoutInspector {
    fn new(lab: &LayoutLabState) -> Self {
        Self { lab: lab.clone() }
    }
}

impl View<Message, IcedBackend> for LayoutInspector {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        ScrollView::new(
            VStack::new_generic()
                .spacing(24.0)
                .padding(Padding {
                    top: 96.0,
                    right: 20.0,
                    bottom: 24.0,
                    left: 20.0,
                })
                .push(
                    VStack::new_generic()
                        .spacing(12.0)
                        .push(
                            Text::<IcedBackend>::new("Spacing")
                                .caption2()
                                .bold()
                                .secondary(),
                        )
                        .push(
                            HStack::new_generic()
                                .spacing(12.0)
                                .push(
                                    Slider::<Message, IcedBackend>::new(
                                        0.0..=64.0,
                                        self.lab.inner_spacing,
                                        |v| Message::UpdateLayoutInnerSpacing(v),
                                    )
                                    .width(Length::Fill),
                                )
                                .push(
                                    Text::<IcedBackend>::new(format!(
                                        "{:.0}",
                                        self.lab.inner_spacing
                                    ))
                                    .caption2()
                                    .secondary(),
                                ),
                        ),
                )
                .push(
                    VStack::new_generic()
                        .spacing(12.0)
                        .push(
                            Text::<IcedBackend>::new("Child Count")
                                .caption2()
                                .bold()
                                .secondary(),
                        )
                        .push(
                            SegmentedPicker::<Message, Theme>::new(
                                vec![
                                    ("1".to_string(), Message::UpdateLayoutChildCount(1)),
                                    ("3".to_string(), Message::UpdateLayoutChildCount(3)),
                                    ("5".to_string(), Message::UpdateLayoutChildCount(5)),
                                    ("8".to_string(), Message::UpdateLayoutChildCount(8)),
                                ],
                                match self.lab.child_count {
                                    1 => 0,
                                    3 => 1,
                                    5 => 2,
                                    8 => 3,
                                    _ => 1,
                                },
                            )
                            .background_color(
                                context.theme.colors.surface_variant.scale_alpha(0.5),
                            ),
                        ),
                )
                .push(
                    VStack::new_generic()
                        .spacing(12.0)
                        .push(
                            Text::<IcedBackend>::new("Alignment")
                                .caption2()
                                .bold()
                                .secondary(),
                        )
                        .push(
                            SegmentedPicker::<Message, Theme>::new(
                                vec![
                                    (
                                        "Start".to_string(),
                                        Message::UpdateLayoutAlignment(Alignment::Start),
                                    ),
                                    (
                                        "Center".to_string(),
                                        Message::UpdateLayoutAlignment(Alignment::Center),
                                    ),
                                    (
                                        "End".to_string(),
                                        Message::UpdateLayoutAlignment(Alignment::End),
                                    ),
                                ],
                                match self.lab.alignment {
                                    Alignment::Start => 0,
                                    Alignment::Center => 1,
                                    Alignment::End => 2,
                                },
                            )
                            .background_color(
                                context.theme.colors.surface_variant.scale_alpha(0.5),
                            ),
                        ),
                ),
        )
        .view(context)
    }
}
