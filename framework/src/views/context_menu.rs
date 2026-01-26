use crate::core::{Backend, Context, IcedBackend, View};
use crate::layout::VStack;
use crate::prelude::*;
use iced::{Element, Length, Renderer, Theme};

pub struct ContextMenu<Message: Clone + 'static, B: Backend = IcedBackend> {
    items: Vec<ContextMenuItem<Message>>,
    _phantom: std::marker::PhantomData<B>,
}

#[derive(Clone)]
pub struct ContextMenuItem<Message: Clone + 'static> {
    pub label: String,
    pub icon: String,
    pub action: Message,
}

impl<Message: Clone + 'static, B: Backend> ContextMenu<Message, B> {
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

impl<Message: Clone + 'static> View<Message, IcedBackend> for ContextMenu<Message, IcedBackend> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let _theme = context.theme;

        let mut list = VStack::<Message, IcedBackend>::new_generic()
            .spacing(4.0)
            .padding(4.0);

        for item in &self.items {
            let label = item.label.clone();
            let icon = item.icon.clone();
            let action = item.action.clone();

            list = list.push(
                crate::controls::Button::<Message, IcedBackend>::label(label)
                    .icon(icon)
                    .variant(Variant::Ghost)
                    .on_press(action)
                    .width(Length::Fill),
            );
        }

        crate::containers::Card::<Message, IcedBackend>::new_generic(list)
            .width(Length::Fixed(180.0))
            .view(context)
    }
}
