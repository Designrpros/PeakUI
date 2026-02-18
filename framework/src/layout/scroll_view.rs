use crate::core::{Backend, Context, IcedBackend, ScrollDirection, View};
use iced::Length;

/// A scrollable container that wraps content and provides styled scrollbars.
pub struct ScrollView<Message: 'static + Send + Sync, B: Backend = IcedBackend> {
    content: Box<dyn View<Message, B> + Send + Sync>,
    width: Length,
    height: Length,
    id: Option<&'static str>,
    show_indicators: bool,
    direction: ScrollDirection,
}

impl<Message: 'static + Send + Sync, B: Backend> ScrollView<Message, B> {
    /// Creates a new generic `ScrollView` with the given content.
    pub fn new(content: impl View<Message, B> + Send + Sync + 'static) -> Self {
        Self {
            content: Box::new(content),
            width: Length::Fill,
            height: Length::Fill,
            id: None,
            show_indicators: true,
            direction: ScrollDirection::Vertical,
        }
    }

    /// Alias for `new`.
    pub fn new_generic(content: impl View<Message, B> + Send + Sync + 'static) -> Self {
        Self::new(content)
    }

    /// Creates a new `ScrollView` from a boxed view.
    pub fn from_boxed(content: Box<dyn View<Message, B> + Send + Sync>) -> Self {
        Self {
            content,
            width: Length::Fill,
            height: Length::Fill,
            id: None,
            show_indicators: true,
            direction: ScrollDirection::Vertical,
        }
    }
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the `ScrollView`.
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the scroll direction.
    pub fn direction(mut self, direction: ScrollDirection) -> Self {
        self.direction = direction;
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

impl<Message: 'static + Send + Sync, B: Backend> View<Message, B> for ScrollView<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let nested_context = context.clone().with_nested_scroll();
        B::scroll_view(
            self.content.view(&nested_context),
            self.width,
            self.height,
            self.id,
            self.show_indicators,
            self.direction,
            context,
        )
    }
}
