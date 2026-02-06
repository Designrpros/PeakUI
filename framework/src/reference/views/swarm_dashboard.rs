use super::super::app::Message;
use crate::prelude::*;

pub struct SwarmDashboardView {
    pub peak_id: String,
}

impl SwarmDashboardView {
    pub fn new(peak_id: String) -> Self {
        Self { peak_id }
    }
}

impl View<Message, IcedBackend> for SwarmDashboardView {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let content = VStack::<Message, IcedBackend>::new()
            .spacing(32.0)
            .padding(16.0)
            .width(Length::Fill)
            .push(
                VStack::<Message, IcedBackend>::new()
                    .spacing(8.0)
                    .push(text("Swarm Dashboard").large_title().bold())
                    .push(text("Manage your decentralized neural nodes.").secondary()),
            )
            .push(
                glass_card(
                    VStack::<Message, IcedBackend>::new()
                        .spacing(16.0)
                        .push(
                            HStack::<Message, IcedBackend>::new()
                                .spacing(12.0)
                                .align_y(iced::Alignment::Center)
                                .push(icon("key").size(20.0).color(context.theme.colors.primary))
                                .push(text("Local Node Identity").headline().bold()),
                        )
                        .push(divider())
                        .push(
                            VStack::<Message, IcedBackend>::new()
                                .spacing(4.0)
                                .push(text("PeakID").caption1().secondary())
                                .push(
                                    Container::new(
                                        text(&self.peak_id).font(Font::MONOSPACE).size(14.0),
                                    )
                                    .padding(12)
                                    .radius(8.0),
                                ),
                        ),
                )
                .padding(24.0),
            )
            .push(
                VStack::<Message, IcedBackend>::new()
                    .spacing(16.0)
                    .push(text("Connected Nodes").headline().bold())
                    .push(
                        Container::new(
                            VStack::<Message, IcedBackend>::new()
                                .spacing(8.0)
                                .align_x(iced::Alignment::Center)
                                .padding(40)
                                .push(icon("share-2").size(48.0).secondary())
                                .push(text("No other nodes detected in the swarm yet.").secondary())
                                .push(
                                    text("Ensure PeakCloud is active on your other devices.")
                                        .caption2()
                                        .secondary(),
                                ),
                        )
                        .width(Length::Fill),
                    ),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .view(context)
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("swarm_dashboard")
            .with_label(format!("PeakID: {}", self.peak_id))
    }
}
