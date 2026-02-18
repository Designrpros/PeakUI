use crate::elements::controls::TextEditor;
use crate::prelude::*;
use crate::reference::app::state::RenderMode;
use crate::reference::Message;

pub fn view(_context: &Context, _render_mode: RenderMode) -> AppPageResult {
    let text_field_demo = VStack::new_generic()
        .spacing(16.0)
        .push(Text::new("TextField").title3().bold())
        .push(Text::new("Single-line text input for short interactions.").subheadline())
        .push(
            crate::elements::controls::TextInput::<crate::reference::Message, IcedBackend>::new(
                "".to_string(),
                "Enter your name...".to_string(),
                |_| Message::None,
            )
            .width(Length::Fill),
        )
        .push(
            crate::elements::controls::TextInput::<crate::reference::Message, IcedBackend>::new(
                "Protected Value".to_string(),
                "Security...".to_string(),
                |_| Message::None,
            )
            .password()
            .width(Length::Fill),
        );

    let text_editor_demo = VStack::new_generic()
        .spacing(16.0)
        .push(Text::new("TextEditor").title3().bold())
        .push(Text::new("Multi-line text editor for rich content.").subheadline())
        .push(
            TextEditor::<crate::reference::Message, IcedBackend>::new(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string(),
                |_| Message::None,
            )
            .height(Length::Fixed(150.0)),
        );

    let view = ScrollView::<crate::reference::Message, IcedBackend>::new_generic(
        VStack::new_generic()
            .spacing(40.0)
            .padding(24.0)
            .push(text_field_demo)
            .push(Divider::<IcedBackend>::new())
            .push(text_editor_demo),
    );

    AppPageResult::new(view)
}
