use crate::dsl::container;
use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view<B: Backend>(_context: &Context) -> PageResult<Message, B> {
    PageResult::new(
        ComponentDoc::<Message, B>::new(
            "ZStack",
            "A depth-based layout component that overlays its children on top of each other, useful for backgrounds and badges.",
            r#"
ZStack::new()
    .push(Image::new("background.jpg"))
    .push(Text::new("Overlay Text"))
"#,
            Arc::new(
                zstack![
                    Rectangle::<B>::new(Length::Fixed(200.0), Length::Fixed(120.0))
                        .color(Color::from_rgb8(40, 40, 40))
                        .radius(12.0),
                    ProxyView::new(move |ctx| {
                        container::<Message, B>(
                            text::<B>("ZStack Content").bold().primary(),
                        )
                        .padding(20)
                        .width(Length::Fixed(200.0))
                        .height(Length::Fixed(120.0))
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .view(ctx)
                    })
                ],
            ),
        )
    )
}
