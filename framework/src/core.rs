use crate::localization::Localization;
use crate::modifiers::{Intent, Variant};
// Force rebuild to pick up peak-icons changes
use iced::{widget::Id, Element, Subscription, Task};
use iced::{Alignment, Color, Length, Padding, Renderer, Shadow, Size, Theme, Vector};
pub use nalgebra::{Isometry3, Point3, Quaternion, Translation3, UnitQuaternion, Vector3};
pub use peak_core::registry::ShellMode;

pub trait App: Sized {
    type Message: Send + Clone + std::fmt::Debug + 'static;
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

pub struct NeuralView<Message: 'static, B: Backend, V: View<Message, B>> {
    inner: V,
    tag: String,
    _phantom: std::marker::PhantomData<(Message, B)>,
}

impl<Message: 'static, B: Backend, V: View<Message, B>> View<Message, B>
    for NeuralView<Message, B, V>
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        self.inner.view(context)
    }

    fn describe(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe(context);
        node.neural_tag = Some(self.tag.clone());
        node
    }

    fn describe_iced(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe_iced(context);
        node.neural_tag = Some(self.tag.clone());
        node
    }
}

pub struct DocumentedView<Message: 'static, B: Backend, V: View<Message, B>> {
    inner: V,
    documentation: String,
    _phantom: std::marker::PhantomData<(Message, B)>,
}

impl<Message: 'static, B: Backend, V: View<Message, B>> View<Message, B>
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
        node.documentation = Some(self.documentation.clone());
        node
    }

    fn describe_iced(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe_iced(context);
        node.documentation = Some(self.documentation.clone());
        node
    }
}

pub struct SpatialBillboard<Message: 'static, B: Backend, V: View<Message, B>> {
    pub inner: V,
    pub active: bool,
    pub _phantom: std::marker::PhantomData<(Message, B)>,
}

impl<Message: 'static, B: Backend, V: View<Message, B>> View<Message, B>
    for SpatialBillboard<Message, B, V>
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let mut child_context = context.clone();
        child_context.billboarding = self.active;
        self.inner.view(&child_context)
    }

    fn describe(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe(context);
        node.neural_tag = Some(format!(
            "{}:spatial:billboard:{}",
            node.neural_tag.as_deref().unwrap_or_default(),
            self.active
        ));
        node
    }

    fn describe_iced(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe_iced(context);
        node.neural_tag = Some(format!(
            "{}:spatial:billboard:{}",
            node.neural_tag.as_deref().unwrap_or_default(),
            self.active
        ));
        node
    }
}

pub struct PhysicalDepthView<Message: 'static, B: Backend, V: View<Message, B>> {
    pub inner: V,
    pub depth: f32,
    pub _phantom: std::marker::PhantomData<(Message, B)>,
}

impl<Message: 'static, B: Backend, V: View<Message, B>> View<Message, B>
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

pub struct NeuralSudo<Message: 'static, B: Backend, V: View<Message, B>> {
    inner: V,
    reason: String,
    _phantom: std::marker::PhantomData<(Message, B)>,
}

impl<Message: 'static, B: Backend, V: View<Message, B>> View<Message, B>
    for NeuralSudo<Message, B, V>
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        self.inner.view(context)
    }

    fn describe(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe(context);
        node.is_protected = true;
        node.protection_reason = Some(self.reason.clone());
        node
    }

    fn describe_iced(&self, context: &Context) -> SemanticNode {
        let mut node = self.inner.describe_iced(context);
        node.is_protected = true;
        node.protection_reason = Some(self.reason.clone());
        node
    }
}

pub use peak_theme::ThemeTokens;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Context {
    pub theme: ThemeTokens,
    pub mode: ShellMode,
    pub device: DeviceType,
    pub size: Size,
    pub safe_area: Padding,
    pub focused_id: Option<String>,
    pub localization: Localization,
    pub peak_id: String,
    pub foreground: Option<Color>,
    pub billboarding: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            mode: ShellMode::Desktop,
            theme: peak_theme::ThemeTokens::default(),
            device: DeviceType::Desktop,
            size: iced::Size::ZERO,
            safe_area: iced::Padding::ZERO,
            focused_id: None,
            localization: Localization::default(),
            peak_id: String::new(),
            foreground: None,
            billboarding: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DeviceType {
    #[default]
    Desktop,
    Mobile,
    TV,
}

impl Context {
    pub fn new(
        mode: ShellMode,
        theme: peak_theme::ThemeTokens,
        size: Size,
        localization: Localization,
    ) -> Self {
        let device = match mode {
            ShellMode::Desktop => DeviceType::Desktop,
            ShellMode::Mobile => DeviceType::Mobile,
            ShellMode::TV | ShellMode::Console | ShellMode::Fireplace => DeviceType::TV,
            _ => DeviceType::Desktop,
        };

        Self {
            theme,
            mode,
            device,
            size,
            safe_area: Padding::default(),
            focused_id: None,
            localization,
            peak_id: String::new(),
            foreground: None,
            billboarding: false,
        }
    }

    pub fn is_focused(&self, id: &str) -> bool {
        self.focused_id.as_deref() == Some(id)
    }

    pub fn with_focus(mut self, id: impl Into<String>) -> Self {
        self.focused_id = Some(id.into());
        self
    }

    pub fn is_slim(&self) -> bool {
        self.size.width < 900.0
    }

    pub fn is_wide(&self) -> bool {
        self.size.width > 1200.0
    }

    pub fn is_mobile(&self) -> bool {
        self.device == DeviceType::Mobile
    }

    pub fn shadow(&self, color: Color, offset: impl Into<Vector>, blur_radius: f32) -> Shadow {
        if cfg!(target_arch = "wasm32") {
            Shadow::default()
        } else {
            Shadow {
                color,
                offset: offset.into(),
                blur_radius,
            }
        }
    }

    pub fn radius(&self, radius: f32) -> iced::border::Radius {
        radius.into()
    }

    pub fn with_safe_area(mut self, padding: Padding) -> Self {
        self.safe_area = padding;
        self
    }

    pub fn t(&self, key: &str) -> String {
        self.localization.simple(key)
    }

    pub fn scale_length(&self, l: Length) -> Length {
        match l {
            Length::Fixed(f) => Length::Fixed(f * self.theme.scaling),
            _ => l,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum Layout {
    Vertical,
    Horizontal,
    Wrap,
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct BoundingBox3D {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

impl BoundingBox3D {
    pub fn new(min: Point3<f32>, max: Point3<f32>) -> Self {
        Self { min, max }
    }

    pub fn zero() -> Self {
        Self::new(Point3::origin(), Point3::origin())
    }

    pub fn size(&self) -> Vector3<f32> {
        self.max - self.min
    }

    pub fn center(&self) -> Point3<f32> {
        self.min + (self.size() / 2.0)
    }

    pub fn intersect_ray(&self, ray: &Ray) -> Option<f32> {
        let mut t_min = f32::MIN;
        let mut t_max = f32::MAX;

        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv_d;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max <= t_min {
                return None;
            }
        }

        if t_min > 0.0 {
            Some(t_min)
        } else if t_max > 0.0 {
            Some(t_max)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn at(&self, t: f32) -> Point3<f32> {
        self.origin + self.direction * t
    }
}

pub struct RayHit<Message = ()> {
    pub distance: f32,
    pub point: Point3<f32>,
    pub normal: Vector3<f32>,
    pub message: Option<Message>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(bound = "")]
pub struct SpatialNode<Message = ()> {
    pub role: String,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub transform: Transform3D,
    pub bounds: BoundingBox3D,
    pub layout: Layout,
    pub is_focused: bool,
    pub billboarding: bool,
    #[serde(skip)]
    pub on_press: Option<Message>,
    pub children: Vec<SpatialNode<Message>>,
}

impl<Message: Clone> SpatialNode<Message> {
    pub fn hit_test(&self, ray: &Ray) -> Option<RayHit<Message>> {
        // 1. Transform ray into local space
        let local_origin = ray.origin - self.transform.position;
        let local_ray = Ray::new(Point3::from(local_origin), ray.direction);

        // 2. Check bounding box
        let dist = self.bounds.intersect_ray(&local_ray)?;

        // 3. Child check (recursive) - find closest child hit
        let mut best_hit: Option<RayHit<Message>> = None;

        for child in &self.children {
            if let Some(hit) = child.hit_test(&local_ray) {
                if best_hit
                    .as_ref()
                    .map_or(true, |bh| hit.distance < bh.distance)
                {
                    best_hit = Some(hit);
                }
            }
        }

        // 4. Return closest hit
        if let Some(mut hit) = best_hit {
            // Transform hit back to parent space
            hit.point = Point3::from(hit.point.coords + self.transform.position);
            Some(hit)
        } else {
            // Hit this node's bounding box
            Some(RayHit {
                distance: dist,
                point: ray.at(dist),
                normal: Vector3::zeros(),
                message: self.on_press.clone(),
            })
        }
    }

    pub fn to_empty(&self) -> SpatialNode<()> {
        SpatialNode {
            role: self.role.clone(),
            width: self.width,
            height: self.height,
            depth: self.depth,
            transform: self.transform,
            bounds: self.bounds,
            layout: self.layout,
            is_focused: self.is_focused,
            billboarding: self.billboarding,
            on_press: None,
            children: self.children.iter().map(|c| c.to_empty()).collect(),
        }
    }
}

fn scale_length(l: Length, scale: f32) -> Length {
    match l {
        Length::Fixed(p) => Length::Fixed(p * scale),
        _ => l,
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct Transform3D {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>, // Euler angles for now, will upgrade to Quat in layout
    pub scale: Vector3<f32>,
}

impl Default for Transform3D {
    fn default() -> Self {
        Self {
            position: Vector3::zeros(),
            rotation: Vector3::zeros(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SpatialBackend;

// Redundant SpatialBackend implementation removed

#[derive(Debug, Clone)]
pub struct TextSpan {
    pub content: String,
    pub color: Option<Color>,
    pub font: Option<iced::Font>,
    pub size: Option<f32>,
}

/// A Backend defines the output type and composition logic for a View.
pub trait Backend: Sized + Clone + 'static {
    type AnyView<Message: 'static>: 'static;

    fn vstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_x: Alignment,
        align_y: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn hstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_x: Alignment,
        align_y: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn wrap<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        run_spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_x: Alignment,
        align_y: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn rich_text<Message: Clone + 'static>(
        spans: Vec<TextSpan>,
        size: f32,
        width: Length,
        alignment: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn text<Message: Clone + 'static>(
        content: String,
        size: f32,
        color: Option<Color>,
        is_bold: bool,
        is_dim: bool,
        intent: Option<Intent>,
        font: Option<iced::Font>,
        width: Length,
        alignment: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn icon<Message: Clone + 'static>(
        name: String,
        size: f32,
        color: Option<Color>,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn divider<Message: 'static>(context: &Context) -> Self::AnyView<Message>;

    fn space<Message: 'static>(
        width: Length,
        height: Length,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn circle<Message: 'static>(
        radius: f32,
        color: Option<Color>,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn capsule<Message: 'static>(
        width: Length,
        height: Length,
        color: Option<Color>,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn rectangle<Message: 'static>(
        width: Length,
        height: Length,
        color: Option<Color>,
        radius: f32,
        border_width: f32,
        border_color: Option<Color>,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn button<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        on_press: Option<Message>,
        variant: Variant,
        intent: Intent,
        width: Length,
        is_compact: bool,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn sidebar_item<Message: Clone + 'static>(
        title: String,
        icon: String,
        is_selected: bool,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn text_input<Message: Clone + 'static>(
        value: String,
        placeholder: String,
        on_change: impl Fn(String) -> Message + 'static,
        on_submit: Option<Message>,
        font: Option<iced::Font>,
        is_secure: bool,
        variant: Variant,
        id: Option<iced::widget::Id>,
        dom_id: Option<String>,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn slider<Message: Clone + 'static>(
        range: std::ops::RangeInclusive<f32>,
        value: f32,
        on_change: impl Fn(f32) -> Message + 'static,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn toggle<Message: Clone + 'static>(
        label: String,
        is_active: bool,
        on_toggle: impl Fn(bool) -> Message + 'static,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn zstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        width: Length,
        height: Length,
        alignment: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn grid<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        columns: usize,
        spacing: f32,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn image<Message: 'static>(
        path: impl Into<String>,
        width: Length,
        height: Length,
        radius: f32,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn video<Message: 'static>(
        path: impl Into<String>,
        width: Length,
        height: Length,
        radius: f32,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn web_view<Message: 'static>(
        url: String,
        width: Length,
        height: Length,
        radius: f32,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn container<Message: 'static>(
        content: Self::AnyView<Message>,
        padding: Padding,
        width: Length,
        height: Length,
        background: Option<Color>,
        radius: f32,
        border_width: f32,
        border_color: Option<Color>,
        shadow: Option<iced::Shadow>,
        align_x: Alignment,
        align_y: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn scroll_view<Message: 'static>(
        content: Self::AnyView<Message>,
        width: Length,
        height: Length,
        id: Option<&'static str>,
        show_indicators: bool,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn mouse_area<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        on_move: Option<Arc<dyn Fn(iced::Point) -> Message + Send + Sync>>,
        on_press: Option<Message>,
        on_release: Option<Message>,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn with_tooltip<Message: 'static>(
        content: Self::AnyView<Message>,
        tooltip: String,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn glass_card<Message: 'static>(
        content: Self::AnyView<Message>,
        padding: Padding,
        width: Length,
        height: Length,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn section<Message: 'static>(
        title: String,
        content: Self::AnyView<Message>,
        width: Length,
        height: Length,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn spatial_modifier<Message: 'static>(
        content: Self::AnyView<Message>,
        position: Vector3<f32>,
        scale: Vector3<f32>,
        rotation: Vector3<f32>,
        context: &Context,
    ) -> Self::AnyView<Message>;
}

impl Backend for SpatialBackend {
    type AnyView<Message: 'static> = SpatialNode<Message>;

    fn vstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_x: Alignment,
        _align_y: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let mut y_offset = 0.0;
        let mut nodes = Vec::new();

        for mut child in children {
            child.transform.position.y = y_offset;
            child.transform.position.z = 1.0; // Restoring hierarchical step
            y_offset += child.height + spacing;
            nodes.push(child);
        }

        SpatialNode {
            role: "vstack".to_string(),
            width: 0.0,
            height: y_offset,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: nodes,
        }
    }

    fn hstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_x: Alignment,
        _align_y: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        let mut x_offset = 0.0;
        let mut nodes = Vec::new();

        for mut child in children {
            child.transform.position.x = x_offset;
            child.transform.position.z = 1.0; // Restoring hierarchical step
            x_offset += child.width + spacing;
            nodes.push(child);
        }

        SpatialNode {
            role: "hstack".to_string(),
            width: x_offset,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Horizontal,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: nodes,
        }
    }

    fn wrap<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _spacing: f32,
        _run_spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_x: Alignment,
        _align_y: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        let mut nodes = Vec::new();
        for mut child in children {
            child.transform.position.z = 1.0;
            nodes.push(child);
        }

        SpatialNode {
            role: "wrap".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Wrap,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: nodes,
        }
    }

    fn rich_text<Message: Clone + 'static>(
        _spans: Vec<TextSpan>,
        _size: f32,
        _width: Length,
        _alignment: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "rich_text".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: vec![],
        }
    }

    fn text<Message: Clone + 'static>(
        _content: String,
        _size: f32,
        _color: Option<Color>,
        _is_bold: bool,
        _is_dim: bool,
        _intent: Option<Intent>,
        _font: Option<iced::Font>,
        _width: Length,
        _alignment: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "text".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: vec![], // SpatialBackend doesn't store text content in children
        }
    }

    fn icon<Message: Clone + 'static>(
        _name: String,
        _size: f32,
        _color: Option<Color>,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "icon".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: vec![],
        }
    }

    fn divider<Message: 'static>(context: &Context) -> Self::AnyView<Message> {
        SpatialNode {
            role: "divider".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: vec![],
        }
    }

    fn space<Message: 'static>(
        _width: Length,
        _height: Length,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "space".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: vec![],
        }
    }

    fn circle<Message: 'static>(
        _radius: f32,
        _color: Option<Color>,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "circle".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: vec![],
        }
    }

    fn capsule<Message: 'static>(
        _width: Length,
        _height: Length,
        _color: Option<Color>,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "capsule".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: vec![],
        }
    }

    fn rectangle<Message: 'static>(
        _width: Length,
        _height: Length,
        _color: Option<Color>,
        _radius: f32,
        _border_width: f32,
        _border_color: Option<Color>,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "rectangle".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: vec![],
        }
    }

    fn button<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        _on_press: Option<Message>,
        _variant: Variant,
        _intent: Intent,
        _width: Length,
        _is_compact: bool,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "button".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: _on_press,
            children: vec![content],
        }
    }

    fn sidebar_item<Message: Clone + 'static>(
        _title: String,
        _icon: String,
        _is_selected: bool,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "sidebar_item".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Horizontal,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: vec![],
        }
    }

    fn text_input<Message: Clone + 'static>(
        _value: String,
        _placeholder: String,
        _on_change: impl Fn(String) -> Message + 'static,
        _on_submit: Option<Message>,
        _font: Option<iced::Font>,
        _is_secure: bool,
        _variant: Variant,
        _id: Option<iced::widget::Id>,
        _dom_id: Option<String>,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "text_input".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: true,
            billboarding: context.billboarding,
            on_press: _on_submit,
            children: vec![],
        }
    }

    fn slider<Message: Clone + 'static>(
        _range: std::ops::RangeInclusive<f32>,
        _value: f32,
        _on_change: impl Fn(f32) -> Message + 'static,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "slider".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: vec![],
        }
    }

    fn toggle<Message: Clone + 'static>(
        _label: String,
        _is_active: bool,
        _on_toggle: impl Fn(bool) -> Message + 'static,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "toggle".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: vec![],
        }
    }

    fn zstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _width: Length,
        _height: Length,
        _alignment: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        let mut z_offset = 0.0;
        let mut nodes = Vec::new();

        for mut child in children {
            child.transform.position.z = z_offset;
            z_offset += 10.0; // Default depth spacing for ZStack
            nodes.push(child);
        }

        SpatialNode {
            role: "zstack".to_string(),
            width: 0.0,
            height: 0.0,
            depth: z_offset,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: nodes,
        }
    }

    fn grid<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _columns: usize,
        _spacing: f32,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "grid".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children,
        }
    }

    fn image<Message: 'static>(
        _path: impl Into<String>,
        _width: Length,
        _height: Length,
        _radius: f32,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "image".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: vec![],
        }
    }

    fn video<Message: 'static>(
        _path: impl Into<String>,
        _width: Length,
        _height: Length,
        _radius: f32,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "video".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: vec![],
        }
    }

    fn web_view<Message: 'static>(
        _url: String,
        _width: Length,
        _height: Length,
        _radius: f32,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "web_view".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: vec![],
        }
    }

    fn container<Message: 'static>(
        content: Self::AnyView<Message>,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _background: Option<Color>,
        _radius: f32,
        _border_width: f32,
        _border_color: Option<Color>,
        _shadow: Option<iced::Shadow>,
        _align_x: Alignment,
        _align_y: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "container".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: vec![content],
        }
    }

    fn scroll_view<Message: 'static>(
        content: Self::AnyView<Message>,
        _width: Length,
        _height: Length,
        _id: Option<&'static str>,
        _show_indicators: bool,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "scroll_view".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: None,
            children: vec![content],
        }
    }

    fn mouse_area<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        _on_move: Option<Arc<dyn Fn(iced::Point) -> Message + Send + Sync>>,
        _on_press: Option<Message>,
        _on_release: Option<Message>,
        context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "mouse_area".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: context.billboarding,
            on_press: _on_press,
            children: vec![content],
        }
    }

    fn with_tooltip<Message: 'static>(
        content: Self::AnyView<Message>,
        _tooltip: String,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn glass_card<Message: 'static>(
        content: Self::AnyView<Message>,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn section<Message: 'static>(
        _title: String,
        content: Self::AnyView<Message>,
        _width: Length,
        _height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn spatial_modifier<Message: 'static>(
        mut content: Self::AnyView<Message>,
        position: Vector3<f32>,
        scale: Vector3<f32>,
        rotation: Vector3<f32>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content.transform.position += position;
        content.transform.scale.x *= scale.x;
        content.transform.scale.y *= scale.y;
        content.transform.scale.z *= scale.z;
        content.transform.rotation += rotation;
        content
    }
}

/// The default Iced-based GUI backend.
#[derive(Clone, Copy, Debug, Default)]
pub struct IcedBackend;

impl Backend for IcedBackend {
    type AnyView<Message: 'static> = iced::Element<'static, Message, Theme, Renderer>;

    fn vstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_x: Alignment,
        align_y: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::{column, container};
        let scale = context.theme.scaling;

        let col = column(children)
            .spacing(spacing * scale)
            .width(scale_length(width, scale))
            .height(scale_length(height, scale))
            .align_x(align_x);

        let mut c = container(col).padding(Padding {
            top: padding.top * scale,
            right: padding.right * scale,
            bottom: padding.bottom * scale,
            left: padding.left * scale,
        });

        if align_x == Alignment::Center && width != Length::Shrink {
            c = c.center_x(scale_length(width, scale));
        } else if align_x == Alignment::End && width != Length::Shrink {
            c = c.align_x(iced::alignment::Horizontal::Right);
        }

        if align_y == Alignment::Center && height != Length::Shrink {
            c = c.center_y(scale_length(height, scale));
        } else if align_y == Alignment::End && height != Length::Shrink {
            c = c.align_y(iced::alignment::Vertical::Bottom);
        }

        c.width(scale_length(width, scale))
            .height(scale_length(height, scale))
            .into()
    }

    fn hstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_x: Alignment,
        align_y: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::{container, row};
        let scale = context.theme.scaling;

        let r = row(children)
            .spacing(spacing * scale)
            .width(scale_length(width, scale))
            .height(scale_length(height, scale))
            .align_y(align_y);

        let mut c = container(r).padding(Padding {
            top: padding.top * scale,
            right: padding.right * scale,
            bottom: padding.bottom * scale,
            left: padding.left * scale,
        });

        if align_x == Alignment::Center && width != Length::Shrink {
            c = c.center_x(scale_length(width, scale));
        } else if align_x == Alignment::End && width != Length::Shrink {
            c = c.align_x(iced::alignment::Horizontal::Right);
        }

        if align_y == Alignment::Center && height != Length::Shrink {
            c = c.center_y(scale_length(height, scale));
        } else if align_y == Alignment::End && height != Length::Shrink {
            c = c.align_y(iced::alignment::Vertical::Bottom);
        }

        c.width(scale_length(width, scale))
            .height(scale_length(height, scale))
            .into()
    }

    fn wrap<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        _run_spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_x: Alignment,
        align_y: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::{container, row};
        let scale = context.theme.scaling;

        let w = row(children)
            .spacing(spacing * scale)
            .width(scale_length(width, scale))
            .height(scale_length(height, scale))
            .align_y(align_y)
            .wrap();

        let mut c = container(w).padding(Padding {
            top: padding.top * scale,
            right: padding.right * scale,
            bottom: padding.bottom * scale,
            left: padding.left * scale,
        });

        if align_x == Alignment::Center && width != Length::Shrink {
            c = c.center_x(scale_length(width, scale));
        } else if align_x == Alignment::End && width != Length::Shrink {
            c = c.align_x(iced::alignment::Horizontal::Right);
        }

        c.width(scale_length(width, scale))
            .height(scale_length(height, scale))
            .into()
    }

    fn rich_text<Message: Clone + 'static>(
        spans: Vec<TextSpan>,
        size: f32,
        width: Length,
        alignment: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::rich_text;
        // iced re-exports Span in advanced::text usually, or just import from where used.
        // check imports in MarkdownView: iced::advanced::text::Span
        use iced::advanced::text::Span;

        let scale = context.theme.scaling;
        let scaled_size = size * scale;

        let iced_spans: Vec<Span<'static, ()>> = spans
            .into_iter()
            .map(|s| {
                let mut span = Span::new(s.content);
                if let Some(c) = s.color {
                    span = span.color(c);
                }
                if let Some(f) = s.font {
                    span = span.font(f);
                }
                if let Some(sz) = s.size {
                    span = span.size(sz * scale);
                }
                span
            })
            .collect();

        rich_text(iced_spans)
            .size(scaled_size)
            .width(width)
            .align_x(alignment)
            .into()
    }

    fn text<Message: Clone + 'static>(
        content: String,
        size: f32,
        color: Option<Color>,
        is_bold: bool,
        is_dim: bool,
        intent: Option<Intent>,
        font: Option<iced::Font>,
        width: Length,
        alignment: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::text;

        let base_color = color.unwrap_or_else(|| {
            let theme = &context.theme;
            if let Some(i) = intent {
                match i {
                    Intent::Primary => theme.colors.primary,
                    Intent::Secondary => theme.colors.secondary,
                    Intent::Accent => theme.colors.accent,
                    Intent::Success => theme.colors.success,
                    Intent::Warning => theme.colors.warning,
                    Intent::Danger => theme.colors.danger,
                    Intent::Info => theme.colors.info,
                    Intent::Neutral => theme.colors.text_primary,
                }
            } else if is_dim {
                context.theme.colors.text_secondary
            } else {
                context
                    .foreground
                    .unwrap_or(context.theme.colors.text_primary)
            }
        });

        let mut text_color = base_color;
        if is_dim {
            text_color.a *= 0.8; // Mild dimming for subtle hierarchy
        }

        let mut base_font = font.unwrap_or(iced::Font {
            family: iced::font::Family::Name("Fira Sans"),
            ..iced::Font::DEFAULT
        });

        if is_bold {
            base_font.weight = iced::font::Weight::Bold;
        } else {
            base_font.weight = iced::font::Weight::Normal;
        }

        let scaled_size = size * context.theme.scaling;

        #[cfg(target_arch = "wasm32")]
        let t = text(content).shaping(iced::widget::text::Shaping::Advanced);
        #[cfg(not(target_arch = "wasm32"))]
        let t = text(content);

        t.size(scaled_size)
            .color(text_color)
            .font(base_font)
            .width(width)
            .align_x(alignment)
            .align_y(iced::alignment::Vertical::Center)
            .into()
    }

    fn icon<Message: Clone + 'static>(
        name: String,
        size: f32,
        color: Option<Color>,
        context: &Context,
    ) -> Self::AnyView<Message> {
        let theme = context.theme;
        let final_color = color.unwrap_or(theme.colors.text_primary);

        let hex_color = format!(
            "#{:02X}{:02X}{:02X}",
            (final_color.r * 255.0) as u8,
            (final_color.g * 255.0) as u8,
            (final_color.b * 255.0) as u8
        );

        // 1. Try embedded icons first
        if let Some(svg_data) = peak_icons::get_icon(&name) {
            // log::debug!("Icon '{}' found in EMBEDDED storage.");
            let colored_svg = svg_data
                .replace("currentColor", &hex_color)
                .replace("fill=\"#000000\"", &format!("fill=\"{}\"", hex_color))
                .replace("fill=\"black\"", &format!("fill=\"{}\"", hex_color));

            return iced::widget::svg(iced::widget::svg::Handle::from_memory(
                colored_svg.into_bytes(),
            ))
            .width(size)
            .height(size)
            .into();
        }

        // 2. Asset Pipeline (SystemIcon)
        if let Some(icon) = crate::assets::SystemIcon::from_name(&name) {
            let path = crate::assets::Asset::Icon(icon).path();
            return iced::widget::svg(iced::widget::svg::Handle::from_path(path))
                .width(size)
                .height(size)
                .into();
        }

        // 3. Fallback (Desktop only typically, but we unify now)
        // If we really want to keep peak_core::icons for desktop we can:
        #[cfg(not(target_arch = "wasm32"))]
        {
            let handle = peak_core::icons::get_ui_icon(&name, &hex_color);
            iced::widget::svg(handle).width(size).height(size).into()
        }

        #[cfg(target_arch = "wasm32")]
        {
            // Fallback for WASM if SystemIcon fails?
            // Just try to load by name as relative path
            let path = format!("assets/icons/system/ui/{}.svg", name);
            iced::widget::svg(iced::widget::svg::Handle::from_path(path))
                .width(size)
                .height(size)
                .into()
        }
    }

    fn divider<Message: 'static>(context: &Context) -> Self::AnyView<Message> {
        use iced::widget::container;
        let divider_color = context.theme.colors.divider;
        container(iced::widget::Space::new().height(1).width(Length::Fill))
            .style(move |_| container::Style {
                background: Some(divider_color.into()),
                ..Default::default()
            })
            .into()
    }

    fn space<Message: 'static>(
        width: Length,
        height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        iced::widget::Space::new()
            .width(width)
            .height(height)
            .into()
    }

    fn circle<Message: 'static>(
        radius: f32,
        color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::container;
        container(
            iced::widget::Space::new()
                .width(Length::Fixed(radius * 2.0))
                .height(Length::Fixed(radius * 2.0)),
        )
        .width(radius * 2.0)
        .height(radius * 2.0)
        .style(move |_| container::Style {
            background: color.map(iced::Background::Color),
            border: iced::Border {
                radius: radius.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    }

    fn capsule<Message: 'static>(
        width: Length,
        height: Length,
        color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::container;
        container(iced::widget::Space::new().width(width).height(height))
            .width(width)
            .height(height)
            .style(move |_| container::Style {
                background: color.map(iced::Background::Color),
                border: iced::Border {
                    radius: 1000.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
    }

    fn rectangle<Message: 'static>(
        width: Length,
        height: Length,
        color: Option<Color>,
        radius: f32,
        border_width: f32,
        border_color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::container;

        container(
            iced::widget::Space::new()
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(width)
        .height(height)
        .style({
            let b_color = border_color.unwrap_or(Color::TRANSPARENT);
            move |_| container::Style {
                background: color.map(iced::Background::Color),
                border: iced::Border {
                    color: b_color,
                    width: border_width,
                    radius: if cfg!(target_arch = "wasm32") {
                        0.0
                    } else {
                        radius
                    }
                    .into(),
                },
                ..Default::default()
            }
        })
        .into()
    }

    fn button<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        on_press: Option<Message>,
        variant: Variant,
        intent: Intent,
        width: Length,
        is_compact: bool,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::button;
        let theme = context.theme;

        if variant == Variant::Plain {
            return button(content)
                .on_press_maybe(on_press)
                .padding(Padding::ZERO)
                .style(move |_, _| button::Style {
                    background: None,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    ..Default::default()
                })
                .into();
        }

        let b = button(
            iced::widget::container(content)
                .width(width)
                .height(Length::Fill)
                .center_x(width)
                .center_y(Length::Fill),
        )
        .on_press_maybe(on_press)
        .padding(if variant == Variant::Compact {
            Padding::ZERO
        } else {
            Padding::from([0, 16])
        })
        .height(if variant == Variant::Compact {
            Length::Shrink
        } else {
            Length::Fixed(if is_compact {
                32.0 * theme.scaling
            } else {
                44.0 * theme.scaling
            })
        })
        .style(move |_, status| {
            let color = match intent {
                Intent::Primary => theme.colors.primary,
                Intent::Secondary => theme.colors.secondary,
                Intent::Accent => theme.colors.accent,
                Intent::Success => theme.colors.success,
                Intent::Warning => theme.colors.warning,
                Intent::Danger => theme.colors.danger,
                Intent::Info => theme.colors.info,
                Intent::Neutral => theme.colors.surface,
            };

            let text_color = match intent {
                Intent::Accent => theme.colors.on_accent,
                Intent::Secondary => theme.colors.on_secondary,
                _ => theme.colors.on_primary,
            };

            match variant {
                Variant::Solid => button::Style {
                    background: Some(if status == button::Status::Hovered {
                        let mut c = color;
                        c.a = 0.8;
                        c.into()
                    } else {
                        color.into()
                    }),
                    text_color: text_color,
                    border: iced::Border {
                        radius: 32.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Variant::Soft => button::Style {
                    background: Some({
                        let mut c = color;
                        c.a = 0.1;
                        if status == button::Status::Hovered {
                            c.a = 0.2;
                        }
                        c.into()
                    }),
                    text_color: color,
                    border: iced::Border {
                        radius: 32.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Variant::Outline => button::Style {
                    background: if status == button::Status::Hovered {
                        let mut c = color;
                        c.a = 0.05;
                        Some(c.into())
                    } else {
                        None
                    },
                    text_color: color,
                    border: iced::Border {
                        color,
                        width: 1.0,
                        radius: 32.0.into(),
                    },
                    ..Default::default()
                },
                Variant::Ghost => button::Style {
                    background: if status == button::Status::Hovered {
                        let mut c = color;
                        c.a = 0.1;
                        Some(c.into())
                    } else {
                        None
                    },
                    text_color: color,
                    border: iced::Border {
                        radius: 32.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Variant::Compact => button::Style {
                    background: None,
                    text_color: color,
                    border: iced::Border {
                        width: 0.0,
                        radius: 0.0.into(),
                        color: iced::Color::TRANSPARENT,
                    },
                    ..Default::default()
                },
                Variant::Plain => button::Style {
                    background: None,
                    border: iced::Border::default(),
                    ..Default::default()
                },
            }
        });

        if context.device == DeviceType::Mobile {
            crate::mobile::GestureArena::new(b).into()
        } else {
            b.into()
        }
    }

    fn sidebar_item<Message: Clone + 'static>(
        title: String,
        icon: String,
        is_selected: bool,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use crate::atoms::{Icon, Text};
        use crate::layout::HStack;
        use iced::widget::container;

        let theme = context.theme;
        let content = HStack::<Message, Self>::new_generic()
            .spacing(12.0)
            .padding(iced::Padding {
                top: 8.0,
                right: 12.0,
                bottom: 8.0,
                left: 12.0,
            })
            .align_y(iced::Alignment::Center)
            .push(Icon::<Self>::new(icon).size(18.0))
            .push(Text::<Self>::new(title).body().bold());

        if is_selected {
            container(content.view(context))
                .style({
                    let bg_color = theme.colors.primary;
                    let radius_val = if cfg!(target_arch = "wasm32") {
                        0.0
                    } else {
                        8.0
                    };
                    move |_theme| container::Style {
                        background: Some(bg_color.into()),
                        border: iced::Border {
                            radius: radius_val.into(),
                            ..Default::default()
                        },
                        text_color: Some(iced::Color::WHITE),
                        ..Default::default()
                    }
                })
                .width(Length::Fill)
                .into()
        } else {
            content.view(context)
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn text_input<Message: Clone + 'static>(
        value: String,
        placeholder: String,
        on_change: impl Fn(String) -> Message + 'static,
        on_submit: Option<Message>,
        font: Option<iced::Font>,
        is_secure: bool,
        variant: Variant,
        id: Option<iced::widget::Id>,
        dom_id: Option<String>,
        context: &Context,
    ) -> Self::AnyView<Message> {
        let mut input = iced::widget::text_input(&placeholder, &value)
            .on_input(on_change)
            .secure(is_secure);

        // Clone id for later use in dom_id generation (before it gets moved)
        let widget_id_for_dom = id.clone();

        if let Some(id) = id {
            input = input.id(id);
        }

        if let Some(msg) = on_submit {
            input = input.on_submit(msg);
        }

        if let Some(font) = font {
            input = input.font(font);
        }

        // Apply variant style
        input = match variant {
            Variant::Ghost => {
                let colors = context.theme.colors;
                input.style(move |_theme, _status| iced::widget::text_input::Style {
                    background: iced::Background::Color(iced::Color::TRANSPARENT),
                    border: iced::Border::default(),
                    icon: colors.text_secondary,
                    placeholder: colors.text_secondary,
                    value: colors.text_primary,
                    selection: colors.primary,
                })
            }
            _ => input,
        };

        let element: iced::Element<'static, Message, Theme, Renderer> = input.padding(10).into();

        // Only use FocusBridge on actual mobile devices
        // Need both touch capability AND mobile user agent (MacBooks have touch trackpads!)
        let is_mobile_device = {
            if let Some(window) = web_sys::window() {
                let navigator = window.navigator();
                let user_agent = navigator.user_agent().unwrap_or_default().to_lowercase();

                // Check if user agent indicates mobile device
                let is_mobile_ua = user_agent.contains("mobile")
                    || user_agent.contains("android")
                    || user_agent.contains("iphone")
                    || user_agent.contains("ipad")
                    || user_agent.contains("ipod");

                // Also check for touch capability
                let has_touch =
                    js_sys::Reflect::has(&window, &wasm_bindgen::JsValue::from_str("ontouchstart"))
                        .unwrap_or(false);

                let max_touch_points = js_sys::Reflect::get(
                    &navigator,
                    &wasm_bindgen::JsValue::from_str("maxTouchPoints"),
                )
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);

                // Require BOTH mobile user agent AND touch capability
                // This excludes desktop with trackpad
                is_mobile_ua && (has_touch || max_touch_points > 0.0)
            } else {
                false
            }
        };

        if is_mobile_device {
            // On actual mobile devices, use overlay for keyboard support
            let final_dom_id = dom_id.or_else(|| {
                widget_id_for_dom
                    .as_ref()
                    .map(|widget_id| format!("text-input-{:?}", widget_id))
                    .or_else(|| {
                        use std::collections::hash_map::DefaultHasher;
                        use std::hash::{Hash, Hasher};
                        let mut hasher = DefaultHasher::new();
                        value.hash(&mut hasher);
                        placeholder.hash(&mut hasher);
                        Some(format!("text-input-{}", hasher.finish()))
                    })
            });

            if let Some(dom_id) = final_dom_id {
                wasm_portal::FocusBridge::new(element, dom_id, value).into()
            } else {
                element
            }
        } else {
            // Desktop (no touch): use normal Iced input (works perfectly)
            element
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn text_input<Message: Clone + 'static>(
        value: String,
        placeholder: String,
        on_change: impl Fn(String) -> Message + 'static,
        on_submit: Option<Message>,
        font: Option<iced::Font>,
        is_secure: bool,
        variant: Variant,
        id: Option<iced::widget::Id>,
        _dom_id: Option<String>,
        context: &Context,
    ) -> Self::AnyView<Message> {
        let mut input = iced::widget::text_input(&placeholder, &value)
            .on_input(on_change)
            .secure(is_secure);

        if let Some(id) = id {
            input = input.id(id);
        }

        if let Some(msg) = on_submit {
            input = input.on_submit(msg);
        }

        if let Some(font) = font {
            input = input.font(font);
        }

        // Apply variant style
        input = match variant {
            Variant::Ghost => {
                let colors = context.theme.colors;
                input.style(move |_theme, _status| iced::widget::text_input::Style {
                    background: iced::Background::Color(iced::Color::TRANSPARENT),
                    border: iced::Border::default(),
                    icon: colors.text_secondary,
                    placeholder: colors.text_secondary,
                    value: colors.text_primary,
                    selection: colors.primary,
                })
            }
            _ => input,
        };

        input.padding(10).into()
    }

    fn slider<Message: Clone + 'static>(
        range: std::ops::RangeInclusive<f32>,
        value: f32,
        on_change: impl Fn(f32) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        iced::widget::slider(range, value, on_change).into()
    }

    fn toggle<Message: Clone + 'static>(
        label: String,
        is_active: bool,
        on_toggle: impl Fn(bool) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        iced::widget::toggler(is_active)
            .label(label)
            .on_toggle(on_toggle)
            .into()
    }

    fn zstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        width: Length,
        height: Length,
        _alignment: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let s = iced::widget::stack(children).width(width).height(height);
        s.into()
    }

    fn grid<Message: 'static>(
        mut children: Vec<Self::AnyView<Message>>,
        columns: usize,
        spacing: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        if columns == 0 {
            return iced::widget::column(children).spacing(spacing).into();
        }
        let mut rows = Vec::new();
        while !children.is_empty() {
            let chunk: Vec<_> = children
                .drain(0..std::cmp::min(columns, children.len()))
                .map(|child| iced::widget::container(child).width(Length::Fill).into())
                .collect();
            rows.push(
                iced::widget::row(chunk)
                    .spacing(spacing)
                    .width(Length::Fill)
                    .into(),
            );
        }
        iced::widget::column(rows)
            .spacing(spacing)
            .width(Length::Fill)
            .into()
    }

    fn image<Message: 'static>(
        path: impl Into<String>,
        width: Length,
        height: Length,
        radius: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let p: String = path.into();

        #[cfg(target_arch = "wasm32")]
        {
            use iced::widget::container;
            use iced::widget::image as iced_image;
            use iced::Color;

            match wasm_portal::get_image(p) {
                wasm_portal::ImageState::Loaded(handle) => container(
                    iced_image(handle)
                        .width(width)
                        .height(height)
                        .content_fit(iced::ContentFit::Cover),
                )
                .style(move |_| container::Style {
                    border: iced::Border {
                        radius: radius.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .into(),
                wasm_portal::ImageState::Loading => {
                    container(iced::widget::Space::new().width(width).height(height))
                        .style(move |_| container::Style {
                            background: Some(iced::Background::Color(Color::from_rgba(
                                1.0, 1.0, 1.0, 0.1,
                            ))),
                            border: iced::Border {
                                radius: radius.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .into()
                }
                wasm_portal::ImageState::Error => {
                    container(iced::widget::Space::new().width(width).height(height))
                        .style(move |_| container::Style {
                            background: Some(iced::Background::Color(Color::from_rgb(
                                1.0, 0.0, 0.0,
                            ))),
                            border: iced::Border {
                                radius: radius.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .into()
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            use iced::widget::container;

            // On native, we want relative paths from the current working directory
            let path_str = if p.starts_with('/') {
                p[1..].to_string()
            } else {
                p
            };

            let handle = iced::widget::image::Handle::from_path(path_str);

            container(
                iced::widget::image(handle)
                    .width(width)
                    .height(height)
                    .content_fit(iced::ContentFit::Cover),
            )
            .style(move |_| container::Style {
                border: iced::Border {
                    radius: radius.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
        }
    }

    fn video<Message: 'static>(
        _path: impl Into<String>,
        width: Length,
        height: Length,
        radius: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::{container, text};

        // Pending Implementation Message
        container(
            text("Video support is currently pending implementation in the PeakUI framework.")
                .size(14.0)
                .color(Color::WHITE.scale_alpha(0.6)),
        )
        .width(width)
        .height(height)
        .center_x(width)
        .center_y(height)
        .style(move |theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(palette.background.weak.color.into()),
                border: iced::Border {
                    radius: radius.into(),
                    color: Color::WHITE.scale_alpha(0.1),
                    width: 1.0,
                },
                ..Default::default()
            }
        })
        .into()
    }

    fn web_view<Message: 'static>(
        url: String,
        width: Length,
        height: Length,
        radius: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        #[cfg(target_arch = "wasm32")]
        {
            return wasm_portal::WebView::new(url, width, height, radius).into();
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            use iced::widget::{column, container, text};
            use iced::Alignment;

            container(
                column![
                    text("Native Web Support Not Supported")
                        .size(16.0)
                        .color(Color::WHITE),
                    text(url).size(12.0).color(Color::WHITE.scale_alpha(0.5))
                ]
                .spacing(8)
                .align_x(Alignment::Center),
            )
            .width(width)
            .height(height)
            .center_x(width)
            .center_y(height)
            .style(move |_| container::Style {
                background: Some(Color::BLACK.into()),
                border: iced::Border {
                    radius: radius.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
        }
    }

    fn container<Message: 'static>(
        content: Self::AnyView<Message>,
        padding: Padding,
        width: Length,
        height: Length,
        background: Option<Color>,
        radius: f32,
        border_width: f32,
        border_color: Option<Color>,
        shadow: Option<iced::Shadow>,
        align_x: Alignment,
        align_y: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::container;
        let scale = context.theme.scaling;

        let mut c = container(content)
            .padding(Padding {
                top: padding.top * scale,
                right: padding.right * scale,
                bottom: padding.bottom * scale,
                left: padding.left * scale,
            })
            .width(scale_length(width, scale))
            .height(scale_length(height, scale))
            .style(move |_| container::Style {
                background: background.map(iced::Background::Color),
                border: iced::Border {
                    radius: (radius * scale).into(),
                    width: border_width * scale,
                    color: border_color.unwrap_or(iced::Color::TRANSPARENT),
                },
                shadow: shadow.unwrap_or_default(),
                ..Default::default()
            });

        if align_x == Alignment::Center && width != Length::Shrink {
            c = c.center_x(scale_length(width, scale));
        } else if align_x == Alignment::End && width != Length::Shrink {
            c = c.align_x(iced::alignment::Horizontal::Right);
        }

        if align_y == Alignment::Center && height != Length::Shrink {
            c = c.center_y(scale_length(height, scale));
        } else if align_y == Alignment::End && height != Length::Shrink {
            c = c.align_y(iced::alignment::Vertical::Bottom);
        }

        c.into()
    }

    fn scroll_view<Message: 'static>(
        content: Self::AnyView<Message>,
        width: Length,
        height: Length,
        id: Option<&'static str>,
        show_indicators: bool,
        context: &Context,
    ) -> Self::AnyView<Message> {
        let text_color = context.theme.colors.text_primary;
        let mut scroll = iced::widget::scrollable(content)
            .width(width)
            .height(height);

        if !show_indicators {
            scroll = scroll.style(|_, _| iced::widget::scrollable::Style {
                container: iced::widget::container::Style::default(),
                vertical_rail: iced::widget::scrollable::Rail {
                    background: None,
                    border: iced::Border::default(),
                    scroller: iced::widget::scrollable::Scroller {
                        background: iced::Color::TRANSPARENT.into(),
                        border: iced::Border::default(),
                    },
                },
                horizontal_rail: iced::widget::scrollable::Rail {
                    background: None,
                    border: iced::Border::default(),
                    scroller: iced::widget::scrollable::Scroller {
                        background: iced::Color::TRANSPARENT.into(),
                        border: iced::Border::default(),
                    },
                },
                gap: None,
                auto_scroll: iced::widget::scrollable::AutoScroll {
                    background: iced::Background::Color(iced::Color::TRANSPARENT),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    icon: iced::Color::TRANSPARENT,
                },
            });
        } else {
            scroll = scroll.style(move |_, status| {
                let scroller_alpha = match status {
                    iced::widget::scrollable::Status::Hovered { .. } => 0.3,
                    iced::widget::scrollable::Status::Dragged { .. } => 0.5,
                    _ => 0.05,
                };

                iced::widget::scrollable::Style {
                    container: iced::widget::container::Style::default(),
                    vertical_rail: iced::widget::scrollable::Rail {
                        background: None,
                        border: iced::Border::default(),
                        scroller: iced::widget::scrollable::Scroller {
                            background: iced::Color {
                                a: scroller_alpha,
                                ..text_color
                            }
                            .into(),
                            border: iced::Border {
                                radius: 2.0.into(),
                                width: 0.0,
                                ..Default::default()
                            },
                        },
                    },
                    horizontal_rail: iced::widget::scrollable::Rail {
                        background: None,
                        border: iced::Border::default(),
                        scroller: iced::widget::scrollable::Scroller {
                            background: iced::Color {
                                a: scroller_alpha,
                                ..text_color
                            }
                            .into(),
                            border: iced::Border {
                                radius: 2.0.into(),
                                width: 0.0,
                                ..Default::default()
                            },
                        },
                    },
                    gap: None,
                    auto_scroll: iced::widget::scrollable::AutoScroll {
                        background: iced::Background::Color(iced::Color::TRANSPARENT),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                        icon: iced::Color::TRANSPARENT,
                    },
                }
            });
        }

        if !show_indicators {
            scroll = scroll.style(|_, _| iced::widget::scrollable::Style {
                container: iced::widget::container::Style::default(),
                vertical_rail: iced::widget::scrollable::Rail {
                    background: None,
                    border: iced::Border::default(),
                    scroller: iced::widget::scrollable::Scroller {
                        background: iced::Color::TRANSPARENT.into(),
                        border: iced::Border::default(),
                    },
                },
                horizontal_rail: iced::widget::scrollable::Rail {
                    background: None,
                    border: iced::Border::default(),
                    scroller: iced::widget::scrollable::Scroller {
                        background: iced::Color::TRANSPARENT.into(),
                        border: iced::Border::default(),
                    },
                },
                gap: None,
                auto_scroll: iced::widget::scrollable::AutoScroll {
                    background: iced::Background::Color(iced::Color::TRANSPARENT),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    icon: iced::Color::TRANSPARENT,
                },
            });
        } else {
            scroll = scroll.style(move |_, status| {
                let scroller_alpha = match status {
                    iced::widget::scrollable::Status::Hovered { .. } => 0.3,
                    iced::widget::scrollable::Status::Dragged { .. } => 0.5,
                    _ => 0.05,
                };

                iced::widget::scrollable::Style {
                    container: iced::widget::container::Style::default(),
                    vertical_rail: iced::widget::scrollable::Rail {
                        background: None,
                        border: iced::Border::default(),
                        scroller: iced::widget::scrollable::Scroller {
                            background: iced::Color {
                                a: scroller_alpha,
                                ..text_color
                            }
                            .into(),
                            border: iced::Border {
                                radius: 2.0.into(),
                                width: 0.0,
                                ..Default::default()
                            },
                        },
                    },
                    horizontal_rail: iced::widget::scrollable::Rail {
                        background: None,
                        border: iced::Border::default(),
                        scroller: iced::widget::scrollable::Scroller {
                            background: iced::Color {
                                a: scroller_alpha,
                                ..text_color
                            }
                            .into(),
                            border: iced::Border {
                                radius: 2.0.into(),
                                width: 0.0,
                                ..Default::default()
                            },
                        },
                    },
                    gap: None,
                    auto_scroll: iced::widget::scrollable::AutoScroll {
                        background: iced::Background::Color(iced::Color::TRANSPARENT),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                        icon: iced::Color::TRANSPARENT,
                    },
                }
            });
        }

        if let Some(id) = id {
            scroll = scroll.id(Id::new(id));
        }
        scroll.into()
    }

    fn mouse_area<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        on_move: Option<Arc<dyn Fn(iced::Point) -> Message + Send + Sync>>,
        on_press: Option<Message>,
        on_release: Option<Message>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::mouse_area;
        let mut area = mouse_area(content);
        if let Some(f) = on_move {
            let f_clone = f.clone();
            area = area.on_move(move |p| f_clone(p));
        }
        if let Some(msg) = on_press {
            area = area.on_press(msg);
        }
        if let Some(msg) = on_release {
            area = area.on_release(msg);
        }
        area.into()
    }

    fn with_tooltip<Message: 'static>(
        content: Self::AnyView<Message>,
        tooltip_text: String,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::tooltip;
        let scale = context.theme.scaling;

        tooltip(
            content,
            iced::widget::text(tooltip_text).size(14.0 * scale),
            tooltip::Position::Bottom,
        )
        .into()
    }

    fn glass_card<Message: 'static>(
        content: Self::AnyView<Message>,
        padding: Padding,
        width: Length,
        height: Length,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::container;
        let theme = context.theme;
        let mut bg = theme.colors.surface;
        bg.a = theme.glass_opacity;

        let shadow = context.shadow(
            theme.shadow_color,
            iced::Vector::new(theme.shadow_offset[0], theme.shadow_offset[1]),
            theme.shadow_blur,
        );

        let scale = theme.scaling;
        let radius = context.radius(theme.radius * scale);
        let padding = Padding {
            top: padding.top * scale,
            right: padding.right * scale,
            bottom: padding.bottom * scale,
            left: padding.left * scale,
        };

        container(content)
            .padding(padding)
            .width(scale_length(width, scale))
            .height(scale_length(height, scale))
            .center_y(Length::Fill)
            .style(move |_| container::Style {
                background: Some(bg.into()),
                border: iced::Border {
                    radius,
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.15),
                    width: 1.0 * scale,
                },
                shadow,
                ..Default::default()
            })
            .into()
    }

    fn section<Message: 'static>(
        title: String,
        content: Self::AnyView<Message>,
        width: Length,
        height: Length,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::{column, container, text};
        let scale = context.theme.scaling;

        container(
            column![
                text(title)
                    .size(12.0 * scale)
                    .color(context.theme.colors.text_primary.scale_alpha(0.6)),
                content
            ]
            .spacing(8.0 * scale),
        )
        .width(scale_length(width, scale))
        .height(scale_length(height, scale))
        .into()
    }

    fn spatial_modifier<Message: 'static>(
        content: Self::AnyView<Message>,
        _position: Vector3<f32>,
        _scale: Vector3<f32>,
        _rotation: Vector3<f32>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }
}

/// A Terminal-based TUI backend.
#[derive(Clone, Copy, Debug, Default)]
pub struct TermBackend;

impl Backend for TermBackend {
    type AnyView<Message: 'static> = String;

    fn rich_text<Message: Clone + 'static>(
        _spans: Vec<TextSpan>,
        _size: f32,
        _width: Length,
        _alignment: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        String::new()
    }

    fn with_tooltip<Message: 'static>(
        content: Self::AnyView<Message>,
        tooltip: String,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!("{} (Tooltip: {})", content, tooltip)
    }

    fn glass_card<Message: 'static>(
        content: Self::AnyView<Message>,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!("(GLASS)\n{}", content)
    }

    fn section<Message: 'static>(
        title: String,
        content: Self::AnyView<Message>,
        _width: Length,
        _height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!("\x1b[1;2m# {}\x1b[0m\n{}", title.to_uppercase(), content)
    }

    fn vstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_x: Alignment,
        _align_y: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        children.join("\n")
    }

    fn hstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_x: Alignment,
        _align_y: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        children.join(" ")
    }

    fn wrap<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _spacing: f32,
        _run_spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_x: Alignment,
        _align_y: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        children.join(" ")
    }

    fn text<Message: Clone + 'static>(
        content: String,
        _size: f32,
        _color: Option<Color>,
        is_bold: bool,
        is_dim: bool,
        intent: Option<Intent>,
        _font: Option<iced::Font>,
        _width: Length,
        _alignment: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let mut out = content;
        if is_bold {
            out = format!("\x1b[1m{}\x1b[0m", out);
        } else if is_dim {
            out = format!("\x1b[2m{}\x1b[0m", out);
        }

        if let Some(i) = intent {
            let code = match i {
                Intent::Primary => "34",
                Intent::Secondary => "30",
                Intent::Accent => "35", // Magenta for accent in terminal
                Intent::Success => "32",
                Intent::Warning => "33",
                Intent::Danger => "31",
                Intent::Info => "36",
                Intent::Neutral => "0",
            };
            out = format!("\x1b[{}m{}\x1b[0m", code, out);
        }
        out
    }

    fn icon<Message: Clone + 'static>(
        name: String,
        _size: f32,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let symbol = match name.as_str() {
            "settings" => "⚙",
            "terminal" => "",
            "chevron_right" => "",
            _ => "○",
        };
        format!("\x1b[36m{}\x1b[0m", symbol)
    }

    fn divider<Message: 'static>(_context: &Context) -> Self::AnyView<Message> {
        "────────────────────".to_string()
    }

    fn space<Message: 'static>(
        _width: Length,
        _height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        " ".to_string()
    }

    fn circle<Message: 'static>(
        _radius: f32,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        "O".to_string()
    }

    fn capsule<Message: 'static>(
        _width: Length,
        _height: Length,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        "=".to_string()
    }

    fn rectangle<Message: 'static>(
        _width: Length,
        _height: Length,
        _color: Option<Color>,
        _radius: f32,
        _border_width: f32,
        _border_color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        "█".to_string()
    }

    fn button<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        _on_press: Option<Message>,
        _variant: Variant,
        _intent: Intent,
        _width: Length,
        _is_compact: bool,
        context: &Context,
    ) -> Self::AnyView<Message> {
        if context.is_focused("button") {
            format!("> [ {} ] <", content)
        } else {
            format!("  [ {} ]  ", content)
        }
    }

    fn sidebar_item<Message: Clone + 'static>(
        title: String,
        _icon: String,
        is_selected: bool,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        if is_selected {
            format!("\x1b[1;34m {}\x1b[0m", title)
        } else {
            format!("  {}", title)
        }
    }

    fn text_input<Message: Clone + 'static>(
        value: String,
        placeholder: String,
        _on_change: impl Fn(String) -> Message + 'static,
        _on_submit: Option<Message>,
        _font: Option<iced::Font>,
        is_secure: bool,
        _variant: Variant,
        _id: Option<iced::widget::Id>,
        _dom_id: Option<String>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!(
            "[Input:{}:{}:{}]",
            value,
            placeholder,
            if is_secure { "***" } else { "" }
        )
    }

    fn slider<Message: Clone + 'static>(
        _range: std::ops::RangeInclusive<f32>,
        value: f32,
        _on_change: impl Fn(f32) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!("[---X---] {:.2}", value)
    }

    fn toggle<Message: Clone + 'static>(
        label: String,
        is_active: bool,
        _on_toggle: impl Fn(bool) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!("{} [{}]", label, if is_active { "ON" } else { "OFF" })
    }

    fn zstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _width: Length,
        _height: Length,
        _alignment: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        children.join("\n")
    }

    fn grid<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _columns: usize,
        _spacing: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        children.join(" | ")
    }

    fn image<Message: 'static>(
        path: impl Into<String>,
        _width: Length,
        _height: Length,
        _radius: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!("[IMG: {}]", path.into())
    }

    fn video<Message: 'static>(
        path: impl Into<String>,
        _width: Length,
        _height: Length,
        _radius: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!("[VIDEO: {}]", path.into())
    }

    fn web_view<Message: 'static>(
        url: String,
        _width: Length,
        _height: Length,
        _radius: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!("[WEB: {}]", url)
    }

    fn container<Message: 'static>(
        content: Self::AnyView<Message>,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _background: Option<Color>,
        _radius: f32,
        _border_width: f32,
        _border_color: Option<Color>,
        _shadow: Option<iced::Shadow>,
        _align_x: Alignment,
        _align_y: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn scroll_view<Message: 'static>(
        content: Self::AnyView<Message>,
        _width: Length,
        _height: Length,
        _id: Option<&'static str>,
        _show_indicators: bool,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn spatial_modifier<Message: 'static>(
        content: Self::AnyView<Message>,
        _position: Vector3<f32>,
        _scale: Vector3<f32>,
        _rotation: Vector3<f32>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn mouse_area<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        _on_move: Option<Arc<dyn Fn(iced::Point) -> Message + Send + Sync>>,
        _on_press: Option<Message>,
        _on_release: Option<Message>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }
}

/// A View describes *what* to render, given a Context.
pub trait View<Message: 'static, B: Backend = IcedBackend> {
    fn view(&self, context: &Context) -> B::AnyView<Message>;

    fn neural(self, tag: impl Into<String>) -> NeuralView<Message, B, Self>
    where
        Self: Sized + 'static,
    {
        NeuralView {
            inner: self,
            tag: tag.into(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn billboard(self, active: bool) -> SpatialBillboard<Message, B, Self>
    where
        Self: Sized + 'static,
    {
        SpatialBillboard {
            inner: self,
            active,
            _phantom: std::marker::PhantomData,
        }
    }

    fn physical_depth(self, depth: f32) -> PhysicalDepthView<Message, B, Self>
    where
        Self: Sized + 'static,
    {
        PhysicalDepthView {
            inner: self,
            depth,
            _phantom: std::marker::PhantomData,
        }
    }

    fn document(self, message: impl Into<String>) -> DocumentedView<Message, B, Self>
    where
        Self: Sized + 'static,
    {
        DocumentedView {
            inner: self,
            documentation: message.into(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn describe(&self, _context: &Context) -> SemanticNode {
        SemanticNode {
            role: "view".to_string(),
            ..Default::default()
        }
    }

    /// Generates a semantic description of the view for AI agents, specifically for Iced backend.
    /// This is a temporary workaround for the fact that `describe` is generic over `B`,
    /// but `iced::Element` cannot be easily converted to `SemanticNode` without knowing its internal structure.
    /// This method should ideally be removed once `describe` can be made to work with `iced::Element` directly.
    fn describe_iced(&self, _context: &Context) -> SemanticNode {
        SemanticNode {
            role: "iced_view".to_string(),
            ..Default::default()
        }
    }

    fn sudo(self, reason: impl Into<String>) -> NeuralSudo<Message, B, Self>
    where
        Self: Sized + 'static,
    {
        NeuralSudo {
            inner: self,
            reason: reason.into(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn into_box(self) -> Box<dyn View<Message, B>>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }

    fn on_tap_gesture(self, msg: Message) -> crate::gestures::TapGesture<Message, B, Self>
    where
        Self: Sized + 'static,
    {
        crate::gestures::TapGesture::new(self, msg)
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for Box<dyn View<Message, B>> {
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

/// A semantic representation of a UI component for AI agents.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SemanticNode {
    #[serde(rename = "r")]
    pub role: String,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "l", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "c", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "ch", skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<SemanticNode>,
    #[serde(rename = "t", skip_serializing_if = "Option::is_none")]
    pub neural_tag: Option<String>,
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(rename = "a", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<AccessibilityNode>,
    #[serde(rename = "p", skip_serializing_if = "is_false")]
    pub is_protected: bool,
    #[serde(rename = "pr", skip_serializing_if = "Option::is_none")]
    pub protection_reason: Option<String>,
    #[serde(rename = "z", skip_serializing_if = "Option::is_none")]
    pub depth: Option<f32>,
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f32; 3]>,
}

fn is_false(b: &bool) -> bool {
    !*b
}

impl SemanticNode {
    /// Recursively find a node that matches the predicate
    pub fn find_deep<F>(&self, predicate: &F) -> Option<&SemanticNode>
    where
        F: Fn(&SemanticNode) -> bool,
    {
        if predicate(self) {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find_deep(predicate) {
                return Some(found);
            }
        }
        None
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemanticRecord {
    pub id: String,
    pub collection: String,
    pub content: String,
    pub vector: Option<Vec<f32>>,
    pub metadata: serde_json::Value,
    pub timestamp: u64,
}

pub trait DataProvider: Send + Sync {
    fn save(&self, record: SemanticRecord) -> Task<std::result::Result<(), String>>;
    fn find(&self, query: String) -> Task<std::result::Result<Vec<SemanticRecord>, String>>;
    fn delete(&self, id: String) -> Task<std::result::Result<(), String>>;

    /// Async version for internal use (e.g. by IntelligenceProvider)
    fn async_find(
        &self,
        query: String,
    ) -> iced::futures::future::BoxFuture<'static, std::result::Result<Vec<SemanticRecord>, String>>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatCompletionMessage {
    pub role: String,
    pub content: String,
}

pub trait IntelligenceProvider: Send + Sync {
    fn chat(
        &self,
        messages: Vec<ChatCompletionMessage>,
    ) -> Task<std::result::Result<String, String>>;

    fn chat_stream(
        &self,
        _messages: Vec<ChatCompletionMessage>,
    ) -> iced::futures::stream::BoxStream<'static, std::result::Result<String, String>> {
        use iced::futures::StreamExt;
        iced::futures::stream::empty().boxed()
    }

    fn reason(
        &self,
        prompt: String,
        context: Vec<SemanticNode>,
    ) -> Task<std::result::Result<String, String>> {
        let context_json = serde_json::to_string(&context).unwrap_or_default();
        let messages = vec![
            ChatCompletionMessage {
                role: "system".to_string(),
                content: format!("UI Context: {}", context_json),
            },
            ChatCompletionMessage {
                role: "user".to_string(),
                content: prompt,
            },
        ];
        self.chat(messages)
    }

    fn execute_tool(
        &self,
        name: String,
        args: serde_json::Value,
    ) -> Task<std::result::Result<serde_json::Value, String>>;

    fn get_system_context(&self) -> String {
        "Peak Intelligence Provider".to_string()
    }
}

#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct AccessibilityNode {
    #[serde(rename = "r")]
    pub role: String,
    #[serde(rename = "l")]
    pub label: String,
    #[serde(rename = "h", skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
    #[serde(rename = "v", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "s", skip_serializing_if = "Vec::is_empty")]
    pub states: Vec<String>,
}

/// An AI-focused backend that renders UIs into semantic data.
#[derive(Clone, Copy, Debug, Default)]
pub struct AIBackend;

impl Backend for AIBackend {
    type AnyView<Message: 'static> = SemanticNode;

    fn rich_text<Message: Clone + 'static>(
        _spans: Vec<TextSpan>,
        _size: f32,
        _width: Length,
        _alignment: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::default()
    }

    fn with_tooltip<Message: 'static>(
        content: Self::AnyView<Message>,
        _tooltip: String,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn glass_card<Message: 'static>(
        content: Self::AnyView<Message>,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn section<Message: 'static>(
        _title: String,
        content: Self::AnyView<Message>,
        _width: Length,
        _height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn vstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_x: Alignment,
        _align_y: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let mut children = children;
        for child in &mut children {
            child.depth = Some(child.depth.unwrap_or(0.0) + 1.0);
        }

        SemanticNode {
            accessibility: None,
            role: "vstack".to_string(),
            label: None,
            content: None,
            children,
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn hstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_x: Alignment,
        _align_y: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let mut children = children;
        for child in &mut children {
            child.depth = Some(child.depth.unwrap_or(0.0) + 1.0);
        }

        SemanticNode {
            accessibility: None,
            role: "hstack".to_string(),
            label: None,
            content: None,
            children,
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn wrap<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _spacing: f32,
        _run_spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_x: Alignment,
        _align_y: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let mut children = children;
        for child in &mut children {
            child.depth = Some(child.depth.unwrap_or(0.0) + 1.0);
        }

        SemanticNode {
            accessibility: None,
            role: "wrap".to_string(),
            label: None,
            content: None,
            children,
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn text<Message: Clone + 'static>(
        content: String,
        _size: f32,
        _color: Option<Color>,
        _is_bold: bool,
        _is_dim: bool,
        _intent: Option<Intent>,
        _font: Option<iced::Font>,
        _width: Length,
        _alignment: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            accessibility: None,
            role: "text".to_string(),
            label: None,
            content: Some(content),
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn icon<Message: Clone + 'static>(
        name: String,
        _size: f32,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            accessibility: None,
            role: "icon".to_string(),
            label: Some(name),
            content: None,
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn divider<Message: 'static>(_context: &Context) -> Self::AnyView<Message> {
        SemanticNode {
            accessibility: None,
            role: "divider".to_string(),
            label: None,
            content: None,
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn space<Message: 'static>(
        _width: Length,
        _height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            accessibility: None,
            role: "space".to_string(),
            label: None,
            content: None,
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn circle<Message: 'static>(
        _radius: f32,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            accessibility: None,
            role: "circle".to_string(),
            label: None,
            content: None,
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn capsule<Message: 'static>(
        _width: Length,
        _height: Length,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            accessibility: None,
            role: "capsule".to_string(),
            label: None,
            content: None,
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn rectangle<Message: 'static>(
        _width: Length,
        _height: Length,
        _color: Option<Color>,
        _radius: f32,
        _border_width: f32,
        _border_color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            accessibility: None,
            role: "rectangle".to_string(),
            label: None,
            content: None,
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn button<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        _on_press: Option<Message>,
        variant: Variant,
        intent: Intent,
        _width: Length,
        _is_compact: bool,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            accessibility: None,
            role: "button".to_string(),
            label: Some(format!("{:?}_{:?}", variant, intent)),
            content: None,
            children: vec![content],
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn sidebar_item<Message: Clone + 'static>(
        title: String,
        icon: String,
        is_selected: bool,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            accessibility: None,
            role: "sidebar_item".to_string(),
            label: Some(title),
            content: Some(icon),
            children: vec![SemanticNode {
                role: "state".to_string(),
                label: Some("selected".to_string()),
                content: Some(is_selected.to_string()),
                ..Default::default()
            }],
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn text_input<Message: Clone + 'static>(
        value: String,
        _placeholder: String,
        _on_change: impl Fn(String) -> Message + 'static,
        _on_submit: Option<Message>,
        _font: Option<iced::Font>,
        _is_secure: bool,
        _variant: Variant,
        _id: Option<iced::widget::Id>,
        _dom_id: Option<String>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            accessibility: None,
            role: "text_input".to_string(),
            label: Some(value.clone()),
            content: Some(value),
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn slider<Message: Clone + 'static>(
        _range: std::ops::RangeInclusive<f32>,
        value: f32,
        _on_change: impl Fn(f32) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            accessibility: None,
            role: "slider".to_string(),
            label: None,
            content: Some(value.to_string()),
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn toggle<Message: Clone + 'static>(
        label: String,
        is_active: bool,
        _on_toggle: impl Fn(bool) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            accessibility: None,
            role: "toggle".to_string(),
            label: Some(label),
            content: Some(is_active.to_string()),
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn zstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _width: Length,
        _height: Length,
        _alignment: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            accessibility: None,
            role: "zstack".to_string(),
            label: None,
            content: None,
            children,
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn grid<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        columns: usize,
        _spacing: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            accessibility: None,
            role: "grid".to_string(),
            label: Some(format!("columns: {}", columns)),
            content: None,
            children,
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }

    fn image<Message: 'static>(
        path: impl Into<String>,
        _width: Length,
        _height: Length,
        _radius: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "image".to_string(),
            label: Some(path.into()),
            ..Default::default()
        }
    }

    fn video<Message: 'static>(
        path: impl Into<String>,
        _width: Length,
        _height: Length,
        _radius: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "video".to_string(),
            label: Some(path.into()),
            ..Default::default()
        }
    }

    fn web_view<Message: 'static>(
        url: String,
        _width: Length,
        _height: Length,
        _radius: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "web_view".to_string(),
            label: Some(url),
            ..Default::default()
        }
    }

    fn container<Message: 'static>(
        content: Self::AnyView<Message>,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _background: Option<Color>,
        _radius: f32,
        _border_width: f32,
        _border_color: Option<Color>,
        _shadow: Option<iced::Shadow>,
        _align_x: Alignment,
        _align_y: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn scroll_view<Message: 'static>(
        content: Self::AnyView<Message>,
        _width: Length,
        _height: Length,
        _id: Option<&'static str>,
        _show_indicators: bool,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn mouse_area<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        _on_move: Option<Arc<dyn Fn(iced::Point) -> Message + Send + Sync>>,
        _on_press: Option<Message>,
        _on_release: Option<Message>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn spatial_modifier<Message: 'static>(
        mut content: Self::AnyView<Message>,
        position: Vector3<f32>,
        scale: Vector3<f32>,
        _rotation: Vector3<f32>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content.depth = Some(content.depth.unwrap_or(0.0) + position.z);
        let s = content.scale.unwrap_or([1.0, 1.0, 1.0]);
        content.scale = Some([s[0] * scale.x, s[1] * scale.y, s[2] * scale.z]);
        content
    }
}

/// A responsive helper.
pub fn responsive<Message>(
    mode: ShellMode,
    theme: peak_theme::ThemeTokens,
    localization: Localization,
    f: impl Fn(Context) -> iced::Element<'static, Message, Theme, Renderer> + 'static,
) -> iced::Element<'static, Message, Theme, Renderer>
where
    Message: 'static,
{
    iced::widget::responsive(move |size| {
        let context = Context::new(mode, theme, size, localization.clone());
        f(context)
    })
    .into()
}

pub struct ProxyView<Message: Clone + 'static, B: Backend = IcedBackend> {
    view_fn: Box<dyn Fn(&Context) -> B::AnyView<Message>>,
}

impl<Message: Clone + 'static, B: Backend> ProxyView<Message, B> {
    pub fn new<F>(view_fn: F) -> Self
    where
        F: Fn(&Context) -> B::AnyView<Message> + 'static,
    {
        Self {
            view_fn: Box::new(view_fn),
        }
    }
}

impl<Message: Clone + 'static, B: Backend> View<Message, B> for ProxyView<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        (self.view_fn)(context)
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_portal {
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::mouse;
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget};
    use iced::widget::image::Handle;
    use iced::{Element, Length, Rectangle, Size, Theme};
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    pub struct WebView {
        url: String,
        width: Length,
        height: Length,
        radius: f32,
        id: u64,
    }

    impl WebView {
        pub fn new(url: String, width: Length, height: Length, radius: f32) -> Self {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            url.hash(&mut hasher);
            let id = hasher.finish();

            Self {
                url,
                width,
                height,
                radius,
                id,
            }
        }
    }

    impl<Message, Renderer> Widget<Message, Theme, Renderer> for WebView
    where
        Renderer: renderer::Renderer,
    {
        fn size(&self) -> Size<Length> {
            Size::new(self.width, self.height)
        }

        fn layout(
            &mut self,
            _tree: &mut widget::Tree,
            _renderer: &Renderer,
            limits: &layout::Limits,
        ) -> layout::Node {
            layout::Node::new(limits.resolve(self.width, self.height, Size::ZERO))
        }

        fn draw(
            &self,
            _tree: &widget::Tree,
            _renderer: &mut Renderer,
            _theme: &Theme,
            _style: &renderer::Style,
            layout: Layout<'_>,
            _cursor: iced::mouse::Cursor,
            _viewport: &Rectangle,
        ) {
            use wasm_bindgen::JsCast;
            use web_sys::{window, HtmlIFrameElement};

            let bounds = layout.bounds();
            let element_id = format!("peakui-webview-{}", self.id);

            let window = window().unwrap();
            let document = window.document().unwrap();

            // Try to find existing iframe
            let element = document.get_element_by_id(&element_id);

            let iframe = if let Some(el) = element {
                el.dyn_into::<HtmlIFrameElement>().unwrap()
            } else {
                // Create new iframe
                let iframe = document
                    .create_element("iframe")
                    .unwrap()
                    .dyn_into::<HtmlIFrameElement>()
                    .unwrap();

                iframe.set_id(&element_id);
                iframe.set_src(&self.url);
                iframe.set_attribute("allow", "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture").unwrap();
                iframe.set_attribute("allowfullscreen", "true").unwrap();
                iframe.set_attribute("frameborder", "0").unwrap();

                // Base styles
                let style = iframe.style();
                style.set_property("position", "absolute").unwrap();
                style.set_property("border", "none").unwrap();
                style.set_property("z-index", "1000").unwrap(); // Ensure it's on top
                style.set_property("pointer-events", "auto").unwrap();
                style.set_property("visibility", "visible").unwrap();
                style.set_property("opacity", "1").unwrap();

                document.body().unwrap().append_child(&iframe).unwrap();
                iframe
            };

            // Update heartbeat
            iframe
                .set_attribute("data-last-updated", &js_sys::Date::now().to_string())
                .unwrap();

            // Update position and size
            let style = iframe.style();
            style
                .set_property("left", &format!("{}px", bounds.x))
                .unwrap();
            style
                .set_property("top", &format!("{}px", bounds.y))
                .unwrap();
            style
                .set_property("width", &format!("{}px", bounds.width))
                .unwrap();
            style
                .set_property("height", &format!("{}px", bounds.height))
                .unwrap();
            style
                .set_property("border-radius", &format!("{}px", self.radius))
                .unwrap();

            // Handle visibility (if bounds are zero or outside viewport, we could hide it)
            if bounds.width <= 0.0 || bounds.height <= 0.0 {
                style.set_property("display", "none").unwrap();
            } else {
                style.set_property("display", "block").unwrap();
            }
        }
    }

    impl<'a, Message, Renderer> From<WebView> for Element<'a, Message, Theme, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        fn from(webview: WebView) -> Self {
            Self::new(webview)
        }
    }

    #[derive(Clone)]
    pub enum ImageState {
        Loading,
        Loaded(Handle),
        Error,
    }

    static IMAGE_CACHE: Lazy<Arc<Mutex<HashMap<String, ImageState>>>> =
        Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

    pub fn get_image(path: String) -> ImageState {
        let mut cache = IMAGE_CACHE.lock().unwrap();

        if let Some(state) = cache.get(&path) {
            return state.clone();
        }

        // Not in cache, start loading
        cache.insert(path.clone(), ImageState::Loading);

        let path_clone = path.clone();
        use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
            let url = if path_clone.starts_with("http") {
                path_clone.clone()
            } else {
                let win = web_sys::window().unwrap();
                let origin = win.location().origin().unwrap_or_default();
                let clean = if path_clone.starts_with('/') {
                    &path_clone[1..]
                } else {
                    &path_clone
                };
                format!("{}/{}", origin, clean)
            };

            match reqwest::get(&url).await {
                Ok(resp) => {
                    if let Ok(bytes) = resp.bytes().await {
                        let handle = Handle::from_bytes(bytes);
                        let mut cache = IMAGE_CACHE.lock().unwrap();
                        cache.insert(path_clone, ImageState::Loaded(handle));
                    } else {
                        let mut cache = IMAGE_CACHE.lock().unwrap();
                        cache.insert(path_clone, ImageState::Error);
                    }
                }
                Err(_) => {
                    let mut cache = IMAGE_CACHE.lock().unwrap();
                    cache.insert(path_clone, ImageState::Error);
                }
            }
        });

        ImageState::Loading
    }

    /// Creates or updates a transparent overlay input positioned exactly over an Iced widget
    /// This enables proper mobile keyboard triggering and IME support on all WASM targets
    pub fn create_overlay_input(
        id: &str,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        current_value: &str,
    ) {
        use wasm_bindgen::JsCast;
        use web_sys::{window, HtmlInputElement};

        let window = window().unwrap();
        let document = window.document().unwrap();

        // Try to find existing overlay or create new one
        let input = if let Some(el) = document.get_element_by_id(id) {
            el.dyn_into::<HtmlInputElement>().ok()
        } else {
            // Create new overlay input
            if let Ok(element) = document.create_element("input") {
                if let Ok(input) = element.dyn_into::<HtmlInputElement>() {
                    input.set_id(id);
                    input.set_type("text");

                    // Set up event bridge: forward overlay input to Iced via keyboard events
                    // This is the industry-standard approach for canvas apps
                    use wasm_bindgen::closure::Closure;
                    use web_sys::Event;

                    let input_clone = input.clone();
                    let event_handler = Closure::wrap(Box::new(move |_event: Event| {
                        // When user types in overlay, get the new value
                        let value = input_clone.value();
                        log::debug!("Overlay input changed: '{}'", value);

                        // Keyboard events naturally bubble from input to window
                        // Iced's event loop listens to window keyboard events
                        // Typing in overlay → keyboard events → window → Iced receives them
                    })
                        as Box<dyn FnMut(Event)>);

                    // Attach event listener to overlay
                    let _ = input.add_event_listener_with_callback(
                        "input",
                        event_handler.as_ref().unchecked_ref(),
                    );

                    // Keep closure alive by leaking it (necessary for event listeners)
                    event_handler.forget();

                    // Append to body
                    if let Some(body) = document.body() {
                        let _ = body.append_child(&input);
                    }

                    log::info!("Created overlay input with event bridge: {}", id);
                    Some(input)
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(input) = input {
            // Update styling - transparent overlay positioned exactly over widget
            let style = input.style();
            let _ = style.set_property("position", "absolute");
            let _ = style.set_property("left", &format!("{}px", x));
            let _ = style.set_property("top", &format!("{}px", y));
            let _ = style.set_property("width", &format!("{}px", width));
            let _ = style.set_property("height", &format!("{}px", height));

            // Style the overlay to trigger keyboard but stay invisible
            // The overlay's job is to: 1) Receive clicks/taps, 2) Show mobile keyboard, 3) Capture typing
            // Iced canvas below handles the visual rendering
            let _ = style.set_property("background", "transparent");
            let _ = style.set_property("border", "none");
            let _ = style.set_property("outline", "none");

            // Make visually invisible but still functional
            let _ = style.set_property("color", "transparent");
            let _ = style.set_property("caret-color", "transparent");
            let _ = style.set_property("opacity", "0.01"); // Nearly invisible but clickable

            // Allow clicks/taps to focus the input (triggers keyboard)
            let _ = style.set_property("pointer-events", "auto");

            // Critical for mobile: min font-size prevents iOS zoom
            let _ = style.set_property("font-size", "16px");

            // Position above canvas to intercept clicks
            let _ = style.set_property("z-index", "1000");

            // Sync value from Iced state
            if input.value() != current_value {
                input.set_value(current_value);
            }

            log::debug!(
                "Updated overlay input {} at ({}, {}) {}x{}",
                id,
                x,
                y,
                width,
                height
            );

            // Note: Keyboard events from the overlay automatically bubble to the window,
            // which Iced listens to. This provides automatic DOM → Iced text sync.
            // The overlay captures physical typing, browser shows it in overlay.value,
            // and simultaneously Iced receives keyboard events to update its state.
        } else {
            log::error!("Failed to create overlay input: {}", id);
        }
    }

    /// Legacy function for compatibility - directs to overlay creation
    pub fn focus_input(id: &str) {
        use wasm_bindgen::JsCast;
        use web_sys::{window, HtmlInputElement};

        let window = window().unwrap();
        let document = window.document().unwrap();

        if let Some(el) = document.get_element_by_id(id) {
            if let Ok(input) = el.dyn_into::<HtmlInputElement>() {
                let _ = input.focus();
                log::debug!("Focused existing overlay: {}", id);
            }
        } else {
            log::warn!("focus_input called but no overlay exists for: {}", id);
        }
    }

    pub struct FocusBridge<'a, Message, Theme, Renderer> {
        inner: Element<'a, Message, Theme, Renderer>,
        dom_id: String,
        current_value: String,
    }

    impl<'a, Message, Theme, Renderer> FocusBridge<'a, Message, Theme, Renderer> {
        pub fn new(
            inner: impl Into<Element<'a, Message, Theme, Renderer>>,
            dom_id: String,
            current_value: String,
        ) -> Self {
            Self {
                inner: inner.into(),
                dom_id,
                current_value,
            }
        }
    }

    impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
        for FocusBridge<'a, Message, Theme, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        fn tag(&self) -> widget::tree::Tag {
            self.inner.as_widget().tag()
        }

        fn state(&self) -> widget::tree::State {
            self.inner.as_widget().state()
        }

        fn children(&self) -> Vec<widget::Tree> {
            self.inner.as_widget().children()
        }

        fn diff(&self, tree: &mut widget::Tree) {
            self.inner.as_widget().diff(tree)
        }

        fn size(&self) -> Size<Length> {
            self.inner.as_widget().size()
        }

        fn layout(
            &mut self,
            tree: &mut widget::Tree,
            renderer: &Renderer,
            limits: &layout::Limits,
        ) -> layout::Node {
            self.inner.as_widget_mut().layout(tree, renderer, limits)
        }

        fn draw(
            &self,
            tree: &widget::Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
        ) {
            // Create/update transparent overlay input with exact bounds
            let bounds = layout.bounds();
            create_overlay_input(
                &self.dom_id,
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
                &self.current_value,
            );

            // Draw the inner Iced widget normally
            self.inner
                .as_widget()
                .draw(tree, renderer, theme, style, layout, cursor, viewport)
        }

        fn update(
            &mut self,
            state: &mut widget::Tree,
            event: &iced::Event,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn iced::advanced::Clipboard,
            shell: &mut iced::advanced::Shell<'_, Message>,
            viewport: &Rectangle,
        ) {
            match event {
                iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | iced::Event::Touch(iced::touch::Event::FingerPressed { .. }) => {
                    if let Some(_cursor_position) = cursor.position_in(layout.bounds()) {
                        let bounds = layout.bounds();

                        // Ensure overlay exists with current bounds
                        create_overlay_input(
                            &self.dom_id,
                            bounds.x,
                            bounds.y,
                            bounds.width,
                            bounds.height,
                            &self.current_value,
                        );

                        // Focus the overlay
                        focus_input(&self.dom_id);

                        log::info!("FocusBridge activated overlay for: {}", self.dom_id);
                    }
                }
                iced::Event::Window(iced::window::Event::RedrawRequested(_)) => {
                    // On every frame, check if overlay value changed and sync to Iced
                    // This is the DOM → Iced synchronization mechanism
                    use wasm_bindgen::JsCast;
                    use web_sys::HtmlInputElement;

                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Some(el) = document.get_element_by_id(&self.dom_id) {
                                if let Ok(input) = el.dyn_into::<HtmlInputElement>() {
                                    let overlay_value = input.value();
                                    if overlay_value != self.current_value {
                                        log::info!(
                                            "Overlay value changed: '{}' -> '{}'",
                                            self.current_value,
                                            overlay_value
                                        );
                                        // Update our stored value - this will be used in next draw()
                                        self.current_value = overlay_value;
                                        // Request redraw to show the new text
                                        shell.request_redraw();
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }

            self.inner.as_widget_mut().update(
                state, event, layout, cursor, renderer, clipboard, shell, viewport,
            )
        }

        fn mouse_interaction(
            &self,
            state: &widget::Tree,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
            renderer: &Renderer,
        ) -> mouse::Interaction {
            self.inner
                .as_widget()
                .mouse_interaction(state, layout, cursor, viewport, renderer)
        }

        fn operate(
            &mut self,
            state: &mut widget::Tree,
            layout: Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn widget::Operation,
        ) {
            self.inner
                .as_widget_mut()
                .operate(state, layout, renderer, operation)
        }
    }

    impl<'a, Message: 'static, Theme: 'a, Renderer> From<FocusBridge<'a, Message, Theme, Renderer>>
        for Element<'a, Message, Theme, Renderer>
    where
        Renderer: renderer::Renderer + 'static,
    {
        fn from(bridge: FocusBridge<'a, Message, Theme, Renderer>) -> Self {
            Self::new(bridge)
        }
    }
}
