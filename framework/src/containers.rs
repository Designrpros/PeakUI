use crate::core::{Backend, Context, IcedBackend, TermBackend, View};
use iced::widget::{column, container, text};
use iced::{Color, Element, Length, Padding, Renderer, Theme};

pub struct Card<Message: 'static, B: Backend = IcedBackend> {
    content: Box<dyn View<Message, B>>,
    padding: Padding,
    width: Length,
    height: Length,
}

impl<Message: 'static> Card<Message, IcedBackend> {
    pub fn new(content: impl View<Message, IcedBackend> + 'static) -> Self {
        Self::new_generic(content)
    }
}

impl<Message: 'static> Card<Message, TermBackend> {
    pub fn new_tui(content: impl View<Message, TermBackend> + 'static) -> Self {
        Self::new_generic(content)
    }
}

impl<Message: 'static, B: Backend> Card<Message, B> {
    pub fn new_generic(content: impl View<Message, B> + 'static) -> Self {
        Self {
            content: Box::new(content),
            padding: Padding::from(16),
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

impl<Message: 'static> View<Message, IcedBackend> for Card<Message, IcedBackend> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;

        let shadow = context.shadow(
            theme.shadow_color,
            iced::Vector::new(theme.shadow_offset[0], theme.shadow_offset[1]),
            theme.shadow_blur,
        );

        container(
            container(self.content.view(context))
                .padding(self.padding)
                .width(self.width)
                .height(self.height)
                .style({
                    let radius = context.radius(theme.radius);
                    let bg_color = theme.colors.surface;
                    let border_color = theme.colors.border.scale_alpha(0.5);
                    let text_color = theme.colors.text_primary;
                    move |_theme| container::Style {
                        background: Some(bg_color.into()),
                        border: iced::Border {
                            radius,
                            color: border_color,
                            width: 1.0,
                        },
                        text_color: Some(text_color),
                        ..Default::default()
                    }
                }),
        )
        .width(self.width)
        .height(self.height)
        .style({
            let radius = context.radius(theme.radius);
            let border_color = theme.colors.border;
            move |_theme| container::Style {
                border: iced::Border {
                    radius,
                    color: border_color,
                    width: 1.0,
                },
                shadow,
                ..Default::default()
            }
        })
        .into()
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode { 
        crate::core::SemanticNode { accessibility: None, 
            role: "card".to_string(),
            label: None,
            content: None,
            children: vec![self.content.describe(context)],
            neural_tag: None,
            documentation: None,
        }
    }
}

impl<Message: 'static> View<Message, TermBackend> for Card<Message, TermBackend> {
    fn view(&self, context: &Context) -> String {
        let inner = self.content.view(context);
        let lines: Vec<&str> = inner.lines().collect();
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0) + 4;

        let mut out = String::new();
        out.push_str("┌");
        out.push_str(&"─".repeat(width - 2));
        out.push_str("┐\n");

        for line in lines {
            out.push_str("│ ");
            out.push_str(line);
            out.push_str(&" ".repeat(width - 4 - line.len()));
            out.push_str(" │\n");
        }

        out.push_str("└");
        out.push_str(&"─".repeat(width - 2));
        out.push_str("┘");
        out
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode { 
        crate::core::SemanticNode { accessibility: None, 
            role: "card".to_string(),
            label: None,
            content: None,
            children: vec![self.content.describe(context)],
            neural_tag: None,
            documentation: None,
        }
    }
}

pub struct Section<Message: 'static, B: Backend = IcedBackend> {
    title: String,
    content: Box<dyn View<Message, B>>,
    width: Length,
    height: Length,
}

impl<Message: 'static> Section<Message, IcedBackend> {
    pub fn new(
        title: impl Into<String>,
        content: impl View<Message, IcedBackend> + 'static,
    ) -> Self {
        Self::new_generic(title, content)
    }
}

impl<Message: 'static> Section<Message, TermBackend> {
    pub fn new_tui(
        title: impl Into<String>,
        content: impl View<Message, TermBackend> + 'static,
    ) -> Self {
        Self::new_generic(title, content)
    }
}

impl<Message: 'static, B: Backend> Section<Message, B> {
    pub fn new_generic(title: impl Into<String>, content: impl View<Message, B> + 'static) -> Self {
        Self {
            title: title.into(),
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

impl<Message: 'static> View<Message, IcedBackend> for Section<Message, IcedBackend> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        container(
            column![
                text(self.title.clone())
                    .size(12)
                    .color(context.theme.colors.text_primary.scale_alpha(0.6)),
                self.content.view(context)
            ]
            .spacing(8),
        )
        .width(self.width)
        .height(self.height)
        .into()
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode { 
        crate::core::SemanticNode { accessibility: None, 
            role: "section".to_string(),
            label: Some(self.title.clone()),
            content: None,
            children: vec![self.content.describe(context)],
            neural_tag: None,
            documentation: None,
        }
    }
}

impl<Message: 'static> View<Message, TermBackend> for Section<Message, TermBackend> {
    fn view(&self, context: &Context) -> String {
        format!(
            "\x1b[1;2m# {}\x1b[0m\n{}",
            self.title.to_uppercase(),
            self.content.view(context)
        )
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode { 
        crate::core::SemanticNode { accessibility: None, 
            role: "section".to_string(),
            label: Some(self.title.clone()),
            content: None,
            children: vec![self.content.describe(context)],
            neural_tag: None,
            documentation: None,
        }
    }
}

pub struct GlassCard<Message: 'static, B: Backend = IcedBackend> {
    content: Box<dyn View<Message, B>>,
    padding: Padding,
    width: Length,
    height: Length,
}

impl<Message: 'static, B: Backend> GlassCard<Message, B> {
    pub fn new(content: impl View<Message, B> + 'static) -> Self {
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

impl<Message: 'static> View<Message, IcedBackend> for GlassCard<Message, IcedBackend> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;
        let mut bg = theme.colors.surface;
        bg.a = theme.glass_opacity;

        let shadow = context.shadow(
            theme.shadow_color,
            iced::Vector::new(theme.shadow_offset[0], theme.shadow_offset[1]),
            theme.shadow_blur,
        );

        let radius = context.radius(theme.radius);

        container(self.content.view(context))
            .padding(self.padding)
            .width(self.width)
            .height(self.height)
            .style(move |_| container::Style {
                background: Some(bg.into()),
                border: iced::Border {
                    radius,
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.15),
                    width: 1.0,
                },
                shadow,
                ..Default::default()
            })
            .into()
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode { 
        crate::core::SemanticNode { accessibility: None, 
            role: "glass_card".to_string(),
            label: None,
            content: None,
            children: vec![self.content.describe(context)],
            neural_tag: None,
            documentation: None,
        }
    }
}

impl<Message: 'static> View<Message, TermBackend> for GlassCard<Message, TermBackend> {
    fn view(&self, context: &Context) -> String {
        format!("(GLASS)\n{}", self.content.view(context))
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode { 
        crate::core::SemanticNode { accessibility: None, 
            role: "glass_card".to_string(),
            label: None,
            content: None,
            children: vec![self.content.describe(context)],
            neural_tag: None,
            documentation: None,
        }
    }
}
