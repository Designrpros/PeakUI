use super::super::app::Message;
use super::super::page::PageResult;
use crate::prelude::*;

use super::super::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context, _is_mobile: bool) -> PageResult {
    let preview = VStack::<Message, IcedBackend>::new_generic()
        .spacing(24.0)
        .push(Text::<IcedBackend>::new("View Memoization").title2().bold())
        .push(
            Text::<IcedBackend>::new(
                "Optimize rendering by memoizing view construction logic using `ProxyView` or smart caching.",
            )
            .secondary(),
        );

    PageResult::new(ComponentDoc::new(
        "Performance",
        "Techniques for maintaining 120fps even with complex UI trees.",
        r#"
// Use ProxyView to isolate context dependency
ProxyView::new(move |context| {
    // This closure only re-runs when context changes
    expensive_view_computation(context)
})
"#,
        Arc::new(preview),
    ))
}
