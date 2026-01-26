use super::super::app::{Message, RenderMode, SizingLabState, SizingType};
use super::super::page::PageResult;
use crate::core::{Backend, SpatialBackend};
use crate::prelude::*;

use super::super::views::ComponentDoc;
use std::sync::Arc;

pub fn view(
    context: &Context,
    _is_mobile: bool,
    lab: &SizingLabState,
    render_mode: RenderMode,
) -> PageResult {
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
        "Basic Sizing",
        "Control element dimensions with fixed, fill, or relative sizing units.",
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
### Sizing Units
**PeakUI** provides three primary ways to control size:

- **Fixed:** Explicit pixel values. Best for icons or small decorative elements.
- **Fill:** Occupy all remaining space in the container. Best for main content areas.
- **Shrink:** Only take up the space required by children. Best for labels and button containers.
"#,
    )
    .props_table(
        r#"
| Unit | Description |
| :--- | :--- |
| `Length::Fixed(f32)` | Absolute size in logical pixels. |
| `Length::Fill` | Dynamic expansion to fill parent. |
| `Length::Shrink` | Dynamic contraction to fit content. |
| `.width(Length)` | Set horizontal constraint. |
| `.height(Length)` | Set vertical constraint. |
"#,
    );

    PageResult::new(doc).inspector(SizingInspector::new(lab))
}

fn create_preview<B: Backend>(lab: &SizingLabState) -> VStack<Message, B> {
    let lab = lab.clone();
    let width = match lab.width_type {
        SizingType::Fixed => Length::Fixed(lab.fixed_width),
        SizingType::Fill => Length::Fill,
        SizingType::Shrink => Length::Shrink,
    };

    let height = match lab.height_type {
        SizingType::Fixed => Length::Fixed(lab.fixed_height),
        SizingType::Fill => Length::Fill,
        SizingType::Shrink => Length::Shrink,
    };

    VStack::new_generic()
        .spacing(32.0)
        .width(Length::Shrink)
        .push(ProxyView::new(move |ctx| {
            VStack::new_generic()
                .spacing(12.0)
                .width(Length::Shrink)
                .push(
                    Text::<B>::new(format!(
                        "Preview: {:?} x {:?}",
                        lab.width_type, lab.height_type
                    ))
                    .caption2()
                    .secondary(),
                )
                .push(
                    Rectangle::<B>::new(width, height)
                        .color(ctx.theme.colors.primary)
                        .radius(8.0),
                )
                .view(ctx)
        }))
}

fn generate_code(lab: &SizingLabState) -> String {
    let w = match lab.width_type {
        SizingType::Fixed => format!("Length::Fixed({:.1})", lab.fixed_width),
        SizingType::Fill => "Length::Fill".to_string(),
        SizingType::Shrink => "Length::Shrink".to_string(),
    };
    let h = match lab.height_type {
        SizingType::Fixed => format!("Length::Fixed({:.1})", lab.fixed_height),
        SizingType::Fill => "Length::Fill".to_string(),
        SizingType::Shrink => "Length::Shrink".to_string(),
    };

    format!("container(content)\n    .width({})\n    .height({})", w, h)
}

struct SizingInspector {
    lab: SizingLabState,
}

impl SizingInspector {
    fn new(lab: &SizingLabState) -> Self {
        Self { lab: lab.clone() }
    }
}

impl View<Message, IcedBackend> for SizingInspector {
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
                            Text::<IcedBackend>::new("Width Type")
                                .caption2()
                                .bold()
                                .secondary(),
                        )
                        .push(
                            SegmentedPicker::<Message, Theme>::new(
                                vec![
                                    (
                                        "Fixed".to_string(),
                                        Message::UpdateSizingWidthType(SizingType::Fixed),
                                    ),
                                    (
                                        "Fill".to_string(),
                                        Message::UpdateSizingWidthType(SizingType::Fill),
                                    ),
                                    (
                                        "Shrink".to_string(),
                                        Message::UpdateSizingWidthType(SizingType::Shrink),
                                    ),
                                ],
                                match self.lab.width_type {
                                    SizingType::Fixed => 0,
                                    SizingType::Fill => 1,
                                    SizingType::Shrink => 2,
                                },
                            )
                            .background_color(
                                context.theme.colors.surface_variant.scale_alpha(0.5),
                            ),
                        ),
                )
                .push(if self.lab.width_type == SizingType::Fixed {
                    VStack::new_generic()
                        .spacing(12.0)
                        .push(
                            Text::<IcedBackend>::new("Fixed Width")
                                .caption2()
                                .bold()
                                .secondary(),
                        )
                        .push(
                            Slider::<Message, IcedBackend>::new(
                                0.0..=600.0,
                                self.lab.fixed_width,
                                |v| Message::UpdateSizingFixedWidth(v),
                            )
                            .width(Length::Fill),
                        )
                } else {
                    VStack::new_generic()
                })
                .push(Divider::new())
                .push(
                    VStack::new_generic()
                        .spacing(12.0)
                        .push(
                            Text::<IcedBackend>::new("Height Type")
                                .caption2()
                                .bold()
                                .secondary(),
                        )
                        .push(
                            SegmentedPicker::<Message, Theme>::new(
                                vec![
                                    (
                                        "Fixed".to_string(),
                                        Message::UpdateSizingHeightType(SizingType::Fixed),
                                    ),
                                    (
                                        "Fill".to_string(),
                                        Message::UpdateSizingHeightType(SizingType::Fill),
                                    ),
                                    (
                                        "Shrink".to_string(),
                                        Message::UpdateSizingHeightType(SizingType::Shrink),
                                    ),
                                ],
                                match self.lab.height_type {
                                    SizingType::Fixed => 0,
                                    SizingType::Fill => 1,
                                    SizingType::Shrink => 2,
                                },
                            )
                            .background_color(
                                context.theme.colors.surface_variant.scale_alpha(0.5),
                            ),
                        ),
                )
                .push(if self.lab.height_type == SizingType::Fixed {
                    VStack::new_generic()
                        .spacing(12.0)
                        .push(
                            Text::<IcedBackend>::new("Fixed Height")
                                .caption2()
                                .bold()
                                .secondary(),
                        )
                        .push(
                            Slider::<Message, IcedBackend>::new(
                                0.0..=400.0,
                                self.lab.fixed_height,
                                |v| Message::UpdateSizingFixedHeight(v),
                            )
                            .width(Length::Fill),
                        )
                } else {
                    VStack::new_generic()
                }),
        )
        .view(context)
    }
}
