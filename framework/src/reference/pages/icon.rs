use super::super::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use std::sync::Arc;

pub fn view(_context: &Context, search_query: String, icon_limit: usize) -> PageResult {
    let icons = peak_icons::available_icons();
    let query = search_query.to_lowercase();

    let filtered_icons: Vec<_> = icons
        .into_iter()
        .filter(|name| name.to_lowercase().contains(&query))
        .collect();

    let total_count = filtered_icons.len();
    let display_icons = &filtered_icons[..std::cmp::min(icon_limit, total_count)];

    let mut grid = ResponsiveGrid::<Message, IcedBackend>::new().spacing(12.0);

    for name in display_icons {
        grid = grid.push(
            VStack::<Message, IcedBackend>::new_generic()
                .spacing(8.0)
                .align_x(iced::Alignment::Center)
                .push(
                    Card::new(
                        Icon::<IcedBackend>::new(*name)
                            .size(24.0)
                            .color(Color::from_rgb8(180, 140, 100)), // Explicitly use Peak primary color
                    )
                    .width(Length::Fixed(64.0))
                    .height(Length::Fixed(64.0)),
                )
                .push(
                    Text::<IcedBackend>::new(*name)
                        .caption2()
                        .secondary()
                        .width(Length::Shrink),
                ),
        );
    }

    let preview = HStack::<Message, IcedBackend>::new_generic()
        .spacing(24.0)
        .push(
            Icon::<IcedBackend>::new("zap")
                .size(32.0)
                .color(Color::from_rgb8(0, 122, 255)),
        )
        .push(
            Icon::<IcedBackend>::new("settings-2")
                .size(32.0)
                .color(Color::from_rgb8(142, 142, 147)),
        )
        .push(
            Icon::<IcedBackend>::new("heart-handshake")
                .size(32.0)
                .color(Color::from_rgb8(255, 59, 48)),
        )
        .push(
            Icon::<IcedBackend>::new("check-check")
                .size(32.0)
                .color(Color::from_rgb8(52, 199, 89)),
        );

    let mut footer = VStack::<Message, IcedBackend>::new_generic()
        .spacing(24.0)
        .push(
            Text::new(format!(
                "Icon Library (Showing {} of {} icons)",
                display_icons.len(),
                total_count
            ))
            .title2(),
        )
        .push(if filtered_icons.is_empty() {
            Text::new("No icons found matching your search.")
                .secondary()
                .into_box()
        } else {
            grid.into_box()
        });

    if total_count > icon_limit {
        footer = footer.push(
            HStack::<Message, IcedBackend>::new_generic()
                .width(Length::Fill)
                .align_y(iced::Alignment::Center)
                .push(
                    Button::label(format!(
                        "Load More Icons ({} remaining)",
                        total_count - icon_limit
                    ))
                    .on_press(Message::LoadMoreIcons)
                    .variant(Variant::Solid)
                    .intent(Intent::Primary),
                ),
        );
    }

    PageResult::new(
        ComponentDoc::new(
            "Icon",
            "A flexible icon component powered by the Lucide icon set. Use the global search bar (tagged 'global-search') to find specific icons across the framework.",
            r#"
Icon::new("zap")
    .size(24.0)
    .primary()

Icon::new("settings-2")
    .secondary()
"#,
            Arc::new(preview)
        )
        .extra_content(footer)
    ).searchable("Icons", "Find icon...", |s| Message::Search(s))
}
