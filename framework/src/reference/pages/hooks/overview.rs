use crate::reference::app::Message;
use crate::engine::navigation::PageResult;
use crate::prelude::*;

use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context, _is_mobile: bool) -> PageResult<Message> {
    let preview = VStack::<Message, IcedBackend>::new_generic()
        .spacing(32.0)
        .push(
            VStack::<Message, IcedBackend>::new_generic()
                .spacing(12.0)
                .push(Text::<IcedBackend>::new("Counter Example").title2().bold())
                .push(
                    Text::<IcedBackend>::new(
                        "This demonstrates using simple message-based state updates.",
                    )
                    .secondary(),
                )
                .push(
                    HStack::<Message, IcedBackend>::new_generic()
                        .spacing(12.0)
                        .push(Button::<Message>::label("-"))
                        .push(Text::<IcedBackend>::new("0").title3().bold())
                        .push(Button::<Message>::label("+")),
                ),
        );

    PageResult::new(ComponentDoc::new(
        "Hooks & State",
        "Functional components and state management patterns using the built-in update cycle.",
        r#"
// Handling state updates in your View
fn update(msg: Message, model: &mut Model) {
    match msg {
        Message::Increment => model.counter += 1,
        Message::Decrement => model.counter -= 1,
    }
}
"#,
        Arc::new(preview),
    ))
}
