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
    let code_snippet = "Tabbar::new()\n    .push(TabItem::new(\"Feed\", \"rss\"))\n    .push(TabItem::new(\"Search\", \"search\"))".to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "Tabbar",
        "A bottom navigation component typically used on mobile devices to switch between top-level application sections.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Bottom-First Interaction\nTab bars are optimized for one-handed use on mobile devices. They provide high-velocity switching between independent contexts.\n\n- **Visual Feedback**: Active tabs should clearly differentiate themselves from inactive ones.\n- **Kernel Layout**: While native on iOS/Android, it adapts as a floating dock on Desktop and a numbered list in TUI."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.new()` | N/A | Creates an empty tab bar. |\n| `.push(item)` | `TabItem` | Adds a tab icon/label. |\n| `.on_change(m)`| `Message` | Emitted when a tab is selected. |"
    );

    PageResult::new(doc)
}

fn create_preview<B: Backend>() -> HStack<Message, B> {
    hstack::<Message, B>().spacing(32.0).push(
        hstack::<Message, B>()
            .spacing(32.0)
            .push(icon::<B>("rss").size(24.0).secondary())
            .push(icon::<B>("search").size(24.0).primary_color())
            .push(icon::<B>("user").size(24.0).secondary()),
    )
}
