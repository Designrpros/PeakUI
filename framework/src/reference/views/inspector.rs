use super::super::app::Message;
use crate::prelude::*;

pub struct InspectorView;

impl InspectorView {
    pub fn new() -> Self {
        Self
    }
}

impl View<Message, IcedBackend> for InspectorView {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        container(
            ScrollView::new(
                VStack::new_generic()
                    .spacing(16.0)
                    .padding(Padding {
                        top: 96.0,
                        right: 24.0,
                        bottom: 24.0,
                        left: 24.0,
                    })
                    .push(
                        Text::<IcedBackend>::new("On this page")
                            .caption2()
                            .bold()
                            .secondary(),
                    )
                    .push(
                        VStack::new_generic()
                            .spacing(16.0)
                            .push(
                                Text::<IcedBackend>::new("How to set an element to a width?")
                                    .caption2()
                                    .bold(),
                            )
                            .push(
                                Text::<IcedBackend>::new("Fixed widths")
                                    .caption2()
                                    .secondary(),
                            )
                            .push(
                                Text::<IcedBackend>::new("Percentage widths")
                                    .caption2()
                                    .secondary(),
                            ),
                    ),
            )
            .view(context),
        )
        .width(260)
        .height(Length::Fill)
        .into()
    }
}
