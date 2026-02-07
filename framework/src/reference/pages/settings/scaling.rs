use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::Message;

pub fn view(_context: &Context, _is_mobile: bool) -> PageResult<Message> {
    PageResult::new(crate::core::ProxyView::new(move |context| {
        VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .spacing(32.0)
            .padding(Padding {
                top: context.safe_area.top,
                right: 20.0,
                bottom: context.safe_area.bottom,
                left: 20.0,
            })
            // Hero Header
            .push(
                VStack::<Message, IcedBackend>::new_generic()
                    .spacing(8.0)
                    .push(Text::<IcedBackend>::new("Scaling").large_title().bold())
                    .push(
                        Text::<IcedBackend>::new("PeakUI is built on a responsive unit system that scales gracefully.")
                            .title3()
                            .secondary(),
                    )
            )
            .push(Divider::<IcedBackend>::new())

            // Interface Density Section
            .push(
                crate::containers::Section::new(
                    "Interface Density",
                    VStack::<Message, IcedBackend>::new_generic()
                        .spacing(16.0)
                        .push(
                            Text::<IcedBackend>::new(
                                "Adjust interface density and font scaling to match your preference.",
                            )
                            .secondary(),
                        )
                        .push(
                            HStack::<Message, IcedBackend>::new_generic()
                                .spacing(12.0)
                                .push(
                                    Button::<Message>::label("Compact")
                                        .variant(if context.theme.scaling < 0.9 { Variant::Soft } else { Variant::Ghost })
                                        .on_press(Message::SetScaling(0.8))
                                )
                                .push(
                                    Button::<Message>::label("Default")
                                        .variant(if (context.theme.scaling - 1.0).abs() < 0.01 { Variant::Soft } else { Variant::Ghost })
                                        .on_press(Message::SetScaling(1.0))
                                )
                                .push(
                                    Button::<Message>::label("Comfortable")
                                        .variant(if context.theme.scaling > 1.1 { Variant::Soft } else { Variant::Ghost })
                                        .on_press(Message::SetScaling(1.2))
                                ),
                        )
                )
                .width(Length::Fill)
            )
            .push(Divider::<IcedBackend>::new())

            // Implementation Reference
            .push(
                 crate::containers::Section::new(
                    "Implementation",
                    crate::views::CodeBlock::rust(
                        r#"
                            // Using the scaled context in your views
                            let spacing = context.scale(16.0);
                            let font_size = context.scale(theme.typography.body.size);
                        "#
                    )
                    .on_copy(Message::CopyCode)
                )
                .width(Length::Fill)
            )
            .view(context)
    }))
}
