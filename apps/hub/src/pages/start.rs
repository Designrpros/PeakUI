use crate::app::Message;
use peak_ui::prelude::*;

pub fn view(_context: &Context) -> PageResult<Message> {
    PageResult::new(StartPage)
}

struct StartPage;

impl View<Message, IcedBackend> for StartPage {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let t = context.theme;

        VStack::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .padding(Padding {
                top: 80.0,
                right: 48.0,
                bottom: 80.0,
                left: 48.0,
            })
            .spacing(48.0)
            .push(
                VStack::new()
                    .align_x(Alignment::Center)
                    .spacing(24.0)
                    .push(
                        Container::new(
                            Icon::new("activity")
                                .size(48.0)
                                .color(Color::from_rgb8(177, 140, 107)) // Match the icon color from image
                        )
                        .padding(40.0)
                        .background(Color::from_rgb8(245, 240, 232)) // Match the soft beige background
                        .radius(32.0)
                    )
                    .push(
                        Text::new("Peak Hub")
                            .large_title()
                            .bold()
                            .color(t.colors.text_primary)
                    )
                    .push(
                        Text::new("Your central swarm command tower.")
                            .body()
                            .secondary()
                            .color(t.colors.text_secondary)
                    )
            )
            .push(
                HStack::new()
                    .spacing(64.0)
                    .push(stat_block("Devices", "1", context))
                    .push(stat_block("Latency", "4ms", context))
                    .push(stat_block("Uptime", "99.9%", context))
            )
            .push(
                Text::new("Peak Hub is the dedicated application for managing the PeakSuite ecosystem. From here, you can orchestrate your data layer, monitor P2P connectivity, and approve autonomous intelligence actions.")
                    .body()
                    .secondary()
                    .align_center()
                    .width(Length::Fixed(500.0))
            )
            .view(context)
    }
}

fn stat_block(label: &str, value: &str, _context: &Context) -> VStack<Message, IcedBackend> {
    VStack::new()
        .spacing(8.0)
        .align_x(Alignment::Start)
        .width(Length::Fixed(120.0))
        .push(Text::new(label).caption2().secondary().bold())
        .push(Text::new(value).title2().bold())
}
