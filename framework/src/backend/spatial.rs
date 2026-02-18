use super::{Backend, TextSpan};
use crate::style::{Context, Intent, Radius, ScrollDirection, Variant};
use iced::{Alignment, Color, Length, Padding};
use nalgebra::{Point3, Vector3};
use std::sync::Arc;

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

    pub fn from_size(width: f32, height: f32, depth: f32) -> Self {
        Self::new(
            Point3::new(-width / 2.0, -height / 2.0, -depth / 2.0),
            Point3::new(width / 2.0, height / 2.0, depth / 2.0),
        )
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

#[derive(Debug, Clone, serde::Serialize)]
#[serde(bound = "")]
pub struct SpatialNode<Message = ()> {
    pub role: Arc<str>,
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

pub struct RayHit<Message = ()> {
    pub distance: f32,
    pub point: Point3<f32>,
    pub normal: Vector3<f32>,
    pub message: Option<Message>,
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct Transform3D {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
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

#[derive(Debug, Clone, Copy, Default)]
pub struct SpatialBackend;

impl Backend for SpatialBackend {
    type AnyView<Message: 'static + Send + Sync> = SpatialNode<Message>;

    fn semantic_node<Message: 'static + Send + Sync>(
        node: crate::semantic::SemanticNode,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: node.role.clone().into(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn vstack<Message: 'static + Send + Sync>(
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
        let mut max_width = 0.0f32;
        let mut nodes = Vec::new();

        for mut child in children {
            child.transform.position.y = y_offset;
            child.transform.position.z = 1.0;
            y_offset += child.height + spacing;
            max_width = max_width.max(child.width);
            nodes.push(child);
        }

        SpatialNode {
            role: "vstack".into(),
            width: max_width,
            height: y_offset,
            depth: 1.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::from_size(max_width, y_offset, 1.0),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: nodes,
        }
    }

    fn hstack<Message: 'static + Send + Sync>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_x: Alignment,
        _align_y: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let mut x_offset = 0.0;
        let mut max_height = 0.0f32;
        let mut nodes = Vec::new();

        for mut child in children {
            child.transform.position.x = x_offset;
            child.transform.position.z = 1.0;
            x_offset += child.width + spacing;
            max_height = max_height.max(child.height);
            nodes.push(child);
        }

        SpatialNode {
            role: "hstack".into(),
            width: x_offset,
            height: max_height,
            depth: 1.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::from_size(x_offset, max_height, 1.0),
            layout: Layout::Horizontal,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: nodes,
        }
    }

    fn wrap<Message: 'static + Send + Sync>(
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
        SpatialNode {
            role: "wrap".into(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Wrap,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children,
        }
    }

    fn rich_text<Message: Clone + 'static + Send + Sync>(
        _spans: Vec<TextSpan>,
        _size: f32,
        _width: Length,
        _alignment: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "rich_text".into(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn text<Message: Clone + 'static + Send + Sync>(
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
        SpatialNode {
            role: "text".into(),
            width: (content.len() as f32) * 10.0, // Rough estimate
            height: 20.0,
            depth: 1.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::from_size((content.len() as f32) * 10.0, 20.0, 1.0),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn icon<Message: Clone + 'static + Send + Sync>(
        _name: String,
        size: f32,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "icon".into(),
            width: size,
            height: size,
            depth: 1.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::from_size(size, size, 1.0),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn divider<Message: 'static + Send + Sync>(_context: &Context) -> Self::AnyView<Message> {
        SpatialNode {
            role: "divider".into(),
            width: 100.0,
            height: 1.0,
            depth: 1.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::from_size(100.0, 1.0, 1.0),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn space<Message: 'static + Send + Sync>(
        _width: Length,
        _height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "space".into(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn circle<Message: 'static + Send + Sync>(
        radius: f32,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "circle".into(),
            width: radius * 2.0,
            height: radius * 2.0,
            depth: 1.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::from_size(radius * 2.0, radius * 2.0, 1.0),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn arc<Message: 'static + Send + Sync>(
        radius: f32,
        _start_angle: f32,
        _end_angle: f32,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "arc".into(),
            width: radius * 2.0,
            height: radius * 2.0,
            depth: 1.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::from_size(radius * 2.0, radius * 2.0, 1.0),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn path<Message: 'static + Send + Sync>(
        _points: Vec<iced::Point>,
        _color: Option<Color>,
        _width: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "path".into(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn capsule<Message: 'static + Send + Sync>(
        _width: Length,
        _height: Length,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "capsule".into(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn rectangle<Message: 'static + Send + Sync, R: Into<Radius>>(
        _width: Length,
        _height: Length,
        _color: Option<Color>,
        _radius: R,
        _border_width: f32,
        _border_color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "rectangle".into(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn button<Message: Clone + 'static + Send + Sync>(
        content: Self::AnyView<Message>,
        on_press: Option<Message>,
        _variant: Variant,
        _intent: Intent,
        _width: Length,
        _height: Length,
        _is_compact: bool,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let mut node = content;
        node.role = "button".into();
        node.on_press = on_press;
        node
    }

    fn sidebar_item<Message: Clone + Send + Sync + 'static>(
        _title: String,
        _icon: String,
        _is_selected: bool,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "sidebar_item".into(),
            width: 200.0,
            height: 40.0,
            depth: 1.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::from_size(200.0, 40.0, 1.0),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn text_input<Message: Clone + 'static + Send + Sync>(
        _value: String,
        _placeholder: String,
        _on_change: impl Fn(String) -> Message + Send + Sync + 'static,
        _on_submit: Option<Message>,
        _font: Option<iced::Font>,
        _is_secure: bool,
        _variant: Variant,
        _id: Option<iced::widget::Id>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "text_input".into(),
            width: 200.0,
            height: 40.0,
            depth: 1.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::from_size(200.0, 40.0, 1.0),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn slider<Message: Clone + 'static + Send + Sync>(
        _range: std::ops::RangeInclusive<f32>,
        _value: f32,
        _on_change: impl Fn(f32) -> Message + Send + Sync + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "slider".into(),
            width: 200.0,
            height: 20.0,
            depth: 1.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::from_size(200.0, 20.0, 1.0),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn toggle<Message: Clone + 'static + Send + Sync>(
        _label: String,
        _is_active: bool,
        _on_toggle: impl Fn(bool) -> Message + Send + Sync + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "toggle".into(),
            width: 100.0,
            height: 40.0,
            depth: 1.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::from_size(100.0, 40.0, 1.0),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn zstack<Message: 'static + Send + Sync>(
        children: Vec<Self::AnyView<Message>>,
        _width: Length,
        _height: Length,
        _alignment: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "zstack".into(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children,
        }
    }

    fn grid<Message: 'static + Send + Sync>(
        children: Vec<Self::AnyView<Message>>,
        _columns: usize,
        _spacing: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "grid".into(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::zero(),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children,
        }
    }

    fn image<Message: 'static + Send + Sync, S: Into<String>, R: Into<Radius>>(
        _path: S,
        _width: Length,
        _height: Length,
        _radius: R,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "image".into(),
            width: 100.0,
            height: 100.0,
            depth: 1.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::from_size(100.0, 100.0, 1.0),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn video<Message: 'static + Send + Sync, S: Into<String>, R: Into<Radius>>(
        _path: S,
        _width: Length,
        _height: Length,
        _radius: R,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "video".into(),
            width: 100.0,
            height: 100.0,
            depth: 1.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::from_size(100.0, 100.0, 1.0),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn web_view<Message: 'static + Send + Sync, R: Into<Radius>>(
        _url: String,
        _width: Length,
        _height: Length,
        _radius: R,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "web_view".into(),
            width: 100.0,
            height: 100.0,
            depth: 1.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::from_size(100.0, 100.0, 1.0),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn container<Message: 'static + Send + Sync, R: Into<Radius>>(
        content: Self::AnyView<Message>,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _background: Option<Color>,
        _radius: R,
        _border_width: f32,
        _border_color: Option<Color>,
        _shadow: Option<iced::Shadow>,
        _align_x: Alignment,
        _align_y: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn scroll_view<Message: 'static + Send + Sync>(
        content: Self::AnyView<Message>,
        _width: Length,
        _height: Length,
        _id: Option<&'static str>,
        _show_indicators: bool,
        _direction: ScrollDirection,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn mouse_area<Message: Clone + 'static + Send + Sync>(
        content: Self::AnyView<Message>,
        _on_move: Option<Arc<dyn Fn(iced::Point) -> Message + Send + Sync>>,
        _on_press: Option<Message>,
        _on_release: Option<Message>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn with_tooltip<Message: 'static + Send + Sync>(
        content: Self::AnyView<Message>,
        _tooltip: Arc<str>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn glass_card<Message: 'static + Send + Sync>(
        content: Self::AnyView<Message>,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn section<Message: 'static + Send + Sync>(
        _title: String,
        content: Self::AnyView<Message>,
        _width: Length,
        _height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn spatial_modifier<Message: 'static + Send + Sync>(
        mut content: Self::AnyView<Message>,
        position: Vector3<f32>,
        scale: Vector3<f32>,
        rotation: Vector3<f32>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content.transform.position += position;
        content.transform.scale = Vector3::new(
            content.transform.scale.x * scale.x,
            content.transform.scale.y * scale.y,
            content.transform.scale.z * scale.z,
        );
        content.transform.rotation += rotation;
        content
    }

    fn text_editor<Message: Clone + Send + Sync + 'static>(
        _content: String,
        _on_change: impl Fn(String) -> Message + Send + Sync + 'static,
        _font: Option<iced::Font>,
        _id: Option<iced::widget::Id>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "text_editor".into(),
            width: 300.0,
            height: 200.0,
            depth: 1.0,
            transform: Transform3D::default(),
            bounds: BoundingBox3D::from_size(300.0, 200.0, 1.0),
            layout: Layout::Vertical,
            is_focused: false,
            billboarding: false,
            on_press: None,
            children: Vec::new(),
        }
    }

    fn menu<Message: Clone + Send + Sync + 'static>(
        content: Self::AnyView<Message>,
        _items: Vec<crate::views::context_menu::ContextMenuItem<Message>>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let mut node = content;
        node.role = "menu".into();
        node
    }
}
