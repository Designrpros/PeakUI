use super::super::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult {
    PageResult::new(
        ComponentDoc::new(
            "Overlay",
            "A layout modifier that places a view on top of another base view, constrained by the base's bounds.",
            r#"
let base = Rectangle::new(64.0, 64.0)
    .color(theme.colors.surface_variant)
    .radius(12.0);

let dot = Circle::new(8.0, Some(theme.colors.success));

base.overlay(
    container(dot).padding(4.0), 
    Alignment::End
)
"#,
            Arc::new(
                HStack::<Message, IcedBackend>::new_generic()
                    .spacing(24.0)
                    .push(
                        // Example 1: Status Indicator
                        Rectangle::new(64.0.into(), 64.0.into())
                            .color(Color::from_rgb8(60, 60, 60))
                            .radius(12.0)
                            .overlay(
                                ProxyView::new(move |ctx| {
                                    container(
                                        Circle::<IcedBackend>::new(8.0)
                                            .color(ctx.theme.colors.success)
                                            .view(ctx)
                                    )
                                    .padding(4.0)
                                    .into()
                                }),
                                Alignment::End
                            )
                    )
                    .push(
                        // Example 2: Border overlay
                        ProxyView::new(move |ctx| {
                            container(
                                Text::<IcedBackend>::new("Layered View")
                                    .body()
                                    .bold()
                                    .view(ctx)
                            )
                            .padding(24.0)
                            .into()
                        })
                        .overlay(
                            ProxyView::new(move |ctx| {
                                let primary = ctx.theme.colors.primary;
                                container(Space::<IcedBackend>::new(0.0.into(), 0.0.into()).view(ctx))
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .style(move |_| container::Style {
                                        border: Border {
                                            color: primary,
                                            width: 2.0,
                                            radius: 8.0.into(),
                                        },
                                        ..Default::default()
                                    })
                                    .into()
                            }),
                            Alignment::Center
                        )
                    )
            )
        )
    )
}
