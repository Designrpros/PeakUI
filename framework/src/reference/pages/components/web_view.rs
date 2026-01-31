use crate::reference::app::Message;
use crate::navigation::PageResult;
use crate::atoms::WebView;
use crate::prelude::*;

pub fn view(_context: &Context) -> PageResult<Message> {
    PageResult::new(ProxyView::<Message, IcedBackend>::new(move |ctx| {
        let content = iced::widget::column![
            Text::<IcedBackend>::new("WebView")
                .large_title()
                .color(ctx.theme.colors.text_primary)
                .view(ctx),
            Text::<IcedBackend>::new(
                "A cross-platform web component for rendering web content, iframes, and web-based video players."
            )
            .body()
            .color(ctx.theme.colors.text_secondary)
            .view(ctx),
            Section::<Message, IcedBackend>::new(
                "YOUTUBE EMBED",
                VStack::<Message, IcedBackend>::new_generic()
                    .spacing(20.0)
                    .push(
                        Text::<IcedBackend>::new("Mount Everest - 8K HDR")
                            .subheadline()
                    )
                    .push(
                        WebView::<IcedBackend>::new("https://www.youtube.com/embed/0pIyIMqwu0E")
                            .width(Length::Fill)
                            .height(Length::Fixed(400.0))
                            .radius(16.0)
                    )
            )
            .view(ctx),
            Section::<Message, IcedBackend>::new(
                "FEATURES",
                VStack::<Message, IcedBackend>::new_generic()
                    .spacing(12.0)
                    .push(Text::<IcedBackend>::new("• Iframe support for WASM").body())
                    .push(Text::<IcedBackend>::new("• Cross-platform URL rendering").body())
                    .push(Text::<IcedBackend>::new("• Native Wry integration (Planned)").body())
                    .push(Text::<IcedBackend>::new("• Glassmorphism border support").body())
            )
            .view(ctx)
        ]
        .spacing(40);

        container(content).width(Length::Fill).padding(40).into()
    }))
}
