use crate::reference::app::Message;
use crate::navigation::PageResult;
use crate::prelude::*;

pub fn view(_context: &Context) -> PageResult<Message> {
    PageResult::new(ProxyView::<Message, IcedBackend>::new(move |ctx| {
        let content = iced::widget::column![
            Text::<IcedBackend>::new("Video")
                .large_title()
                .color(ctx.theme.colors.text_primary)
                .view(ctx),
            Text::<IcedBackend>::new(
                "A high-fidelity video component for media playback within the Peak Swarm."
            )
            .body()
            .color(ctx.theme.colors.text_secondary)
            .view(ctx),
            Section::<Message, IcedBackend>::new(
                "SHOWCASE",
                VStack::<Message, IcedBackend>::new_generic()
                    .spacing(20.0)
                    .push(Text::<IcedBackend>::new("Framework Integration").subheadline())
                    .push(
                        Video::<IcedBackend>::new("pending")
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
                    .push(Text::<IcedBackend>::new("• Adaptive sizing (Fixed/Fill)").body())
                    .push(Text::<IcedBackend>::new("• Customizable border radius").body())
                    .push(Text::<IcedBackend>::new("• High-fidelity placeholder for WASM").body())
                    .push(Text::<IcedBackend>::new("• Native ffmpeg integration (Planned)").body())
            )
            .view(ctx)
        ]
        .spacing(40);

        container(content).width(Length::Fill).padding(40).into()
    }))
}
