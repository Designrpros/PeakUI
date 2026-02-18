pub use crate::backend::{AIBackend, Backend, IcedBackend, TextSpan};
pub use crate::engine::accessibility::{AccessibilityNode, AccessibilityRole};
#[cfg(feature = "intelligence")]
pub use crate::semantic::IntelligenceProvider;
pub use crate::semantic::{ChatCompletionMessage, DataProvider, SemanticNode, SemanticRecord};
pub use crate::style::{Context, DeviceType, Intent, ScrollDirection, Variant};
use iced::{Element, Subscription, Task, Theme};
pub use nalgebra::{Isometry3, Point3, Quaternion, Translation3, UnitQuaternion, Vector3};
pub use peak_core::registry::ShellMode;
use std::sync::Arc;

/// The core entry point for a PeakUI application.
pub trait App: Sized {
    type Message: Send + Sync + Clone + std::fmt::Debug + 'static;
    type Flags: Clone + Send + 'static;

    fn new(flags: Self::Flags) -> (Self, Task<Self::Message>);
    fn update(&mut self, message: Self::Message) -> Task<Self::Message>;
    fn view(&self) -> Element<'_, Self::Message>;
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }
    fn theme(&self) -> Theme {
        Theme::Dark
    }
    fn title(&self) -> String {
        "Peak App".into()
    }
    fn window_settings(_flags: &Self::Flags) -> iced::window::Settings {
        iced::window::Settings::default()
    }

    fn run(flags: Self::Flags) -> iced::Result
    where
        Self: 'static,
    {
        let settings = Self::window_settings(&flags);
        iced::application(move || Self::new(flags.clone()), Self::update, Self::view)
            .title(Self::title)
            .subscription(Self::subscription)
            .theme(Self::theme)
            .window(settings)
            .run()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Headset {
    VisionPro,
    Quest3,
    Generic,
}

pub struct NeuralView<Message: 'static + Send + Sync, B: Backend, V: View<Message, B>> {
    inner: V,
    tag: Arc<str>,
    _phantom: std::marker::PhantomData<(Message, B)>,
}

impl<Message: 'static + Send + Sync, B: Backend, V: View<Message, B>> View<Message, B>
    for NeuralView<Message, B, V>
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        self.inner.view(context)
    }

    fn describe(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe(context);
        node.neural_tag = Some(self.tag.to_string().into());
        node
    }

    fn describe_iced(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe_iced(context);
        node.neural_tag = Some(self.tag.to_string().into());
        node
    }
}

pub struct DocumentedView<Message: 'static + Send + Sync, B: Backend, V: View<Message, B>> {
    inner: V,
    documentation: Arc<str>,
    _phantom: std::marker::PhantomData<(Message, B)>,
}

impl<Message: 'static + Send + Sync, B: Backend, V: View<Message, B>> View<Message, B>
    for DocumentedView<Message, B, V>
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::with_tooltip(
            self.inner.view(context),
            self.documentation.clone(),
            context,
        )
    }

    fn describe(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe(context);
        node.documentation = Some(self.documentation.to_string().into());
        node
    }

    fn describe_iced(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe_iced(context);
        node.documentation = Some(self.documentation.to_string().into());
        node
    }
}

pub struct SpatialBillboard<Message: 'static + Send + Sync, B: Backend, V: View<Message, B>> {
    pub inner: V,
    pub active: bool,
    pub _phantom: std::marker::PhantomData<(Message, B)>,
}

impl<Message: 'static + Send + Sync, B: Backend, V: View<Message, B>> View<Message, B>
    for SpatialBillboard<Message, B, V>
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let mut child_context = context.clone();
        child_context.billboarding = self.active;
        self.inner.view(&child_context)
    }

    fn describe(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe(context);
        node.neural_tag = Some(
            format!(
                "{}:spatial:billboard:{}",
                node.neural_tag.as_deref().unwrap_or_default(),
                self.active
            )
            .into(),
        );
        node
    }

    fn describe_iced(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe_iced(context);
        node.neural_tag = Some(
            format!(
                "{}:spatial:billboard:{}",
                node.neural_tag.as_deref().unwrap_or_default(),
                self.active
            )
            .into(),
        );
        node
    }
}

pub struct PhysicalDepthView<Message: 'static + Send + Sync, B: Backend, V: View<Message, B>> {
    pub inner: V,
    pub depth: f32,
    pub _phantom: std::marker::PhantomData<(Message, B)>,
}

impl<Message: 'static + Send + Sync, B: Backend, V: View<Message, B>> View<Message, B>
    for PhysicalDepthView<Message, B, V>
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let view = self.inner.view(context);
        B::spatial_modifier(
            view,
            Vector3::new(0.0, 0.0, self.depth),
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::zeros(),
            context,
        )
    }

    fn describe(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe(context);
        node.depth = Some(self.depth);
        node
    }

    fn describe_iced(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe_iced(context);
        node.depth = Some(self.depth);
        node
    }
}

pub struct NeuralSudo<Message: 'static + Send + Sync, B: Backend, V: View<Message, B>> {
    inner: V,
    reason: Arc<str>,
    _phantom: std::marker::PhantomData<(Message, B)>,
}

impl<Message: 'static + Send + Sync, B: Backend, V: View<Message, B>> View<Message, B>
    for NeuralSudo<Message, B, V>
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        self.inner.view(context)
    }

    fn describe(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe(context);
        node.is_protected = true;
        node.protection_reason = Some(self.reason.to_string().into());
        node
    }

    fn describe_iced(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe_iced(context);
        node.is_protected = true;
        node.protection_reason = Some(self.reason.to_string().into());
        node
    }
}

pub use peak_theme::ThemeTokens;

/// The primary trait for all UI components in the PeakUI framework.
pub trait View<Message: 'static + Send + Sync, B: Backend = IcedBackend> {
    fn view(&self, context: &Context) -> B::AnyView<Message>;

    fn describe(&self, _context: &Context) -> SemanticNode {
        SemanticNode::default()
    }

    fn describe_iced(&self, context: &Context) -> SemanticNode {
        self.describe(context)
    }

    fn neural_tag(self, tag: impl Into<Arc<str>>) -> NeuralView<Message, B, Self>
    where
        Self: Sized,
    {
        NeuralView {
            inner: self,
            tag: tag.into(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn documented(self, documentation: impl Into<Arc<str>>) -> DocumentedView<Message, B, Self>
    where
        Self: Sized,
    {
        DocumentedView {
            inner: self,
            documentation: documentation.into(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn into_box(self) -> Box<dyn View<Message, B> + Send + Sync>
    where
        Self: Sized + Send + Sync + 'static,
    {
        Box::new(self)
    }

    fn billboarded(self, active: bool) -> SpatialBillboard<Message, B, Self>
    where
        Self: Sized,
    {
        SpatialBillboard {
            inner: self,
            active,
            _phantom: std::marker::PhantomData,
        }
    }

    fn physical_depth(self, depth: f32) -> PhysicalDepthView<Message, B, Self>
    where
        Self: Sized,
    {
        PhysicalDepthView {
            inner: self,
            depth,
            _phantom: std::marker::PhantomData,
        }
    }

    fn sudo(self, reason: impl Into<Arc<str>>) -> NeuralSudo<Message, B, Self>
    where
        Self: Sized,
    {
        NeuralSudo {
            inner: self,
            reason: reason.into(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn boxed(self) -> Box<dyn View<Message, B> + Send + Sync>
    where
        Self: Sized + Send + Sync + 'static,
    {
        Box::new(self)
    }

    fn on_tap_gesture(
        self,
        message: Message,
    ) -> crate::engine::gestures::TapGesture<Message, B, Self>
    where
        Self: Sized,
    {
        crate::engine::gestures::TapGesture::new(self, message)
    }
}

impl<Message: 'static + Send + Sync, B: Backend> View<Message, B>
    for Box<dyn View<Message, B> + Send + Sync>
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        self.as_ref().view(context)
    }

    fn describe(&self, context: &Context) -> SemanticNode {
        self.as_ref().describe(context)
    }

    fn describe_iced(&self, context: &Context) -> SemanticNode {
        self.as_ref().describe_iced(context)
    }
}

pub use crate::backend::spatial::{SpatialBackend, SpatialNode};
pub use crate::backend::term::TermBackend;

impl<Message: 'static + Send + Sync, B: Backend> View<Message, B> for SemanticNode {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::semantic_node(self.clone(), context)
    }

    fn describe(&self, _context: &Context) -> SemanticNode {
        self.clone()
    }
}

pub fn responsive<Message>(
    f: impl Fn(DeviceType) -> Box<dyn View<Message> + Send + Sync> + Send + Sync + 'static,
) -> ProxyView<Message>
where
    Message: Clone + Send + Sync + 'static,
{
    ProxyView::new(move |ctx| f(ctx.device).view(ctx))
}

pub struct ProxyView<Message: Clone + Send + Sync + 'static, B: Backend = IcedBackend> {
    view_fn: Arc<dyn Fn(&Context) -> B::AnyView<Message> + Send + Sync>,
    _phantom: std::marker::PhantomData<B>,
}

impl<Message: Clone + Send + Sync + 'static, B: Backend> ProxyView<Message, B> {
    pub fn new(view_fn: impl Fn(&Context) -> B::AnyView<Message> + Send + Sync + 'static) -> Self {
        Self {
            view_fn: Arc::new(view_fn),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Message: Clone + Send + Sync + 'static, B: Backend> View<Message, B>
    for ProxyView<Message, B>
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        (*self.view_fn)(context)
    }
}
