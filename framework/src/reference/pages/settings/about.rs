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
                    .spacing(24.0)
                    .align_x(Alignment::Center)
                    .push(
                        Image::new(crate::assets::Asset::MeshBackground.path())
                            .width(Length::Fixed(120.0))
                            .height(Length::Fixed(120.0))
                            .radius(60.0),
                    )
                    .push(
                        VStack::<Message, IcedBackend>::new_generic()
                            .spacing(8.0)
                            .align_x(Alignment::Center)
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
                            ),
                    ),
            )
            .push(Divider::<IcedBackend>::new())
            // Implementation Reference
            .push(
                crate::containers::Section::new(
                    "About System",
                    VStack::new_generic()
                        .spacing(16.0)
                        .push(
                            Text::<IcedBackend>::new("Diagnostics and version information.")
                                .body()
                                .secondary(),
                        )
                        .push(
                            crate::views::CodeBlock::rust(
                                r#"
                                    // Accessing build metadata
                                    const VERSION: &str = env!("CARGO_PKG_VERSION");
                                "#,
                            )
                            .on_copy(Message::CopyCode),
                        ),
                )
                .width(Length::Fill),
            )
            .view(context)
    }))
}
