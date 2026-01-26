pub fn style_glass_card(is_light: bool) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(if is_light {
            iced::Color::from_rgba(1.0, 1.0, 1.0, 0.90).into()
        } else {
            iced::Color::from_rgba(0.1, 0.1, 0.1, 0.90).into()
        }),
        border: iced::Border {
            radius: 24.0.into(),
            width: 1.0,
            color: if is_light {
                iced::Color::from_rgba(1.0, 1.0, 1.0, 0.8)
            } else {
                iced::Color::from_rgba(1.0, 1.0, 1.0, 0.1)
            },
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            offset: iced::Vector::new(0.0, 20.0),
            blur_radius: 60.0,
        },
        ..Default::default()
    }
}

pub fn style_soft_input(
    status: iced::widget::text_input::Status,
    is_light: bool,
) -> iced::widget::text_input::Style {
    let base_bg = if is_light {
        iced::Color::from_rgba(0.96, 0.96, 0.97, 1.0) // #F5F5F7
    } else {
        iced::Color::from_rgba(1.0, 1.0, 1.0, 0.1)
    };

    let active = iced::widget::text_input::Style {
        background: base_bg.into(),
        border: iced::Border {
            radius: 12.0.into(),
            width: 0.0,
            color: iced::Color::TRANSPARENT,
        },
        icon: if is_light {
            iced::Color::BLACK
        } else {
            iced::Color::WHITE
        },
        placeholder: iced::Color::from_rgba(0.5, 0.5, 0.5, 1.0),
        value: if is_light {
            iced::Color::BLACK
        } else {
            iced::Color::WHITE
        },
        selection: iced::Color::from_rgba(0.0, 0.48, 1.0, 0.3),
    };

    if status == iced::widget::text_input::Status::Focused {
        iced::widget::text_input::Style {
            background: if is_light {
                iced::Color::WHITE.into()
            } else {
                iced::Color::from_rgba(0.2, 0.2, 0.2, 1.0).into()
            },
            border: iced::Border {
                width: 2.0,
                color: if is_light {
                    iced::Color::BLACK
                } else {
                    iced::Color::WHITE
                }, // Monochrome Focus
                radius: 12.0.into(),
            },
            ..active
        }
    } else {
        active
    }
}

pub fn style_pill_button(
    status: iced::widget::button::Status,
    is_light: bool,
) -> iced::widget::button::Style {
    let base_bg = if is_light {
        iced::Color::BLACK
    } else {
        iced::Color::WHITE
    };

    let hover_bg = if is_light {
        iced::Color::from_rgba(0.2, 0.2, 0.2, 1.0)
    } else {
        iced::Color::from_rgba(0.9, 0.9, 0.9, 1.0)
    };

    let text_color = if is_light {
        iced::Color::WHITE
    } else {
        iced::Color::BLACK
    };

    let bg = if status == iced::widget::button::Status::Hovered {
        hover_bg
    } else {
        base_bg
    };

    iced::widget::button::Style {
        background: Some(bg.into()),
        text_color,
        border: iced::Border {
            radius: 30.0.into(),
            width: 0.0,
            color: iced::Color::TRANSPARENT,
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.2), // Subtle shadow
            offset: iced::Vector::new(0.0, 4.0),
            blur_radius: 10.0,
        },
    }
}

pub fn style_secondary_button(
    _status: iced::widget::button::Status,
    is_light: bool,
) -> iced::widget::button::Style {
    let text_col = if is_light {
        iced::Color::from_rgb8(100, 100, 100)
    } else {
        iced::Color::from_rgb8(200, 200, 200)
    };
    iced::widget::button::Style {
        background: None,
        text_color: text_col,
        ..Default::default()
    }
}

pub const WAVEFORM_PINK: iced::Color = iced::Color::from_rgb(1.0, 0.2, 0.45);
pub const WAVEFORM_ACCENT: iced::Color = iced::Color::from_rgb(0.0, 1.0, 0.8);

pub fn style_jukebox_glass(is_light: bool) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(if is_light {
            iced::Color::from_rgba(1.0, 1.0, 1.0, 0.4).into()
        } else {
            iced::Color::from_rgba(0.0, 0.0, 0.0, 0.4).into()
        }),
        border: iced::Border {
            radius: 20.0.into(),
            width: 1.0,
            color: if is_light {
                iced::Color::from_rgba(0.0, 0.0, 0.0, 0.1)
            } else {
                iced::Color::from_rgba(1.0, 1.0, 1.0, 0.1)
            },
        },
        ..Default::default()
    }
}

pub fn style_genre_card(color: iced::Color) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(color.into()),
        border: iced::Border {
            radius: 16.0.into(),
            width: 0.0,
            color: iced::Color::TRANSPARENT,
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: iced::Vector::new(0.0, 4.0),
            blur_radius: 12.0,
        },
        ..Default::default()
    }
}
