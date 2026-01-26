use super::super::app::Message;
use super::super::page::PageResult;
use crate::prelude::*;

pub fn view(context: &Context, _is_mobile: bool) -> PageResult {
    let mut page_col = VStack::<Message, IcedBackend>::new_generic()
        .width(Length::Fill)
        .padding(Padding::from(32.0))
        .spacing(32.0);

    // Header
    page_col = page_col.push(
        VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .spacing(12.0)
            .push(
                Text::<IcedBackend>::new("Shapes")
                    .large_title()
                    .bold()
                    .width(Length::Fill),
            )
            .push(
                Text::<IcedBackend>::new(
                    "Primary geometric atoms that can be used for indicators, progress, and custom UI elements.",
                )
                .body()
                .secondary()
                .width(Length::Fill),
            ),
    );

    // Circle Section
    page_col = page_col.push(
        VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .spacing(16.0)
            .push(Text::new("Circle").title3().bold())
            .push(
                HStack::<Message, IcedBackend>::new_generic()
                    .spacing(20.0)
                    .push(Circle::new(10.0).color(context.theme.colors.primary))
                    .push(Circle::new(20.0).color(context.theme.colors.success))
                    .push(Circle::new(30.0).color(context.theme.colors.danger))
                    .push(Circle::new(40.0).color(context.theme.colors.warning)),
            ),
    );

    // Capsule Section
    page_col = page_col.push(
        VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .spacing(16.0)
            .push(Text::new("Capsule").title3().bold())
            .push(
                HStack::<Message, IcedBackend>::new_generic()
                    .spacing(20.0)
                    .push(
                        Capsule::new(Length::Fixed(100.0), Length::Fixed(40.0))
                            .color(context.theme.colors.primary),
                    )
                    .push(
                        Capsule::new(Length::Fixed(60.0), Length::Fixed(40.0))
                            .color(context.theme.colors.success),
                    )
                    .push(
                        Capsule::new(Length::Fixed(200.0), Length::Fixed(20.0))
                            .color(context.theme.colors.text_secondary.scale_alpha(0.2)),
                    ),
            ),
    );

    // Timeline Integration (Example)
    page_col = page_col.push(
        VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .spacing(16.0)
            .push(Text::new("Usage in Timeline").title3().bold())
            .push(
                Text::new("Shapes are ideal for creating connected components like timelines.")
                    .caption1()
                    .secondary(),
            )
            .push(
                HStack::<Message, IcedBackend>::new_generic()
                    .spacing(16.0)
                    .push(
                        VStack::<Message, IcedBackend>::new_generic()
                            .width(Length::Fixed(20.0))
                            .align_x(Alignment::Center)
                            .push(
                                Rectangle::new(2.0.into(), Length::Fixed(20.0))
                                    .color(context.theme.colors.border),
                            )
                            .push(Circle::new(6.0).color(context.theme.colors.primary))
                            .push(
                                Rectangle::new(2.0.into(), Length::Fixed(20.0))
                                    .color(context.theme.colors.border),
                            ),
                    )
                    .push(
                        VStack::new()
                            .padding(Padding {
                                top: 12.0,
                                ..Default::default()
                            })
                            .push(Text::new("Active State Indicator").bold()),
                    ),
            ),
    );

    page_col.sidebar_toggle(Message::ToggleSidebar)
}
