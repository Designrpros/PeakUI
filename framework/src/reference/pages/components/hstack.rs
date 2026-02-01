use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult<Message> {
    PageResult::new(
        ComponentDoc::new(
            "HStack",
            "A horizontal stack layout component that arranges its children in a row with optional spacing and vertical alignment.",
            r#"
HStack::new()
    .spacing(16.0)
    .align_y(Alignment::Center)
    .push(Icon::new("heart"))
    .push(Text::new("Favorites"))
"#,
            Arc::new(
                hstack![
                    icon("user").size(20.0).primary(),
                    text("Profile Settings").body().bold()
                ]
                .spacing(16.0),
            )
        )
    )
}
