use crate::reference::intelligence::mcp;

use crate::prelude::*;
use crate::reference::AppPageResult;
use crate::reference::app::Message;

pub fn view(_context: &Context, _is_mobile: bool) -> AppPageResult {
    AppPageResult::new(
        VStack::new_generic()
            .spacing(24.0)
            .padding(Padding {
                top: _context.safe_area.top,
                right: 20.0,
                bottom: _context.safe_area.bottom,
                left: 20.0,
            })
            .push(
                Text::<IcedBackend>::new("Framework Semantic Schema")
                    .large_title()
                    .bold(),
            )
            .push(
                Text::<IcedBackend>::new("This metadata is served via MCP for LLM extraction.")
                    .title3()
                    .secondary(),
            )
            .push(Divider::<IcedBackend>::new())
            .push(ProxyView::new(move |ctx| {
                let theme = ctx.theme;
                container(
                    Text::<IcedBackend>::new(format!("{:#?}", mcp::get_framework_schema()))
                        .body()
                        .view(ctx),
                )
                .padding(24)
                .width(Length::Fill)
                .style({
                    let radius = if cfg!(target_arch = "wasm32") {
                        0.0
                    } else {
                        12.0
                    }
                    .into();
                    let bg_color = theme.colors.surface_variant.scale_alpha(0.3);
                    let border_color = theme.colors.border.scale_alpha(0.1);
                    move |_| container::Style {
                        background: Some(bg_color.into()),
                        border: Border {
                            radius,
                            color: border_color,
                            width: 1.0,
                        },
                        ..Default::default()
                    }
                })
                .into()
            })),
    )
    .sidebar_toggle(Message::ToggleSidebar)
}
