use crate::reference::app::Message;

use crate::prelude::*;
use crate::reference::AppPageResult;

use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context, _is_mobile: bool) -> AppPageResult {
    let preview = VStack::<Message, IcedBackend>::new_generic()
        .spacing(24.0)
        .push(Text::<IcedBackend>::new("View Memoization").title2().bold())
        .push(
            Text::<IcedBackend>::new(
                "Optimize rendering by memoizing view construction logic using `ProxyView` or smart caching.",
            )
            .secondary(),
        );

    AppPageResult::new(ComponentDoc::new(
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
