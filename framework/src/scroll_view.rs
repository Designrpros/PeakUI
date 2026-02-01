use crate::core::{Backend, Context, IcedBackend, TermBackend, View};
use iced::Length;

/// A scrollable container that wraps content and provides styled scrollbars.
pub struct ScrollView<Message: 'static, B: Backend = IcedBackend> {
    content: Box<dyn View<Message, B>>,
    width: Length,
    height: Length,
    id: Option<&'static str>,
    show_indicators: bool,
}

impl<Message: 'static> ScrollView<Message, IcedBackend> {
    /// Creates a new `ScrollView` for the Iced backend.
    pub fn new(content: impl View<Message, IcedBackend> + 'static) -> Self {
        Self::new_generic(content)
    }

    /// Creates a new `ScrollView` for the Iced backend from a boxed view.
    pub fn from_boxed(content: Box<dyn View<Message, IcedBackend>>) -> Self {
        Self {
            content,
            width: Length::Fill,
            height: Length::Fill,
            id: None,
            show_indicators: true,
        }
    }
}

impl<Message: 'static> ScrollView<Message, TermBackend> {
    /// Creates a new `ScrollView` for the Term backend (TUI).
    pub fn new_tui(content: impl View<Message, TermBackend> + 'static) -> Self {
        Self::new_generic(content)
    }
}

impl<Message: 'static, B: Backend> ScrollView<Message, B> {
    /// Creates a new generic `ScrollView` with the given content.
    pub fn new_generic(content: impl View<Message, B> + 'static) -> Self {
        Self {
            content: Box::new(content),
            width: Length::Fill,
            height: Length::Fill,
            id: None,
            show_indicators: true,
        }
    }

    /// Sets the width of the `ScrollView`.
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the `ScrollView`.
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Hides the scroll indicators (scrollbars).
    pub fn hide_indicators(mut self) -> Self {
        self.show_indicators = false;
        self
    }

    /// Alias for `hide_indicators()`.
    pub fn hide(self) -> Self {
        self.hide_indicators()
    }

    /// Sets the ID of the `ScrollView`.
    pub fn id(mut self, id: &'static str) -> Self {
        self.id = Some(id);
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for ScrollView<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::scroll_view(
            self.content.view(context),
            self.width,
            self.height,
            self.id,
            self.show_indicators,
            context,
        )
    }
}
