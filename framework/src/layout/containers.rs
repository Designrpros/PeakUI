use crate::core::{Backend, Context, IcedBackend, TermBackend, View};
use iced::{Length, Padding};
use std::borrow::Cow;

pub struct Card<Message: 'static + Send + Sync, B: Backend = IcedBackend> {
    content: Box<dyn View<Message, B> + Send + Sync>,
    padding: Padding,
    width: Length,
    height: Length,
    background: Option<iced::Color>,
    radius: f32,
    border_width: f32,
    border_color: Option<iced::Color>,
}

impl<Message: 'static + Send + Sync> Card<Message, IcedBackend> {
    pub fn new(content: impl View<Message, IcedBackend> + Send + Sync + 'static) -> Self {
        Self::new_generic(content)
    }
}

impl<Message: 'static + Send + Sync> Card<Message, TermBackend> {
    pub fn new_tui(content: impl View<Message, TermBackend> + Send + Sync + 'static) -> Self {
        Self::new_generic(content)
    }
}

impl<Message: 'static + Send + Sync, B: Backend> Card<Message, B> {
    pub fn new_generic(content: impl View<Message, B> + Send + Sync + 'static) -> Self {
        Self {
            content: Box::new(content),
            padding: Padding::from(16),
            width: Length::Fill,
            height: Length::Shrink,
            background: None,
            radius: 8.0,
            border_width: 0.0,
            border_color: None,
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn background(mut self, color: impl Into<iced::Color>) -> Self {
        self.background = Some(color.into());
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn border(mut self, width: f32, color: impl Into<iced::Color>) -> Self {
        self.border_width = width;
        self.border_color = Some(color.into());
        self
    }
}

impl<Message: 'static + Send + Sync, B: Backend> View<Message, B> for Card<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let content_view = self.content.view(context);
        B::container(
            content_view,
            self.padding,
            self.width,
            self.height,
            self.background,
            self.radius,
            self.border_width,
            self.border_color,
            None,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        )
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("card").push_child(self.content.describe(context))
    }
}

pub struct Section<Message: 'static + Send + Sync, B: Backend = IcedBackend> {
    title: String,
    content: Box<dyn View<Message, B> + Send + Sync>,
    width: Length,
    height: Length,
}

impl<Message: 'static + Send + Sync> Section<Message, IcedBackend> {
    pub fn new(
        title: impl Into<Cow<'static, str>>,
        content: impl View<Message, IcedBackend> + Send + Sync + 'static,
    ) -> Self {
        Self::new_generic(title, content)
    }
}

impl<Message: 'static + Send + Sync> Section<Message, TermBackend> {
    pub fn new_tui(
        title: impl Into<Cow<'static, str>>,
        content: impl View<Message, TermBackend> + Send + Sync + 'static,
    ) -> Self {
        Self::new_generic(title, content)
    }
}

impl<Message: 'static + Send + Sync, B: Backend> Section<Message, B> {
    pub fn new_generic(
        title: impl Into<Cow<'static, str>>,
        content: impl View<Message, B> + Send + Sync + 'static,
    ) -> Self {
        Self {
            title: title.into().into_owned(),
            content: Box::new(content),
            width: Length::Fill,
            height: Length::Shrink,
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
}

impl<Message: 'static + Send + Sync, B: Backend> View<Message, B> for Section<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::section(
            self.title.clone(),
            self.content.view(context),
            self.width,
            self.height,
            context,
        )
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("section")
            .with_label(self.title.clone())
            .push_child(self.content.describe(context))
    }
}

pub struct GlassCard<Message: 'static + Send + Sync, B: Backend = IcedBackend> {
    content: Box<dyn View<Message, B> + Send + Sync>,
    padding: Padding,
    width: Length,
    height: Length,
}

impl<Message: 'static + Send + Sync, B: Backend> GlassCard<Message, B> {
    pub fn new(content: impl View<Message, B> + Send + Sync + 'static) -> Self {
        Self {
            content: Box::new(content),
            padding: Padding::from(20),
            width: Length::Fill,
            height: Length::Shrink,
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }
}

impl<Message: 'static + Send + Sync, B: Backend> View<Message, B> for GlassCard<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let content_view = self.content.view(context);
        B::glass_card(content_view, self.padding, self.width, self.height, context)
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("glass_card").push_child(self.content.describe(context))
    }
}
