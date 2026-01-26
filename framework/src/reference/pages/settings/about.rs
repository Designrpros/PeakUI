use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context, _is_mobile: bool) -> PageResult {
    let preview = VStack::<Message, IcedBackend>::new_generic()
        .spacing(24.0)
        .align_x(Alignment::Center)
        .push(
            Image::new(crate::assets::Asset::MeshBackground.path())
                .width(Length::Fixed(120.0))
                .height(Length::Fixed(120.0))
                .radius(60.0),
        )
        .push(
            Text::<IcedBackend>::new("PeakUI Reference App")
                .title2()
                .bold(),
        )
        .push(Text::<IcedBackend>::new("Version 0.1.0 (Alpha)").secondary())
        .push(
            Text::<IcedBackend>::new("© 2026 PeakOS Project")
                .caption1()
                .secondary(),
        );

    PageResult::new(ComponentDoc::new(
        "About System",
        "Diagnostics and version information.",
        r#"
// Accessing build metadata
const VERSION: &str = env!("CARGO_PKG_VERSION");
"#,
        Arc::new(preview),
    ))
}
