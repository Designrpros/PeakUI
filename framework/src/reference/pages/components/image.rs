use crate::core::{AIBackend, Backend, IcedBackend, SpatialBackend, TermBackend};

use crate::prelude::*;
use crate::reference::AppPageResult;
use crate::reference::app::{Message, RenderMode};
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(ctx: &Context, render_mode: RenderMode) -> AppPageResult {
    // --- 1. Preview Construction ---
    let preview_view = create_preview::<IcedBackend>();
    let terminal_preview = create_preview::<TermBackend>().view(ctx);
    let neural_preview = create_preview::<AIBackend>().view(ctx);
    let spatial_preview = create_preview::<SpatialBackend>().view(ctx);

    // --- 2. Code Snippet ---
    let code_snippet = "Image::new(\"assets/landscape.jpg\")\n    .width(Length::Fill)\n    .height(Length::Fixed(200.0))\n    .radius(12.0)".to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "Image",
        "A responsive image component that adapts to its container and supports light/dark alternate sources.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Multi-Kernel Imaging\nImages in PeakUI are more than just bitmapped data. They are responsive assets that adapt their representation based on the active kernel.\n\n- **Canvas (GUI)**: Renders high-resolution textures with optional hardware acceleration.\n- **Terminal (TUI)**: Automatically converts image color and luminance into high-density ASCII or ANSI block characters.\n- **Neural (AI)**: Provides semantic metadata (like ALT text or AI-generated descriptions) to the LLM instead of raw pixels."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.new(path)` | `&str` | Path to the image asset. |\n| `.width(len)` | `Length` | Horizontal sizing. |\n| `.height(len)` | `Length` | Vertical sizing. |\n| `.radius(r)` | `f32` | Corner rounding (where supported). |"
    );

    AppPageResult::new(doc)
}

fn create_preview<B: Backend>() -> VStack<Message, B> {
    vstack::<Message, B>()
        .spacing(16.0)
        .push(
            image::<B>("assets/mountain_sunset_warm.jpg")
                .width(Length::Fill)
                .height(Length::Fixed(240.0))
                .radius(16.0),
        )
        .push(
            text::<B>("Mountain Sunset (Standard)")
                .caption2()
                .secondary(),
        )
}
