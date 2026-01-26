use super::super::app::Message;
use super::super::page::PageResult;
use crate::prelude::*;

use super::super::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context, _is_mobile: bool) -> PageResult {
    let mcp_schema = format!("{:#?}", crate::reference::mcp::get_framework_schema());
    let preview = VStack::<Message, IcedBackend>::new_generic()
        .spacing(24.0)
        .push(Text::<IcedBackend>::new("The System Kernel").title2().bold())
        .push(
            Text::<IcedBackend>::new(
                "PeakOS Core exposes a unified API surface that abstracts over the underlying hardware and OS primitives.",
            )
            .secondary(),
        )
        .push(Divider::<IcedBackend>::new())
        .push(Text::<IcedBackend>::new("MCP Schema Export").caption1().bold().secondary())

        .push(ProxyView::new(move |context| {
            let mcp_schema = mcp_schema.clone();
            container(
                ScrollView::new(
                    Text::<IcedBackend>::new(mcp_schema)
                        .caption2()
                        .font(Font::MONOSPACE)
                )
                .height(Length::Fixed(300.0))
                .view(context)
            )
            .padding(16)
            .style({
                let theme = context.theme;
                move |_| iced::widget::container::Style {
                     background: Some(theme.colors.surface_variant.scale_alpha(0.2).into()),
                     border: Border {
                         radius: 8.0.into(),
                         ..Default::default()
                     },
                     ..Default::default()
                }
            })
            .into()
        }));

    PageResult::new(ComponentDoc::new(
        "PeakOS Core",
        "The fundamental runtime providing IO, networking, and hardware access.",
        r#"
// System-level primitives are safe and async
let battery = System::power().battery_level().await?;
let wifi = System::network().scan().await?;
"#,
        Arc::new(preview),
    ))
}
