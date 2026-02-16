use super::super::app::{Message, ShellMessage};
use crate::prelude::*;

pub struct TabBarView {
    pub navigation_mode: String,
}

impl TabBarView {
    pub fn new(navigation_mode: String) -> Self {
        Self { navigation_mode }
    }
}

impl View<Message, IcedBackend> for TabBarView {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let nav_mode = &self.navigation_mode;

        // Expanded Tab List for "Different Purposes"
        ToolbarGroup::new()
            .padding(Padding {
                top: if context.is_slim() { 18.0 } else { 24.0 },
                right: if context.is_slim() { 16.0 } else { 24.0 }, // Reduced sides
                bottom: if context.is_slim() { 18.0 } else { 24.0 },
                left: if context.is_slim() { 16.0 } else { 24.0 }, // Reduced sides
            })
            .spacing(if context.is_slim() { 32.0 } else { 40.0 }) // Increased spacing
            // 1. Start: The Narrative Book
            .push(
                ToolbarItem::new()
                    .icon("sparkles")
                    .icon_size(if context.is_slim() { 24.0 } else { 28.0 })
                    .active(nav_mode == "Start")
                    .on_press(Message::Shell(ShellMessage::SetNavigationMode(
                        "Start".into(),
                    ))),
            )
            // 2. Catalog: Visual Gallery & Components
            .push(
                ToolbarItem::new()
                    .icon("layout-grid")
                    .icon_size(if context.is_slim() { 24.0 } else { 28.0 })
                    .active(nav_mode == "Catalog")
                    .on_press(Message::Shell(ShellMessage::SetNavigationMode(
                        "Catalog".into(),
                    ))),
            )
            // 4. Data: PeakDB & Cloud
            .push(
                ToolbarItem::new()
                    .icon("database")
                    .icon_size(if context.is_slim() { 24.0 } else { 28.0 })
                    .active(nav_mode == "Data")
                    .on_press(Message::Shell(ShellMessage::SetNavigationMode(
                        "Data".into(),
                    ))),
            )
            // 5. Settings: Preferences
            .push(
                ToolbarItem::new()
                    .icon("settings-2")
                    .icon_size(if context.is_slim() { 24.0 } else { 28.0 })
                    .active(nav_mode == "Settings")
                    .on_press(Message::Shell(ShellMessage::SetNavigationMode(
                        "Settings".into(),
                    ))),
            )
            .shadow(iced::Shadow {
                color: iced::Color::from_rgba8(0, 0, 0, 0.1),
                offset: iced::Vector::new(0.0, 10.0),
                blur_radius: 40.0,
            })
            .view(context)
    }
}
