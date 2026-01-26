use super::super::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult {
    PageResult::new(
        ComponentDoc::new(
            "Modal",
            "An overlay component that captures the user's attention for critical tasks or information, dimming the background content.",
            r#"
Modal::new(
    VStack::new()
        .push(Text::new("Confirm Action").bold())
        .push(Button::label("Continue"))
)
.on_close(Message::CloseModal)
"#,
            Arc::new(VStack::<Message, IcedBackend>::new_generic()
                .height(Length::Fixed(120.0))
                .spacing(16.0)
                .push(Text::<IcedBackend>::new("Modals are rendered as overlays on the root stack.").body().secondary()))
        )
    )
}
