use crate::atoms::{Icon, Text};
use crate::core::{Backend, Context, IcedBackend, View};
use crate::modifiers::Variant;
use iced::{Alignment, Border, Color, Length, Padding, Shadow, Vector};

pub struct ToolbarItem<Message: 'static> {
    label: Option<String>,
    icon: Option<String>,
    on_press: Option<Message>,
    active: bool,
}

impl<Message: 'static> ToolbarItem<Message> {
    pub fn new() -> Self {
        Self {
            label: None,
            icon: None,
            on_press: None,
            active: false,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(on_press);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

impl<Message: Clone + 'static> View<Message, IcedBackend> for ToolbarItem<Message> {
    fn view(
        &self,
        context: &Context,
    ) -> iced::Element<'static, Message, iced::Theme, iced::Renderer> {
        let theme = context.theme;
        let active = self.active;

        let mut content = crate::layout::HStack::<Message, IcedBackend>::new_generic()
            .width(Length::Shrink)
            .spacing(8.0)
            .align_y(Alignment::Center);

        if let Some(icon_name) = &self.icon {
            let color = if active {
                theme.colors.primary
            } else {
                theme.colors.text_secondary
            };
            content = content.push(
                Icon::<IcedBackend>::new(icon_name.clone())
                    .size(18.0)
                    .color(color),
            );
        }

        if let Some(label_text) = &self.label {
            content = content.push(
                Text::<IcedBackend>::new(label_text.clone())
                    .callout()
                    .bold()
                    .color(if active {
                        theme.colors.primary
                    } else {
                        theme.colors.text_secondary
                    }),
            );
        }

        crate::controls::Button::new(content)
            .variant(Variant::Ghost)
            .on_press_maybe(self.on_press.clone())
            .view(context)
    }
}

pub struct ToolbarGroup<Message: 'static> {
    items: Vec<Box<dyn View<Message, IcedBackend>>>,
    padding: Padding,
}

impl<Message: 'static> ToolbarGroup<Message> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            padding: Padding {
                top: 8.0,
                right: 16.0,
                bottom: 8.0,
                left: 16.0,
            },
        }
    }

    pub fn push(mut self, item: impl View<Message, IcedBackend> + 'static) -> Self {
        self.items.push(Box::new(item));
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }
}

impl<Message: 'static> View<Message, IcedBackend> for ToolbarGroup<Message> {
    fn view(
        &self,
        context: &Context,
    ) -> iced::Element<'static, Message, iced::Theme, iced::Renderer> {
        let theme = context.theme;
        let is_grouped = self.items.len() > 1;

        let items: Vec<_> = self.items.iter().map(|item| item.view(context)).collect();
        let row = IcedBackend::hstack(
            items,
            8.0,
            self.padding,
            Length::Shrink,
            Length::Shrink,
            Alignment::Center,
            theme.scaling,
        );

        let radius = context.radius(24.0);
        if is_grouped {
            iced::widget::container(row)
                .width(Length::Shrink)
                .style({
                    let bg_color = theme.colors.surface;
                    let border_color = theme.colors.border.scale_alpha(0.1);
                    let shadow = if cfg!(target_arch = "wasm32") {
                        Shadow::default()
                    } else {
                        Shadow {
                            color: Color {
                                a: 0.1,
                                ..Color::BLACK
                            },
                            offset: Vector::new(0.0, 4.0),
                            blur_radius: 12.0,
                        }
                    };
                    move |_| iced::widget::container::Style {
                        background: Some(bg_color.into()),
                        border: Border {
                            radius,
                            color: border_color,
                            width: 1.0,
                        },
                        shadow,
                        ..Default::default()
                    }
                })
                .into()
        } else {
            row
        }
    }
}
