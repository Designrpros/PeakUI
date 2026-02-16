use crate::reference::app::Message;

use crate::prelude::*;
use crate::reference::AppPageResult;

use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context, _is_mobile: bool) -> AppPageResult {
    let preview = VStack::<Message, IcedBackend>::new_generic()
        .spacing(24.0)
        .push(Text::<IcedBackend>::new("Async Actions & Commands").title2().bold())
        .push(
            Text::<IcedBackend>::new(
                "PeakUI manages side effects (like HTTP requests, timers, or window resizing) through Commands returning from the update function.",
            )
            .secondary(),
        );

    AppPageResult::new(ComponentDoc::new(
        "Side Effects",
        "Perform asynchronous operations safely using the Command pattern.",
        r#"
fn update(msg: Message, model: &mut Model) -> Task<Message> {
    match msg {
        Message::Interaction(InteractionMessage::Action) => {
            // Task::perform(do_async_work(), Message::Lab(LabMessage::Result))
            Task::none()
        }
        _ => Task::none(),
    }
}
"#,
        Arc::new(preview),
    ))
}
