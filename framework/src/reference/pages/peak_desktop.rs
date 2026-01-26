use super::super::app::Message;
use super::super::page::PageResult;
use crate::prelude::*;

use super::super::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context, _is_mobile: bool) -> PageResult {
    let preview = VStack::<Message, IcedBackend>::new_generic()
        .spacing(24.0)
        .push(Text::<IcedBackend>::new("Window Management Reimagined").title2().bold())
        .push(
            Text::<IcedBackend>::new(
                "PeakDesktop brings tiling, stacking, and floating windows together in a unified spatial model.",
            )
            .secondary(),
        );

    PageResult::new(ComponentDoc::new(
        "PeakDesktop",
        "The flagship environment for productivity, built entirely on PeakUI.",
        r#"
// PeakDesktop defines the shell environment
let desktop = Desktop::new()
    .with_layout(LayoutMode::Tiling)
    .with_decorations(Theme::Glass);
"#,
        Arc::new(preview),
    ))
}
