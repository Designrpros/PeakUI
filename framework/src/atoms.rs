use crate::core::{Backend, Context, IcedBackend, ProxyView, View};
use crate::modifiers::Intent;
use iced::{Alignment, Color, Length, Padding};
use std::borrow::Cow;
use std::marker::PhantomData;
use std::sync::Arc;
pub mod badge;

#[derive(Clone)]
pub struct Text<B: Backend = IcedBackend> {
    content: Cow<'static, str>,
    size: f32,
    color: Option<Color>,
    intent: Option<Intent>,
    is_bold: bool,
    is_dim: bool,
    alignment: Alignment,
    font: Option<iced::Font>,
    width: Length,
    _phantom: PhantomData<B>,
}

impl<B: Backend> Text<B> {
    pub fn new(content: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content: content.into(),
            size: 14.0,
            color: None,
            intent: None,
            is_bold: false,
            is_dim: false,
            alignment: Alignment::Start,
            font: None,
            width: Length::Shrink,
            _phantom: PhantomData,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn font(mut self, font: iced::Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = Some(intent);
        self
    }

    pub fn bold(mut self) -> Self {
        self.is_bold = true;
        self
    }

    pub fn dim(mut self) -> Self {
        self.is_dim = true;
        self
    }

    pub fn center(mut self) -> Self {
        self.alignment = Alignment::Center;
        self
    }

    pub fn align_center(mut self) -> Self {
        self.alignment = Alignment::Center;
        self
    }

    pub fn align(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn align_start(mut self) -> Self {
        self.alignment = Alignment::Start;
        self
    }

    pub fn align_end(mut self) -> Self {
        self.alignment = Alignment::End;
        self
    }

    pub fn large_title(mut self) -> Self {
        self.size = 32.0;
        self.is_bold = true;
        self
    }

    pub fn title1(mut self) -> Self {
        self.size = 28.0;
        self.is_bold = true;
        self
    }

    pub fn title2(mut self) -> Self {
        self.size = 22.0;
        self.is_bold = true;
        self
    }

    pub fn title3(mut self) -> Self {
        self.size = 20.0;
        self.is_bold = true;
        self
    }

    pub fn headline(mut self) -> Self {
        self.size = 17.0;
        self.is_bold = true;
        self
    }

    pub fn body(mut self) -> Self {
        self.size = 17.0;
        self
    }

    pub fn callout(mut self) -> Self {
        self.size = 16.0;
        self
    }

    pub fn subheadline(mut self) -> Self {
        self.size = 15.0;
        self.is_dim = true;
        self
    }

    pub fn footnote(mut self) -> Self {
        self.size = 13.0;
        self.is_dim = true;
        self
    }

    pub fn caption1(mut self) -> Self {
        self.size = 12.0;
        self.is_dim = true;
        self
    }

    pub fn caption2(mut self) -> Self {
        self.size = 11.0;
        self.is_dim = true;
        self
    }

    pub fn secondary(mut self) -> Self {
        self.is_dim = true;
        self
    }

    pub fn primary(mut self) -> Self {
        self.intent = Some(Intent::Primary);
        self
    }

    pub fn italic(mut self) -> Self {
        self.font.get_or_insert(iced::Font::default()).style = iced::font::Style::Italic;
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn wrap(mut self) -> Self {
        self.width = Length::Fill;
        self
    }
}

impl<Message: Clone + 'static, B: Backend> View<Message, B> for Text<B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::text(
            self.content.as_ref().to_string(),
            self.size,
            self.color,
            self.is_bold,
            self.is_dim,
            self.intent,
            self.font,
            self.width,
            self.alignment,
            context,
        )
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("text").with_content(self.content.clone())
    }
}

#[derive(Clone, Copy)]
pub struct Rectangle<B: Backend = IcedBackend> {
    width: Length,
    height: Length,
    color: Option<Color>,
    radius: f32,
    border_width: f32,
    border_color: Option<Color>,
    _phantom: PhantomData<B>,
}

impl<B: Backend> Rectangle<B> {
    pub fn new(width: Length, height: Length) -> Self {
        Self {
            width,
            height,
            color: None,
            radius: 0.0,
            border_width: 0.0,
            border_color: None,
            _phantom: PhantomData,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = if cfg!(target_arch = "wasm32") {
            0.0
        } else {
            radius
        };
        self
    }

    pub fn border(mut self, width: f32, color: Color) -> Self {
        self.border_width = width;
        self.border_color = Some(color);
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for Rectangle<B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::rectangle(
            self.width,
            self.height,
            self.color,
            self.radius,
            self.border_width,
            self.border_color,
            context,
        )
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("rectangle")
    }
}

#[derive(Clone, Copy)]
pub struct Circle<B: Backend = IcedBackend> {
    radius: f32,
    color: Option<Color>,
    _phantom: PhantomData<B>,
}

impl<B: Backend> Circle<B> {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            color: None,
            _phantom: PhantomData,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for Circle<B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::circle(self.radius, self.color, context)
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("circle")
    }
}

#[derive(Clone, Copy)]
pub struct Capsule<B: Backend = IcedBackend> {
    width: Length,
    height: Length,
    color: Option<Color>,
    _phantom: PhantomData<B>,
}

impl<B: Backend> Capsule<B> {
    pub fn new(width: Length, height: Length) -> Self {
        Self {
            width,
            height,
            color: None,
            _phantom: PhantomData,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for Capsule<B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::capsule(self.width, self.height, self.color, context)
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("capsule")
    }
}

#[derive(Clone, Copy)]
pub struct Space<B: Backend = IcedBackend> {
    width: Length,
    height: Length,
    _phantom: PhantomData<B>,
}

impl<B: Backend> Space<B> {
    pub fn new(width: Length, height: Length) -> Self {
        Self {
            width,
            height,
            _phantom: PhantomData,
        }
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for Space<B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::space(self.width, self.height, context)
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("space")
    }
}

#[derive(Clone, Copy)]
pub struct Divider<B: Backend = IcedBackend> {
    _phantom: PhantomData<B>,
}

impl<B: Backend> Divider<B> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for Divider<B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::divider(context)
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("divider")
    }
}

#[derive(Clone)]
pub struct Icon<B: Backend + Clone = IcedBackend> {
    name: Cow<'static, str>,
    size: f32,
    color: Option<Color>,
    _phantom: PhantomData<B>,
}

impl<B: Backend> Icon<B> {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            size: 24.0,
            color: None,
            _phantom: PhantomData,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn primary<M>(self) -> ProxyView<M, B>
    where
        M: Clone + 'static,
    {
        ProxyView::new(move |ctx| {
            let mut icon = self.clone();
            icon.color = Some(ctx.theme.colors.primary);
            icon.view(ctx)
        })
    }

    pub fn primary_color<M>(self) -> ProxyView<M, B>
    where
        M: Clone + 'static,
    {
        ProxyView::new(move |ctx| {
            let mut icon = self.clone();
            icon.color = Some(ctx.theme.colors.primary);
            icon.view(ctx)
        })
    }

    pub fn secondary<M>(self) -> ProxyView<M, B>
    where
        M: Clone + 'static,
    {
        ProxyView::new(move |ctx| {
            let mut icon = self.clone();
            icon.color = Some(ctx.theme.colors.text_secondary);
            icon.view(ctx)
        })
    }
}

impl<Message: Clone + 'static, B: Backend> View<Message, B> for Icon<B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::icon(
            self.name.as_ref().to_string(),
            self.size,
            self.color,
            context,
        )
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("icon").with_label(self.name.clone())
    }
}

pub struct Image<B: Backend = IcedBackend> {
    path: Cow<'static, str>,
    width: Length,
    height: Length,
    radius: f32,
    _phantom: PhantomData<B>,
}

impl<B: Backend> Image<B> {
    pub fn new(path: impl Into<Cow<'static, str>>) -> Self {
        Self {
            path: path.into(),
            width: Length::Shrink,
            height: Length::Shrink,
            radius: 0.0,
            _phantom: PhantomData,
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

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for Image<B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::image(
            self.path.as_ref().to_string(),
            self.width,
            self.height,
            self.radius,
            context,
        )
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("image")
            .with_label(self.path.clone())
            .with_accessibility(crate::core::AccessibilityNode {
                role: crate::core::AccessibilityRole::Image,
                label: format!("Image: {}", self.path).into(),
                ..Default::default()
            })
    }
}

pub struct Video<B: Backend = IcedBackend> {
    path: Cow<'static, str>,
    width: Length,
    height: Length,
    radius: f32,
    _phantom: PhantomData<B>,
}

impl<B: Backend> Video<B> {
    pub fn new(path: impl Into<Cow<'static, str>>) -> Self {
        Self {
            path: path.into(),
            width: Length::Shrink,
            height: Length::Shrink,
            radius: 0.0,
            _phantom: PhantomData,
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

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for Video<B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::video(
            self.path.as_ref().to_string(),
            self.width,
            self.height,
            self.radius,
            context,
        )
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("video").with_label(self.path.clone())
    }
}

pub struct WebView<B: Backend = IcedBackend> {
    url: Cow<'static, str>,
    width: Length,
    height: Length,
    radius: f32,
    _phantom: PhantomData<B>,
}

impl<B: Backend> WebView<B> {
    pub fn new(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            url: url.into(),
            width: Length::Fill,
            height: Length::Fill,
            radius: 0.0,
            _phantom: PhantomData,
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

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for WebView<B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::web_view(
            self.url.as_ref().to_string(),
            self.width,
            self.height,
            self.radius,
            context,
        )
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("web_view").with_label(self.url.clone())
    }
}

#[derive(Clone)]
pub struct Container<Message: 'static, B: Backend = IcedBackend> {
    content: Arc<dyn View<Message, B>>,
    padding: Padding,
    width: Length,
    height: Length,
    background: Option<Color>,
    border_radius: f32,
    border_width: f32,
    border_color: Option<Color>,
    shadow: Option<iced::Shadow>,
    align_x: Alignment,
    align_y: Alignment,
    _phantom: PhantomData<B>,
}

impl<Message: 'static, B: Backend> Container<Message, B> {
    pub fn new(content: impl View<Message, B> + 'static) -> Self {
        Self {
            content: Arc::new(content),
            padding: Padding::default(),
            width: Length::Shrink,
            height: Length::Shrink,
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: None,
            shadow: None,
            align_x: Alignment::Start,
            align_y: Alignment::Start,
            _phantom: PhantomData,
        }
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
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

    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    pub fn shadow(mut self, shadow: iced::Shadow) -> Self {
        self.shadow = Some(shadow);
        self
    }

    pub fn border(mut self, width: f32, color: Color) -> Self {
        self.border_width = width;
        self.border_color = Some(color);
        self
    }

    pub fn align_x(mut self, alignment: Alignment) -> Self {
        self.align_x = alignment;
        self
    }

    pub fn align_y(mut self, alignment: Alignment) -> Self {
        self.align_y = alignment;
        self
    }

    pub fn center_x(mut self, width: Length) -> Self {
        self.width = width;
        self.align_x = Alignment::Center;
        self
    }

    pub fn center_y(mut self, height: Length) -> Self {
        self.height = height;
        self.align_y = Alignment::Center;
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for Container<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::container(
            self.content.view(context),
            self.padding,
            self.width,
            self.height,
            self.background,
            self.border_radius,
            self.border_width,
            self.border_color,
            self.shadow,
            self.align_x,
            self.align_y,
            context,
        )
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("container").push_child(self.content.describe(context))
    }
}
