use iced::{Element, Task, Theme};
use peak_core::registry::ShellMode;
use peak_theme::{ThemeTokens, ThemeTone};
use peak_ui::prelude::*;

pub fn main() -> iced::Result {
    iced::application("PeakUI Showcase", App::update, App::view).run()
}

struct App {
    catalog: Catalog,
}

#[derive(Debug, Clone)]
enum Message {
    Catalog(CatalogMessage),
}

impl Default for App {
    fn default() -> Self {
        Self {
            catalog: Catalog::new(),
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Catalog(msg) => self.catalog.update(msg).map(Message::Catalog),
        }
    }

    fn view(&self) -> Element<'_, Message, Theme, iced::Renderer> {
        // Hardcode mode/tone for standalone example, or make them adjustable
        let mode = ShellMode::Desktop;
        let tone = ThemeTone::Light;
        let tokens = ThemeTokens::get(mode, tone);

        // Clone for closure capture
        let catalog = self.catalog.clone();

        responsive(mode, tokens, move |context| {
            catalog.view(&context).map(Message::Catalog)
        })
    }
}
