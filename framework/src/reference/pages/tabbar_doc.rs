use super::super::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult {
    PageResult::new(
        ComponentDoc::new(
            "Tabbar",
            "A bottom navigation component typically used on mobile devices to switch between top-level application sections.",
            r#"
Tabbar::new()
    .push(TabItem::new("Feed", "rss"))
    .push(TabItem::new("Search", "search"))
    .push(TabItem::new("Profile", "user"))
"#,
            Arc::new(HStack::<Message, IcedBackend>::new_generic()
                .spacing(24.0)
                .push(Icon::<IcedBackend>::new("rss").size(24.0).secondary())
                .push(Icon::<IcedBackend>::new("search").size(24.0).secondary())
                .push(Icon::<IcedBackend>::new("user").size(24.0).secondary()))
        )
    )
}
