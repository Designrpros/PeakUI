use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult<Message> {
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
            Arc::new(
                vstack![
                    text("Large Title").large_title(),
                    text("Title 1").title1(),
                    text("Headline").headline(),
                    text("Body text with default styling.").body(),
                    text("Caption text is smaller and often dimmer.").caption1(),
                    hstack![
                        text("Primary").body().primary(),
                        text("Secondary").body().secondary(),
                        text("Danger").body().intent(crate::modifiers::Intent::Danger)
                    ]
                    .spacing(16.0)
                ]
                .spacing(16.0),
            )
        )
    )
}
