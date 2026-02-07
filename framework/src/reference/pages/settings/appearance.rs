use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::Message;

pub fn view(_context: &Context, _is_mobile: bool) -> PageResult<Message> {
    PageResult::new(crate::core::ProxyView::new(move |context| {
        // 1. Pre-calculate the theme grid
        let mut theme_grid = VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .spacing(8.0);

        for chunk in peak_theme::PeakTheme::all().chunks(3) {
            let mut row = HStack::new_generic().width(Length::Fill).spacing(12.0);
            for theme in chunk {
                row = row.push(
                    Button::<Message>::label(theme.display_name())
                        .width(Length::Fill)
                        .on_press(Message::SetThemeKind(*theme))
                        .variant(Variant::Soft),
                );
            }
            theme_grid = theme_grid.push(row);
        }

        // 2. Build the main page
        VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .spacing(32.0)
            .padding(Padding {
                top: context.safe_area.top,
                right: 20.0,
                bottom: context.safe_area.bottom,
                left: 20.0,
            })
            // Hero Title
            .push(Text::<IcedBackend>::new("Settings").large_title().bold())
            .push(
                Text::<IcedBackend>::new("Customize your PeakOS experience.")
                    .title3()
                    .secondary(),
            )
            .push(Divider::<IcedBackend>::new())
            // 1. Appearance Section
            .push(
                crate::containers::Section::new(
                    "Appearance",
                    VStack::<Message, IcedBackend>::new_generic()
                        .width(Length::Fill)
                        .spacing(16.0)
                        .push(
                            Text::<IcedBackend>::new("Choose a theme that suits your environment.")
                                .body()
                                .secondary(),
                        )
                        .push(
                            HStack::<Message, IcedBackend>::new_generic()
                                .spacing(12.0)
                                .push(
                                    Button::<Message>::label("Light Mode")
                                        .variant(if context.theme.tone == ThemeTone::Light {
                                            Variant::Soft
                                        } else {
                                            Variant::Ghost
                                        })
                                        .on_press(Message::SetTheme(ThemeTone::Light)),
                                )
                                .push(
                                    Button::<Message>::label("Dark Mode")
                                        .variant(if context.theme.tone == ThemeTone::Dark {
                                            Variant::Soft
                                        } else {
                                            Variant::Ghost
                                        })
                                        .on_press(Message::SetTheme(ThemeTone::Dark)),
                                ),
                        ),
                )
                .width(Length::Fill),
            )
            .push(Divider::<IcedBackend>::new())
            // 2. Theme Selection
            .push(
                crate::containers::Section::new(
                    "Theme",
                    VStack::<Message, IcedBackend>::new_generic()
                        .width(Length::Fill)
                        .spacing(16.0)
                        .push(
                            Text::<IcedBackend>::new("Select a visual identity.")
                                .body()
                                .secondary(),
                        )
                        .push(theme_grid),
                )
                .width(Length::Fill),
            )
            // 3. Developer Reference
            .push(Divider::<IcedBackend>::new())
            .push(
                crate::containers::Section::new(
                    "Implementation",
                    VStack::new_generic()
                        .spacing(16.0)
                        .push(
                            Text::<IcedBackend>::new("How to handle theme switching in your app:")
                                .body()
                                .secondary(),
                        )
                        .push(
                            CodeBlock::rust(
                                r#"
                                    // 1. Handle the message in your Update loop
                                    match message {
                                        Message::SetTheme(tone) => {
                                            self.theme_tone = tone;
                                            // Navigation results automatically react 
                                            // to context.theme changes.
                                        }
                                    }

                                    // 2. Access tokens in your Root View
                                    let tokens = ThemeTokens::get(mode, self.theme_tone);
                                "#,
                            )
                            .on_copy(Message::CopyCode),
                        ),
                )
                .width(Length::Fill),
            )
            .view(context)
    }))
}
