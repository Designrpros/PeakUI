use super::super::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult {
    PageResult::new(
        ComponentDoc::new(
            "NavigationSplitView",
            "A structural navigation component that implements a master-detail pattern, managing the transitions between primary and secondary content.",
            r#"
NavigationSplitView::new(
    Sidebar::new()
        .push(NavigationLink::new("Item 1", Page::Page1)),
    DetailView::new()
)
"#,
            Arc::new(VStack::<Message, IcedBackend>::new_generic()
                .spacing(16.0)
                .push(Text::<IcedBackend>::new("NavigationSplitView is the foundation of adaptive layouts.").body().secondary()))
        )
    )
}
