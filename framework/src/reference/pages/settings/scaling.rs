use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context, _is_mobile: bool) -> PageResult {
    let preview = VStack::<Message, IcedBackend>::new_generic()
        .spacing(24.0)
        .push(
            Text::<IcedBackend>::new("Accessibility & Scaling")
                .title2()
                .bold(),
        )
        .push(
            Text::<IcedBackend>::new(
                "Adjust interface density and font scaling to match your preference.",
            )
            .secondary(),
        )
        .push(
            HStack::<Message, IcedBackend>::new_generic()
                .spacing(12.0)
                .push(Button::<Message>::label("Compact"))
                .push(Button::<Message>::label("Default").variant(Variant::Soft))
                .push(Button::<Message>::label("Comfortable")),
        );

    PageResult::new(ComponentDoc::new(
        "Scaling",
        "PeakUI is built on a responsive unit system that scales gracefully.",
        r#"
// Using the scaled context in your views
let spacing = context.scale(16.0);
let font_size = context.scale(theme.typography.body.size);
"#,
        Arc::new(preview),
    ))
}
