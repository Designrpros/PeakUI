use super::super::app::Message;
use super::super::page::PageResult;
use crate::prelude::*;

pub fn view(_context: &Context) -> PageResult {
    PageResult::new(ProxyView::<Message, IcedBackend>::new(move |ctx| {
        let content = iced::widget::column![
            Text::<IcedBackend>::new("Images")
                .large_title()
                .color(ctx.theme.colors.text_primary)
                .view(ctx),
            Text::<IcedBackend>::new("A responsive image component that adapts to its container and supports light/dark alternate sources.")
                .body()
                .color(ctx.theme.colors.text_secondary)
                .view(ctx),
            Section::<Message, IcedBackend>::new(
                "BASIC USAGE",
                VStack::<Message, IcedBackend>::new_generic()
                    .spacing(20.0)
                    .push(
                        Text::<IcedBackend>::new("Standard image loaded from path.")
                            .subheadline()
                    )
                    .push(
                        Image::<IcedBackend>::new("/assets/mountain_sunset_warm.jpg")
                            .width(Length::Fill)
                            .height(Length::Fixed(200.0))
                            .radius(12.0)
                    )
            )
            .view(ctx),
            Section::<Message, IcedBackend>::new(
                "STYLING OPTIONS",
                ResponsiveGrid::<Message, IcedBackend>::new_generic()
                    .spacing(20.0)
                    .push(
                        VStack::<Message, IcedBackend>::new_generic()
                            .spacing(10.0)
                            .push(Text::<IcedBackend>::new("Rounded Corners").caption1())
                            .push(
                                Image::<IcedBackend>::new("/assets/poolsuite_luxury-kopi.jpg")
                                    .width(Length::Fill)
                                    .height(Length::Fixed(150.0))
                                    .radius(24.0)
                            )
                    )
                    .push(
                        VStack::<Message, IcedBackend>::new_generic()
                            .spacing(10.0)
                            .push(Text::<IcedBackend>::new("Fixed Size").caption1())
                            .push(
                                Image::<IcedBackend>::new("/assets/poolsuite_luxury-kopi.jpg")
                                    .width(Length::Fixed(150.0))
                                    .height(Length::Fixed(150.0))
                                    .radius(8.0)
                            )
                    )
            )
            .view(ctx),
            Section::<Message, IcedBackend>::new(
                "RESPONSIVE DESIGN",
                VStack::<Message, IcedBackend>::new_generic()
                    .spacing(20.0)
                    .push(
                        Text::<IcedBackend>::new("Full width image that fills its container, similar to Next.js Image component behavior.")
                            .subheadline()
                    )
                    .push(
                        Image::<IcedBackend>::new("/assets/mountain_sunset_warm.jpg")
                            .width(Length::Fill)
                            .height(Length::Fixed(300.0))
                            .radius(16.0)
                    )
            )
            .view(ctx)
        ]
        .spacing(40);

        container(content).width(Length::Fill).padding(40).into()
    }))
}
