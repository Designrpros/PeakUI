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
        crate::core::SemanticNode { accessibility: None, 
            role: "gesture_detector".to_string(),
            label: None,
            content: None,
            children: vec![self.content.describe(context)],
            neural_tag: None,
            documentation: None,
        }
    }
}
