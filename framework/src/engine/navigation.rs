use crate::core::{Backend, Context, IcedBackend, ScrollDirection, SemanticNode, View};
use crate::elements::atoms::{Icon, Text};
use crate::layout::{HStack, VStack};
use crate::style::{Intent, Variant};
use iced::{Alignment, Color, Length, Padding};
use std::marker::PhantomData;
use std::sync::Arc;

/// Represents a page or destination in the navigation system.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Page {
    #[default]
    Home,
    Components,
    Settings,
    Custom(String),
}

/// Configuration for search functionality in navigation views.
#[derive(Clone, Default)]
pub struct SearchConfig {
    pub query: String,
    pub placeholder: String,
}

/// The result of a page view, including its view and metadata.
pub struct PageResult<Message: 'static + Send + Sync, B: Backend = IcedBackend> {
    pub view: Box<dyn View<Message, B> + Send + Sync>,
    pub title: String,
    pub inspector: Option<Box<dyn View<Message, B> + Send + Sync>>,
    pub search_config: Option<SearchConfig>,
    pub toolbar_items: Vec<Box<dyn View<Message, B> + Send + Sync>>,
    pub sidebar_toggle: Option<Message>,
}

impl<Message: 'static + Send + Sync, B: Backend> PageResult<Message, B> {
    pub fn new(view: impl View<Message, B> + Send + Sync + 'static) -> Self {
        Self {
            view: Box::new(view),
            title: String::new(),
            inspector: None,
            search_config: None,
            toolbar_items: Vec::new(),
            sidebar_toggle: None,
        }
    }

    pub fn searchable(
        mut self,
        _title: &str,
        placeholder: &str,
        _on_change: impl Fn(String) -> Message + Send + Sync + 'static,
    ) -> Self {
        self.search_config = Some(SearchConfig {
            query: String::new(),
            placeholder: placeholder.to_string(),
        });
        self
    }

    pub fn sidebar_toggle(mut self, message: Message) -> Self {
        self.sidebar_toggle = Some(message);
        self
    }

    pub fn inspector(mut self, view: impl View<Message, B> + Send + Sync + 'static) -> Self {
        self.inspector = Some(Box::new(view));
        self
    }
}

impl<Message: 'static + Send + Sync, B: Backend> From<VStack<Message, B>>
    for PageResult<Message, B>
{
    fn from(stack: VStack<Message, B>) -> Self {
        Self::new(stack)
    }
}

/// Extension trait for adding navigation-specific behavior to views.
pub trait ViewExt<Message: 'static + Send + Sync, B: Backend>:
    View<Message, B> + Send + Sync + Sized
{
    fn searchable(
        self,
        title: &str,
        placeholder: &str,
        on_change: impl Fn(String) -> Message + Send + Sync + 'static,
    ) -> PageResult<Message, B>
    where
        Self: 'static,
    {
        PageResult::new(self).searchable(title, placeholder, on_change)
    }

    fn sidebar_toggle(self, message: Message) -> PageResult<Message, B>
    where
        Self: 'static,
    {
        PageResult::new(self).sidebar_toggle(message)
    }

    fn inspector(
        self,
        view: impl View<Message, B> + Send + Sync + 'static,
    ) -> PageResult<Message, B>
    where
        Self: 'static,
    {
        PageResult::new(self).inspector(view)
    }

    fn nav_list(self) -> NavigationListView<Message, B>
    where
        Self: View<Message, B> + Send + Sync + 'static,
    {
        NavigationListView::new().push(self)
    }

    // --- Tailwind-style Utility Modifiers ---

    fn padding(
        self,
        padding: impl Into<iced::Padding>,
    ) -> crate::elements::atoms::Container<Message, B>
    where
        Self: 'static,
    {
        crate::elements::atoms::Container::new(self).padding(padding)
    }

    fn width(self, width: Length) -> crate::elements::atoms::Container<Message, B>
    where
        Self: 'static,
    {
        crate::elements::atoms::Container::new(self).width(width)
    }

    fn height(self, height: Length) -> crate::elements::atoms::Container<Message, B>
    where
        Self: 'static,
    {
        crate::elements::atoms::Container::new(self).height(height)
    }

    fn background(self, color: Color) -> crate::elements::atoms::Container<Message, B>
    where
        Self: 'static,
    {
        crate::elements::atoms::Container::new(self).background(color)
    }

    fn corner_radius(self, radius: f32) -> crate::elements::atoms::Container<Message, B>
    where
        Self: 'static,
    {
        crate::elements::atoms::Container::new(self).radius(radius)
    }

    fn border(self, width: f32, color: Color) -> crate::elements::atoms::Container<Message, B>
    where
        Self: 'static,
    {
        crate::elements::atoms::Container::new(self).border(width, color)
    }

    fn shadow(self, shadow: iced::Shadow) -> crate::elements::atoms::Container<Message, B>
    where
        Self: 'static,
    {
        crate::elements::atoms::Container::new(self).shadow(shadow)
    }
}

impl<V: View<Message, B> + Send + Sync + 'static, Message: 'static + Send + Sync, B: Backend>
    ViewExt<Message, B> for V
{
}

// Explicitly implement for VStack to resolve ambiguity
impl<Message: 'static + Send + Sync, B: Backend> VStack<Message, B> {
    pub fn searchable(
        self,
        title: &str,
        placeholder: &str,
        on_change: impl Fn(String) -> Message + Send + Sync + 'static,
    ) -> PageResult<Message, B> {
        PageResult::new(self).searchable(title, placeholder, on_change)
    }

    pub fn sidebar_toggle(self, message: Message) -> PageResult<Message, B> {
        PageResult::new(self).sidebar_toggle(message)
    }
}

/// A component that represents a navigation destination in a sidebar or list.
pub struct NavigationLink<Message: Clone + Send + Sync + 'static, B: Backend = IcedBackend> {
    label: String,
    icon: String,
    destination: Message,
    is_active: bool,
    _phantom: PhantomData<B>,
}

impl<Message: Clone + Send + Sync + 'static, B: Backend> NavigationLink<Message, B> {
    pub fn new(label: impl Into<String>, icon: impl Into<String>, destination: Message) -> Self {
        Self {
            label: label.into(),
            icon: icon.into(),
            destination,
            is_active: false,
            _phantom: PhantomData,
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.is_active = active;
        self
    }
}

impl<Message: Clone + Send + Sync + 'static, B: Backend> View<Message, B>
    for NavigationLink<Message, B>
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let active = self.is_active;

        let inner = HStack::<Message, B>::new_generic()
            .width(Length::Fill)
            .spacing(12.0)
            .padding(Padding::from([12, 16]))
            .align_y(Alignment::Center)
            .push(Icon::<B>::new(self.icon.clone()).size(16.0).secondary())
            .push(if active {
                Text::<B>::new(self.label.clone())
                    .caption1()
                    .bold()
                    .width(Length::Fill)
            } else {
                Text::<B>::new(self.label.clone())
                    .caption1()
                    .width(Length::Fill)
            })
            .view(context);

        B::button(
            inner,
            Some(self.destination.clone()),
            Variant::Ghost,
            Intent::Neutral,
            Length::Fill,
            Length::Shrink,
            false,
            context,
        )
    }

    fn describe(&self, _context: &Context) -> SemanticNode {
        SemanticNode::new("navigation_link").with_label(self.label.clone())
    }
}

/// A container for detail content in a master-detail split view.
pub struct DetailView<
    V: View<Message, B> + 'static,
    Message: Clone + Send + Sync + 'static,
    B: Backend = IcedBackend,
> {
    content: V,
    _phantom: PhantomData<(Message, B)>,
}

impl<V: View<Message, B> + 'static, Message: Clone + Send + Sync + 'static, B: Backend>
    DetailView<V, Message, B>
{
    pub fn new(content: V) -> Self {
        Self {
            content,
            _phantom: PhantomData,
        }
    }
}

impl<V: View<Message, B> + 'static, Message: Clone + Send + Sync + 'static, B: Backend>
    View<Message, B> for DetailView<V, Message, B>
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let inner = self.content.view(context);
        B::container(
            inner,
            Padding::from([48, 64]),
            Length::Fill,
            Length::Fill,
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

    fn describe(&self, context: &Context) -> SemanticNode {
        SemanticNode::new("detail_view").push_child(self.content.describe(context))
    }
}

/// A structural sidebar component for primary navigation.
pub struct Sidebar<Message: Clone + Send + Sync + 'static, B: Backend = IcedBackend> {
    title: String,
    items: Vec<Box<dyn View<Message, B> + Send + Sync>>,
    search: Option<(String, Arc<dyn Fn(String) -> Message + Send + Sync>)>,
}

impl<Message: Clone + Send + Sync + 'static, B: Backend> Sidebar<Message, B> {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
            search: None,
        }
    }

    pub fn with_search(
        mut self,
        query: impl Into<String>,
        on_change: impl Fn(String) -> Message + Send + Sync + 'static,
    ) -> Self {
        self.search = Some((query.into(), Arc::new(on_change)));
        self
    }

    pub fn item(
        mut self,
        label: impl Into<String>,
        icon: impl Into<String>,
        destination: Message,
        is_active: bool,
    ) -> Self {
        let link = NavigationLink::new(label, icon, destination).active(is_active);
        self.items.push(Box::new(link));
        self
    }

    pub fn push(mut self, item: impl View<Message, B> + Send + Sync + 'static) -> Self {
        self.items.push(Box::new(item));
        self
    }
}

impl<Message: Clone + Send + Sync + 'static, B: Backend> View<Message, B> for Sidebar<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let mut views = Vec::new();

        views.push(
            Text::<B>::new(self.title.clone())
                .headline()
                .bold()
                .secondary()
                .view(context),
        );

        if let Some((query, on_change)) = &self.search {
            use crate::prelude::TextInput;
            // Add search bar if configured
            views.push(
                TextInput::<Message, B>::new(query.clone(), "Search...", {
                    let cb = on_change.clone();
                    move |s| (cb)(s)
                })
                .view(context),
            );
        }

        for item in &self.items {
            views.push(item.view(context));
        }

        let bottom_padding = if context.is_slim() { 120.0 } else { 32.0 };

        let inner = B::vstack(
            views,
            12.0,
            Padding {
                top: 32.0,
                right: 20.0,
                bottom: bottom_padding,
                left: 20.0,
            },
            Length::Fill,
            Length::Shrink,
            Alignment::Start,
            Alignment::Start,
            context,
        );

        B::scroll_view(
            inner,
            Length::Fill,
            Length::Fill,
            None,
            false,
            ScrollDirection::Vertical,
            context,
        )
    }

    fn describe(&self, context: &Context) -> SemanticNode {
        SemanticNode::new("sidebar")
            .with_label(self.title.clone())
            .extend_children(self.items.iter().map(|i| i.describe(context)))
    }
}

pub struct NavigationListView<Message: 'static + Send + Sync, B: Backend = IcedBackend> {
    stack: VStack<Message, B>,
}

impl<Message: 'static + Send + Sync, B: Backend> Default for NavigationListView<Message, B> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Message: 'static + Send + Sync, B: Backend> NavigationListView<Message, B> {
    pub fn new() -> Self {
        Self {
            stack: VStack::new_generic().spacing(4.0).width(Length::Fill),
        }
    }

    pub fn push(mut self, link: impl View<Message, B> + Send + Sync + 'static) -> Self {
        self.stack = self.stack.push(link);
        self
    }
}

impl<Message: 'static + Send + Sync, B: Backend> View<Message, B>
    for NavigationListView<Message, B>
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        self.stack.view(context)
    }

    fn describe(&self, context: &Context) -> SemanticNode {
        self.stack.describe(context)
    }
}
