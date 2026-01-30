use crate::app::Message;
use peak_ui::prelude::*;
use peak_ui::reference::model::Page;

pub struct SidebarView {
    pub active_tab: Page,
}

impl SidebarView {
    pub fn new(active_tab: Page) -> Self {
        Self { active_tab }
    }
}

impl View<Message, IcedBackend> for SidebarView {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let t = context.theme;

        let mut col = VStack::<Message, IcedBackend>::new_generic()
            .spacing(8.0)
            .padding(Padding::from([24, 16]))
            .width(Length::Fill);

        // Branding
        col = col
            .push(
                HStack::<Message, IcedBackend>::new_generic()
                    .spacing(12.0)
                    .align_y(Alignment::Center)
                    .push(
                        Icon::<IcedBackend>::new("activity")
                            .size(20.0)
                            .color(t.colors.primary),
                    )
                    .push(Text::<IcedBackend>::new("PEAK HUB").bold().size(14.0)),
            )
            .push(Space::<IcedBackend>::new(0.0.into(), 32.0.into()));

        // Sections
        col = col
            .push(self.section_header("ORCHESTRATION"))
            .push(self.item("Dashboard", "layout-dashboard", Page::Introduction))
            .push(self.item("Swarm Health", "pulse", Page::SwarmDashboard))
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()));

        col = col
            .push(self.section_header("ECOSYSTEM"))
            .push(self.item("PeakDB", "database", Page::PeakDB))
            .push(self.item("PeakCloud", "cloud", Page::PeakCloud))
            .push(self.item("Roadmap", "milestone", Page::PeakHub))
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()));

        col.view(context)
    }
}

impl SidebarView {
    fn section_header(&self, label: &str) -> impl View<Message, IcedBackend> {
        Text::<IcedBackend>::new(label)
            .caption2()
            .bold()
            .secondary()
    }

    fn item(&self, label: &str, icon: &str, page: Page) -> impl View<Message, IcedBackend> {
        let active = self.active_tab == page;

        Button::<Message, IcedBackend>::new(
            HStack::<Message, IcedBackend>::new_generic()
                .spacing(12.0)
                .align_y(Alignment::Center)
                .push(Icon::<IcedBackend>::new(icon).size(14.0).secondary())
                .push(
                    Text::<IcedBackend>::new(label)
                        .caption1()
                        .width(Length::Fill),
                ),
        )
        .width(Length::Fill)
        .variant(if active {
            Variant::Soft
        } else {
            Variant::Ghost
        })
        .on_press(Message::SetTab(page))
    }
}
