use crate::reference::app::Message;
use crate::navigation::PageResult;
use crate::prelude::*;

pub fn view(context: &Context, is_mobile: bool) -> PageResult<Message> {
    let _ = context;
    PageResult::new(ProxyView::<Message, IcedBackend>::new(move |ctx| {
        let t = ctx.theme;
        let mut root = VStack::new().width(Length::Fill).spacing(0.0);

        // Header
        root = root.push(
            Container::new(
                VStack::new()
                    .spacing(24.0)
                    .align_x(Alignment::Start)
                    .push(
                        Button::new(
                            HStack::new()
                                .spacing(8.0)
                                .align_y(Alignment::Center)
                                .push(Icon::new("arrow-left").size(16.0))
                                .push(Text::new("Back to Landing").caption1().bold()),
                        )
                        .variant(Variant::Ghost)
                        .on_press(Message::SetTab(crate::reference::model::Page::Landing)),
                    )
                    .push(
                        HStack::new()
                            .spacing(16.0)
                            .align_y(Alignment::Center)
                            .push(Icon::new("eye").size(32.0).color(t.colors.primary))
                            .push(Text::new("PeakUI").size(48.0).bold()),
                    )
                    .push(
                        Text::new("The First AI-Native Semantic Interface")
                            .size(24.0)
                            .secondary(),
                    ),
            )
            .padding(if is_mobile {
                Padding::from(24.0)
            } else {
                Padding {
                    top: 80.0,
                    right: 80.0,
                    bottom: 40.0,
                    left: 80.0,
                }
            })
            .width(Length::Fill)
            .background(Color::WHITE),
        );

        // Content
        root = root.push(
            Container::new(
                VStack::new()
                    .spacing(32.0)
                    .push(Text::new("Beyond Pixels").headline().bold())
                    .push(Text::new("PeakUI is not just about drawing buttons; it's about defining the bridge between human intent and machine execution. By exposing a rich semantic tree instead of raw pixels, PeakUI enables AI agents to understand, interact with, and even generate user interfaces in real-time.").body().secondary())
                    .push(
                        ResponsiveGrid::new()
                            .spacing(24.0)
                            .push(feature_item("Universal", "Deploy to Linux, macOS, Windows, WASM, TUI, and VR from one codebase.", "globe", ctx))
                            .push(feature_item("Deterministic", "Zero-latency interaction for automated agents.", "target", ctx))
                            .push(feature_item("Sustainable", "Reduces AI vision energy consumption by up to 99%.", "leaf", ctx))
                    )
            )
            .padding(if is_mobile { Padding::from(24.0) } else { Padding { top: 40.0, right: 80.0, bottom: 80.0, left: 80.0 } })
        );

        ScrollView::new(root).view(ctx)
    }))
}

fn feature_item(
    title: &str,
    desc: &str,
    icon: &str,
    context: &Context,
) -> Container<Message, IcedBackend> {
    let t = context.theme;
    Container::new(
        VStack::new()
            .spacing(16.0)
            .push(Icon::new(icon).size(24.0).color(t.colors.primary))
            .push(Text::new(title).bold())
            .push(Text::new(desc).caption1().secondary()),
    )
    .padding(20.0)
    .border(1.0, t.colors.border)
    .radius(8.0)
    .width(Length::Fill)
}
