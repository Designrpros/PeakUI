use crate::core::{AIBackend, Backend, IcedBackend, SpatialBackend, TermBackend};

use crate::prelude::*;
use crate::reference::app::{LabMessage, Message, RenderMode};
use crate::reference::views::ComponentDoc;
use crate::reference::AppPageResult;
use std::sync::Arc;

pub fn view(ctx: &Context, render_mode: RenderMode) -> AppPageResult {
    // --- 1. Preview Construction ---
    let preview_view = create_preview::<IcedBackend>();
    let terminal_preview = create_preview::<TermBackend>().view(ctx);
    let neural_preview = create_preview::<AIBackend>().view(ctx);
    let spatial_preview = create_preview::<SpatialBackend>().view(ctx);

    // --- 2. Code Snippet ---
    let code_snippet = "ScrollView::new(\n    VStack::new()\n        .push(long_content)\n)\n.direction(ScrollDirection::Vertical)".to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "ScrollView",
        "A container that enables scrolling for content that exceeds the available screen space, supporting both vertical and horizontal directions.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::Lab(LabMessage::SetRenderMode(mode)))
    .theory(
       "### Handling Overflow\nScroll views provide access to infinite content within a finite viewport. They are critical for mobile-first and dense data interfaces.\n\n- **Virtualization Support**: Planned optimization for ultra-long lists to keep memory usage low.\n- **Gesture Responsive**: Automatically handles touch and wheel events across all supported backends.\n- **Spatial Navigation**: In 3D kernels, scroll views can manifest as sliding panels or volumetric carousels."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.new(content)` | `View` | Creates a scrollable container around the content. |\n| `.direction(dir)` | `ScrollDirection` | Vertical, Horizontal, or Both. |\n| `.width(len)` | `Length` | Sets the width of the scrollable area. |"
    );

    AppPageResult::new(doc)
}

fn create_preview<B: Backend>() -> VStack<Message, B> {
    vstack::<Message, B>()
        .spacing(12.0)
        .push(text::<B>("Scroll down to see depth...").body().secondary())
        .push(
            scroll_view::<Message, B>(
                vstack::<Message, B>()
                    .spacing(8.0)
                    .push(
                        rectangle::<B>(Length::Fill, Length::Fixed(100.0))
                            .color(Color::from_rgb(0.2, 0.2, 0.1))
                            .radius(8.0),
                    )
                    .push(
                        rectangle::<B>(Length::Fill, Length::Fixed(100.0))
                            .color(Color::from_rgb(0.3, 0.3, 0.2))
                            .radius(8.0),
                    )
                    .push(
                        rectangle::<B>(Length::Fill, Length::Fixed(100.0))
                            .color(Color::from_rgb(0.4, 0.4, 0.3))
                            .radius(8.0),
                    )
                    .push(text::<B>("Bottom reached!").body().bold().primary()),
            )
            .height(Length::Fixed(200.0)),
        )
}
