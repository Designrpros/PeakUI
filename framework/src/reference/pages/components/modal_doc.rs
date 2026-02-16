use crate::core::{AIBackend, Backend, IcedBackend, SpatialBackend};

use crate::prelude::*;
use crate::reference::app::{LabMessage, Message, RenderMode};
use crate::reference::views::ComponentDoc;
use crate::reference::AppPageResult;
use std::sync::Arc;

pub fn view(ctx: &Context, render_mode: RenderMode) -> AppPageResult {
    // --- 1. Preview Construction ---
    let preview_view = create_preview::<IcedBackend>();
    let terminal_preview = "Modal: [ Confirm Action ]".to_string();
    let neural_preview = vstack::<Message, AIBackend>()
        .push(text("Modal Confirmation"))
        .describe(ctx);
    let spatial_preview = vstack::<Message, SpatialBackend>()
        .push(text("Modal"))
        .view(ctx);

    // --- 2. Code Snippet ---
    let code_snippet = "Modal::new(\n    VStack::new()\n        .push(Text::new(\"Alert\").bold())\n)\n.on_close(Message::CloseModal)".to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "Modal",
        "An overlay component that captures the user's attention for critical tasks or information, dimming the background content.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::Lab(LabMessage::SetRenderMode(mode)))
    .theory(
       "### Disruptive Attention\nModals are intentional disruptions. Use them sparingly for destructive actions, mandatory confirmations, or high-focus data entry.\n\n- **Backdrop**: The 'scrim' or dimming layer helps focus the user by visually softening non-interactive content.\n- **Keyboard Interop**: Should automatically handle 'Escape' to close (Planned)."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.new(content)` | `View` | Creates the modal with inner content. |\n| `.on_close(m)` | `Message` | Triggered when the user clicks the backdrop. |\n| `.width(len)` | `Length` | Sets the maximum width of the dialog. |"
    );

    AppPageResult::new(doc)
}

fn create_preview<B: Backend>() -> VStack<Message, B> {
    vstack::<Message, B>().spacing(16.0).push(
        vstack::<Message, B>()
            .spacing(8.0)
            .push(text::<B>("Sample Modal Dialog").caption2().secondary())
            .push(
                GlassCard::<Message, B>::new(
                    vstack::<Message, B>()
                        .spacing(12.0)
                        .push(text::<B>("Confirm Action").bold())
                        .push(text::<B>("Are you sure you want to proceed?").body())
                        .push(Button::<Message, B>::label("Continue").intent(Intent::Primary)),
                )
                .padding(20.0),
            ),
    )
}
