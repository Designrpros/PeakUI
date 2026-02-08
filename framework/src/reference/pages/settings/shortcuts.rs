
use crate::prelude::*;
use crate::reference::AppPageResult;
use crate::reference::app::Message;

pub fn view(_context: &Context, _is_mobile: bool) -> AppPageResult {
    AppPageResult::new(crate::core::ProxyView::new(move |context| {
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
                    .push(Text::<IcedBackend>::new("Shortcuts").large_title().bold())
                    .push(
                        Text::<IcedBackend>::new(
                            "Register global keyboard accelerators for power users.",
                        )
                        .title3()
                        .secondary(),
                    ),
            )
            .push(Divider::<IcedBackend>::new())
            // Key Bindings Section
            .push(
                crate::layout::containers::Section::new(
                    "Key Bindings",
                    VStack::<Message, IcedBackend>::new_generic()
                        .spacing(16.0)
                        .push(shortcut_row("Toggle Sidebar", "Cmd B"))
                        .push(shortcut_row("Toggle Search", "Cmd K"))
                        .push(shortcut_row("Switch Theme", "Cmd T")),
                )
                .width(Length::Fill),
            )
            .push(Divider::<IcedBackend>::new())
            // Implementation Reference
            .push(
                crate::layout::containers::Section::new(
                    "Implementation",
                    crate::views::CodeBlock::rust(
                        r#"
                            // Handling keyboard events in your subscription
                            subscription: |state| {
                                iced::event::listen().map(Message::Event)
                            }
                        "#,
                    )
                    .on_copy(Message::CopyCode),
                )
                .width(Length::Fill),
            )
            .view(context)
    }))
}

fn shortcut_row(label: &str, keys: &str) -> impl View<Message, IcedBackend> {
    HStack::new_generic()
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .push(Text::<IcedBackend>::new(label.to_string()).body())
        .push(Space::new(Length::Fill, Length::Shrink))
        .push({
            let keys = keys.to_string();
            // Note: This internal ProxyView is fine, it just styles the container.
            // It receives the context from the parent view.
            crate::core::ProxyView::new(move |context| {
                container(
                    Text::<IcedBackend>::new(keys.clone())
                        .caption2()
                        .bold()
                        .view(context),
                )
                .padding(Padding::from([4, 8]))
                .style({
                    let theme = context.theme;
                    move |_| iced::widget::container::Style {
                        background: Some(theme.colors.surface_variant.into()),
                        border: Border {
                            radius: 4.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
                .into()
            })
        })
}
