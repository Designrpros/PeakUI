use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context, _is_mobile: bool) -> PageResult {
    let preview = VStack::<Message, IcedBackend>::new_generic()
        .spacing(24.0)
        .push(Text::<IcedBackend>::new("Key Bindings").title2().bold())
        .push(shortcut_row("Toggle Sidebar", "⌘ B"))
        .push(shortcut_row("Toggle Search", "⌘ K"))
        .push(shortcut_row("Switch Theme", "⌘ T"));

    PageResult::new(ComponentDoc::new(
        "Keyboard Shortcuts",
        "Register global keyboard accelerators for power users.",
        r#"
// Handling keyboard events
subscription: |state| {
    iced::event::listen().map(Message::Event)
}
"#,
        Arc::new(preview),
    ))
}

fn shortcut_row(label: &str, keys: &str) -> impl View<Message, IcedBackend> {
    HStack::new_generic()
        .width(Length::Fill)
        .push(Text::<IcedBackend>::new(label).body())
        .push(Space::new(Length::Fill, Length::Shrink))
        .push({
            let keys = keys.to_string();
            ProxyView::new(move |context| {
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
