use super::super::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult {
    PageResult::new(
        ComponentDoc::new(
            "Divider",
            "A simple visual separator for grouping content and defining logical boundaries in your layout.",
            r#"
VStack::new()
    .push(Text::new("Section 1"))
    .push(Divider::new())
    .push(Text::new("Section 2"))
"#,
            Arc::new(VStack::<Message, IcedBackend>::new_generic()
                .spacing(16.0)
                .push(Text::<IcedBackend>::new("Item Above").body())
                .push(Divider::<IcedBackend>::new())
                .push(Text::<IcedBackend>::new("Item Below").body()))
        )
    )
}
