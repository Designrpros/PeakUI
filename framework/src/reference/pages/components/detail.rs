
use crate::prelude::*;
use crate::reference::AppPageResult;
use crate::reference::app::Message;

pub fn view(name: &str, context: &Context, _is_mobile: bool) -> AppPageResult {
    let name = name.to_string();
    AppPageResult::new(
        vstack::<Message, IcedBackend>()
            .width(Length::Fill)
            .spacing(24.0)
            .padding(Padding {
                top: context.safe_area.top,
                right: 20.0,
                bottom: 24.0,
                left: 20.0,
            })
            .push(
                text::<IcedBackend>(format!("Component: {}", name))
                    .large_title()
                    .bold(),
            )
            .push(
                text::<IcedBackend>("This component is part of the PeakUI Standard Library.")
                    .title3()
                    .secondary(),
            )
            .push(divider::<IcedBackend>())
            .push(
                text::<IcedBackend>("Interactive Playground")
                    .title3()
                    .bold(),
            )
            .push(
                vstack::<Message, IcedBackend>()
                    .spacing(20.0)
                    .push(
                        hstack::<Message, IcedBackend>()
                            .spacing(12.0)
                            .align_y(Alignment::Center)
                            .push(button_label::<Message, IcedBackend>("Primary Button"))
                            .push(
                                button_label::<Message, IcedBackend>("Secondary")
                                    .variant(Variant::Outline),
                            )
                            .push(
                                button_label::<Message, IcedBackend>("Ghost")
                                    .variant(Variant::Ghost),
                            ),
                    )
                    .push(
                        vstack::<Message, IcedBackend>()
                            .spacing(12.0)
                            .push(text::<IcedBackend>("Form Controls").callout().bold())
                            .push(toggle::<Message, IcedBackend>(
                                "Example Toggle",
                                true,
                                |_| Message::ToggleSearch,
                            ))
                            .push(slider::<Message, IcedBackend>(0.0..=100.0, 50.0, |_| {
                                Message::ToggleSearch
                            })),
                    ),
            )
            .push(divider::<IcedBackend>())
            .push(text::<IcedBackend>("Best Practices").title3().bold())
            .push(
                text::<IcedBackend>(
                    "Use consistent spacing and prioritize readability in your component layouts.",
                )
                .secondary(),
            ),
    )
}
