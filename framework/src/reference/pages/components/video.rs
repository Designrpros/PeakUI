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
    let code_snippet = "Video::new(\"assets/teaser.mp4\")\n    .width(Length::Fill)\n    .height(Length::Fixed(300.0))\n    .radius(16.0)".to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "Video",
        "A high-fidelity video component for media playback within the Peak Swarm.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Cinematic Motion\nVideo components in PeakUI are designed for ambient motion and high-fidelity media presentation.\n\n- **Kernel Playback**: While Canvas uses native media layers, Terminal backends may render frames as high-speed ASCII animation (experimental).\n- **Neural Interaction**: Agents can 'watch' video streams by receiving semantic frame descriptions and timeline metadata.\n- **Spatial Audio**: In 3D kernels, video sound is spatially localized to the screen's position in the virtual room."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.new(path)` | `&str` | Path to the video asset. |\n| `.width(len)` | `Length` | Sizing of the video player. |\n| `.radius(r)` | `f32` | Corner rounding for the media layer. |"
    );

    AppPageResult::new(doc)
}

fn create_preview<B: Backend>() -> VStack<Message, B> {
    vstack::<Message, B>().spacing(16.0).push(
        video::<B>("pending")
            .width(Length::Fill)
            .height(Length::Fixed(240.0))
            .radius(16.0),
    )
}
