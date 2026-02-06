use crate::core::{AIBackend, Backend, IcedBackend, SpatialBackend, TermBackend};
use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::{Message, RenderMode};
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(ctx: &Context, render_mode: RenderMode) -> PageResult<Message> {
    // --- 1. Preview Construction ---
    let preview_view = create_preview::<IcedBackend>();
    let terminal_preview = create_preview::<TermBackend>().view(ctx);
    let neural_preview = create_preview::<AIBackend>().view(ctx);
    let spatial_preview = create_preview::<SpatialBackend>().view(ctx);

    // --- 2. Code Snippet ---
    let code_snippet = "VStack::new()\n    .spacing(12.0)\n    .align_x(Alignment::Center)\n    .push(Text::new(\"Top\"))\n    .push(Text::new(\"Bottom\"))".to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "VStack",
        "A vertical stack layout component that arranges its children in a column with optional spacing and alignment.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Gravity and Flow\nVertical stacks are the backbone of list-based and form-based layouts. They provide a predictable downward flow of information.\n\n- **Spacing Management**: Decouples the distance between elements from the elements themselves.\n- **Alignment Control**: Allows sub-pixel precision in horizontal positioning (Start, Center, End).\n- **WASM Performance**: Highly optimized for browser layout engines while remaining portable to native backends."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.spacing(f32)` | `f32` | Distance between consecutive children. |\n| `.align_x(align)` | `Alignment` | Horizontal alignment of children. |\n| `.push(view)` | `View` | Adds a new child to the bottom of the stack. |"
    );

    PageResult::new(doc)
}

fn create_preview<B: Backend>() -> VStack<Message, B> {
    vstack::<Message, B>()
        .spacing(16.0)
        .push(text::<B>("Top Level Item").body().bold())
        .push(
            text::<B>("Supporting detail text underneath.")
                .body()
                .secondary(),
        )
        .push(
            hstack::<Message, B>()
                .spacing(8.0)
                .push(text::<B>("Group 1").caption2())
                .push(text::<B>("Group 2").caption2()),
        )
}
