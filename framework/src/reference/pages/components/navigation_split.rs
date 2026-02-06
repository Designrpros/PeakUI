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
    let code_snippet =
        "NavigationSplitView::new(\n    Sidebar::new(),\n    DetailView::new()\n)".to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "NavigationSplitView",
        "A structural navigation component that implements a master-detail pattern, managing transitions between primary and secondary content.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Master-Detail Architecture\nThe Split View is the root of most complex applications. It manages the layout of the sidebar and the main content area, providing a consistent navigation experience.\n\n- **Adaptive Layout**: Automatically handles sidebar collapsing on smaller viewports.\n- **State Management**: Orchestrates the active page state and detail view rendering."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.new(side, main)` | `View, View` | Creates a split container. |\n| `.sidebar_width(w)` | `f32` | Sets the preferred sidebar width. |"
    );

    PageResult::new(doc)
}

fn create_preview<B: Backend>() -> VStack<Message, B> {
    vstack::<Message, B>().spacing(16.0).push(
        vstack::<Message, B>()
            .spacing(8.0)
            .push(text::<B>("Split View Wireframe").caption2().secondary())
            .push(
                hstack::<Message, B>()
                    .push(
                        vstack::<Message, B>().width(Length::Fixed(80.0)).push(
                            rectangle::<B>(Length::Fill, Length::Fill)
                                .color(Color::from_rgb8(40, 40, 40)),
                        ),
                    )
                    .push(
                        vstack::<Message, B>().width(Length::Fill).push(
                            rectangle::<B>(Length::Fill, Length::Fill)
                                .color(Color::from_rgb8(20, 20, 20)),
                        ),
                    )
                    .height(Length::Fixed(120.0)),
            ),
    )
}
