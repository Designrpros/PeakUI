use crate::style::{Context, Intent, Radius, ScrollDirection, Variant};
use iced::{Alignment, Color, Length, Padding};
use nalgebra::Vector3;
use std::sync::Arc;

pub mod ai;
pub use ai::AIBackend;
pub mod iced_backend;
pub use iced_backend::IcedBackend;
pub mod spatial;
pub use spatial::{SpatialBackend, SpatialNode};
pub mod term;
pub use term::TermBackend;

pub mod color_serde {
    use iced::Color;
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(color: &Option<Color>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(c) = color {
            let rgba = [c.r, c.g, c.b, c.a];
            rgba.serialize(serializer)
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rgba: Option<[f32; 4]> = Option::deserialize(deserializer)?;
        Ok(rgba.map(|[r, g, b, a]| Color { r, g, b, a }))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextSpan {
    pub content: String,
    #[serde(with = "color_serde")]
    pub color: Option<Color>,
    pub size: Option<f32>,
    pub is_bold: bool,
    pub is_dim: bool,
    #[serde(skip)]
    pub font: Option<iced::Font>,
}

impl TextSpan {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            color: None,
            size: None,
            is_bold: false,
            is_dim: false,
            font: None,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
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

    pub fn font(mut self, font: iced::Font) -> Self {
        self.font = Some(font);
        self
    }
}

pub trait Backend: Sized + Clone + 'static {
    type AnyView<Message: 'static>: 'static;

    fn semantic_node<Message: 'static>(
        node: crate::semantic::SemanticNode,
        context: &Context,
    ) -> Self::AnyView<Message>;

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
        radius: f32, // Circles still use a single radius value
        color: Option<Color>,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn arc<Message: 'static>(
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        color: Option<Color>,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn path<Message: 'static>(
        points: Vec<iced::Point>,
        color: Option<Color>,
        width: f32,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn capsule<Message: 'static>(
        width: Length,
        height: Length,
        color: Option<Color>,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn rectangle<Message: 'static, R: Into<Radius>>(
        width: Length,
        height: Length,
        color: Option<Color>,
        radius: R,
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
        height: Length,
        is_compact: bool,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn sidebar_item<Message: Clone + Send + Sync + 'static>(
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

    fn image<Message: 'static, S: Into<String>, R: Into<Radius>>(
        path: S,
        width: Length,
        height: Length,
        radius: R,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn video<Message: 'static, S: Into<String>, R: Into<Radius>>(
        path: S,
        width: Length,
        height: Length,
        radius: R,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn web_view<Message: 'static, R: Into<Radius>>(
        url: String,
        width: Length,
        height: Length,
        radius: R,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn container<Message: 'static, R: Into<Radius>>(
        content: Self::AnyView<Message>,
        padding: Padding,
        width: Length,
        height: Length,
        background: Option<Color>,
        radius: R,
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
        direction: ScrollDirection,
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
        tooltip: Arc<str>,
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
