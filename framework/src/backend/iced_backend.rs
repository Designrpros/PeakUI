use super::{Backend, TextSpan};
use crate::core::View;
use crate::style::{Context, Intent, Variant};
use iced::{widget::Id, Alignment, Color, Length, Padding, Renderer, Theme};
use nalgebra::Vector3;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Default)]
pub struct IcedBackend;

fn scale_length(l: Length, scale: f32) -> Length {
    match l {
        Length::Fixed(p) => Length::Fixed(p * scale),
        _ => l,
    }
}

impl Backend for IcedBackend {
    type AnyView<Message: 'static> = iced::Element<'static, Message, Theme, Renderer>;

    fn semantic_node<Message: 'static>(
        _node: crate::semantic::SemanticNode,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        iced::widget::container(iced::widget::Space::new().width(0).height(0)).into()
    }

    fn vstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_x: Alignment,
        align_y: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::{column, container};
        let scale = context.theme.scaling;

        let col = column(children)
            .spacing(spacing * scale)
            .width(scale_length(width, scale))
            .height(scale_length(height, scale))
            .align_x(align_x);

        let mut c = container(col).padding(Padding {
            top: padding.top * scale,
            right: padding.right * scale,
            bottom: padding.bottom * scale,
            left: padding.left * scale,
        });

        if align_x == Alignment::Center && width != Length::Shrink {
            c = c.center_x(scale_length(width, scale));
        } else if align_x == Alignment::End && width != Length::Shrink {
            c = c.align_x(iced::alignment::Horizontal::Right);
        }

        if align_y == Alignment::Center && height != Length::Shrink {
            c = c.center_y(scale_length(height, scale));
        } else if align_y == Alignment::End && height != Length::Shrink {
            c = c.align_y(iced::alignment::Vertical::Bottom);
        }

        c.width(scale_length(width, scale))
            .height(scale_length(height, scale))
            .into()
    }

    fn hstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_x: Alignment,
        align_y: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::{container, row};
        let scale = context.theme.scaling;

        let r = row(children)
            .spacing(spacing * scale)
            .width(scale_length(width, scale))
            .height(scale_length(height, scale))
            .align_y(align_y);

        let mut c = container(r).padding(Padding {
            top: padding.top * scale,
            right: padding.right * scale,
            bottom: padding.bottom * scale,
            left: padding.left * scale,
        });

        if align_x == Alignment::Center && width != Length::Shrink {
            c = c.center_x(scale_length(width, scale));
        } else if align_x == Alignment::End && width != Length::Shrink {
            c = c.align_x(iced::alignment::Horizontal::Right);
        }

        if align_y == Alignment::Center && height != Length::Shrink {
            c = c.center_y(scale_length(height, scale));
        } else if align_y == Alignment::End && height != Length::Shrink {
            c = c.align_y(iced::alignment::Vertical::Bottom);
        }

        c.width(scale_length(width, scale))
            .height(scale_length(height, scale))
            .into()
    }

    fn wrap<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        _run_spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_x: Alignment,
        align_y: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::{container, row};
        let scale = context.theme.scaling;

        let w = row(children)
            .spacing(spacing * scale)
            .width(scale_length(width, scale))
            .height(scale_length(height, scale))
            .align_y(align_y)
            .wrap();

        let mut c = container(w).padding(Padding {
            top: padding.top * scale,
            right: padding.right * scale,
            bottom: padding.bottom * scale,
            left: padding.left * scale,
        });

        if align_x == Alignment::Center && width != Length::Shrink {
            c = c.center_x(scale_length(width, scale));
        } else if align_x == Alignment::End && width != Length::Shrink {
            c = c.align_x(iced::alignment::Horizontal::Right);
        }

        c.width(scale_length(width, scale))
            .height(scale_length(height, scale))
            .into()
    }

    fn rich_text<Message: Clone + 'static>(
        spans: Vec<TextSpan>,
        size: f32,
        width: Length,
        alignment: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::advanced::text::Span;
        use iced::widget::rich_text;

        let scale = context.theme.scaling;
        let scaled_size = size * scale;

        let iced_spans: Vec<Span<'static, ()>> = spans
            .into_iter()
            .map(|s| {
                let mut span = Span::new(s.content.to_string());
                if let Some(c) = s.color {
                    span = span.color(c);
                }
                if let Some(f) = s.font {
                    span = span.font(f);
                }
                if let Some(sz) = s.size {
                    span = span.size(sz * scale);
                }
                span
            })
            .collect();

        rich_text(iced_spans)
            .size(scaled_size)
            .width(width)
            .align_x(alignment)
            .into()
    }

    fn text<Message: Clone + 'static>(
        content: String,
        size: f32,
        color: Option<Color>,
        is_bold: bool,
        is_dim: bool,
        intent: Option<Intent>,
        font: Option<iced::Font>,
        width: Length,
        alignment: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::text;

        let base_color = color.unwrap_or_else(|| {
            let theme = &context.theme;
            if let Some(i) = intent {
                match i {
                    Intent::Primary => theme.colors.primary,
                    Intent::Secondary => theme.colors.secondary,
                    Intent::Accent => theme.colors.accent,
                    Intent::Success => theme.colors.success,
                    Intent::Warning => theme.colors.warning,
                    Intent::Danger => theme.colors.danger,
                    Intent::Info => theme.colors.info,
                    Intent::Neutral => theme.colors.text_primary,
                }
            } else if is_dim {
                context.theme.colors.text_secondary
            } else {
                context
                    .foreground
                    .unwrap_or(context.theme.colors.text_primary)
            }
        });

        let mut text_color = base_color;
        if is_dim {
            text_color.a *= 0.8;
        }

        let mut base_font = font.unwrap_or(iced::Font {
            family: iced::font::Family::Name("Fira Sans"),
            ..iced::Font::DEFAULT
        });

        if is_bold {
            base_font.weight = iced::font::Weight::Bold;
        } else {
            base_font.weight = iced::font::Weight::Normal;
        }

        let scaled_size = size * context.theme.scaling;

        #[cfg(target_arch = "wasm32")]
        let t = text(content).shaping(iced::widget::text::Shaping::Advanced);
        #[cfg(not(target_arch = "wasm32"))]
        let t = text(content);

        t.size(scaled_size)
            .color(text_color)
            .font(base_font)
            .width(width)
            .align_x(alignment)
            .align_y(iced::alignment::Vertical::Center)
            .into()
    }

    fn icon<Message: Clone + 'static>(
        name: String,
        size: f32,
        color: Option<Color>,
        context: &Context,
    ) -> Self::AnyView<Message> {
        let theme = &context.theme;
        let final_color = color.unwrap_or(theme.colors.text_primary);

        let mut scale = context.theme.scaling;
        if scale <= 0.0 {
            scale = 1.0;
        }
        let scaled_size = size * scale;

        let hex_color = format!(
            "#{:02X}{:02X}{:02X}",
            (final_color.r * 255.0) as u8,
            (final_color.g * 255.0) as u8,
            (final_color.b * 255.0) as u8
        );

        if let Some(svg_data) = peak_icons::get_icon(&name) {
            let colored_svg = svg_data
                .replace("currentColor", &hex_color)
                .replace("fill=\"#000000\"", &format!("fill=\"{}\"", hex_color))
                .replace("fill=\"black\"", &format!("fill=\"{}\"", hex_color));

            return iced::widget::svg(iced::widget::svg::Handle::from_memory(
                colored_svg.into_bytes(),
            ))
            .width(scaled_size)
            .height(scaled_size)
            .into();
        }

        if let Some(icon) = crate::assets::SystemIcon::from_name(&name) {
            let path = crate::assets::Asset::Icon(icon).path();
            return iced::widget::svg(iced::widget::svg::Handle::from_path(path))
                .width(scaled_size)
                .height(scaled_size)
                .into();
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let handle = peak_core::icons::get_ui_icon(&name, &hex_color);
            iced::widget::svg(handle)
                .width(scaled_size)
                .height(scaled_size)
                .into()
        }

        #[cfg(target_arch = "wasm32")]
        {
            let path = format!("assets/icons/system/ui/{}.svg", name);
            iced::widget::svg(iced::widget::svg::Handle::from_path(path))
                .width(size)
                .height(size)
                .into()
        }
    }

    fn divider<Message: 'static>(context: &Context) -> Self::AnyView<Message> {
        use iced::widget::container;
        let divider_color = context.theme.colors.divider;
        container(iced::widget::Space::new().height(1).width(Length::Fill))
            .style(move |_| container::Style {
                background: Some(divider_color.into()),
                ..Default::default()
            })
            .into()
    }

    fn space<Message: 'static>(
        width: Length,
        height: Length,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        iced::widget::Space::new()
            .width(width)
            .height(height)
            .into()
    }

    fn circle<Message: 'static>(
        radius: f32,
        color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::container;
        container(
            iced::widget::Space::new()
                .width(Length::Fixed(radius * 2.0))
                .height(Length::Fixed(radius * 2.0)),
        )
        .width(radius * 2.0)
        .height(radius * 2.0)
        .style(move |_| container::Style {
            background: color.map(iced::Background::Color),
            border: iced::Border {
                radius: radius.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    }

    fn arc<Message: 'static>(
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        color: Option<Color>,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::canvas::{Canvas, Frame, Geometry, Path, Program};

        struct ArcProgram {
            radius: f32,
            start_angle: f32,
            end_angle: f32,
            color: Color,
        }

        impl<Message> Program<Message, Theme, Renderer> for ArcProgram {
            type State = ();

            fn draw(
                &self,
                _state: &Self::State,
                renderer: &Renderer,
                _theme: &Theme,
                bounds: iced::Rectangle,
                _cursor: iced::mouse::Cursor,
            ) -> Vec<Geometry> {
                let mut frame = Frame::new(renderer, bounds.size());
                let center = iced::Point::new(bounds.width / 2.0, bounds.height / 2.0);
                let path = Path::new(|p| {
                    p.move_to(center);
                    p.arc(iced::widget::canvas::path::Arc {
                        center,
                        radius: self.radius,
                        start_angle: iced::Radians(self.start_angle),
                        end_angle: iced::Radians(self.end_angle),
                    });
                    p.line_to(center);
                    p.close();
                });

                frame.fill(&path, self.color);
                vec![frame.into_geometry()]
            }
        }

        let scale = context.theme.scaling;
        Canvas::new(ArcProgram {
            radius: radius * scale,
            start_angle,
            end_angle,
            color: color.unwrap_or(Color::BLACK),
        })
        .width(Length::Fixed(radius * 2.0 * scale))
        .height(Length::Fixed(radius * 2.0 * scale))
        .into()
    }

    fn path<Message: 'static>(
        points: Vec<iced::Point>,
        color: Option<Color>,
        width: f32,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::canvas::{Canvas, Frame, Geometry, Path, Program, Stroke};

        struct PathProgram {
            points: Vec<iced::Point>,
            color: Color,
            width: f32,
        }

        impl<Message> Program<Message, Theme, Renderer> for PathProgram {
            type State = ();

            fn draw(
                &self,
                _state: &Self::State,
                renderer: &Renderer,
                _theme: &Theme,
                bounds: iced::Rectangle,
                _cursor: iced::mouse::Cursor,
            ) -> Vec<Geometry> {
                let mut frame = Frame::new(renderer, bounds.size());
                if self.points.len() < 2 {
                    return vec![];
                }

                let path = Path::new(|p| {
                    p.move_to(self.points[0]);
                    for i in 1..self.points.len() {
                        p.line_to(self.points[i]);
                    }
                });

                frame.stroke(
                    &path,
                    Stroke {
                        style: iced::widget::canvas::Style::Solid(self.color),
                        width: self.width,
                        line_cap: iced::widget::canvas::LineCap::Round,
                        line_join: iced::widget::canvas::LineJoin::Round,
                        ..Default::default()
                    },
                );
                vec![frame.into_geometry()]
            }
        }

        let scale = context.theme.scaling;
        Canvas::new(PathProgram {
            points,
            color: color.unwrap_or(Color::BLACK),
            width: width * scale,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn capsule<Message: 'static>(
        width: Length,
        height: Length,
        color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::container;
        container(iced::widget::Space::new().width(width).height(height))
            .width(width)
            .height(height)
            .style(move |_| container::Style {
                background: color.map(iced::Background::Color),
                border: iced::Border {
                    radius: 1000.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
    }

    fn rectangle<Message: 'static>(
        width: Length,
        height: Length,
        color: Option<Color>,
        radius: f32,
        border_width: f32,
        border_color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::container;

        container(
            iced::widget::Space::new()
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(width)
        .height(height)
        .style({
            let b_color = border_color.unwrap_or(Color::TRANSPARENT);
            move |_| container::Style {
                background: color.map(iced::Background::Color),
                border: iced::Border {
                    color: b_color,
                    width: border_width,
                    radius: if cfg!(target_arch = "wasm32") {
                        0.0
                    } else {
                        radius
                    }
                    .into(),
                },
                ..Default::default()
            }
        })
        .into()
    }

    fn button<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        on_press: Option<Message>,
        variant: Variant,
        intent: Intent,
        width: Length,
        height: Length,
        is_compact: bool,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::button;
        let theme = &context.theme;

        if variant == Variant::Plain {
            return button(content)
                .on_press_maybe(on_press)
                .padding(Padding::ZERO)
                .style(move |_, _| button::Style {
                    background: None,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    ..Default::default()
                })
                .into();
        }

        let b = button(
            iced::widget::container(content)
                .width(width)
                .height(Length::Fill)
                .center_x(width)
                .center_y(Length::Fill),
        )
        .on_press_maybe(on_press)
        .padding(if variant == Variant::Compact {
            Padding::ZERO
        } else {
            Padding::from([0, 16])
        })
        .height(if height != Length::Shrink {
            height
        } else if variant == Variant::Compact {
            Length::Shrink
        } else {
            Length::Fixed(if is_compact {
                32.0 * theme.scaling
            } else {
                44.0 * theme.scaling
            })
        })
        .style({
            let theme = theme.clone();
            move |_, status| {
                let color = match intent {
                    Intent::Primary => theme.colors.primary,
                    Intent::Secondary => theme.colors.secondary,
                    Intent::Accent => theme.colors.accent,
                    Intent::Success => theme.colors.success,
                    Intent::Warning => theme.colors.warning,
                    Intent::Danger => theme.colors.danger,
                    Intent::Info => theme.colors.info,
                    Intent::Neutral => theme.colors.surface,
                };

                let text_color = match intent {
                    Intent::Accent => theme.colors.on_accent,
                    Intent::Secondary => theme.colors.on_secondary,
                    _ => theme.colors.on_primary,
                };

                match variant {
                    Variant::Solid => button::Style {
                        background: Some(if status == button::Status::Hovered {
                            let mut c = color;
                            c.a = 0.8;
                            c.into()
                        } else {
                            color.into()
                        }),
                        text_color: text_color,
                        border: iced::Border {
                            radius: 32.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Variant::Soft => button::Style {
                        background: Some({
                            let mut c = color;
                            c.a = 0.1;
                            if status == button::Status::Hovered {
                                c.a = 0.2;
                            }
                            c.into()
                        }),
                        text_color: color,
                        border: iced::Border {
                            radius: 32.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Variant::Outline => button::Style {
                        background: if status == button::Status::Hovered {
                            let mut c = color;
                            c.a = 0.05;
                            Some(c.into())
                        } else {
                            None
                        },
                        text_color: color,
                        border: iced::Border {
                            color,
                            width: 1.0,
                            radius: 32.0.into(),
                        },
                        ..Default::default()
                    },
                    Variant::Ghost => button::Style {
                        background: if status == button::Status::Hovered {
                            let mut c = color;
                            c.a = 0.1;
                            Some(c.into())
                        } else {
                            None
                        },
                        text_color: color,
                        border: iced::Border {
                            radius: 32.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Variant::Compact => button::Style {
                        background: None,
                        text_color: color,
                        border: iced::Border {
                            width: 0.0,
                            radius: 0.0.into(),
                            color: iced::Color::TRANSPARENT,
                        },
                        ..Default::default()
                    },
                    Variant::Plain => button::Style {
                        background: None,
                        border: iced::Border::default(),
                        ..Default::default()
                    },
                }
            }
        });

        b.into()
    }

    fn sidebar_item<Message: Clone + Send + Sync + 'static>(
        title: String,
        icon: String,
        is_selected: bool,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use crate::atoms::{Icon, Text};
        use crate::layout::HStack;
        use iced::widget::container;

        let theme = &context.theme;
        let content = HStack::<Message, Self>::new_generic()
            .spacing(12.0)
            .padding(iced::Padding {
                top: 8.0,
                right: 12.0,
                bottom: 8.0,
                left: 12.0,
            })
            .align_y(iced::Alignment::Center)
            .push(Icon::<Self>::new(icon).size(18.0))
            .push(Text::<Self>::new(title).body().bold());

        if is_selected {
            container(content.view(context))
                .style({
                    let bg_color = theme.colors.primary;
                    let radius_val = if cfg!(target_arch = "wasm32") {
                        0.0
                    } else {
                        8.0
                    };
                    move |_theme| container::Style {
                        background: Some(bg_color.into()),
                        border: iced::Border {
                            radius: radius_val.into(),
                            ..Default::default()
                        },
                        text_color: Some(iced::Color::WHITE),
                        ..Default::default()
                    }
                })
                .width(Length::Fill)
                .into()
        } else {
            content.view(context)
        }
    }

    fn text_input<Message: Clone + 'static>(
        value: String,
        placeholder: String,
        on_change: impl Fn(String) -> Message + 'static,
        on_submit: Option<Message>,
        font: Option<iced::Font>,
        is_secure: bool,
        variant: Variant,
        id: Option<iced::widget::Id>,
        context: &Context,
    ) -> Self::AnyView<Message> {
        let mut input = iced::widget::text_input(&placeholder, &value)
            .on_input(on_change)
            .secure(is_secure);

        if let Some(id_val) = id {
            input = input.id(id_val);
        }

        if let Some(msg) = on_submit {
            input = input.on_submit(msg);
        }

        if let Some(font_val) = font {
            input = input.font(font_val);
        }

        input = match variant {
            Variant::Ghost => {
                let colors = context.theme.colors;
                input.style(move |_theme, _status| iced::widget::text_input::Style {
                    background: iced::Background::Color(iced::Color::TRANSPARENT),
                    border: iced::Border::default(),
                    icon: colors.text_secondary,
                    placeholder: colors.text_secondary,
                    value: colors.text_primary,
                    selection: colors.primary,
                })
            }
            _ => input,
        };

        input.padding(10).into()
    }

    fn slider<Message: Clone + 'static>(
        range: std::ops::RangeInclusive<f32>,
        value: f32,
        on_change: impl Fn(f32) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        iced::widget::slider(range, value, on_change).into()
    }

    fn toggle<Message: Clone + 'static>(
        label: String,
        is_active: bool,
        on_toggle: impl Fn(bool) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        iced::widget::toggler(is_active)
            .label(label)
            .on_toggle(on_toggle)
            .into()
    }

    fn zstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        width: Length,
        height: Length,
        alignment: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::container;

        let iced_alignment = match alignment {
            Alignment::Center => iced::Alignment::Center,
            Alignment::Start => iced::Alignment::Start,
            Alignment::End => iced::Alignment::End,
        };

        let aligned_children: Vec<Self::AnyView<Message>> = children
            .into_iter()
            .map(|child| {
                container(child)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(iced_alignment)
                    .align_y(iced_alignment)
                    .into()
            })
            .collect();

        iced::widget::stack(aligned_children)
            .width(width)
            .height(height)
            .into()
    }

    fn grid<Message: 'static>(
        mut children: Vec<Self::AnyView<Message>>,
        columns: usize,
        spacing: f32,
        context: &Context,
    ) -> Self::AnyView<Message> {
        let scale = context.theme.scaling;
        let scaled_spacing = spacing * scale;

        if columns == 0 {
            return iced::widget::column(children)
                .spacing(scaled_spacing)
                .into();
        }
        let mut rows = Vec::new();
        while !children.is_empty() {
            let chunk: Vec<_> = children
                .drain(0..std::cmp::min(columns, children.len()))
                .map(|child| iced::widget::container(child).width(Length::Fill).into())
                .collect();
            rows.push(
                iced::widget::row(chunk)
                    .spacing(scaled_spacing)
                    .width(Length::Fill)
                    .into(),
            );
        }
        iced::widget::column(rows)
            .spacing(scaled_spacing)
            .width(Length::Fill)
            .into()
    }

    fn image<Message: 'static>(
        path: impl Into<String>,
        width: Length,
        height: Length,
        radius: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let p: String = path.into();

        #[cfg(target_arch = "wasm32")]
        {
            use iced::widget::container;
            use iced::widget::image as iced_image;

            match wasm_portal::get_image(p) {
                wasm_portal::ImageState::Loaded(handle) => container(
                    iced_image(handle)
                        .width(width)
                        .height(height)
                        .content_fit(iced::ContentFit::Cover),
                )
                .style(move |_| container::Style {
                    border: iced::Border {
                        radius: radius.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .into(),
                wasm_portal::ImageState::Loading => {
                    container(iced::widget::Space::new().width(width).height(height))
                        .style(move |_| container::Style {
                            background: Some(iced::Background::Color(Color::from_rgba(
                                1.0, 1.0, 1.0, 0.1,
                            ))),
                            border: iced::Border {
                                radius: radius.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .into()
                }
                wasm_portal::ImageState::Error => {
                    container(iced::widget::Space::new().width(width).height(height))
                        .style(move |_| container::Style {
                            background: Some(iced::Background::Color(Color::from_rgb(
                                1.0, 0.0, 0.0,
                            ))),
                            border: iced::Border {
                                radius: radius.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .into()
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            use iced::widget::container;

            let path_str = if p.starts_with('/') {
                p[1..].to_string()
            } else {
                p
            };

            let handle = iced::widget::image::Handle::from_path(path_str);

            container(
                iced::widget::image(handle)
                    .width(width)
                    .height(height)
                    .content_fit(iced::ContentFit::Cover),
            )
            .style(move |_| container::Style {
                border: iced::Border {
                    radius: radius.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
        }
    }

    fn video<Message: 'static>(
        path: impl Into<String>,
        width: Length,
        height: Length,
        _radius: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let _p: String = path.into();
        iced::widget::Space::new()
            .width(width)
            .height(height)
            .into()
    }

    fn web_view<Message: 'static>(
        url: String,
        width: Length,
        height: Length,
        radius: f32,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        #[cfg(target_arch = "wasm32")]
        {
            return wasm_portal::WebView::new(url, width, height, radius).into();
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            use iced::widget::{column, container, text};
            container(
                column![
                    text("Native Web Support Not Supported")
                        .size(16.0)
                        .color(Color::WHITE),
                    text(url).size(12.0).color(Color::WHITE.scale_alpha(0.5))
                ]
                .spacing(8)
                .align_x(Alignment::Center),
            )
            .width(width)
            .height(height)
            .center_x(width)
            .center_y(height)
            .style(move |_| container::Style {
                background: Some(Color::BLACK.into()),
                border: iced::Border {
                    radius: radius.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
        }
    }

    fn container<Message: 'static>(
        content: Self::AnyView<Message>,
        padding: Padding,
        width: Length,
        height: Length,
        background: Option<Color>,
        radius: f32,
        border_width: f32,
        border_color: Option<Color>,
        shadow: Option<iced::Shadow>,
        align_x: Alignment,
        align_y: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::container;
        let scale = context.theme.scaling;

        let mut c = container(content)
            .padding(Padding {
                top: padding.top * scale,
                right: padding.right * scale,
                bottom: padding.bottom * scale,
                left: padding.left * scale,
            })
            .width(scale_length(width, scale))
            .height(scale_length(height, scale))
            .style(move |_| container::Style {
                background: background.map(iced::Background::Color),
                border: iced::Border {
                    radius: (radius * scale).into(),
                    width: border_width * scale,
                    color: border_color.unwrap_or(iced::Color::TRANSPARENT),
                },
                shadow: shadow.unwrap_or_default(),
                ..Default::default()
            });

        if align_x == Alignment::Center && width != Length::Shrink {
            c = c.center_x(scale_length(width, scale));
        } else if align_x == Alignment::End && width != Length::Shrink {
            c = c.align_x(iced::alignment::Horizontal::Right);
        }

        if align_y == Alignment::Center && height != Length::Shrink {
            c = c.center_y(scale_length(height, scale));
        } else if align_y == Alignment::End && height != Length::Shrink {
            c = c.align_y(iced::alignment::Vertical::Bottom);
        }

        c.into()
    }

    fn scroll_view<Message: 'static>(
        content: Self::AnyView<Message>,
        width: Length,
        height: Length,
        id: Option<&'static str>,
        show_indicators: bool,
        direction: crate::style::ScrollDirection,
        context: &Context,
    ) -> Self::AnyView<Message> {
        let text_color = context.theme.colors.text_primary;

        let final_height = if context.is_inside_scrollable
            && height == Length::Fill
            && direction != crate::style::ScrollDirection::Horizontal
        {
            Length::Fixed(300.0)
        } else {
            height
        };

        let mut scroll = iced::widget::scrollable(content)
            .width(width)
            .height(final_height)
            .direction(match direction {
                crate::style::ScrollDirection::Vertical => {
                    iced::widget::scrollable::Direction::Vertical(
                        iced::widget::scrollable::Scrollbar::default()
                            .width(4.0)
                            .margin(0.0)
                            .scroller_width(4.0),
                    )
                }
                crate::style::ScrollDirection::Horizontal => {
                    iced::widget::scrollable::Direction::Horizontal(
                        iced::widget::scrollable::Scrollbar::default()
                            .width(4.0)
                            .margin(0.0)
                            .scroller_width(4.0),
                    )
                }
                crate::style::ScrollDirection::Both => iced::widget::scrollable::Direction::Both {
                    vertical: iced::widget::scrollable::Scrollbar::default()
                        .width(4.0)
                        .margin(0.0)
                        .scroller_width(4.0),
                    horizontal: iced::widget::scrollable::Scrollbar::default()
                        .width(4.0)
                        .margin(0.0)
                        .scroller_width(4.0),
                },
            });

        scroll = scroll.style(move |_, status| {
            if !show_indicators {
                iced::widget::scrollable::Style {
                    container: iced::widget::container::Style::default(),
                    vertical_rail: iced::widget::scrollable::Rail {
                        background: None,
                        border: iced::Border::default(),
                        scroller: iced::widget::scrollable::Scroller {
                            background: iced::Color::TRANSPARENT.into(),
                            border: iced::Border::default(),
                        },
                    },
                    horizontal_rail: iced::widget::scrollable::Rail {
                        background: None,
                        border: iced::Border::default(),
                        scroller: iced::widget::scrollable::Scroller {
                            background: iced::Color::TRANSPARENT.into(),
                            border: iced::Border::default(),
                        },
                    },
                    gap: None,
                    auto_scroll: iced::widget::scrollable::AutoScroll {
                        background: iced::Background::Color(iced::Color::TRANSPARENT),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                        icon: iced::Color::TRANSPARENT,
                    },
                }
            } else {
                let scroller_alpha = match status {
                    iced::widget::scrollable::Status::Hovered { .. } => 0.4,
                    iced::widget::scrollable::Status::Dragged { .. } => 0.6,
                    _ => 0.15,
                };

                iced::widget::scrollable::Style {
                    container: iced::widget::container::Style::default(),
                    vertical_rail: iced::widget::scrollable::Rail {
                        background: None,
                        border: iced::Border::default(),
                        scroller: iced::widget::scrollable::Scroller {
                            background: iced::Color {
                                a: scroller_alpha,
                                ..text_color
                            }
                            .into(),
                            border: iced::Border {
                                radius: 2.0.into(),
                                width: 0.0,
                                ..Default::default()
                            },
                        },
                    },
                    horizontal_rail: iced::widget::scrollable::Rail {
                        background: None,
                        border: iced::Border::default(),
                        scroller: iced::widget::scrollable::Scroller {
                            background: iced::Color {
                                a: scroller_alpha,
                                ..text_color
                            }
                            .into(),
                            border: iced::Border {
                                radius: 2.0.into(),
                                width: 0.0,
                                ..Default::default()
                            },
                        },
                    },
                    gap: None,
                    auto_scroll: iced::widget::scrollable::AutoScroll {
                        background: iced::Background::Color(iced::Color::TRANSPARENT),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                        icon: iced::Color::TRANSPARENT,
                    },
                }
            }
        });

        if let Some(id_val) = id {
            scroll = scroll.id(Id::new(id_val));
        }
        scroll.into()
    }

    fn mouse_area<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        on_move: Option<Arc<dyn Fn(iced::Point) -> Message + Send + Sync>>,
        on_press: Option<Message>,
        on_release: Option<Message>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::mouse_area;
        let mut area = mouse_area(content);
        if let Some(f) = on_move {
            let f_clone = f.clone();
            area = area.on_move(move |p| f_clone(p));
        }
        if let Some(msg) = on_press {
            area = area.on_press(msg);
        }
        if let Some(msg) = on_release {
            area = area.on_release(msg);
        }
        area.into()
    }

    fn with_tooltip<Message: 'static>(
        content: Self::AnyView<Message>,
        tooltip_text: Arc<str>,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::tooltip;
        let scale = context.theme.scaling;

        tooltip(
            content,
            iced::widget::text(tooltip_text.to_string()).size(14.0 * scale),
            tooltip::Position::Bottom,
        )
        .into()
    }

    fn glass_card<Message: 'static>(
        content: Self::AnyView<Message>,
        padding: Padding,
        width: Length,
        height: Length,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::container;
        let theme = &context.theme;
        let mut bg = theme.colors.surface;
        bg.a = theme.glass_opacity;

        let shadow = context.shadow(
            theme.colors.divider,        // Fallback shadow color if not specified
            iced::Vector::new(0.0, 4.0), // Default offset
            8.0,                         // Default blur
        );

        let scale = theme.scaling;
        let radius = context.radius(theme.radius * scale);
        let padding = Padding {
            top: padding.top * scale,
            right: padding.right * scale,
            bottom: padding.bottom * scale,
            left: padding.left * scale,
        };

        let scaled_width = scale_length(width, scale);
        let scaled_height = scale_length(height, scale);

        let mut c = container(content)
            .padding(padding)
            .width(scaled_width)
            .height(scaled_height);

        if height != Length::Shrink {
            c = c.center_y(scaled_height);
        }

        c.style(move |_| container::Style {
            background: Some(bg.into()),
            border: iced::Border {
                radius,
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.15),
                width: 1.0 * scale,
            },
            shadow,
            ..Default::default()
        })
        .into()
    }

    fn section<Message: 'static>(
        title: String,
        content: Self::AnyView<Message>,
        width: Length,
        height: Length,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::{column, text};
        let scale = context.theme.scaling;

        column![
            text(title.to_uppercase())
                .size(10.0 * scale)
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..iced::Font::DEFAULT
                })
                .color(context.theme.colors.text_secondary),
            content
        ]
        .spacing(8.0 * scale)
        .width(scale_length(width, scale))
        .height(scale_length(height, scale))
        .into()
    }

    fn spatial_modifier<Message: 'static>(
        content: Self::AnyView<Message>,
        _position: Vector3<f32>,
        _scale: Vector3<f32>,
        _rotation: Vector3<f32>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        content
    }
}

#[cfg(target_arch = "wasm32")]
pub mod wasm_portal {
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget};
    use iced::widget::image::Handle;
    use iced::{Element, Length, Rectangle, Size, Theme};
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    pub struct WebView {
        url: String,
        width: Length,
        height: Length,
        radius: f32,
        id: u64,
    }

    impl WebView {
        pub fn new(url: String, width: Length, height: Length, radius: f32) -> Self {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            url.hash(&mut hasher);
            let id = hasher.finish();

            Self {
                url,
                width,
                height,
                radius,
                id,
            }
        }
    }

    impl<Message, Renderer> Widget<Message, Theme, Renderer> for WebView
    where
        Renderer: renderer::Renderer,
    {
        fn size(&self) -> Size<Length> {
            Size::new(self.width, self.height)
        }

        fn layout(
            &mut self,
            _tree: &mut widget::Tree,
            _renderer: &Renderer,
            limits: &layout::Limits,
        ) -> layout::Node {
            layout::Node::new(limits.resolve(self.width, self.height, Size::ZERO))
        }

        fn draw(
            &self,
            _tree: &widget::Tree,
            _renderer: &mut Renderer,
            _theme: &Theme,
            _style: &renderer::Style,
            layout: Layout<'_>,
            _cursor: iced::mouse::Cursor,
            _viewport: &Rectangle,
        ) {
            use wasm_bindgen::JsCast;
            use web_sys::{window, HtmlIFrameElement};

            let bounds = layout.bounds();
            let element_id = format!("peakui-webview-{}", self.id);

            let window = window().unwrap();
            let document = window.document().unwrap();

            // Try to find existing iframe
            let element = document.get_element_by_id(&element_id);

            let iframe = if let Some(el) = element {
                el.dyn_into::<HtmlIFrameElement>().unwrap()
            } else {
                // Create new iframe
                let iframe = document
                    .create_element("iframe")
                    .unwrap()
                    .dyn_into::<HtmlIFrameElement>()
                    .unwrap();

                iframe.set_id(&element_id);
                iframe.set_src(&self.url);
                iframe.set_attribute("allow", "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture").unwrap();
                iframe.set_attribute("allowfullscreen", "true").unwrap();
                iframe.set_attribute("frameborder", "0").unwrap();

                // Base styles
                let style = iframe.style();
                style.set_property("position", "absolute").unwrap();
                style.set_property("border", "none").unwrap();
                style.set_property("z-index", "1000").unwrap(); // Ensure it's on top
                style.set_property("pointer-events", "auto").unwrap();
                style.set_property("visibility", "visible").unwrap();
                style.set_property("opacity", "1").unwrap();

                document.body().unwrap().append_child(&iframe).unwrap();
                iframe
            };

            // Update heartbeat
            iframe
                .set_attribute("data-last-updated", &js_sys::Date::now().to_string())
                .unwrap();

            // Update position and size
            let style = iframe.style();
            style
                .set_property("left", &format!("{}px", bounds.x))
                .unwrap();
            style
                .set_property("top", &format!("{}px", bounds.y))
                .unwrap();
            style
                .set_property("width", &format!("{}px", bounds.width))
                .unwrap();
            style
                .set_property("height", &format!("{}px", bounds.height))
                .unwrap();
            style
                .set_property("border-radius", &format!("{}px", self.radius))
                .unwrap();

            // Handle visibility (if bounds are zero or outside viewport, we could hide it)
            if bounds.width <= 0.0 || bounds.height <= 0.0 {
                style.set_property("display", "none").unwrap();
            } else {
                style.set_property("display", "block").unwrap();
            }
        }
    }

    impl<'a, Message, Renderer> From<WebView> for Element<'a, Message, Theme, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        fn from(webview: WebView) -> Self {
            Self::new(webview)
        }
    }

    #[derive(Clone)]
    pub enum ImageState {
        Loading,
        Loaded(Handle),
        Error,
    }

    static IMAGE_CACHE: Lazy<Arc<Mutex<HashMap<String, ImageState>>>> =
        Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

    pub fn get_image(path: String) -> ImageState {
        let mut cache = IMAGE_CACHE.lock().unwrap();

        if let Some(state) = cache.get(&path) {
            return state.clone();
        }

        // Not in cache, start loading
        cache.insert(path.clone(), ImageState::Loading);

        let path_clone = path.clone();
        use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
            let url = if path_clone.starts_with("http") {
                path_clone.clone()
            } else {
                let win = web_sys::window().unwrap();
                let origin = win.location().origin().unwrap_or_default();
                let clean = if path_clone.starts_with('/') {
                    &path_clone[1..]
                } else {
                    &path_clone
                };
                format!("{}/{}", origin, clean)
            };

            match reqwest::get(&url).await {
                Ok(resp) => {
                    if let Ok(bytes) = resp.bytes().await {
                        let handle = Handle::from_bytes(bytes);
                        let mut cache = IMAGE_CACHE.lock().unwrap();
                        cache.insert(path_clone, ImageState::Loaded(handle));
                    } else {
                        let mut cache = IMAGE_CACHE.lock().unwrap();
                        cache.insert(path_clone, ImageState::Error);
                    }
                }
                Err(_) => {
                    let mut cache = IMAGE_CACHE.lock().unwrap();
                    cache.insert(path_clone, ImageState::Error);
                }
            }
        });

        ImageState::Loading
    }
}
