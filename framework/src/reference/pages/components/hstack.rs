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
    let code_snippet = "HStack::new()\n    .spacing(16.0)\n    .align_y(Alignment::Center)\n    .push(Icon::new(\"zap\"))\n    .push(Text::new(\"HStack\"))".to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "HStack",
        "A horizontal stack layout component that arranges its children in a row with optional spacing and vertical alignment.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Horizontal Flow\nHorizontal stacks are used for side-by-side elements like toolbars, navigation links, and labeled inputs.\n\n- **Natural Reading Direction**: Maps directly to the human eye's natural scanning pattern in LTR/RTL languages.\n- **Vertical Alignment**: Allows synchronization of elements with different internal heights (Start, Center, End).\n- **Responsive Behavior**: Often paired with `ResponsiveGrid` or `Wrap` components to handle viewport constraints."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.spacing(f32)` | `f32` | Distance between consecutive children. |\n| `.align_y(align)` | `Alignment` | Vertical alignment of children. |\n| `.push(view)` | `View` | Adds a new child to the end of the stack. |"
    );

    PageResult::new(doc)
}

fn create_preview<B: Backend>() -> HStack<Message, B> {
    hstack::<Message, B>()
        .spacing(16.0)
        .align_y(Alignment::Center)
        .push(icon::<B>("layers").size(32.0).primary_color())
        .push(
            vstack::<Message, B>()
                .spacing(2.0)
                .push(text::<B>("Horizontal Layout").body().bold())
                .push(text::<B>("Side-by-side").caption2().secondary()),
        )
}
