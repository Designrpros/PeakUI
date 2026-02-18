use crate::core::Backend;
use crate::core::View;
use crate::elements::atoms::{
    Capsule, Circle, Container, Divider, Icon, Image, Rectangle, Space, Text, Video, WebView,
};
use crate::elements::controls::{Button, Slider, TextInput, Toggle};
use crate::layout::containers::{GlassCard, Section};
use crate::layout::scroll_view::ScrollView;
use crate::layout::{HStack, ResponsiveGrid, VStack, Wrap, ZStack};
use iced::Length;
use std::borrow::Cow;

pub fn text<B: Backend>(content: impl Into<Cow<'static, str>>) -> Text<B> {
    Text::new(content)
}

pub fn icon<B: Backend>(name: impl Into<Cow<'static, str>>) -> Icon<B> {
    Icon::new(name)
}

pub fn divider<B: Backend>() -> Divider<B> {
    Divider::new()
}

pub fn image<B: Backend>(name: impl Into<Cow<'static, str>>) -> Image<B> {
    Image::new(name)
}

pub fn video<B: Backend>(path: impl Into<Cow<'static, str>>) -> Video<B> {
    Video::new(path)
}

pub fn web_view<B: Backend>(url: impl Into<Cow<'static, str>>) -> WebView<B> {
    WebView::new(url)
}

pub fn space<B: Backend>(width: impl Into<Length>, height: impl Into<Length>) -> Space<B> {
    Space::new(width.into(), height.into())
}

pub fn rectangle<B: Backend>(width: impl Into<Length>, height: impl Into<Length>) -> Rectangle<B> {
    Rectangle::new(width.into(), height.into())
}

pub fn circle<B: Backend>(radius: f32) -> Circle<B> {
    Circle::new(radius)
}

pub fn capsule<B: Backend>(width: impl Into<Length>, height: impl Into<Length>) -> Capsule<B> {
    Capsule::new(width.into(), height.into())
}

pub fn container<Message: 'static + Send + Sync, B: Backend>(
    content: impl View<Message, B> + Send + Sync + 'static,
) -> Container<Message, B> {
    Container::new(content)
}

pub fn vstack<Message: 'static + Send + Sync, B: Backend>() -> VStack<Message, B> {
    VStack::new()
}

pub fn hstack<Message: 'static + Send + Sync, B: Backend>() -> HStack<Message, B> {
    HStack::new()
}

pub fn zstack<Message: 'static + Send + Sync, B: Backend>() -> ZStack<Message, B> {
    ZStack::new()
}

pub fn grid<Message: 'static + Send + Sync, B: Backend>() -> ResponsiveGrid<Message, B> {
    ResponsiveGrid::new()
}

pub fn wrap<Message: 'static + Send + Sync, B: Backend>() -> Wrap<Message, B> {
    Wrap::new()
}

pub fn glass_card<Message: 'static + Send + Sync, B: Backend>(
    content: impl View<Message, B> + Send + Sync + 'static,
) -> GlassCard<Message, B> {
    GlassCard::new(content)
}

pub fn section<Message: 'static + Send + Sync, B: Backend>(
    title: impl Into<Cow<'static, str>>,
    content: impl View<Message, B> + Send + Sync + 'static,
) -> Section<Message, B> {
    Section::new_generic(title, content)
}

pub fn button<Message: Clone + Send + Sync + 'static, B: Backend>(
    content: impl View<Message, B> + Send + Sync + 'static,
) -> Button<Message, B> {
    Button::new(content)
}

pub fn button_label<Message: Clone + Send + Sync + 'static, B: Backend>(
    label: impl Into<Cow<'static, str>>,
) -> Button<Message, B> {
    Button::new(Text::new(label))
}

pub fn text_input<Message: Clone + Send + Sync + 'static, B: Backend>(
    value: impl Into<String>,
    placeholder: impl Into<Cow<'static, str>>,
    on_change: impl Fn(String) -> Message + Send + Sync + 'static,
) -> TextInput<Message, B> {
    TextInput::new(value, placeholder, on_change)
}

pub fn toggle<Message: Clone + Send + Sync + 'static, B: Backend>(
    label: impl Into<Cow<'static, str>>,
    is_active: bool,
    on_toggle: impl Fn(bool) -> Message + Send + Sync + 'static,
) -> Toggle<Message, B> {
    Toggle::new(label, is_active, on_toggle)
}

pub fn slider<Message: Clone + Send + Sync + 'static, B: Backend>(
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    on_change: impl Fn(f32) -> Message + Send + Sync + 'static,
) -> Slider<Message, B> {
    Slider::new(range, value, on_change)
}

pub fn scroll_view<Message: 'static + Send + Sync, B: Backend>(
    content: impl View<Message, B> + Send + Sync + 'static,
) -> ScrollView<Message, B> {
    ScrollView::new_generic(content)
}
