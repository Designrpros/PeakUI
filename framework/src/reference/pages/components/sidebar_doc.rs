use crate::core::{AIBackend, Backend, IcedBackend, SpatialBackend, TermBackend};

use crate::prelude::*;
use crate::reference::app::{LabMessage, Message, RenderMode};
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(ctx: &Context, render_mode: RenderMode) -> AppPageResult {
    // --- 1. Preview Construction ---
    let preview_view = create_preview::<IcedBackend>();
    let terminal_preview = create_preview::<TermBackend>().view(ctx);
    let neural_preview = create_preview::<AIBackend>().view(ctx);
    let spatial_preview = create_preview::<SpatialBackend>().view(ctx);

    // --- 2. Code Snippet ---
    let code_snippet = "Sidebar::new()\n    .push(SidebarItem::new(\"Home\", \"home\", AppPage::Home))\n    .push(SidebarItem::new(\"Settings\", \"settings\", AppPage::Settings))".to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "Sidebar",
        "A structural component that provides primary navigation for your application, supporting hierarchical trees and sectioned lists.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::Lab(LabMessage::SetRenderMode(mode)))
    .theory(
       "### Cognitive Anchoring\nThe sidebar serves as the primary cognitive anchor for the user. It provides a stable surface for navigation that persists across different page transitions.\n\n- **Hierarchy**: Supports nested items for complex information architectures.\n- **Adaptability**: Collapses on mobile/compact views to maximize content space while remaining accessible via gestures."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.new()` | N/A | Creates an empty sidebar container. |\n| `.push(item)` | `SidebarItem` | Adds a navigation entry to the list. |\n| `.width(len)` | `Length` | Sets the fixed or relative width. |"
    );

    AppPageResult::new(doc)
}

fn create_preview<B: Backend>() -> VStack<Message, B> {
    vstack::<Message, B>().spacing(16.0).push(
        vstack::<Message, B>()
            .spacing(8.0)
            .push(text::<B>("Sample Sidebar").caption2().secondary())
            .push(
                vstack::<Message, B>()
                    .spacing(8.0)
                    .push(text::<B>("• Home").body())
                    .push(text::<B>("• Components").body().primary())
                    .push(text::<B>("• Settings").body()),
            ),
    )
}
