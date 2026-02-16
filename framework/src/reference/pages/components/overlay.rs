use crate::core::{AIBackend, Backend, IcedBackend, SpatialBackend, TermBackend};

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
    let code_snippet = "let base = Rectangle::new(64.0, 64.0);\nbase.overlay(\n    Circle::new(8.0).color(colors.success), \n    Alignment::End\n)".to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "Overlay",
        "A layout modifier that places a view on top of another base view, constrained by the base's bounds.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::Lab(LabMessage::SetRenderMode(mode)))
    .theory(
       "### Layered Composition\nOverlays allow for Z-axis composition without the complexity of a ZStack. They are ideal for status indicators, badges, and contextual decorations.\n\n- **Relative Positioning**: The overlay content is positioned relative to the base view's coordinate space.\n- **Clipping**: Content is generally constrained to the base view's bounding box."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.overlay(view, align)` | `View, Alignment` | Places a view on top of the base. |\n| `Alignment`| `Alignment` | Corners (TopStart, End, etc.) or Center. |"
    );

    AppPageResult::new(doc)
}

fn create_preview<B: Backend>(ctx: &Context) -> VStack<Message, B> {
    let colors = &ctx.theme.colors;
    vstack::<Message, B>().spacing(32.0).push(
        vstack::<Message, B>()
            .spacing(8.0)
            .push(text::<B>("Status Indicator").caption2().secondary())
            .push(
                rectangle::<B>(64.0f32, 64.0f32)
                    .color(colors.surface_variant)
                    .radius(12.0)
                    .overlay(circle::<B>(8.0).color(colors.success), Alignment::End),
            ),
    )
}
