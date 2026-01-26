use crate::modifiers::{Intent, Variant};
// Force rebuild to pick up peak-icons changes
use iced::{Alignment, Color, Length, Padding, Renderer, Shadow, Size, Theme, Vector};
pub use peak_core::registry::ShellMode;
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceType {
    Desktop,
    Mobile,
    TV,
}

impl Context {
    pub fn new(mode: ShellMode, theme: ThemeTokens, size: Size) -> Self {
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
        if cfg!(target_arch = "wasm32") {
            0.0.into()
        } else {
            radius.into()
        }
    }

    pub fn with_safe_area(mut self, padding: Padding) -> Self {
        self.safe_area = padding;
        self
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SpatialNode {
    pub role: String,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub transform: Transform3D,
    pub is_focused: bool,
    pub children: Vec<SpatialNode>,
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct Transform3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation_y: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SpatialBackend;

impl Backend for SpatialBackend {
    type AnyView<Message: 'static> = SpatialNode;

    fn vstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_x: Alignment,
        _scale: f32,
    ) -> Self::AnyView<Message> {
        let mut y_offset = 0.0;
        let mut nodes = Vec::new();

        for mut child in children {
            child.transform.y = y_offset;
            y_offset += child.height + spacing;
            nodes.push(child);
        }

        SpatialNode {
            role: "vstack".to_string(),
            width: 0.0,
            height: y_offset,
            depth: 0.0,
            transform: Transform3D::default(),
            is_focused: false,
            children: nodes,
        }
    }

    fn hstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_y: Alignment,
        _scale: f32,
    ) -> Self::AnyView<Message> {
        let mut x_offset = 0.0;
        let mut nodes = Vec::new();

        for mut child in children {
            child.transform.x = x_offset;
            x_offset += child.width + spacing;
            nodes.push(child);
        }

        SpatialNode {
            role: "hstack".to_string(),
            width: x_offset,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            is_focused: false,
            children: nodes,
        }
    }

    fn text<Message: Clone + 'static>(
        content: String,
        size: f32,
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
            role: "text".to_string(),
            width: content.len() as f32 * 10.0,
            height: size,
            depth: 1.0,
            transform: Transform3D::default(),
            is_focused: false,
            children: vec![],
        }
    }

    fn icon<Message: Clone + 'static>(
        name: String,
        size: f32,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: format!("icon:{}", name),
            width: size,
            height: size,
            depth: 1.0,
            transform: Transform3D::default(),
            is_focused: false,
            children: vec![],
        }
    }

    fn divider<Message: 'static>(_context: &Context) -> Self::AnyView<Message> {
        SpatialNode {
            role: "divider".to_string(),
            width: 100.0,
            height: 1.0,
            depth: 1.0,
            transform: Transform3D::default(),
            is_focused: false,
            children: vec![],
        }
    }

    fn space<Message: 'static>(_width: Length, _height: Length) -> Self::AnyView<Message> {
        SpatialNode {
            role: "space".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            is_focused: false,
            children: vec![],
        }
    }

    fn circle<Message: 'static>(radius: f32, _color: Option<Color>) -> Self::AnyView<Message> {
        SpatialNode {
            role: "circle".to_string(),
            width: radius * 2.0,
            height: radius * 2.0,
            depth: 1.0,
            transform: Transform3D::default(),
            is_focused: false,
            children: vec![],
        }
    }

    fn capsule<Message: 'static>(
        _width: Length,
        _height: Length,
        _color: Option<Color>,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "capsule".to_string(),
            width: 100.0,
            height: 40.0,
            depth: 1.0,
            transform: Transform3D::default(),
            is_focused: false,
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
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "rectangle".to_string(),
            width: 100.0,
            height: 100.0,
            depth: 1.0,
            transform: Transform3D::default(),
            is_focused: false,
            children: vec![],
        }
    }

    fn button<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        _on_press: Option<Message>,
        _variant: Variant,
        _intent: Intent,
        context: &Context,
    ) -> Self::AnyView<Message> {
        let is_focused = context.is_focused("button");

        SpatialNode {
            role: "button".to_string(),
            width: content.width + 16.0,
            height: content.height + 16.0,
            depth: if is_focused { 20.0 } else { 5.0 },
            transform: Transform3D::default(),
            is_focused,
            children: vec![content],
        }
    }

    fn sidebar_item<Message: Clone + 'static>(
        title: String,
        icon: String,
        is_selected: bool,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: format!("sidebar_item:{}:{}:{}", title, icon, is_selected),
            width: 200.0,
            height: 40.0,
            depth: 2.0,
            transform: Transform3D::default(),
            is_focused: false,
            children: vec![],
        }
    }

    fn text_input<Message: Clone + 'static>(
        value: String,
        placeholder: String,
        _on_change: impl Fn(String) -> Message + 'static,
        _on_submit: Option<Message>,
        _font: Option<iced::Font>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: format!("input:{}:{}", value, placeholder),
            width: 200.0,
            height: 40.0,
            depth: 2.0,
            transform: Transform3D::default(),
            is_focused: false,
            children: vec![],
        }
    }

    fn slider<Message: Clone + 'static>(
        _range: std::ops::RangeInclusive<f32>,
        value: f32,
        _on_change: impl Fn(f32) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: format!("slider:{}", value),
            width: 200.0,
            height: 20.0,
            depth: 2.0,
            transform: Transform3D::default(),
            is_focused: false,
            children: vec![],
        }
    }

    fn toggle<Message: Clone + 'static>(
        label: String,
        is_active: bool,
        _on_toggle: impl Fn(bool) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: format!("toggle:{}:{}", label, is_active),
            width: 200.0,
            height: 40.0,
            depth: 2.0,
            transform: Transform3D::default(),
            is_focused: false,
            children: vec![],
        }
    }

    fn zstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _width: Length,
        _height: Length,
        _alignment: Alignment,
    ) -> Self::AnyView<Message> {
        let mut z_offset = 0.0;
        let mut nodes = Vec::new();

        for mut child in children {
            child.transform.z = z_offset;
            z_offset += child.depth + 1.0;
            nodes.push(child);
        }

        SpatialNode {
            role: "zstack".to_string(),
            width: 0.0,
            height: 0.0,
            depth: z_offset,
            transform: Transform3D::default(),
            is_focused: false,
            children: nodes,
        }
    }

    fn grid<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _columns: usize,
        _spacing: f32,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "grid".to_string(),
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            transform: Transform3D::default(),
            is_focused: false,
            children,
        }
    }

    fn image<Message: 'static>(
        path: impl Into<String>,
        _width: Length,
        _height: Length,
        _radius: f32,
    ) -> Self::AnyView<Message> {
        let p = path.into();
        SpatialNode {
            role: format!("image:{}", p),
            width: 100.0,
            height: 100.0,
            depth: 1.0,
            transform: Transform3D::default(),
            is_focused: false,
            children: vec![],
        }
    }

    fn container<Message: 'static>(
        content: Self::AnyView<Message>,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SpatialNode {
            role: "container".to_string(),
            width: content.width,
            height: content.height,
            depth: content.depth,
            transform: Transform3D::default(),
            is_focused: false,
            children: vec![content],
        }
    }

    fn scroll_view<Message: 'static>(content: Self::AnyView<Message>) -> Self::AnyView<Message> {
        SpatialNode {
            role: "scroll_view".to_string(),
            width: content.width,
            height: content.height,
            depth: content.depth,
            transform: Transform3D::default(),
            is_focused: false,
            children: vec![content],
        }
    }

    fn mouse_area<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        _on_move: Option<Arc<dyn Fn(iced::Point) -> Message + Send + Sync>>,
        _on_press: Option<Message>,
        _on_release: Option<Message>,
    ) -> Self::AnyView<Message> {
        content
    }

    fn with_tooltip<Message: 'static>(
        content: Self::AnyView<Message>,
        _tooltip: String,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }
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
        scale: f32,
    ) -> Self::AnyView<Message>;

    fn hstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_y: Alignment,
        scale: f32,
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

    fn space<Message: 'static>(width: Length, height: Length) -> Self::AnyView<Message>;

    fn circle<Message: 'static>(radius: f32, color: Option<Color>) -> Self::AnyView<Message>;

    fn capsule<Message: 'static>(
        width: Length,
        height: Length,
        color: Option<Color>,
    ) -> Self::AnyView<Message>;

    fn rectangle<Message: 'static>(
        width: Length,
        height: Length,
        color: Option<Color>,
        radius: f32,
        border_width: f32,
        border_color: Option<Color>,
    ) -> Self::AnyView<Message>;

    fn button<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        on_press: Option<Message>,
        variant: Variant,
        intent: Intent,
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
    ) -> Self::AnyView<Message>;

    fn grid<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        columns: usize,
        spacing: f32,
    ) -> Self::AnyView<Message>;

    fn image<Message: 'static>(
        path: impl Into<String>,
        width: Length,
        height: Length,
        radius: f32,
    ) -> Self::AnyView<Message>;

    fn container<Message: 'static>(
        content: Self::AnyView<Message>,
        padding: Padding,
        width: Length,
        height: Length,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn scroll_view<Message: 'static>(content: Self::AnyView<Message>) -> Self::AnyView<Message>;

    fn mouse_area<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        on_move: Option<Arc<dyn Fn(iced::Point) -> Message + Send + Sync>>,
        on_press: Option<Message>,
        on_release: Option<Message>,
    ) -> Self::AnyView<Message>;

    fn with_tooltip<Message: 'static>(
        content: Self::AnyView<Message>,
        tooltip: String,
        context: &Context,
    ) -> Self::AnyView<Message>;
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
        scale: f32,
    ) -> Self::AnyView<Message> {
        use iced::widget::{column, container};
        container(
            column(children)
                .spacing(spacing * scale)
                .align_x(align_x)
                .width(width)
                .height(height),
        )
        .padding(Padding {
            top: padding.top * scale,
            right: padding.right * scale,
            bottom: padding.bottom * scale,
            left: padding.left * scale,
        })
        .width(width)
        .height(height)
        .into()
    }

    fn hstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_y: Alignment,
        scale: f32,
    ) -> Self::AnyView<Message> {
        use iced::widget::{container, row};
        container(
            row(children)
                .spacing(spacing * scale)
                .align_y(align_y)
                .width(width)
                .height(height),
        )
        .padding(Padding {
            top: padding.top * scale,
            right: padding.right * scale,
            bottom: padding.bottom * scale,
            left: padding.left * scale,
        })
        .width(width)
        .height(height)
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
        use iced::widget::text::Span;
        use iced::widget::{rich_text, text};

        let base_color = color.unwrap_or_else(|| {
            if let Some(i) = intent {
                match i {
                    Intent::Primary => context.theme.colors.primary,
                    Intent::Secondary => context.theme.colors.secondary,
                    Intent::Success => context.theme.colors.success,
                    Intent::Warning => context.theme.colors.warning,
                    Intent::Danger => context.theme.colors.danger,
                    Intent::Info => context.theme.colors.info,
                    Intent::Neutral => context.theme.colors.text_primary,
                }
            } else if is_dim {
                context.theme.colors.text_secondary
            } else {
                context.theme.colors.text_primary
            }
        });

        let mut base_font = font.unwrap_or_default();
        if is_bold {
            base_font.weight = iced::font::Weight::Bold;
        }

        let scaled_size = size * context.theme.scaling;

        // --- Simplified Markdown Parsing ---
        let mut spans = Vec::new();
        let mut remaining = content.as_str();
        let mut has_parsed = false;

        while !remaining.is_empty() {
            let next_bold = remaining.find("**");
            let next_code = remaining.find('`');

            match (next_bold, next_code) {
                (Some(b_idx), Some(c_idx)) if b_idx < c_idx => {
                    if b_idx > 0 {
                        spans.push(
                            Span::new(remaining[..b_idx].to_string())
                                .color(base_color)
                                .font(base_font),
                        );
                    }
                    if let Some(end) = remaining[b_idx + 2..].find("**") {
                        let inner = &remaining[b_idx + 2..b_idx + 2 + end];
                        let mut bold_font = base_font;
                        bold_font.weight = iced::font::Weight::Bold;
                        spans.push(
                            Span::new(inner.to_string())
                                .color(context.theme.colors.text_primary)
                                .font(bold_font),
                        );
                        remaining = &remaining[b_idx + 2 + end + 2..];
                        has_parsed = true;
                    } else {
                        spans.push(
                            Span::new(remaining[..b_idx + 2].to_string())
                                .color(base_color)
                                .font(base_font),
                        );
                        remaining = &remaining[b_idx + 2..];
                    }
                }
                (Some(b_idx), _) => {
                    if b_idx > 0 {
                        spans.push(
                            Span::new(remaining[..b_idx].to_string())
                                .color(base_color)
                                .font(base_font),
                        );
                    }
                    if let Some(end) = remaining[b_idx + 2..].find("**") {
                        let inner = &remaining[b_idx + 2..b_idx + 2 + end];
                        let mut bold_font = base_font;
                        bold_font.weight = iced::font::Weight::Bold;
                        spans.push(
                            Span::new(inner.to_string())
                                .color(context.theme.colors.text_primary)
                                .font(bold_font),
                        );
                        remaining = &remaining[b_idx + 2 + end + 2..];
                        has_parsed = true;
                    } else {
                        spans.push(
                            Span::new(remaining[..b_idx + 2].to_string())
                                .color(base_color)
                                .font(base_font),
                        );
                        remaining = &remaining[b_idx + 2..];
                    }
                }
                (_, Some(c_idx)) => {
                    if c_idx > 0 {
                        spans.push(
                            Span::new(remaining[..c_idx].to_string())
                                .color(base_color)
                                .font(base_font),
                        );
                    }
                    if let Some(end) = remaining[c_idx + 1..].find('`') {
                        let inner = &remaining[c_idx + 1..c_idx + 1 + end];
                        spans.push(
                            Span::new(inner.to_string())
                                .color(context.theme.colors.primary)
                                .font(iced::Font::MONOSPACE),
                        );
                        remaining = &remaining[c_idx + 1 + end + 1..];
                        has_parsed = true;
                    } else {
                        spans.push(
                            Span::new(remaining[..c_idx + 1].to_string())
                                .color(base_color)
                                .font(base_font),
                        );
                        remaining = &remaining[c_idx + 1..];
                    }
                }
                _ => {
                    spans.push(
                        Span::new(remaining.to_string())
                            .color(base_color)
                            .font(base_font),
                    );
                    remaining = "";
                }
            }
        }

        if !has_parsed {
            text(content)
                .size(scaled_size)
                .color(base_color)
                .font(base_font)
                .width(width)
                .align_x(alignment)
                .into()
        } else {
            rich_text(spans)
                .size(scaled_size)
                .width(width)
                .align_x(alignment)
                .into()
        }
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
            #[cfg(target_arch = "wasm32")]
            log::debug!("Icon '{}' found in EMBEDDED storage.", name);

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
        use iced::widget::{container, Rule};
        let divider_color = context.theme.colors.divider;
        container(Rule::horizontal(1))
            .style(move |_| container::Style {
                text_color: Some(divider_color),
                ..Default::default()
            })
            .into()
    }

    fn space<Message: 'static>(width: Length, height: Length) -> Self::AnyView<Message> {
        iced::widget::Space::new(width, height).into()
    }

    fn circle<Message: 'static>(radius: f32, color: Option<Color>) -> Self::AnyView<Message> {
        use iced::widget::container;
        container(iced::widget::Space::new(
            Length::Fixed(radius * 2.0),
            Length::Fixed(radius * 2.0),
        ))
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
    ) -> Self::AnyView<Message> {
        use iced::widget::container;
        container(iced::widget::Space::new(width, height))
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
    ) -> Self::AnyView<Message> {
        use iced::widget::container;

        container(iced::widget::Space::new(Length::Fill, Length::Fill))
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
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::button;
        let theme = context.theme;

        button(content)
            .on_press_maybe(on_press)
            .style(move |_, status| {
                let color = match intent {
                    Intent::Primary => theme.colors.primary,
                    Intent::Secondary => theme.colors.secondary,
                    Intent::Success => theme.colors.success,
                    Intent::Warning => theme.colors.warning,
                    Intent::Danger => theme.colors.danger,
                    Intent::Info => theme.colors.info,
                    Intent::Neutral => theme.colors.surface,
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
                        text_color: theme.colors.text_primary,
                        border: iced::Border {
                            radius: 8.0.into(),
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
                            radius: 8.0.into(),
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
                            radius: 8.0.into(),
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
                            radius: 8.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                }
            })
            .into()
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
                    move |_| container::Style {
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

    fn text_input<Message: Clone + 'static>(
        value: String,
        placeholder: String,
        on_change: impl Fn(String) -> Message + 'static,
        on_submit: Option<Message>,
        font: Option<iced::Font>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let mut input = iced::widget::text_input(&placeholder, &value).on_input(on_change);

        if let Some(msg) = on_submit {
            input = input.on_submit(msg);
        }

        if let Some(font) = font {
            input = input.font(font);
        }

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
    ) -> Self::AnyView<Message> {
        let s = iced::widget::stack(children).width(width).height(height);
        s.into()
    }

    fn grid<Message: 'static>(
        mut children: Vec<Self::AnyView<Message>>,
        columns: usize,
        spacing: f32,
    ) -> Self::AnyView<Message> {
        let mut rows = Vec::new();
        while !children.is_empty() {
            let chunk: Vec<_> = children
                .drain(0..std::cmp::min(columns, children.len()))
                .collect();
            rows.push(iced::widget::row(chunk).spacing(spacing).into());
        }
        iced::widget::column(rows).spacing(spacing).into()
    }

    fn image<Message: 'static>(
        path: impl Into<String>,
        width: Length,
        height: Length,
        radius: f32,
    ) -> Self::AnyView<Message> {
        use iced::widget::container;
        let p: String = path.into();

        // 2. Asset Pipeline: No more magic path hacking.
        // The Asset type already provides the correct path.

        container(
            iced::widget::image(p)
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

    fn container<Message: 'static>(
        content: Self::AnyView<Message>,
        padding: Padding,
        width: Length,
        height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::container;
        container(content)
            .padding(padding)
            .width(width)
            .height(height)
            .into()
    }

    fn scroll_view<Message: 'static>(content: Self::AnyView<Message>) -> Self::AnyView<Message> {
        use iced::widget::scrollable;
        scrollable(content).into()
    }

    fn mouse_area<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        on_move: Option<Arc<dyn Fn(iced::Point) -> Message + Send + Sync>>,
        on_press: Option<Message>,
        on_release: Option<Message>,
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
        tooltip_label: String,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::{container, tooltip};
        tooltip(
            content,
            iced::widget::text(tooltip_label)
                .size(14.0 * context.theme.scaling)
                .color(context.theme.colors.text_primary),
            tooltip::Position::FollowCursor,
        )
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(palette.background.weak.color.into()),
                border: iced::Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                text_color: Some(palette.background.weak.text),
                ..Default::default()
            }
        })
        .into()
    }
}

/// A Terminal-based TUI backend.
#[derive(Clone, Copy, Debug, Default)]
pub struct TermBackend;

impl Backend for TermBackend {
    type AnyView<Message: 'static> = String;

    fn with_tooltip<Message: 'static>(
        content: Self::AnyView<Message>,
        tooltip_label: String,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!("{} (Tooltip: {})", content, tooltip_label)
    }

    fn vstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_x: Alignment,
        _scale: f32,
    ) -> Self::AnyView<Message> {
        children.join("\n")
    }

    fn hstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_y: Alignment,
        _scale: f32,
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
                Intent::Success => "32",
                Intent::Warning => "33",
                Intent::Danger => "31",
                Intent::Info => "36",
                _ => "0",
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

    fn space<Message: 'static>(_width: Length, _height: Length) -> Self::AnyView<Message> {
        " ".to_string()
    }

    fn circle<Message: 'static>(_radius: f32, _color: Option<Color>) -> Self::AnyView<Message> {
        "O".to_string()
    }

    fn capsule<Message: 'static>(
        _width: Length,
        _height: Length,
        _color: Option<Color>,
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
    ) -> Self::AnyView<Message> {
        "█".to_string()
    }

    fn button<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        _on_press: Option<Message>,
        _variant: Variant,
        _intent: Intent,
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
        _placeholder: String,
        _on_change: impl Fn(String) -> Message + 'static,
        _on_submit: Option<Message>,
        _font: Option<iced::Font>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!("[Input: {}]", value)
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
    ) -> Self::AnyView<Message> {
        children.join("\n")
    }

    fn grid<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _columns: usize,
        _spacing: f32,
    ) -> Self::AnyView<Message> {
        children.join(" | ")
    }

    fn image<Message: 'static>(
        path: impl Into<String>,
        _width: Length,
        _height: Length,
        _radius: f32,
    ) -> Self::AnyView<Message> {
        format!("[IMG: {}]", path.into())
    }

    fn container<Message: 'static>(
        content: Self::AnyView<Message>,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn scroll_view<Message: 'static>(content: Self::AnyView<Message>) -> Self::AnyView<Message> {
        content
    }

    fn mouse_area<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        _on_move: Option<Arc<dyn Fn(iced::Point) -> Message + Send + Sync>>,
        _on_press: Option<Message>,
        _on_release: Option<Message>,
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

    /// Generates a semantic description of the view for AI agents.
    fn describe(&self, _context: &Context) -> SemanticNode {
        // Default implementation for basic views
        SemanticNode {
            role: "view".to_string(),
            label: None,
            content: None,
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
        }
    }

    /// Generates a semantic description of the view for AI agents, specifically for Iced backend.
    /// This is a temporary workaround for the fact that `describe` is generic over `B`,
    /// but `iced::Element` cannot be easily converted to `SemanticNode` without knowing its internal structure.
    /// This method should ideally be removed once `describe` can be made to work with `iced::Element` directly.
    fn describe_iced(&self, _context: &Context) -> SemanticNode {
        SemanticNode {
            role: "iced_view".to_string(),
            label: None,
            content: None,
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
        }
    }

    fn into_box(self) -> Box<dyn View<Message, B>>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
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
    pub role: String,
    pub label: Option<String>,
    pub content: Option<String>,
    pub children: Vec<SemanticNode>,
    pub neural_tag: Option<String>,
    pub documentation: Option<String>,
}

/// An AI-focused backend that renders UIs into semantic data.
#[derive(Clone, Copy, Debug, Default)]
pub struct AIBackend;

impl Backend for AIBackend {
    type AnyView<Message: 'static> = SemanticNode;

    fn with_tooltip<Message: 'static>(
        content: Self::AnyView<Message>,
        tooltip: String,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let mut node = content;
        node.documentation = Some(tooltip);
        node
    }

    fn vstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_x: Alignment,
        _scale: f32,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "vstack".to_string(),
            label: None,
            content: None,
            children,
            neural_tag: None,
            documentation: None,
        }
    }

    fn hstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_y: Alignment,
        _scale: f32,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "hstack".to_string(),
            label: None,
            content: None,
            children,
            neural_tag: None,
            documentation: None,
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
            role: "text".to_string(),
            label: None,
            content: Some(content),
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
        }
    }

    fn icon<Message: Clone + 'static>(
        name: String,
        _size: f32,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "icon".to_string(),
            label: Some(name),
            content: None,
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
        }
    }

    fn divider<Message: 'static>(_context: &Context) -> Self::AnyView<Message> {
        SemanticNode {
            role: "divider".to_string(),
            label: None,
            content: None,
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
        }
    }

    fn space<Message: 'static>(_width: Length, _height: Length) -> Self::AnyView<Message> {
        SemanticNode {
            role: "space".to_string(),
            label: None,
            content: None,
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
        }
    }

    fn circle<Message: 'static>(_radius: f32, _color: Option<Color>) -> Self::AnyView<Message> {
        SemanticNode {
            role: "circle".to_string(),
            label: None,
            content: None,
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
        }
    }

    fn capsule<Message: 'static>(
        _width: Length,
        _height: Length,
        _color: Option<Color>,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "capsule".to_string(),
            label: None,
            content: None,
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
        }
    }

    fn rectangle<Message: 'static>(
        _width: Length,
        _height: Length,
        _color: Option<Color>,
        _radius: f32,
        _border_width: f32,
        _border_color: Option<Color>,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "rectangle".to_string(),
            label: None,
            content: None,
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
        }
    }

    fn button<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        _on_press: Option<Message>,
        variant: Variant,
        intent: Intent,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "button".to_string(),
            label: Some(format!("{:?}_{:?}", variant, intent)),
            content: None,
            children: vec![content],
            neural_tag: None,
            documentation: None,
        }
    }

    fn sidebar_item<Message: Clone + 'static>(
        title: String,
        icon: String,
        is_selected: bool,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "sidebar_item".to_string(),
            label: Some(title),
            content: Some(icon),
            children: vec![SemanticNode {
                role: "state".to_string(),
                label: Some("selected".to_string()),
                content: Some(is_selected.to_string()),
                children: Vec::new(),
                neural_tag: None,
                documentation: None,
            }],
            neural_tag: None,
            documentation: None,
        }
    }

    fn text_input<Message: Clone + 'static>(
        value: String,
        _placeholder: String,
        _on_change: impl Fn(String) -> Message + 'static,
        _on_submit: Option<Message>,
        _font: Option<iced::Font>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "text_input".to_string(),
            label: None,
            content: Some(value),
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
        }
    }

    fn slider<Message: Clone + 'static>(
        _range: std::ops::RangeInclusive<f32>,
        value: f32,
        _on_change: impl Fn(f32) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "slider".to_string(),
            label: None,
            content: Some(value.to_string()),
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
        }
    }

    fn toggle<Message: Clone + 'static>(
        label: String,
        is_active: bool,
        _on_toggle: impl Fn(bool) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "toggle".to_string(),
            label: Some(label),
            content: Some(is_active.to_string()),
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
        }
    }

    fn zstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _width: Length,
        _height: Length,
        _alignment: Alignment,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "zstack".to_string(),
            label: None,
            content: None,
            children,
            neural_tag: None,
            documentation: None,
        }
    }

    fn grid<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        columns: usize,
        _spacing: f32,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "grid".to_string(),
            label: Some(format!("columns: {}", columns)),
            content: None,
            children,
            neural_tag: None,
            documentation: None,
        }
    }

    fn image<Message: 'static>(
        path: impl Into<String>,
        _width: Length,
        _height: Length,
        _radius: f32,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "image".to_string(),
            label: Some(path.into()),
            content: None,
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
        }
    }

    fn container<Message: 'static>(
        content: Self::AnyView<Message>,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }

    fn scroll_view<Message: 'static>(content: Self::AnyView<Message>) -> Self::AnyView<Message> {
        content
    }

    fn mouse_area<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        _on_move: Option<Arc<dyn Fn(iced::Point) -> Message + Send + Sync>>,
        _on_press: Option<Message>,
        _on_release: Option<Message>,
    ) -> Self::AnyView<Message> {
        content
    }
}

/// A responsive helper.
pub fn responsive<Message>(
    mode: ShellMode,
    theme: peak_theme::ThemeTokens,
    f: impl Fn(Context) -> iced::Element<'static, Message, Theme, Renderer> + 'static,
) -> iced::Element<'static, Message, Theme, Renderer>
where
    Message: 'static,
{
    iced::widget::responsive(move |size| {
        let context = Context::new(mode, theme, size);
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
