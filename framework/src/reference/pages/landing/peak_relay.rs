use crate::engine::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::Message;

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
                            .push(Icon::new("share-2").size(32.0).color(t.colors.primary))
                            .push(Text::new("PeakRelay").size(48.0).bold()),
                    )
                    .push(
                        Text::new("Deterministic Swarm Networking")
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
                    .push(Text::new("The Intelligence Mesh").headline().bold())
                    .push(Text::new("PeakRelay is a P2P communication protocol designed for low-latency coordination of autonomous swarms. It replaces centralized message brokers with a gossip-based mesh that is resilient to network partitions and hardware failures.").body().secondary())
                    .push(
                        ResponsiveGrid::new()
                            .spacing(24.0)
                            .push(feature_item("Gossip-Based", "Efficient state propagation across thousands of nodes.", "users", ctx))
                            .push(feature_item("Byzantine Fault-Tolerant", "Resilient against malicious or failing actors.", "shield-check", ctx))
                            .push(feature_item("Edge-First", "Optimized for low-latency, intermittently connected devices.", "wifi-off", ctx))
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
            .push(
                Icon::new(icon.to_string())
                    .size(24.0)
                    .color(t.colors.primary),
            )
            .push(Text::new(title.to_string()).bold())
            .push(Text::new(desc.to_string()).caption1().secondary()),
    )
    .padding(20.0)
    .border(1.0, t.colors.border)
    .radius(8.0)
    .width(Length::Fill)
}
