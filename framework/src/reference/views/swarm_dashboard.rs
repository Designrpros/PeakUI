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
        let content = VStack::new()
            .spacing(32.0)
            .padding(16.0)
            .width(Length::Fill)
            .push(
                VStack::new()
                    .spacing(8.0)
                    .push(Text::new("Swarm Dashboard").large_title().bold())
                    .push(Text::new("Manage your decentralized neural nodes.").secondary()),
            )
            .push(
                GlassCard::new(
                    VStack::new()
                        .spacing(16.0)
                        .push(
                            HStack::new()
                                .spacing(12.0)
                                .align_y(iced::Alignment::Center)
                                .push(
                                    Icon::new("key")
                                        .size(20.0)
                                        .color(context.theme.colors.primary),
                                )
                                .push(Text::new("Local Node Identity").headline().bold()),
                        )
                        .push(Divider::new())
                        .push(
                            VStack::new()
                                .spacing(4.0)
                                .push(Text::new("PeakID").caption1().secondary())
                                .push(
                                    Container::new(
                                        Text::new(&self.peak_id).font(Font::MONOSPACE).size(14.0),
                                    )
                                    .padding(12)
                                    .radius(8.0),
                                ),
                        ),
                )
                .padding(24.0),
            )
            .push(
                VStack::new()
                    .spacing(16.0)
                    .push(Text::new("Connected Nodes").headline().bold())
                    .push(
                        Container::new(
                            VStack::new()
                                .spacing(8.0)
                                .align_x(iced::Alignment::Center)
                                .padding(40)
                                .push(Icon::new("share-2").size(48.0).secondary())
                                .push(
                                    Text::new("No other nodes detected in the swarm yet.")
                                        .secondary(),
                                )
                                .push(
                                    Text::new("Ensure PeakCloud is active on your other devices.")
                                        .caption2()
                                        .secondary(),
                                ),
                        )
                        .width(Length::Fill),
                    ),
            );

        container(content.view(context))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode {
            role: "swarm_dashboard".to_string(),
            label: Some(format!("PeakID: {}", self.peak_id)),
            ..Default::default()
        }
    }
}
