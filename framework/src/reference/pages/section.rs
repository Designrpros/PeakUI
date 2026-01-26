use super::super::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult {
    PageResult::new(ComponentDoc::new(
        "Section",
        "A structural grouping container that adds a title and visual separation to its content.",
        r#"
Section::new(
    "Profile Settings",
    VStack::new()
        .push(Text::new("Username: User123"))
        .push(Button::label("Edit"))
)
"#,
        Arc::new(
            crate::containers::Section::<Message, IcedBackend>::new_generic(
                "Example Section",
                VStack::<Message, IcedBackend>::new_generic()
                    .spacing(8.0)
                    .push(Text::<IcedBackend>::new("This is content inside a section.").body())
                    .push(
                        Button::<Message>::label("Section Action")
                            .variant(crate::modifiers::Variant::Soft),
                    ),
            ),
        ),
    ))
}
