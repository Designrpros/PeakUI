use crate::app::Message;
use peak_ui::prelude::*;

pub fn view(context: &Context) -> PageResult<Message> {
    PageResult::new(SwarmHealthPage {
        peak_id: context.peak_id.clone(),
    })
}

struct SwarmHealthPage {
    peak_id: String,
}

impl View<Message, IcedBackend> for SwarmHealthPage {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let t = context.theme;

        let content = VStack::new()
            .spacing(48.0)
            .padding(Padding {
                top: 48.0,
                right: 48.0,
                bottom: 80.0,
                left: 48.0,
            })
            .width(Length::Fill)
            .push(
                VStack::new()
                    .spacing(12.0)
                    .push(Text::new("Swarm Health").large_title().bold())
                    .push(Text::new("Manage your decentralized neural nodes.").secondary()),
            )
            .push(
                VStack::new()
                    .spacing(24.0)
                    .push(Text::new("Local Node Identity").headline().bold())
                    .push(
                        Container::new(
                            VStack::new()
                                .spacing(12.0)
                                .push(Text::new("PEAK ID").caption2().secondary().bold())
                                .push(
                                    Text::new(&self.peak_id)
                                        .font(Font::MONOSPACE)
                                        .size(14.0)
                                        .color(t.colors.primary),
                                ),
                        )
                        .padding(24.0)
                        .background(t.colors.surface)
                        .border(1.0, t.colors.border)
                        .radius(16.0)
                        .width(Length::Fill),
                    ),
            )
            .push(
                VStack::new()
                    .spacing(24.0)
                    .push(Text::new("Connected Nodes").headline().bold())
                    .push(
                        Container::new(
                            VStack::new()
                                .spacing(16.0)
                                .align_x(Alignment::Center)
                                .padding(64.0)
                                .push(
                                    Icon::new("share-2")
                                        .size(48.0)
                                        .color(t.colors.text_secondary.scale_alpha(0.5)),
                                )
                                .push(
                                    Text::new("No other nodes detected in the swarm yet.")
                                        .secondary()
                                        .align_center(),
                                )
                                .push(
                                    Text::new("Ensure PeakCloud is active on your other devices.")
                                        .caption2()
                                        .secondary()
                                        .align_center(),
                                ),
                        )
                        .width(Length::Fill)
                        .background(t.colors.surface.scale_alpha(0.3))
                        .radius(16.0),
                    ),
            );

        content.view(context)
    }
}
