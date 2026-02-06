use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::Message;

pub fn view(_context: &Context, _is_mobile: bool) -> PageResult<Message> {
    vstack::<Message, IcedBackend>()
        .spacing(24.0)
        .padding(Padding {
            top: _context.safe_area.top,
            right: 20.0,
            bottom: _context.safe_area.bottom,
            left: 20.0,
        })
        .push(text::<IcedBackend>("Floating Docks").large_title().bold())
        .push(
            text::<IcedBackend>("Minimalist navigation with high-density shortcuts.")
                .title3()
                .secondary(),
        )
        .push(divider::<IcedBackend>())
        .push(ProxyView::new(move |theme_ctx| {
            let theme = theme_ctx.theme;
            container(
                Text::<IcedBackend>::new(
                    "The Dock system is designed to provide quick access to commonly used tools without visual clutter.",
                    )
                    .body()
                    .secondary()
                    .view(theme_ctx),
            )
            .padding(40)
            .style({
                let radius = theme_ctx.radius(12.0);
                let bg_color = theme.colors.surface_variant.scale_alpha(0.1);
                move |_| container::Style {
                    background: Some(bg_color.into()),
                    border: Border {
                        radius,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
            .into()
        }))
        .searchable("", "Search docks...", |_| Message::ToggleSearch)
        .sidebar_toggle(Message::ToggleSidebar)
}
