use crate::core::{Context, DeviceType, IcedBackend};
use crate::prelude::*;
use peak_core::registry::ShellMode;
use peak_theme::ThemeTokens;

use super::message::Message;
use super::state::*;
use crate::reference::AppPage;

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        // Debug logging disabled for performance - was causing console spam
        // log::info!(
        //     "App::view: {} Mode, width: {}, show_sidebar: {}",
        //     mode,
        //     self.window_width,
        //     self.show_sidebar
        // );
        // Context is now provided by the helper method, significantly reducing boilerplate
        // and ensuring consistent state (theme, size, mode) across the application.
        if self.show_landing {
            // Create context directly without responsive wrapper for performance
            let size = Size::new(self.window_width, 800.0); // Height doesn't matter for landing
            let mut context = Context::new(
                self.shell_mode(),
                self.context().theme,
                size,
                self.localization.clone(),
            );
            context.tick = self.tick;

            // Capture the search query state
            let query = self.search_query.clone();
            let typewriter_text = self.typewriter_text.clone();
            let active_tab = self.active_tab.clone();
            let db_records = self.db.get_all();

            let content: Element<'_, Message> = match &active_tab {
                AppPage::PeakOSDetail => {
                    crate::reference::pages::landing::peak_os::view(&context, context.is_slim())
                        .view
                        .view(&context)
                        .into()
                }
                AppPage::PeakUIDetail => {
                    crate::reference::pages::landing::peak_ui::view(&context, context.is_slim())
                        .view
                        .view(&context)
                        .into()
                }
                AppPage::PeakDBDetail => crate::reference::pages::landing::peak_db::view(
                    &context,
                    context.is_slim(),
                    db_records.to_vec(),
                )
                .view
                .view(&context)
                .into(),
                AppPage::PeakRelayDetail => {
                    crate::reference::pages::landing::peak_relay::view(&context, context.is_slim())
                        .view
                        .view(&context)
                        .into()
                }
                AppPage::PeakHubDetail => {
                    crate::reference::pages::landing::peak_hub::view(&context, context.is_slim())
                        .view
                        .view(&context)
                        .into()
                }
                _ => crate::reference::pages::landing::view(&context, &query, &typewriter_text)
                    .into(),
            };

            let theme = self.context().theme;
            return container(content)
                .style(move |_| container::Style {
                    background: Some(theme.colors.background.into()),
                    ..Default::default()
                })
                .into();
        }

        // 1. Prepare Content
        let content = crate::reference::views::ContentView::new(self);

        let context_menu_pos = self.context_menu_pos;

        // Neural Export (Exported in update for performance)

        let peak_id = self.peak_id.clone();
        let tick = self.tick;

        // Clone data needed for the responsive closure to avoid 'self' lifetime issues
        let theme = self.theme;
        let tone = self.theme_tone;
        let scaling = self.scaling;
        let localization_closure = self.localization.clone();

        let view = crate::core::responsive(move |device_type| {
            let mut tokens = ThemeTokens::with_theme(theme, tone);
            tokens.scaling = scaling;

            let mut context = Context::new(
                if device_type == DeviceType::Mobile {
                    ShellMode::Mobile
                } else {
                    ShellMode::Desktop
                },
                tokens,
                Size::new(1280.0, 800.0), // Responsive viewport
                localization_closure.clone(),
            );
            context.tick = tick;
            context.peak_id = peak_id.clone().into();

            // Main App Content
            let base_content = content.clone().into_box();

            let mut stack = crate::layout::ZStack::<Message, IcedBackend>::new_generic()
                .push(base_content)
                .width(Length::Fill)
                .height(Length::Fill);

            // Overlay Context Menu
            if let Some(pos) = context_menu_pos {
                let menu = crate::views::ContextMenu::new()
                    .item(
                        "Reload",
                        "rotate-cw",
                        Message::ContextMenuAction("Reload".to_string()),
                    )
                    .item(
                        "Inspect",
                        "search-code",
                        Message::ContextMenuAction("Inspect".to_string()),
                    )
                    .item("Close", "circle-x", Message::CloseContextMenu);

                stack = stack.push(
                    crate::elements::atoms::Container::<Message, IcedBackend>::new(menu)
                        .padding(Padding {
                            top: pos.y,
                            left: pos.x,
                            ..Default::default()
                        })
                        .into_box(),
                );
            }

            // Overlay Sudo Prompt
            if let Some(sudo) = &content.state.pending_sudo_action {
                let prompt = crate::elements::atoms::Container::<Message, IcedBackend>::new(
                    crate::layout::VStack::<Message, IcedBackend>::new_generic()
                        .push(
                            crate::elements::atoms::Text::<IcedBackend>::new(
                                "Neural Sudo Permission",
                            )
                            .title1(),
                        )
                        .push(crate::elements::atoms::Text::<IcedBackend>::new(format!(
                            "AI wants to perform: {:?}",
                            sudo.message
                        )))
                        .push(
                            crate::layout::HStack::<Message, IcedBackend>::new_generic()
                                .push(
                                    crate::elements::controls::Button::<Message, IcedBackend>::new(
                                        crate::elements::atoms::Text::<IcedBackend>::new("Deny")
                                            .color(Color::from_rgb8(50, 50, 50)),
                                    )
                                    .variant(crate::style::Variant::Ghost)
                                    .intent(crate::style::Intent::Neutral)
                                    .on_press(Message::SudoDeny),
                                )
                                .push(
                                    crate::elements::controls::Button::<Message, IcedBackend>::new(
                                        crate::elements::atoms::Text::<IcedBackend>::new("Approve")
                                            .color(Color::WHITE),
                                    )
                                    .variant(crate::style::Variant::Solid)
                                    .intent(crate::style::Intent::Danger)
                                    .on_press(Message::SudoApprove),
                                )
                                .spacing(10.0)
                                .align_x(Alignment::Center),
                        )
                        .spacing(20.0)
                        .align_x(Alignment::Center),
                )
                .padding(30.0)
                .background(Color::WHITE)
                .border(1.0, Color::from_rgb8(220, 220, 220))
                .radius(16.0)
                .shadow(Shadow {
                    color: Color::from_rgba8(0, 0, 0, 0.1),
                    offset: Vector::new(0.0, 10.0),
                    blur_radius: 40.0,
                });

                // Wrap in centering container
                let centered =
                    crate::elements::atoms::Container::<Message, IcedBackend>::new(prompt)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x(Length::Fill)
                        .center_y(Length::Fill);

                stack = stack.push(centered.into_box());
            }

            stack.into_box()
        });

        // Convert View trait object to Element
        let context = self.context();
        view.view(&context)
    }
}
