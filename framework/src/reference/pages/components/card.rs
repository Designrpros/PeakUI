use crate::dsl::container;
use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view<B: Backend>(_context: &Context) -> PageResult<Message, B> {
    PageResult::new(
        ComponentDoc::<Message, B>::new(
            "Card",
            "A stylized container with a background, border, and shadow, used to group related information into a singular unit.",
            r#"
Card::new(
    VStack::new()
        .push(Text::new("Card Title").bold())
        .push(Text::new("Card description content goes here."))
)
"#,
            Arc::new(
                vstack![ProxyView::new(move |ctx| {
                    let colors = &ctx.theme.colors;
                    container::<Message, B>(
                        vstack![
                            text::<B>("Modern Card").title3().bold(),
                            text::<B>("Cards are the building blocks of dashboard interfaces.")
                                .body()
                                .secondary()
                        ]
                        .spacing(8.0),
                    )
                    .padding(24.0)
                    .background(colors.surface)
                    .border(1.0, colors.border)
                    .radius(16.0)
                    .view(ctx)
                })]
                .spacing(20.0),
            ),
        )
    )
}
