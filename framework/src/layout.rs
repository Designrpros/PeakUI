use crate::core::{Backend, Context, IcedBackend, TermBackend, View};
use iced::Length;

/// A vertical stack layout that arranges children from top to bottom.
pub struct VStack<Message: 'static, B: Backend = IcedBackend> {
    children: Vec<Box<dyn View<Message, B>>>,
    spacing: f32,
    padding: iced::Padding,
    width: Length,
    height: Length,
    align_x: iced::Alignment,
    align_y: iced::Alignment,
}

impl<Message: 'static> VStack<Message, IcedBackend> {
    pub fn new() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static> VStack<Message, TermBackend> {
    pub fn new_tui() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static, B: Backend> VStack<Message, B> {
    pub fn new_generic() -> Self {
        Self {
            children: Vec::new(),
            spacing: 0.0,
            padding: iced::Padding::from(0.0),
            width: Length::Fill,
            height: Length::Shrink,
            align_x: iced::Alignment::Start,
            align_y: iced::Alignment::Start,
        }
    }

    pub fn align_y(mut self, align: iced::Alignment) -> Self {
        self.align_y = align;
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn padding(mut self, padding: impl Into<iced::Padding>) -> Self {
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

    pub fn align_x(mut self, align: iced::Alignment) -> Self {
        self.align_x = align;
        self
    }

    pub fn push(mut self, view: impl View<Message, B> + 'static) -> Self {
        self.children.push(Box::new(view));
        self
    }

    pub fn extend<I, V>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: View<Message, B> + 'static,
    {
        for child in iter {
            self.children.push(Box::new(child));
        }
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for VStack<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let child_views = self.children.iter().map(|c| c.view(context)).collect();
        B::vstack(
            child_views,
            self.spacing,
            self.padding,
            self.width,
            self.height,
            self.align_x,
            self.align_y,
            context.theme.scaling,
        )
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        let children = self.children.iter().map(|c| c.describe(context)).collect();
        crate::core::SemanticNode {
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
}

/// A horizontal stack layout that arranges children from left to right.
pub struct HStack<Message: 'static, B: Backend = IcedBackend> {
    children: Vec<Box<dyn View<Message, B>>>,
    spacing: f32,
    padding: iced::Padding,
    width: Length,
    height: Length,
    align_x: iced::Alignment,
    align_y: iced::Alignment,
}

impl<Message: 'static> HStack<Message, IcedBackend> {
    pub fn new() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static> HStack<Message, TermBackend> {
    pub fn new_tui() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static, B: Backend> HStack<Message, B> {
    pub fn new_generic() -> Self {
        Self {
            children: Vec::new(),
            spacing: 0.0,
            padding: iced::Padding::from(0.0),
            width: Length::Fill,
            height: Length::Shrink,
            align_x: iced::Alignment::Start,
            align_y: iced::Alignment::Start,
        }
    }

    pub fn align_x(mut self, align: iced::Alignment) -> Self {
        self.align_x = align;
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn padding(mut self, padding: impl Into<iced::Padding>) -> Self {
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

    pub fn align_y(mut self, align: iced::Alignment) -> Self {
        self.align_y = align;
        self
    }

    pub fn push(mut self, view: impl View<Message, B> + 'static) -> Self {
        self.children.push(Box::new(view));
        self
    }

    pub fn extend<I, V>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: View<Message, B> + 'static,
    {
        for child in iter {
            self.children.push(Box::new(child));
        }
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for HStack<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let child_views = self.children.iter().map(|c| c.view(context)).collect();
        B::hstack(
            child_views,
            self.spacing,
            self.padding,
            self.width,
            self.height,
            self.align_x,
            self.align_y,
            context.theme.scaling,
        )
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        let children = self.children.iter().map(|c| c.describe(context)).collect();
        crate::core::SemanticNode {
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
}

/// A stack layout that layers children on top of each other (Z-axis).
pub struct ZStack<Message: 'static, B: Backend = IcedBackend> {
    children: Vec<Box<dyn View<Message, B>>>,
    width: Length,
    height: Length,
    alignment: iced::Alignment,
}

impl<Message: 'static> ZStack<Message, IcedBackend> {
    pub fn new() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static> ZStack<Message, TermBackend> {
    pub fn new_tui() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static, B: Backend> ZStack<Message, B> {
    pub fn new_generic() -> Self {
        Self {
            children: Vec::new(),
            width: Length::Shrink,
            height: Length::Shrink,
            alignment: iced::Alignment::Start,
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

    pub fn align(mut self, alignment: iced::Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn push(mut self, view: impl View<Message, B> + 'static) -> Self {
        self.children.push(Box::new(view));
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for ZStack<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let child_views = self.children.iter().map(|c| c.view(context)).collect();
        B::zstack(child_views, self.width, self.height, self.alignment)
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        let children = self.children.iter().map(|c| c.describe(context)).collect();
        crate::core::SemanticNode {
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
}

/// Extension trait for layout-related modifiers.
pub trait LayoutExt<Message: Clone + 'static, B: Backend>: View<Message, B> + Sized {
    /// Layers the given view on top of this view using a ZStack.
    fn overlay<V: View<Message, B> + 'static>(
        self,
        overlay: V,
        alignment: iced::Alignment,
    ) -> ZStack<Message, B>
    where
        Self: 'static,
    {
        ZStack::new_generic()
            .push(self)
            .push(overlay)
            .align(alignment)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
    }
    /// Sets an ideal width for the view, scaled by the current theme scaling.
    fn ideal_width(self, width: f32) -> crate::core::ProxyView<Message, B>
    where
        Self: Sized + 'static;

    fn locked(self, is_locked: bool) -> crate::core::ProxyView<Message, B>
    where
        Self: Sized + 'static;
}

impl<V: View<Message, B> + Sized + 'static, Message: Clone + 'static, B: Backend>
    LayoutExt<Message, B> for V
{
    fn ideal_width(self, width: f32) -> crate::core::ProxyView<Message, B>
    where
        Self: Sized + 'static,
    {
        crate::core::ProxyView::new(move |context| {
            let scaled_width = (width * context.theme.scaling).max(100.0).min(1200.0);
            B::hstack(
                vec![self.view(context)],
                0.0,
                iced::Padding::ZERO,
                iced::Length::Fixed(scaled_width),
                iced::Length::Shrink,
                iced::Alignment::Center,
                iced::Alignment::Center,
                context.theme.scaling,
            )
        })
    }

    fn locked(self, _is_locked: bool) -> crate::core::ProxyView<Message, B>
    where
        Self: Sized + 'static,
    {
        crate::core::ProxyView::new(move |context| {
            // This is a semantic modifier, it doesn't change rendering here but can be queried
            // For now, we just pass the view through. In a real system, we'd wrap it in a struct
            // that NavigationSplitView can detect.
            self.view(context)
        })
    }
}

pub struct ResponsiveGrid<Message: 'static, B: Backend = IcedBackend> {
    children: Vec<Box<dyn View<Message, B>>>,
    spacing: f32,
}

impl<Message: 'static> ResponsiveGrid<Message, IcedBackend> {
    pub fn new() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static> ResponsiveGrid<Message, TermBackend> {
    pub fn new_tui() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static, B: Backend> ResponsiveGrid<Message, B> {
    pub fn new_generic() -> Self {
        Self {
            children: Vec::new(),
            spacing: 20.0,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn push(mut self, view: impl View<Message, B> + 'static) -> Self {
        self.children.push(Box::new(view));
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for ResponsiveGrid<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let items_per_row = if context.size.width < 600.0 {
            1
        } else if context.size.width < 900.0 {
            2
        } else if context.size.width < 1200.0 {
            3
        } else if context.size.width < 1600.0 {
            4
        } else {
            5
        };

        let child_views = self.children.iter().map(|c| c.view(context)).collect();
        B::grid(child_views, items_per_row, self.spacing)
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        let items_per_row = if context.size.width < 600.0 {
            1
        } else if context.size.width < 900.0 {
            2
        } else if context.size.width < 1200.0 {
            3
        } else if context.size.width < 1600.0 {
            4
        } else {
            5
        };

        let children = self.children.iter().map(|c| c.describe(context)).collect();
        crate::core::SemanticNode {
            accessibility: None,
            role: "grid".to_string(),
            label: Some(format!("responsive_columns: {}", items_per_row)),
            content: None,
            children,
            neural_tag: None,
            documentation: None,
            ..Default::default()
        }
    }
}
