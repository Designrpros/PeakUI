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
    let code_snippet = "VStack::new()\n    .push(Text::new(\"Top\"))\n    .push(Divider::new())\n    .push(Text::new(\"Bottom\"))".to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "Divider",
        "A simple visual separator for grouping content and defining logical boundaries in your layout.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Visual Boundaries\nDividers are essential for creating logical separation in dense UIs without adding unnecessary layout weight.\n\n- **Platform Adaptive**: In a Terminal, a divider manifests as a line of ASCII characters. On Canvas, it's a sub-pixel border.\n- **Depth Cues**: In Spatial environments, dividers can contribute to the z-index hierarchy, acting as subtle depth breaks."
    )
    .props_table(
        "| Modifier | Description |\n| :--- | :--- |\n| `.new()` | Creates a standard horizontal divider. |\n| `.is_vertical()` | (Planned) Switches to vertical orientation. |"
    );

    PageResult::new(doc)
}

fn create_preview<B: Backend>() -> VStack<Message, B> {
    vstack::<Message, B>()
        .spacing(16.0)
        .push(text::<B>("Item Above").body().secondary())
        .push(divider::<B>())
        .push(text::<B>("Item Below").body().secondary())
}
