use super::super::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult {
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
            Arc::new(VStack::<Message, IcedBackend>::new_generic()
                .spacing(8.0)
                .push(Text::<IcedBackend>::new("Item 1").body().bold())
                .push(Text::<IcedBackend>::new("Item 2").body().secondary())
                .push(Text::<IcedBackend>::new("Item 3").body().secondary()))
        )
    )
}
