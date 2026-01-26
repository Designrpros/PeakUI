use super::super::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult {
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
            Arc::new(HStack::<Message, IcedBackend>::new_generic()
                .spacing(16.0)
                .push(Icon::<IcedBackend>::new("user").size(20.0).primary())
                .push(Text::<IcedBackend>::new("Profile Settings").body().bold()))
        )
    )
}
