use super::super::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult {
    PageResult::new(
        ComponentDoc::new(
            "Sidebar",
            "A structural component that provides primary navigation for your application, supporting hierarchical trees and sectioned lists.",
            r#"
NavigationSplitView::new(
    Sidebar::new()
        .push(SidebarItem::new("Home", "home", Page::Home))
        .push(SidebarItem::new("Settings", "settings", Page::Settings)),
    MainContent::new()
)
"#,
            Arc::new(VStack::<Message, IcedBackend>::new_generic()
                .spacing(16.0)
                .push(Text::<IcedBackend>::new("Refer to NavigationSplitView for implementation details.").body().secondary()))
        )
    )
}
