use iced::widget::{button, column, container, text};
use iced::Element;

pub struct SystemAlert;

impl SystemAlert {
    pub fn view<'a, Message>(
        title: String,
        body: String,
        on_close: Message,
        is_light: bool,
    ) -> Element<'a, Message>
    where
        Message: 'a + Clone,
    {
        let (bg_color, text_color, btn_bg, btn_text) = if is_light {
            (
                iced::Color::from_rgb8(235, 235, 235), // Plain menu bar grey for Light
                iced::Color::BLACK,
                iced::Color::BLACK, // Opposite for button
                iced::Color::WHITE,
            )
        } else {
            (
                iced::Color::from_rgb8(40, 40, 40), // Plain menu bar grey for Dark
                iced::Color::WHITE,
                iced::Color::WHITE, // Opposite for button
                iced::Color::BLACK,
            )
        };

        let content = column![
            text(title).size(24).color(text_color),
            text(body).size(16).color(text_color),
            button(
                text("OK")
                    .color(btn_text)
                    .align_x(iced::alignment::Horizontal::Center)
            )
            .on_press(on_close)
            .padding([8, 24])
            .style({
                let b_text = btn_text;
                let b_bg = btn_bg;
                let radius_val = if cfg!(target_arch = "wasm32") {
                    0.0
                } else {
                    6.0
                };
                move |_, status| {
                    let opacity = if status == iced::widget::button::Status::Hovered {
                        0.8
                    } else {
                        1.0
                    };
                    iced::widget::button::Style {
                        background: Some(iced::Background::Color(iced::Color {
                            a: opacity,
                            ..b_bg
                        })),
                        text_color: b_text,
                        border: iced::Border {
                            radius: radius_val.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            })
        ]
        .spacing(20)
        .align_x(iced::Alignment::Center);

        container(content)
            .width(400)
            .padding(30)
            .style({
                let bg = bg_color;
                let text_col = text_color;
                let radius_val = if cfg!(target_arch = "wasm32") {
                    0.0
                } else {
                    12.0
                };
                let border_col = if is_light {
                    iced::Color::from_rgba8(0, 0, 0, 0.1)
                } else {
                    iced::Color::from_rgba8(255, 255, 255, 0.1)
                };
                let shadow = if cfg!(target_arch = "wasm32") {
                    iced::Shadow::default()
                } else {
                    iced::Shadow {
                        color: iced::Color::BLACK,
                        offset: iced::Vector::new(0.0, 4.0),
                        blur_radius: 16.0,
                    }
                };
                move |_| container::Style {
                    background: Some(iced::Background::Color(bg)),
                    border: iced::Border {
                        color: border_col,
                        width: 1.0,
                        radius: radius_val.into(),
                    },
                    text_color: Some(text_col),
                    shadow,
                }
            })
            .into()
    }
}
