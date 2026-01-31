use crate::reference::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::navigation::PageResult;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult<Message> {
    PageResult::new(
        ComponentDoc::new(
            "Sidebar",
            "A structural component that provides primary navigation for your application, supporting hierarchical trees and sectioned lists.",
            r#"
NavigationSplitView::new(
    Sidebar::new()
        .push(SidebarItem::new("Home", "home", crate::reference::model::Page::Home))
        .push(SidebarItem::new("Settings", "settings", crate::reference::model::Page::Settings)),
    MainContent::new()
)
"#,
            Arc::new(VStack::<Message, IcedBackend>::new_generic()
                .spacing(16.0)
                .push(Text::<IcedBackend>::new("Refer to NavigationSplitView for implementation details.").body().secondary()))
        )
    )
}
