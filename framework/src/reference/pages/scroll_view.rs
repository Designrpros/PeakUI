use super::super::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use iced::widget::scrollable;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult {
    PageResult::new(
        ComponentDoc::new(
            "ScrollView",
            "A container that enables scrolling for content that exceeds the available screen space, supporting both vertical and horizontal directions.",
            r#"
ScrollView::new(
    VStack::new()
        .push(long_content_block_1)
        .push(long_content_block_2)
)
"#,
            Arc::new(ProxyView::new(move |ctx| {
                container(
                    scrollable(
                        VStack::<Message, IcedBackend>::new_generic()
                            .spacing(12.0)
                            .push(Text::<IcedBackend>::new("Scroll down to see more...").body())
                            .push(Space::<IcedBackend>::new(Length::Fill, Length::Fixed(400.0)))
                            .push(Text::<IcedBackend>::new("You reached the bottom!").body().primary().bold())
                            .view(ctx)
                    )
                    .height(200.0)
                )
                .padding(1)
                .into()
            }))
        )
    )
}
