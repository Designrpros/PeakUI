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
    height: Length,
    is_compact: bool,
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
            height: Length::Shrink,
            is_compact: false,
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

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn compact(mut self) -> Self {
        self.is_compact = true;
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
}

impl<Message: Clone + 'static, B: crate::core::Backend> View<Message, B> for Button<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        // ... (existing view implementation)
        let theme = context.theme;
        let variant = self.variant;
        let size = self.size.resolve(context.is_mobile());

        let mut children = Vec::new();

        let mut child_context = context.clone();
        if variant == Variant::Solid {
            let fg = match self.intent {
                Intent::Accent => theme.colors.on_accent,
                Intent::Secondary => theme.colors.on_secondary,
                _ => theme.colors.on_primary,
            };
            child_context.foreground = Some(fg);
        }

        if let Some(icon_name) = &self.icon {
            let icon_color = child_context.foreground;
            children.push(B::icon(icon_name.clone(), 16.0, icon_color, &child_context));
        }

        children.push(self.content.view(&child_context));

        let padding = if variant == Variant::Compact {
            iced::Padding::ZERO
        } else {
            match size {
                ControlSize::Small => iced::Padding::from([2, 6]),
                ControlSize::Medium => iced::Padding::from([6, 12]),
                ControlSize::Large => iced::Padding::from([10, 20]),
                ControlSize::XLarge => iced::Padding::from([14, 28]),
            }
        };

        let inner = B::hstack(
            children,
            8.0,
            padding,
            self.width,
            Length::Shrink,
            Alignment::Center,
            Alignment::Center,
            context,
        );

        B::button(
            inner,
            self.on_press.clone(),
            self.variant,
            self.intent,
            self.width,
            self.height,
            self.is_compact,
            context,
        )
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        let content_node = self.content.describe(context);
        let label = content_node
            .content
            .clone()
            .or(content_node.label.clone())
            .unwrap_or_default();

        crate::core::SemanticNode {
            role: "button".to_string(),
            label: Some(label.clone()),
            content: None,
            children: vec![content_node],
            neural_tag: None,
            documentation: None,
            accessibility: Some(crate::core::AccessibilityNode {
                role: crate::core::AccessibilityRole::Button,
                label: label,
                hint: None,
                value: None,
                states: Vec::new(),
                ..Default::default()
            }),
            ..Default::default()
        }
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

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode {
            role: "toggle".to_string(),
            label: Some(self.label.clone()),
            content: Some(self.is_active.to_string()),
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
            accessibility: Some(crate::core::AccessibilityNode {
                role: crate::core::AccessibilityRole::Switch,
                label: self.label.clone(),
                hint: Some("Double tap to toggle".to_string()),
                value: Some(if self.is_active {
                    "on".to_string()
                } else {
                    "off".to_string()
                }),
                states: if self.is_active {
                    vec!["selected".to_string()]
                } else {
                    Vec::new()
                },
                ..Default::default()
            }),
            ..Default::default()
        }
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

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode {
            role: "slider".to_string(),
            label: None,
            content: Some(self.value.to_string()),
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
            accessibility: Some(crate::core::AccessibilityNode {
                role: crate::core::AccessibilityRole::Slider,
                label: "Slider".to_string(),
                hint: Some(format!(
                    "Range: {:?} - {:?}",
                    self.range.start(),
                    self.range.end()
                )),
                value: Some(format!("{:.2}", self.value)),
                states: Vec::new(),
                ..Default::default()
            }),
            ..Default::default()
        }
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
                    Length::Fixed(20.0),
                    Length::Fixed(20.0),
                    false,
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
                    Length::Fixed(20.0),
                    Length::Fixed(20.0),
                    false,
                    context,
                ),
            ],
            8.0,
            Padding::ZERO,
            Length::Fill,
            Length::Shrink,
            Alignment::Center,
            Alignment::Center,
            context,
        )
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode {
            accessibility: Some(crate::core::AccessibilityNode {
                role: crate::core::AccessibilityRole::SpinButton,
                label: self.label.clone(),
                hint: Some("Use +/- to adjust value".to_string()),
                value: Some(self.value.to_string()),
                ..Default::default()
            }),
            role: "stepper".to_string(),
            label: Some(self.label.clone()),
            content: Some(self.value.to_string()),
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }
}
pub struct TextInput<Message: Clone + 'static, B: Backend = crate::core::IcedBackend> {
    value: String,
    placeholder: String,
    on_change: Arc<dyn Fn(String) -> Message + Send + Sync>,
    on_submit: Option<Message>,
    font: Option<iced::Font>,
    width: Length,
    variant: Variant,
    is_secure: bool,
    id: Option<iced::widget::Id>,
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
            width: Length::Fill,
            variant: Variant::Solid,
            is_secure: false,
            id: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn id(mut self, id: iced::widget::Id) -> Self {
        self.id = Some(id);
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn on_submit(mut self, msg: Message) -> Self {
        self.on_submit = Some(msg);
        self
    }

    pub fn password(mut self) -> Self {
        self.is_secure = true;
        self
    }

    pub fn font(mut self, font: iced::Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }
}

impl<Message: Clone + 'static, B: Backend> View<Message, B> for TextInput<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let on_change = self.on_change.clone();
        let input = B::text_input(
            self.value.clone(),
            self.placeholder.clone(),
            move |s| (on_change)(s),
            self.on_submit.clone(),
            self.font.clone(),
            self.is_secure,
            self.variant,
            self.id.clone(),
            context,
        );
        B::container(
            input,
            Padding::ZERO,
            self.width,
            Length::Shrink,
            None,
            0.0,
            0.0,
            None,
            None,
            Alignment::Start,
            Alignment::Start,
            context,
        )
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode {
            accessibility: Some(crate::core::AccessibilityNode {
                role: crate::core::AccessibilityRole::TextField,
                label: self.placeholder.clone(),
                value: Some(if self.is_secure {
                    "********".to_string()
                } else {
                    self.value.clone()
                }),
                ..Default::default()
            }),
            role: "text_input".to_string(),
            label: Some(self.placeholder.clone()),
            content: Some(if self.is_secure {
                "********".to_string()
            } else {
                self.value.clone()
            }),
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }
}
