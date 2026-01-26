use super::super::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult {
    PageResult::new(
        ComponentDoc::new(
            "Card",
            "A stylized container with a background, border, and shadow, used to group related information into a singular unit.",
            r#"
Card::new(
    VStack::new()
        .push(Text::new("Card Title").bold())
        .push(Text::new("Card description content goes here."))
)
"#,
            Arc::new(VStack::<Message, IcedBackend>::new_generic()
                .spacing(20.0)
                .push(ProxyView::new(move |ctx| {
                    container(
                        VStack::<Message, IcedBackend>::new_generic()
                            .spacing(8.0)
                            .push(Text::<IcedBackend>::new("Modern Card").title3().bold())
                            .push(Text::<IcedBackend>::new("Cards are the building blocks of dashboard interfaces.").body().secondary())
                            .view(ctx)
                    )
                    .padding(24)
                    .style(move |theme: &Theme| {
                        let colors = theme.extended_palette();
                        container::Style {
                            background: Some(colors.background.weak.color.into()),
                            border: Border {
                                color: colors.background.strong.color.scale_alpha(0.1),
                                width: 1.0,
                                radius: 16.0.into(),
                            },
                            ..Default::default()
                        }
                    })
                    .into()
                })))
        )
    )
}
