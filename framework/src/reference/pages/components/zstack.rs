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
    let code_snippet = "ZStack::new()\n    .push(Rectangle::new(Length::Fill, Length::Fill).color(Color::BLACK))\n    .push(Text::new(\"Layered Content\"))".to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "ZStack",
        "A depth-based stack layout component that layers its children on top of each other.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Depth and Layering\nZStacks allow for complex visual compositions where elements overlap, such as backgrounds with text overlays or custom controls.\n\n- **Painting Order**: Children are rendered in the order they are pushed; first added is at the bottom.\n- **Spatial Depth**: In 3D backends, ZStack layers are automatically translated along the Z-axis to prevent flickering (z-fighting).\n- **Neural Transparency**: AI agents understand that ZStack children are visually connected but semantically distinct layers."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.push(view)` | `View` | Adds a new layer on top of the stack. |"
    );

    PageResult::new(doc)
}

fn create_preview<B: Backend>() -> ZStack<Message, B> {
    zstack::<Message, B>()
        .push(
            rectangle::<B>(200.0f32, 120.0f32)
                .color(Color::from_rgb(0.1, 0.1, 0.1))
                .radius(12.0),
        )
        .push(
            vstack::<Message, B>()
                .push(text::<B>("Layered Content").body().bold().primary())
                .push(text::<B>("Visual Depth").caption2().secondary())
                .padding(16),
        )
}
