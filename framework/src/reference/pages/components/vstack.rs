use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult<Message> {
    PageResult::new(
        ComponentDoc::new(
            "VStack",
            "A vertical stack layout component that arranges its children in a column with optional spacing and alignment.",
            r#"
VStack::new()
    .spacing(12.0)
    .align_x(Alignment::Center)
    .push(Text::new("Top"))
    .push(Text::new("Middle"))
    .push(Text::new("Bottom"))
"#,
            Arc::new(
                vstack![
                    text("Item 1").body().bold(),
                    text("Item 2").body().secondary(),
                    text("Item 3").body().secondary()
                ]
                .spacing(8.0),
            )
        )
    )
}
