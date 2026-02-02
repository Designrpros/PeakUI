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

        B::vstack(
            elements,
            10.0,
            Padding::new(20.0),
            Length::Fill,
            Length::Fill,
            iced::Alignment::Center,
            iced::Alignment::Center,
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
                10.0 * scale,
                Some(Color::BLACK),
                false,
                false,
                None,
                None,
                Length::Shrink,
                iced::Alignment::Center,
                context,
            ),
            Padding::new(5.0 * scale),
            Length::Fixed(100.0 * scale),
            Length::Fixed(40.0 * scale),
            Some(color),
            4.0 * scale,
            1.0,
            Some(Color::BLACK),
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
