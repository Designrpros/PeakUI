use super::super::app::Message;
use super::super::page::PageResult;
use crate::prelude::*;

use super::super::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context, _is_mobile: bool) -> PageResult {
    let preview = VStack::<Message, IcedBackend>::new_generic()
        .spacing(24.0)
        .push(Text::<IcedBackend>::new("Instant Synchronization").title2().bold())
        .push(
            Text::<IcedBackend>::new(
                "Your state is automatically replicated across all devices in real-time via the encrypted PeakCloud mesh.",
            )
            .secondary(),
        );

    PageResult::new(ComponentDoc::new(
        "PeakCloud",
        "Zero-config distributed state management for universal applications.",
        r#"
// State marked with `#[sync]` is automatically handled
#[derive(Model, Sync)]
struct AppState {
    #[sync(strategy = "merge")]
    pub todos: Vec<Todo>,
}
"#,
        Arc::new(preview),
    ))
}
