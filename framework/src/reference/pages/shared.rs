use crate::prelude::*;

use crate::reference::app::Message;

pub fn heading<B: Backend, S: Into<String>>(
    content: S,
    context: &Context,
) -> Box<dyn View<Message, B> + Send + Sync + 'static> {
    let theme = context.theme;
    Box::new(
        Text::new(content.into())
            .title2()
            .bold()
            .color(theme.colors.text_primary)
            .width(Length::Fill)
            .wrap(),
    )
}

pub fn sub_heading<B: Backend, S: Into<String>>(
    content: S,
    context: &Context,
) -> Box<dyn View<Message, B> + Send + Sync + 'static> {
    let theme = context.theme;
    Box::new(
        Text::new(content.into())
            .title3()
            .bold()
            .color(theme.colors.text_primary.scale_alpha(0.9))
            .width(Length::Fill)
            .wrap(),
    )
}

pub fn paragraph<B: Backend, S: Into<String>>(
    content: S,
    context: &Context,
) -> Box<dyn View<Message, B> + Send + Sync + 'static> {
    let theme = context.theme;
    Box::new(
        Text::new(content.into())
            .body()
            .color(theme.colors.text_secondary)
            .width(Length::Fill)
            .wrap(),
    )
}

pub fn bullet_list<B: Backend>(
    items: Vec<&str>,
    context: &Context,
) -> Box<dyn View<Message, B> + Send + Sync + 'static> {
    let mut list = VStack::new_generic().spacing(16.0).width(Length::Fill);
    let theme = context.theme;
    let is_slim = context.device == DeviceType::Mobile || context.size.width < 500.0;

    for item in items {
        if is_slim {
            list = list.push(
                VStack::new_generic()
                    .spacing(4.0)
                    .width(Length::Fill)
                    .push(Text::new("•").body().color(theme.colors.primary))
                    .push(
                        Text::new(item.to_string())
                            .body()
                            .color(theme.colors.text_secondary)
                            .width(Length::Fill)
                            .wrap(),
                    ),
            );
        } else {
            list = list.push(
                HStack::new_generic()
                    .spacing(12.0)
                    .width(Length::Fill)
                    .push(Text::new("•").body().color(theme.colors.primary))
                    .push(
                        Text::new(item.to_string())
                            .body()
                            .color(theme.colors.text_secondary)
                            .width(Length::Fill)
                            .wrap(),
                    ),
            );
        }
    }

    Box::new(list)
}

pub fn architecture_item<B: Backend>(
    name: &str,
    desc: &str,
    context: &Context,
) -> Box<dyn View<Message, B> + Send + Sync + 'static> {
    let theme = context.theme;
    let is_slim = context.device == DeviceType::Mobile || context.size.width < 500.0;

    let blue = if theme.tone == peak_theme::ThemeTone::Dark {
        Color::from_rgb(0.4, 0.7, 1.0)
    } else {
        Color::from_rgb(0.0, 0.4, 0.8)
    };

    if is_slim {
        Box::new(
            VStack::new_generic()
                .spacing(4.0)
                .width(Length::Fill)
                .push(Text::new(format!("`{}`", name)).body().bold().color(blue))
                .push(
                    Text::new(desc.to_string())
                        .body()
                        .color(theme.colors.text_secondary)
                        .width(Length::Fill)
                        .wrap(),
                ),
        )
    } else {
        Box::new(
            HStack::new_generic()
                .spacing(12.0)
                .width(Length::Fill)
                .push(Text::new(format!("`{}`", name)).body().bold().color(blue))
                .push(
                    Text::new(format!(": {}", desc))
                        .body()
                        .color(theme.colors.text_secondary)
                        .width(Length::Fill)
                        .wrap(),
                ),
        )
    }
}

pub fn native_divider<B: Backend>() -> Box<dyn View<Message, B> + Send + Sync + 'static> {
    Box::new(Divider::<B>::new())
}

pub fn verdict_badge<B: Backend>(
    score: &str,
    context: &Context,
) -> Box<dyn View<Message, B> + Send + Sync + 'static> {
    let score = score.to_string();
    let theme = context.theme;

    Box::new(ProxyView::new(move |ctx| {
        let content: B::AnyView<Message> = HStack::new_generic()
            .spacing(8.0)
            .align_y(Alignment::Center)
            .push(
                Icon::<B>::new("star")
                    .color(theme.colors.success)
                    .size(16.0),
            )
            .push(
                Text::<B>::new(format!("Verdict: {}", score))
                    .title3()
                    .bold()
                    .color(theme.colors.success),
            )
            .view(ctx);

        B::container(
            content,
            Padding::from([12, 24]),
            Length::Shrink,
            Length::Shrink,
            Some(theme.colors.success.scale_alpha(0.12)),
            12.0,
            1.0,
            Some(theme.colors.success.scale_alpha(0.3)),
            None,
            Alignment::Start,
            Alignment::Start,
            ctx,
        )
    }))
}
