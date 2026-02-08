use crate::core::{AIBackend, Backend, IcedBackend, SpatialBackend, TermBackend};
use crate::engine::navigation::PageResult;
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
    let code_snippet = "WebView::new(\"https://google.com\")\n    .width(Length::Fill)\n    .height(Length::Fixed(400.0))".to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "WebView",
        "A cross-platform web component for rendering web content, iframes, and web-based video players.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Interop & Web Standards\nWebView provides a bridge between the native PeakUI environment and web-standard content. It is essential for complex embeddings and legacy system integration.\n\n- **Backend Independence**: While primarily a Canvas feature, other kernels treat WebViews as linked resources or semantic containers.\n- **Sandbox Security**: Integrated with PeakOS security layers to ensure web content is isolated from core system memory (Planned)."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.new(url)` | `&str` | Loads the specified URL. |\n| `.radius(r)` | `f32` | Corner rounding for the web surface. |\n| `.width(len)` | `Length` | Sizing for the container. |"
    );

    PageResult::new(doc)
}

fn create_preview<B: Backend>() -> VStack<Message, B> {
    vstack::<Message, B>().spacing(16.0).push(
        vstack::<Message, B>()
            .spacing(8.0)
            .push(text::<B>("Sample Render").caption2().secondary())
            .push(
                web_view::<B>("https://www.youtube.com/embed/0pIyIMqwu0E")
                    .width(Length::Fill)
                    .height(Length::Fixed(300.0))
                    .radius(16.0),
            ),
    )
}
