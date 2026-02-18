use crate::core::{Backend, Context, IcedBackend, TermBackend, View};
use iced::{Element, Length, Renderer, Theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormStyle {
    Grouped,
    Plain,
}

pub struct Form<Message: 'static + Send + Sync, B: Backend = IcedBackend> {
    sections: Vec<Box<dyn View<Message, B> + Send + Sync>>,
    style: FormStyle,
}

impl<Message: 'static + Send + Sync> Form<Message, IcedBackend> {
    pub fn new() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static + Send + Sync> Form<Message, TermBackend> {
    pub fn new_tui() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static + Send + Sync, B: Backend> Form<Message, B> {
    pub fn new_generic() -> Self {
        Self {
            sections: Vec::new(),
            style: FormStyle::Grouped,
        }
    }

    pub fn style(mut self, style: FormStyle) -> Self {
        self.style = style;
        self
    }

    pub fn push(mut self, section: impl View<Message, B> + Send + Sync + 'static) -> Self {
        self.sections.push(Box::new(section));
        self
    }
}

impl<Message: 'static + Send + Sync> View<Message, IcedBackend> for Form<Message, IcedBackend> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        // ... (existing implementation)
        let mut column = iced::widget::Column::new()
            .spacing(24.0)
            .width(Length::Fill);

        let theme = context.theme;

        for section in &self.sections {
            match self.style {
                FormStyle::Grouped => {
                    let section_view = section.view(context);

                    let radius = context.radius(theme.radius);
                    let shadow = context.shadow(
                        theme.shadow_color,
                        iced::Vector::new(theme.shadow_offset[0], theme.shadow_offset[1]),
                        theme.shadow_blur,
                    );
                    let surface_color = theme.colors.surface;
                    let border_color = theme.colors.border;
                    let text_primary_color = theme.colors.text_primary;

                    let card = iced::widget::container(
                        iced::widget::container(section_view)
                            .padding(16)
                            .width(Length::Fill)
                            .style({
                                let s_color = surface_color;
                                let b_color = border_color.scale_alpha(0.5);
                                let t_color = text_primary_color;
                                let r = radius;
                                move |_| iced::widget::container::Style {
                                    background: Some(s_color.into()),
                                    border: iced::Border {
                                        radius: r.into(),
                                        color: b_color,
                                        width: 1.0,
                                    },
                                    text_color: Some(t_color),
                                    ..Default::default()
                                }
                            }),
                    )
                    .width(Length::Fill)
                    .style({
                        let b_color = border_color;
                        let r = radius;
                        let s = shadow;
                        move |_| iced::widget::container::Style {
                            border: iced::Border {
                                radius: r.into(),
                                color: b_color,
                                width: 1.0,
                            },
                            shadow: s,
                            ..Default::default()
                        }
                    });

                    column = column.push(card);
                }
                FormStyle::Plain => {
                    column = column.push(section.view(context));
                }
            }
        }

        column.into()
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("form")
            .with_label(format!("{:?}", self.style))
            .extend_children(self.sections.iter().map(|s| s.describe(context)))
    }
}

impl<Message: 'static + Send + Sync> View<Message, TermBackend> for Form<Message, TermBackend> {
    fn view(&self, context: &Context) -> String {
        self.sections
            .iter()
            .map(|s| s.view(context))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}
