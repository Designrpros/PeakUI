use super::super::app::Message;
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
            .padding(Padding::from([12, 16]))
            // 1. Start: The Narrative Book
            .push(
                ToolbarItem::new()
                    .icon("sparkles")
                    .active(nav_mode == "Start")
                    .on_press(Message::SetNavigationMode("Start".into())),
            )
            // 2. Catalog: Visual Gallery & Components
            .push(
                ToolbarItem::new()
                    .icon("grid")
                    .active(nav_mode == "Catalog")
                    .on_press(Message::SetNavigationMode("Catalog".into())),
            )
            // 4. Data: PeakDB & Cloud
            .push(
                ToolbarItem::new()
                    .icon("system")
                    .active(nav_mode == "Data")
                    .on_press(Message::SetNavigationMode("Data".into())),
            )
            // 5. Settings: Preferences
            .push(
                ToolbarItem::new()
                    .icon("settings")
                    .active(nav_mode == "Settings")
                    .on_press(Message::SetNavigationMode("Settings".into())),
            )
            .view(context)
    }
}
