use super::super::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult {
    PageResult::new(
        ComponentDoc::new(
            "ZStack",
            "A depth-based layout component that overlays its children on top of each other, useful for backgrounds and badges.",
            r#"
ZStack::new()
    .push(Image::new("background.jpg"))
    .push(Text::new("Overlay Text"))
"#,
            Arc::new(ZStack::<Message, IcedBackend>::new_generic()
                .push(Rectangle::<IcedBackend>::new(Length::Fixed(200.0), Length::Fixed(120.0))
                    .color(Color::from_rgb8(40, 40, 40))
                    .radius(12.0))
                .push(ProxyView::new(move |ctx| {
                    container(Text::<IcedBackend>::new("ZStack Content").bold().primary().view(ctx))
                        .padding(20)
                        .width(Length::Fixed(200.0))
                        .height(Length::Fixed(120.0))
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .into()
                })))
        )
    )
}
