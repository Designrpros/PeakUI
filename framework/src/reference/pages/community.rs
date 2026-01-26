use super::super::app::Message;
use super::super::page::PageResult;
use crate::prelude::*;

pub fn view(_context: &Context, is_mobile: bool) -> PageResult {
    VStack::new_generic()
        .spacing(24.0)
        .padding(Padding {
            top: 96.0,
            right: if is_mobile { 20.0 } else { 64.0 },
            bottom: 120.0,
            left: if is_mobile { 20.0 } else { 64.0 },
        })
        .push(Text::<IcedBackend>::new("Community").large_title().bold())
        .push(
            Text::<IcedBackend>::new("Connect with other PeakOS developers.")
                .title3()
                .secondary(),
        )
        .push(Divider::<IcedBackend>::new())
        .push(ProxyView::new(move |theme_ctx| {
            let theme = theme_ctx.theme;
            container(
                Text::<IcedBackend>::new(
                    "Join our Discord or GitHub Discussions to share your projects.",
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
        .sidebar_toggle(Message::ToggleSidebar)
}
