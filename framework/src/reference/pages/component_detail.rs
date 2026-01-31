use super::super::app::Message;
use super::super::page::PageResult;
use crate::prelude::*;

pub fn view(name: &str, context: &Context, _is_mobile: bool) -> PageResult {
    let name = name.to_string();
    PageResult::new(
        VStack::new_generic()
            .width(Length::Fill)
            .spacing(24.0)
            .padding(Padding {
                top: context.safe_area.top,
                right: 20.0,
                bottom: 24.0,
                left: 20.0,
            })
            .push(
                Text::<IcedBackend>::new(format!("Component: {}", name))
                    .large_title()
                    .bold(),
            )
            .push(
                Text::<IcedBackend>::new("This component is part of the PeakUI Standard Library.")
                    .title3()
                    .secondary(),
            )
            .push(Divider::<IcedBackend>::new())
            .push(
                Text::<IcedBackend>::new("Interactive Playground")
                    .title3()
                    .bold(),
            )
            .push(
                VStack::new_generic()
                    .spacing(20.0)
                    .push(
                        HStack::new_generic()
                            .spacing(12.0)
                            .align_y(Alignment::Center)
                            .push(Button::label("Primary Button"))
                            .push(Button::label("Secondary").variant(Variant::Outline))
                            .push(Button::label("Ghost").variant(Variant::Ghost)),
                    )
                    .push(
                        VStack::new_generic()
                            .spacing(12.0)
                            .push(Text::<IcedBackend>::new("Form Controls").callout().bold())
                            .push(Toggle::<Message, IcedBackend>::new(
                                "Example Toggle",
                                true,
                                |_| Message::ToggleSearch,
                            ))
                            .push(Slider::<Message, IcedBackend>::new(
                                0.0..=100.0,
                                50.0,
                                |_| Message::ToggleSearch,
                            )),
                    ),
            )
            .push(Divider::<IcedBackend>::new())
            .push(Text::<IcedBackend>::new("Best Practices").title3().bold())
            .push(
                Text::<IcedBackend>::new(
                    "Use consistent spacing and prioritize readability in your component layouts.",
                )
                .secondary(),
            ),
    )
}
