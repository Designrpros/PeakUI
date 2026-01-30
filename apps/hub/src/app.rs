use crate::pages::{health, roadmap, start};
use crate::views::sidebar::SidebarView;
use peak_theme::{PeakTheme, ThemeTone};
use peak_ui::prelude::*;
use peak_ui::reference::model::Page;

pub struct HubApp {
    pub active_tab: Page,
    pub peak_id: String,
    pub show_sidebar: bool,
    pub window_width: f32,
}

impl Default for HubApp {
    fn default() -> Self {
        Self {
            active_tab: Page::Introduction,
            peak_id: "anonymous".to_string(),
            show_sidebar: true,
            window_width: 1200.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetTab(Page),
    ToggleSidebar,
    WindowResized(Size),
}

impl HubApp {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetTab(page) => {
                self.active_tab = page;
                if self.window_width < 900.0 {
                    self.show_sidebar = false;
                }
                Task::none()
            }
            Message::ToggleSidebar => {
                self.show_sidebar = !self.show_sidebar;
                Task::none()
            }
            Message::WindowResized(size) => {
                self.window_width = size.width;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let tokens = ThemeTokens::new(PeakTheme::Peak, ThemeTone::Light);
        let peak_id = self.peak_id.clone();
        let show_sidebar = self.show_sidebar;
        let active_tab = self.active_tab.clone();

        peak_ui::prelude::responsive(
            ShellMode::Desktop,
            tokens.clone(),
            Localization::default(),
            move |mut context| {
                context.peak_id = peak_id.clone();
                let is_mobile = context.is_slim();

                // 2. Content
                let page_result = match active_tab {
                    Page::PeakHub => roadmap::view(&context),
                    Page::SwarmDashboard => health::view(&context),
                    _ => start::view(&context),
                };

                let mut root = HStack::<Message, IcedBackend>::new_generic()
                    .width(Length::Fill)
                    .height(Length::Fill);

                if show_sidebar && !is_mobile {
                    // Use ProxyView for iced-native container styling
                    let tab = active_tab.clone();
                    root = root.push(ProxyView::new(move |ctx| {
                        let sidebar = SidebarView::new(tab.clone());
                        let sidebar_tokens = ThemeTokens::new(PeakTheme::Peak, ThemeTone::Light);

                        iced::widget::container(sidebar.view(ctx))
                            .width(Length::Fixed(260.0))
                            .height(Length::Fill)
                            .style(move |_| container::Style {
                                background: Some(sidebar_tokens.colors.surface.into()),
                                border: Border {
                                    color: sidebar_tokens.colors.border,
                                    width: 1.0,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .into()
                    }));
                }

                root.push(page_result.view).view(&context)
            },
        )
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::event::listen_with(|event, _status, _window| match event {
            iced::Event::Window(iced::window::Event::Resized(size)) => {
                Some(Message::WindowResized(size))
            }
            _ => None,
        })
    }
}
