use super::{Backend, TextSpan};
use crate::style::{Context, Intent, ScrollDirection, Variant};
use iced::{Alignment, Color, Length, Padding};
use nalgebra::Vector3;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Default)]
pub struct TermBackend;

impl Backend for TermBackend {
    type AnyView<Message: 'static> = String;

    fn semantic_node<Message: 'static>(
        node: crate::semantic::SemanticNode,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!("(SEMANTIC: {:?})", node.role)
    }

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
        tooltip: Arc<str>,
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
                Intent::Accent => "35",
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

    fn arc<Message: 'static>(
        _radius: f32,
        _start_angle: f32,
        _end_angle: f32,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        "C".to_string()
    }

    fn path<Message: 'static>(
        points: Vec<iced::Point>,
        _color: Option<Color>,
        _width: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!("~ ({} pts)", points.len())
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
        _height: Length,
        _is_compact: bool,
        context: &Context,
    ) -> Self::AnyView<Message> {
        if context.is_focused("button") {
            format!("> [ {} ] <", content)
        } else {
            format!("  [ {} ]  ", content)
        }
    }

    fn sidebar_item<Message: Clone + Send + Sync + 'static>(
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
        _direction: ScrollDirection,
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
