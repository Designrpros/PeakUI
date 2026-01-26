use crate::atoms::Text;
use crate::core::{Backend, Context, IcedBackend, TermBackend, View};
use crate::scroll_view::ScrollView;
use iced::{Element, Renderer, Theme};

pub struct Console<Message: 'static, B: Backend = IcedBackend> {
    content: String,
    input: Option<Box<dyn View<Message, B>>>,
}

impl<Message: 'static> Console<Message, IcedBackend> {
    pub fn new(content: impl Into<String>) -> Self {
        Self::new_generic(content)
    }
}

impl<Message: 'static> Console<Message, TermBackend> {
    pub fn new_tui(content: impl Into<String>) -> Self {
        Self::new_generic(content)
    }
}

impl<Message: 'static, B: Backend> Console<Message, B> {
    pub fn new_generic(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            input: None,
        }
    }

    pub fn input(mut self, input: impl View<Message, B> + 'static) -> Self {
        self.input = Some(Box::new(input));
        self
    }
}

impl<Message: Clone + 'static> View<Message, IcedBackend> for Console<Message, IcedBackend> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;

        let output = Text::<IcedBackend>::new(&self.content)
            .size(12.0)
            .color(theme.colors.text_primary);

        // Use standard Column for internal layout to avoid recursive view calls if needed,
        // but VStack is preferred for PeakUI context.
        let mut col = iced::widget::Column::new()
            .push(ScrollView::new(output).view(context))
            .spacing(8.0 * context.theme.scaling);

        if let Some(input) = &self.input {
            col = col.push(input.view(context));
        }

        col.into()
    }
}

impl<Message: 'static> View<Message, TermBackend> for Console<Message, TermBackend> {
    fn view(&self, context: &Context) -> String {
        let mut out = format!("CONSOLE OUTPUT:\n{}\n", self.content);
        if let Some(input) = &self.input {
            out.push_str(&format!("INPUT: {}", input.view(context)));
        }
        out
    }
}
