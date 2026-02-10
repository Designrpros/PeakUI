use super::{Backend, TextSpan};
use crate::semantic::SemanticNode;
use crate::style::{Context, Intent, Radius, Variant};
use iced::{Alignment, Color, Length, Padding};
use nalgebra::Vector3;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Default)]
pub struct AIBackend;

impl Backend for AIBackend {
    type AnyView<Message: 'static> = SemanticNode;

    fn semantic_node<Message: 'static>(
        node: crate::semantic::SemanticNode,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        node
    }

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
        _tooltip: Arc<str>,
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

        SemanticNode::new("vstack").extend_children(children)
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

        SemanticNode::new("hstack").extend_children(children)
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

        SemanticNode::new("wrap").extend_children(children)
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
        SemanticNode::new("text").with_content(content)
    }

    fn icon<Message: Clone + 'static>(
        name: String,
        _size: f32,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("icon").with_label(name)
    }

    fn divider<Message: 'static>(_context: &Context) -> Self::AnyView<Message> {
        SemanticNode::new("divider")
    }

    fn space<Message: 'static>(
        _width: Length,
        _height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("space")
    }

    fn circle<Message: 'static>(
        radius: f32,
        color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("circle")
            .with_label(format!("r={}", radius))
            .with_content(format!("{:?}", color))
    }

    fn arc<Message: 'static>(
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("arc")
            .with_label(format!("r={}, {}->{}", radius, start_angle, end_angle))
            .with_content(format!("{:?}", color))
    }

    fn path<Message: 'static>(
        points: Vec<iced::Point>,
        color: Option<Color>,
        width: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("path")
            .with_label(format!("{} pts, w={}", points.len(), width))
            .with_content(format!("{:?}", color))
    }

    fn capsule<Message: 'static>(
        _width: Length,
        _height: Length,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("capsule")
    }

    fn rectangle<Message: 'static, R: Into<Radius>>(
        _width: Length,
        _height: Length,
        _color: Option<Color>,
        _radius: R,
        _border_width: f32,
        _border_color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("rectangle")
    }

    fn button<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        _on_press: Option<Message>,
        variant: Variant,
        intent: Intent,
        _width: Length,
        _height: Length,
        _is_compact: bool,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("button")
            .with_label(format!("{:?}_{:?}", variant, intent))
            .push_child(content)
    }

    fn sidebar_item<Message: Clone + Send + Sync + 'static>(
        title: String,
        icon: String,
        is_selected: bool,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("sidebar_item")
            .with_label(title)
            .with_content(icon)
            .push_child(
                SemanticNode::new("state")
                    .with_label("selected")
                    .with_content(is_selected.to_string()),
            )
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
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("text_input")
            .with_label(value.clone())
            .with_content(value)
    }

    fn slider<Message: Clone + 'static>(
        _range: std::ops::RangeInclusive<f32>,
        value: f32,
        _on_change: impl Fn(f32) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("slider").with_content(value.to_string())
    }

    fn toggle<Message: Clone + 'static>(
        label: String,
        is_active: bool,
        _on_toggle: impl Fn(bool) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("toggle")
            .with_label(label)
            .with_content(is_active.to_string())
    }

    fn zstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _width: Length,
        _height: Length,
        _alignment: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("zstack").extend_children(children)
    }

    fn grid<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        columns: usize,
        _spacing: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("grid")
            .with_label(format!("columns: {}", columns))
            .extend_children(children)
    }

    fn image<Message: 'static, S: Into<String>, R: Into<Radius>>(
        path: S,
        _width: Length,
        _height: Length,
        _radius: R,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("image").with_content(path.into())
    }

    fn video<Message: 'static, S: Into<String>, R: Into<Radius>>(
        path: S,
        _width: Length,
        _height: Length,
        _radius: R,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("video").with_content(path.into())
    }

    fn web_view<Message: 'static, R: Into<Radius>>(
        url: String,
        _width: Length,
        _height: Length,
        _radius: R,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode::new("web_view").with_content(url)
    }

    fn container<Message: 'static, R: Into<Radius>>(
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

    fn scroll_view<Message: 'static>(
        content: Self::AnyView<Message>,
        _width: Length,
        _height: Length,
        _id: Option<&'static str>,
        _show_indicators: bool,
        _direction: crate::style::ScrollDirection,
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
