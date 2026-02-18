use crate::core::{Backend, Context, IcedBackend, View};
use crate::layout::VStack;
use crate::prelude::*;
use iced::Length;

pub struct ContextMenu<Message: Clone + Send + Sync + 'static, B: Backend = IcedBackend> {
    items: Vec<ContextMenuItem<Message>>,
    _phantom: std::marker::PhantomData<B>,
}

#[derive(Clone)]
pub struct ContextMenuItem<Message: Clone + Send + Sync + 'static> {
    pub label: String,
    pub icon: String,
    pub action: Message,
}

impl<Message: Clone + Send + Sync + 'static, B: Backend> ContextMenu<Message, B> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn item(
        mut self,
        label: impl Into<String>,
        icon: impl Into<String>,
        action: Message,
    ) -> Self {
        self.items.push(ContextMenuItem {
            label: label.into(),
            icon: icon.into(),
            action,
        });
        self
    }
}

impl<Message: Clone + Send + Sync + 'static, B: Backend> View<Message, B>
    for ContextMenu<Message, B>
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let mut list = VStack::<Message, B>::new().spacing(4.0).padding(4.0);

        for item in &self.items {
            let label = item.label.clone();
            let icon = item.icon.clone();
            let action = item.action.clone();

            list = list.push(
                crate::elements::controls::Button::<Message, B>::label(label)
                    .icon(icon)
                    .variant(Variant::Ghost)
                    .on_press(action)
                    .width(Length::Fill),
            );
        }

        crate::layout::containers::Card::<Message, B>::new_generic(list)
            .width(Length::Fixed(180.0))
            .view(context)
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("context_menu")
            .with_label(format!("Context Menu ({} items)", self.items.len()))
    }
}
