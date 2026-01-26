use super::super::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult {
    PageResult::new(
        ComponentDoc::new(
            "Text",
            "A fundamental typography component for rendering text with various styles, sizes, and intents.",
            r#"
Text::new("Hello World")
    .headline()
    .bold()
    .color(Color::BLACK)

Text::new("Error Message")
    .caption1()
    .intent(Intent::Danger)
"#,
            Arc::new(VStack::<Message, IcedBackend>::new_generic()
                .spacing(16.0)
                .push(Text::<IcedBackend>::new("Large Title").large_title())
                .push(Text::<IcedBackend>::new("Title 1").title1())
                .push(Text::<IcedBackend>::new("Headline").headline())
                .push(Text::<IcedBackend>::new("Body text with default styling.").body())
                .push(Text::<IcedBackend>::new("Caption text is smaller and often dimmer.").caption1())
                .push(
                    HStack::<Message, IcedBackend>::new_generic()
                        .spacing(16.0)
                        .push(Text::<IcedBackend>::new("Primary").body().primary())
                        .push(Text::<IcedBackend>::new("Secondary").body().secondary())
                        .push(Text::<IcedBackend>::new("Danger").body().intent(crate::modifiers::Intent::Danger))
                )
            )
        )
    )
}
