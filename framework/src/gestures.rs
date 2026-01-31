use crate::core::{Context, View};
use iced::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gesture {
    Tap(Point),
    DoubleTap(Point),
    LongPress(Point),
    Drag {
        start: Point,
        current: Point,
        delta: (f32, f32),
    },
    Pinch {
        center: Point,
        scale: f32,
    },
}

pub struct GestureDetector<V, F> {
    content: V,
    _on_gesture: F,
}

impl<V, F> GestureDetector<V, F> {
    pub fn new(content: V, on_gesture: F) -> Self {
        Self {
            content,
            _on_gesture: on_gesture,
        }
    }
}

impl<Message: 'static, B: crate::core::Backend, V, F> View<Message, B> for GestureDetector<V, F>
where
    V: View<Message, B>,
    F: Fn(Gesture) + 'static,
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        self.content.view(context)
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode {
            role: "gesture_detector".to_string(),
            children: vec![self.content.describe(context)],
            ..Default::default()
        }
    }
}
pub struct TapGesture<Message: 'static, B: crate::core::Backend, V: View<Message, B>> {
    content: V,
    on_tap: Message,
    _phantom: std::marker::PhantomData<(Message, B)>,
}

impl<Message: 'static, B: crate::core::Backend, V: View<Message, B>> TapGesture<Message, B, V> {
    pub fn new(content: V, on_tap: Message) -> Self {
        Self {
            content,
            on_tap,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Message: Clone + 'static, B: crate::core::Backend, V: View<Message, B>> View<Message, B>
    for TapGesture<Message, B, V>
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::button(
            self.content.view(context),
            Some(self.on_tap.clone()),
            crate::modifiers::Variant::Plain,
            crate::modifiers::Intent::Neutral,
            iced::Length::Shrink,
            false,
            context,
        )
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        let mut node = self.content.describe(context);
        node.role = "tap_gesture".to_string();
        node
    }
}
