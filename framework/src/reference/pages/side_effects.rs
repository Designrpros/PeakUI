use super::super::app::Message;
use super::super::page::PageResult;
use crate::prelude::*;

use super::super::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context, _is_mobile: bool) -> PageResult {
    let preview = VStack::<Message, IcedBackend>::new_generic()
        .spacing(24.0)
        .push(Text::<IcedBackend>::new("Async Actions & Commands").title2().bold())
        .push(
            Text::<IcedBackend>::new(
                "PeakUI manages side effects (like HTTP requests, timers, or window resizing) through Commands returning from the update function.",
            )
            .secondary(),
        );

    PageResult::new(ComponentDoc::new(
        "Side Effects",
        "Perform asynchronous operations safely using the Command pattern.",
        r#"
fn update(msg: Message, model: &mut Model) -> Command<Message> {
    match msg {
        Message::FetchData => {
            Command::perform(do_fetch(), Message::DataReceived)
        }
        Message::DataReceived(data) => {
            model.data = data;
            Command::none()
        }
    }
}
"#,
        Arc::new(preview),
    ))
}
