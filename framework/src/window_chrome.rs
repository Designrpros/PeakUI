use iced::widget::{button, container, row, text};
use iced::{Alignment, Color, Element, Length, Padding};

pub fn view<'a, Message>(
    title: String,
    content: Element<'a, Message>,
    on_close: Message,
    on_maximize: Option<Message>,
    is_light: bool,
) -> Element<'a, Message>
where
    Message: 'a + Clone,
{
    let title_bar = container(
        row![
            row![
                // Red (Close)
                Element::from(
                    button(window_control(Color::from_rgb8(255, 69, 58))) // MacOS Red
                        .on_press(on_close)
                        .padding(0)
                        .style(|_theme, _status| button::Style {
                            background: None,
                            ..Default::default()
                        })
                ),
                // Yellow (Minimize - Placeholder)
                window_control(Color::from_rgb8(255, 186, 10)), // MacOS Yellow
                // Green (Maximize)
                if let Some(msg) = on_maximize {
                    Element::from(
                        button(window_control(Color::from_rgb8(50, 215, 75))) // MacOS Green
                            .on_press(msg)
                            .padding(0)
                            .style(|_theme, _status| button::Style {
                                background: None,
                                ..Default::default()
                            }),
                    )
                } else {
                    Element::from(
                        container(window_control(Color::from_rgb8(50, 215, 75))).padding(0),
                    )
                }
            ]
            .spacing(10)
            .padding(Padding {
                left: 4.0,
                ..Padding::ZERO
            }),
            iced::widget::horizontal_space(),
            text(title.to_uppercase())
                .size(11)
                .font(iced::Font::DEFAULT)
                .color(if is_light {
                    Color::from_rgb(0.4, 0.4, 0.4)
                } else {
                    Color::from_rgb(0.6, 0.6, 0.6)
                }),
            iced::widget::horizontal_space(),
        ]
        .width(Length::Fill)
        .height(Length::Fixed(40.0))
        .align_y(Alignment::Center),
    )
    .padding(Padding {
        left: 16.0,
        right: 16.0,
        ..Padding::ZERO
    })
    .style({
        let bg = if is_light {
            Color::WHITE
        } else {
            Color::from_rgb8(40, 40, 40)
        };
        move |_| container::Style {
            background: Some(bg.into()),
            ..Default::default()
        }
    });
    let window_body = container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(0);

    container(iced::widget::column![title_bar, window_body])
        .style({
            let bg = if is_light {
                Color::WHITE
            } else {
                Color::from_rgb8(28, 28, 30)
            };
            let border_color = if is_light {
                Color::from_rgba(0.0, 0.0, 0.0, 0.1)
            } else {
                Color::from_rgba(1.0, 1.0, 1.0, 0.1)
            };
            let radius_val = if cfg!(target_arch = "wasm32") {
                0.0
            } else {
                8.0
            };
            let shadow = if cfg!(target_arch = "wasm32") {
                iced::Shadow::default()
            } else {
                iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 12.0,
                }
            };

            move |_| container::Style {
                background: Some(bg.into()),
                border: iced::Border {
                    color: border_color,
                    width: 1.0,
                    radius: radius_val.into(),
                },
                shadow,
                ..Default::default()
            }
        })
        .into()
}

fn window_control<'a, Message>(color: Color) -> Element<'a, Message>
where
    Message: 'a,
{
    container(iced::widget::text(""))
        .width(10)
        .height(10)
        .style({
            let bg_color = color;
            let radius_val = if cfg!(target_arch = "wasm32") {
                0.0
            } else {
                10.0
            };
            move |_| container::Style {
                background: Some(bg_color.into()),
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: radius_val.into(),
                },
                ..Default::default()
            }
        })
        .into()
}
