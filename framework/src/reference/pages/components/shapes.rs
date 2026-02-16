use crate::prelude::*;
use crate::reference::app::{LabMessage, Message, RenderMode};
use crate::reference::views::ComponentDoc;
use crate::reference::AppPageResult;
use std::sync::Arc;

pub fn view(ctx: &Context, render_mode: RenderMode) -> AppPageResult {
    // --- 1. Preview Construction ---
    let preview_view = create_preview::<IcedBackend>(ctx);
    let terminal_preview = create_preview::<TermBackend>(ctx).view(ctx);
    let neural_preview = create_preview::<AIBackend>(ctx).view(ctx);
    let spatial_preview = create_preview::<SpatialBackend>(ctx).view(ctx);

    // --- 2. Code Snippet ---
    let code_snippet =
        "Circle::new(20.0).color(colors.primary)\nRectangle::new(100.0, 40.0).radius(8.0)"
            .to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "Shapes",
        "Primary geometric atoms that can be used for indicators, progress trackers, and custom UI elements.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::Lab(LabMessage::SetRenderMode(mode)))
    .theory(
       "### Geometric Foundation\nShapes are the lowest-level visual primitives in PeakUI. They allow for the construction of complex components without leaving the framework's semantic model.\n\n- **Performance**: Rendered directly by the kernel's drawing pipeline (CPU/GPU/TUI).\n- **Intent Driven**: Shapes easily adopt theme colors via the `.color()` modifier, supporting both raw values and semantic tokens."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `Circle::new(r)` | `f32` | Creates a circular element. |\n| `Rectangle::new(w, h)` | `Length` | Creates a rectangular surface. |\n| `.radius(r)` | `f32` | Adds rounded corners to rectangles. |\n| `.color(c)` | `Color` | Sets the fill color. |"
    );

    AppPageResult::new(doc)
}

fn create_preview<B: Backend>(ctx: &Context) -> VStack<Message, B> {
    vstack::<Message, B>()
        .spacing(32.0)
        .push(
            vstack::<Message, B>()
                .spacing(8.0)
                .push(text::<B>("Circles").caption2().secondary())
                .push(
                    hstack::<Message, B>()
                        .spacing(20.0)
                        .push(circle::<B>(10.0).color(ctx.theme.colors.primary))
                        .push(circle::<B>(20.0).color(ctx.theme.colors.success))
                        .push(circle::<B>(30.0).color(ctx.theme.colors.danger)),
                ),
        )
        .push(
            vstack::<Message, B>()
                .spacing(8.0)
                .push(text::<B>("Rectangles").caption2().secondary())
                .push(
                    hstack::<Message, B>()
                        .spacing(20.0)
                        .width(Length::Fill)
                        .push(
                            rectangle::<B>(60.0, 40.0)
                                .radius(8.0)
                                .color(ctx.theme.colors.surface_variant),
                        )
                        .push(
                            rectangle::<B>(Length::Fill, 40.0)
                                .radius(20.0)
                                .color(ctx.theme.colors.primary),
                        ),
                ),
        )
}
