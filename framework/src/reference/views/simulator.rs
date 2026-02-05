use crate::core::{Backend, Context, View};
use iced::{Color, Length, Padding};

pub struct SimulatorView<Message> {
    node: crate::core::SpatialNode<()>,
    _phantom: std::marker::PhantomData<Message>,
}

impl<Message> SimulatorView<Message> {
    pub fn new(node: crate::core::SpatialNode<()>) -> Self {
        Self {
            node,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Message: Clone + 'static, B: Backend> View<Message, B> for SimulatorView<Message> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let mut elements: Vec<B::AnyView<Message>> = Vec::new();

        // Recursively render nodes as boxes
        self.render_node::<B>(&self.node, 0.0, 0.0, 1.0, &mut elements, context);

        let content = B::vstack(
            elements,
            12.0,
            Padding::new(24.0),
            Length::Fill,
            Length::Shrink,
            iced::Alignment::Center,
            iced::Alignment::Start,
            context,
        );

        B::scroll_view(
            content,
            Length::Fill,
            Length::Fill,
            None,
            true,
            crate::core::ScrollDirection::Vertical,
            context,
        )
    }
}

impl<Message> SimulatorView<Message> {
    fn render_node<B: Backend + 'static>(
        &self,
        node: &crate::core::SpatialNode<()>,
        parent_x: f32,
        parent_y: f32,
        parent_z: f32,
        elements: &mut Vec<B::AnyView<Message>>,
        context: &Context,
    ) where
        Message: Clone + 'static,
    {
        let x = parent_x + node.transform.position.x;
        let y = parent_y + node.transform.position.y;
        let z = parent_z + node.transform.position.z;

        // Simple projection: just use Z for color/scale
        let scale = 1.0 / (1.0 + z * 0.001);
        let color = if z > 0.0 {
            Color::from_rgb(0.0, 0.5, 1.0) // Foreground
        } else {
            Color::from_rgb(1.0, 1.0, 1.0) // Background
        };

        // Render this node
        elements.push(B::container(
            B::text(
                format!("{}: z={:.1}", node.role, z),
                11.0 * scale,
                Some(Color::WHITE),
                true, // Bold for readability
                false,
                None,
                None,
                Length::Shrink,
                iced::Alignment::Center,
                context,
            ),
            Padding::new(8.0 * scale),
            Length::Fixed(140.0 * scale),
            Length::Fixed(50.0 * scale),
            Some({
                let mut c = color;
                c.a = 0.85; // Slight transparency
                c
            }),
            8.0 * scale,
            1.5 * scale,
            Some(Color::WHITE), // Bright border
            None,
            iced::Alignment::Center,
            iced::Alignment::Center,
            context,
        ));

        // Render children
        for child in &node.children {
            self.render_node::<B>(child, x, y, z, elements, context);
        }
    }
}
