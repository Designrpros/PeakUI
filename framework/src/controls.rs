use crate::core::{Backend, Context, View};
use crate::modifiers::{ControlSize, Intent, Variant};
use iced::{Alignment, Length, Padding};
use std::sync::Arc;

/// A customizable button component that supports labels, icons, and various intents/variants.
pub struct Button<Message, B: crate::core::Backend = crate::core::IcedBackend> {
    content: Box<dyn View<Message, B>>,
    icon: Option<String>,
    on_press: Option<Message>,
    intent: Intent,
    variant: Variant,
    size: ControlSize,
    width: Length,
    _phantom: std::marker::PhantomData<B>,
}

#[derive(Clone, Copy, Default)]
pub enum ButtonStyle {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Ghost,
}

impl<Message: Clone + 'static, B: crate::core::Backend> Button<Message, B> {
    pub fn new(content: impl View<Message, B> + 'static) -> Self {
        Self {
            content: Box::new(content),
            icon: None,
            on_press: None,
            intent: Intent::Primary,
            variant: Variant::Solid,
            size: ControlSize::Medium,
            width: Length::Shrink,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn label(text: impl Into<String>) -> Self {
        Self::new(crate::atoms::Text::<B>::new(text))
    }

    pub fn icon(mut self, name: impl Into<String>) -> Self {
        self.icon = Some(name.into());
        self
    }

    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }

    pub fn on_press_maybe(mut self, msg: Option<Message>) -> Self {
        self.on_press = msg;
        self
    }

    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    #[deprecated(note = "Use .intent() and .variant() instead")]
    pub fn style(mut self, style: ButtonStyle) -> Self {
        match style {
            ButtonStyle::Primary => {
                self.intent = Intent::Primary;
                self.variant = Variant::Solid;
            }
            ButtonStyle::Secondary => {
                self.intent = Intent::Neutral;
                self.variant = Variant::Outline;
            }
            ButtonStyle::Destructive => {
                self.intent = Intent::Danger;
                self.variant = Variant::Solid;
            }
            ButtonStyle::Ghost => {
                self.intent = Intent::Neutral;
                self.variant = Variant::Ghost;
            }
        }
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }
}

impl<Message: Clone + 'static, B: crate::core::Backend> View<Message, B> for Button<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let theme = context.theme;
        let variant = self.variant;
        let size = self.size;

        let mut children = Vec::new();

        if let Some(icon_name) = &self.icon {
            let icon_color = match variant {
                Variant::Solid => Some(theme.colors.on_primary),
                _ => None,
            };
            children.push(B::icon(icon_name.clone(), 16.0, icon_color, context));
        }

        children.push(self.content.view(context));

        let padding = match size {
            ControlSize::Small => iced::Padding::from([4, 8]),
            ControlSize::Medium => iced::Padding::from([8, 16]),
            ControlSize::Large => iced::Padding::from([12, 24]),
            ControlSize::XLarge => iced::Padding::from([16, 32]),
        };

        let inner = B::hstack(
            children,
            8.0,
            padding,
            self.width,
            Length::Shrink,
            Alignment::Center,
            context.theme.scaling,
        );

        B::button(
            inner,
            self.on_press.clone(),
            self.variant,
            self.intent,
            context,
        )
    }
}

pub struct Toggle<Message, B: crate::core::Backend = crate::core::IcedBackend> {
    label: String,
    is_active: bool,
    on_toggle: Arc<dyn Fn(bool) -> Message + Send + Sync>,
    _phantom: std::marker::PhantomData<B>,
}

impl<Message, B: crate::core::Backend> Toggle<Message, B> {
    pub fn new(
        label: impl Into<String>,
        is_active: bool,
        f: impl Fn(bool) -> Message + Send + Sync + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            is_active,
            on_toggle: Arc::new(f),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Message: Clone + 'static, B: crate::core::Backend> View<Message, B> for Toggle<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::toggle(
            self.label.clone(),
            self.is_active,
            {
                let on_toggle = self.on_toggle.clone();
                move |b| (on_toggle)(b)
            },
            context,
        )
    }
}

pub struct Slider<Message, B: crate::core::Backend = crate::core::IcedBackend> {
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    on_change: Arc<dyn Fn(f32) -> Message + Send + Sync>,
    width: Length,
    _phantom: std::marker::PhantomData<B>,
}

impl<Message, B: crate::core::Backend> Slider<Message, B> {
    pub fn new(
        range: std::ops::RangeInclusive<f32>,
        value: f32,
        on_change: impl Fn(f32) -> Message + Send + Sync + 'static,
    ) -> Self {
        Self {
            range,
            value,
            on_change: Arc::new(on_change),
            width: Length::Shrink,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }
}

impl<Message: Clone + 'static, B: crate::core::Backend> View<Message, B> for Slider<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::slider(
            self.range.clone(),
            self.value,
            {
                let on_change = self.on_change.clone();
                move |v| (on_change)(v)
            },
            context,
        )
    }
}

pub struct Stepper<Message, B: crate::core::Backend = crate::core::IcedBackend> {
    label: String,
    value: i32,
    range: std::ops::RangeInclusive<i32>,
    step: i32,
    on_change: Arc<dyn Fn(i32) -> Message + Send + Sync>,
    _phantom: std::marker::PhantomData<B>,
}

impl<Message, B: crate::core::Backend> Stepper<Message, B> {
    pub fn new(
        label: impl Into<String>,
        value: i32,
        on_change: impl Fn(i32) -> Message + Send + Sync + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            value,
            range: 0..=100,
            step: 1,
            on_change: Arc::new(on_change),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn range(mut self, range: std::ops::RangeInclusive<i32>) -> Self {
        self.range = range;
        self
    }

    pub fn step(mut self, step: i32) -> Self {
        self.step = step;
        self
    }
}

impl<Message: Clone + 'static, B: crate::core::Backend> View<Message, B> for Stepper<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        // For simplicity in Phase 2, we just render a text label + value for generic Stepper
        // Full interactive stepper logic would require more generic primitives
        B::hstack(
            vec![
                B::text(
                    format!("{}: {}", self.label, self.value),
                    14.0,
                    None,
                    false,
                    false,
                    None,
                    None,
                    Length::Fill,
                    Alignment::Start,
                    context,
                ),
                B::button(
                    B::text(
                        "-".to_string(),
                        16.0,
                        None,
                        true,
                        false,
                        None,
                        None,
                        Length::Fixed(20.0),
                        Alignment::Center,
                        context,
                    ),
                    Some((self.on_change)(self.value - self.step)),
                    Variant::Outline,
                    Intent::Neutral,
                    context,
                ),
                B::button(
                    B::text(
                        "+".to_string(),
                        16.0,
                        None,
                        true,
                        false,
                        None,
                        None,
                        Length::Fixed(20.0),
                        Alignment::Center,
                        context,
                    ),
                    Some((self.on_change)(self.value + self.step)),
                    Variant::Outline,
                    Intent::Neutral,
                    context,
                ),
            ],
            8.0,
            Padding::ZERO,
            Length::Fill,
            Length::Shrink,
            Alignment::Center,
            context.theme.scaling,
        )
    }
}
pub struct TextInput<Message: Clone + 'static, B: Backend = crate::core::IcedBackend> {
    value: String,
    placeholder: String,
    on_change: Arc<dyn Fn(String) -> Message + Send + Sync>,
    on_submit: Option<Message>,
    font: Option<iced::Font>,
    _phantom: std::marker::PhantomData<B>,
}

impl<Message: Clone + 'static, B: Backend> TextInput<Message, B> {
    pub fn new(
        value: impl Into<String>,
        placeholder: impl Into<String>,
        on_change: impl Fn(String) -> Message + Send + Sync + 'static,
    ) -> Self {
        Self {
            value: value.into(),
            placeholder: placeholder.into(),
            on_change: Arc::new(on_change),
            on_submit: None,
            font: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn on_submit(mut self, msg: Message) -> Self {
        self.on_submit = Some(msg);
        self
    }

    pub fn font(mut self, font: iced::Font) -> Self {
        self.font = Some(font);
        self
    }
}

impl<Message: Clone + 'static, B: Backend> View<Message, B> for TextInput<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let on_change = self.on_change.clone();
        B::text_input(
            self.value.clone(),
            self.placeholder.clone(),
            move |s| (on_change)(s),
            self.on_submit.clone(),
            self.font,
            context,
        )
    }
}
