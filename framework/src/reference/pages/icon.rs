use super::super::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult {
    PageResult::new(
        ComponentDoc::new(
            "Icon",
            "A flexible icon component powered by the Lucide icon set, supporting semantic coloring and reactive scaling.",
            r#"
Icon::new("zap")
    .size(24.0)
    .primary()

Icon::new("settings")
    .secondary()
"#,
            Arc::new(HStack::<Message, IcedBackend>::new_generic()
                .spacing(24.0)
                .push(Icon::<IcedBackend>::new("zap").size(32.0).color(Color::from_rgb8(0, 122, 255)))
                .push(Icon::<IcedBackend>::new("settings").size(32.0).color(Color::from_rgb8(142, 142, 147)))
                .push(Icon::<IcedBackend>::new("heart").size(32.0).color(Color::from_rgb8(255, 59, 48)))
                .push(Icon::<IcedBackend>::new("check-circle").size(32.0).color(Color::from_rgb8(52, 199, 89))))
        )
    )
}
