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
        "Section::new(\n    \"Group Title\",\n    VStack::new().push(content)\n)".to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "Section",
        "A structural grouping container that adds a title and visual separation to its content.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Structural Hierarchy\nSections provide the high-level skeleton for information-dense pages. They anchor content with a semantic title and provide consistent vertical rhythm.\n\n- **Anchor points**: Sections are often used as navigation targets for jump-links.\n- **Visual separation**: Automatically applies padding and spacing according to the active theme's structural tokens."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.new(title, content)` | `&str, View` | Creates a titled container for grouping. |\n| `.width(len)` | `Length` | Sizing for the entire section. |"
    );

    PageResult::new(doc)
}

fn create_preview<B: Backend>() -> VStack<Message, B> {
    vstack::<Message, B>()
        .spacing(16.0)
        .push(section::<Message, B>(
            "User Settings",
            vstack::<Message, B>()
                .spacing(8.0)
                .push(
                    text::<B>("Manage your account preferences here.")
                        .body()
                        .secondary(),
                )
                .push(
                    button_label::<Message, B>("Edit Profile").variant(crate::style::Variant::Soft),
                ),
        ))
}
