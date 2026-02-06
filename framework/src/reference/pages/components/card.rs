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
    let code_snippet = "Card::new(\n    VStack::new()\n        .push(Text::new(\"Card Title\").bold())\n        .push(Text::new(\"Card content.\"))\n)".to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "Card",
        "A stylized container with a background, border, and shadow, used to group related information into a singular unit.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Atomic Grouping\nCards are the fundamental unit of surface-based UI. They provide visual context and hierarchy by separating independent concepts into distinct surfaces.\n\n- **Surface elevation**: In GUI, cards use subtle shadows to imply depth. In TUI, they use character-based borders.\n- **Adaptive Content**: Cards automatically handle inner layout constraints while maintaining their stylized container rules."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.new(content)` | `View` | Wraps any view in a card surface. |\n| `.radius(r)` | `f32` | Corner rounding for the surface. |\n| `.padding(p)` | `Padding` | Internal spacing for the content. |"
    );

    PageResult::new(doc)
}

fn create_preview<B: Backend>() -> VStack<Message, B> {
    vstack::<Message, B>().spacing(16.0).push(
        vstack::<Message, B>()
            .spacing(8.0)
            .push(text::<B>("Sample Card").caption2().secondary())
            .push(
                crate::dsl::container::<Message, B>(
                    vstack::<Message, B>()
                        .spacing(8.0)
                        .push(text::<B>("Title").title3().bold())
                        .push(
                            text::<B>("Card description with consistent spacing.")
                                .body()
                                .secondary(),
                        ),
                )
                .padding(24.0)
                .background(Color::from_rgb(0.15, 0.15, 0.15))
                .border(1.0, Color::from_rgb(0.3, 0.3, 0.3))
                .radius(16.0),
            ),
    )
}
