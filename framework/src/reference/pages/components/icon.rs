use crate::core::{AIBackend, Backend, IcedBackend, SpatialBackend, TermBackend};

use crate::prelude::*;
use crate::reference::AppPageResult;
use crate::reference::app::{IconLabState, Message, RenderMode};
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(
    ctx: &Context,
    lab: &IconLabState,
    render_mode: RenderMode,
    search_query: String,
    icon_limit: usize,
) -> AppPageResult {
    // --- 1. Library Filter ---
    let icons = peak_icons::available_icons();
    let query = search_query.to_lowercase();
    let filtered_icons: Vec<_> = icons
        .into_iter()
        .filter(|name| name.to_lowercase().contains(&query))
        .collect();

    let total_count = filtered_icons.len();
    let display_icons = &filtered_icons[..std::cmp::min(icon_limit, total_count)];

    // --- 2. Preview Construction ---
    let preview_view = create_preview::<IcedBackend>(lab);
    let terminal_preview = create_preview::<TermBackend>(lab).view(ctx);
    let neural_preview = create_preview::<AIBackend>(lab).view(ctx);
    let spatial_preview = create_preview::<SpatialBackend>(lab).view(ctx);

    // --- 3. Code Snippet ---
    let code_snippet = generate_code(lab);

    let library_section = {
        let mut grid = ResponsiveGrid::<Message, IcedBackend>::new()
            .columns(5)
            .mobile_columns(2)
            .spacing(16.0);

        for name in display_icons {
            let name_str = name.to_string();
            let is_selected = lab.selected_icon == name_str;

            let button_content = vstack::<Message, IcedBackend>()
                .push(
                    icon::<IcedBackend>(name_str.clone())
                        .size(48.0)
                        .color(if is_selected {
                            ctx.theme.colors.on_primary
                        } else {
                            ctx.theme.colors.primary
                        }),
                )
                .push(
                    text::<IcedBackend>(name_str.clone())
                        .size(11.0)
                        .dim()
                        .align_center()
                        .width(Length::Fill)
                        .color(if is_selected {
                            ctx.theme.colors.on_primary
                        } else {
                            ctx.theme.colors.text_secondary
                        }),
                )
                .spacing(8.0)
                .padding(12)
                .align_x(iced::Alignment::Center)
                .width(Length::Fill)
                .height(Length::Fixed(140.0));

            grid = grid.push(
                button::<Message, IcedBackend>(button_content)
                    .variant(if is_selected {
                        Variant::Solid
                    } else {
                        Variant::Ghost
                    })
                    .on_press(Message::UpdateIconLabIcon(name_str))
                    .width(Length::Fill)
                    .height(Length::Fixed(140.0)),
            );
        }

        let section_content = vstack::<Message, IcedBackend>()
            .push(
                text::<IcedBackend>(format!("Icon Library ({} icons found)", total_count))
                    .title2()
                    .bold(),
            )
            .push(if total_count == 0 {
                text::<IcedBackend>("No icons found matching your search.")
                    .secondary()
                    .into_box()
            } else {
                grid.into_box()
            })
            .spacing(24.0);

        if total_count > icon_limit {
            let footer = hstack::<Message, IcedBackend>()
                .push(
                    button_label::<Message, IcedBackend>(format!(
                        "Load More ({} remaining)",
                        total_count - icon_limit
                    ))
                    .on_press(Message::LoadMoreIcons)
                    .variant(Variant::Solid),
                )
                .width(Length::Fill)
                .align_x(iced::Alignment::Center);

            crate::layout::containers::Section::new(
                "Library",
                vstack![section_content, footer].spacing(40.0),
            )
            .width(Length::Fill)
        } else {
            crate::layout::containers::Section::new("Library", section_content).width(Length::Fill)
        }
    };

    // --- 5. Component Documentation ---
    let doc = ComponentDoc::new(
        "Icon",
        "A vector-based icon component powered by the Lucide set. Icons are fully styleable and backend-portable.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Vector Intelligence\nIcons in **PeakUI** are not mere images. They are semantic symbols that the framework understands at a deep level.\n\n- **Crisp Rendering**: Guaranteed sharpness at any scale thanks to SVG/Path-based rendering.\n- **Backend Translation**: In a terminal, icons are mapped to their nearest Unicode or ASCII equivalent.\n- **Semantic Tags**: AI agents use icon names as contextual cues for UI understanding."
    )
    .props_table(
        "| Modifier | Description |\n| :--- | :--- |\n| `.new(name)` | Initialize with a Lucide icon identifier (e.g., \"zap\", \"check\"). |\n| `.size(f32)` | Sets the icon size (default is 24.0). |\n| `.color(Color)` | Sets the stroke/fill color. |\n| `.primary()` | Applies the theme's primary color. |\n| `.secondary()` | Applies the theme's secondary (muted) color. |"
    )
    .extra_content(library_section);

    AppPageResult::new(doc)
        .searchable("Icons", "Find icon...", |s| Message::Search(s))
        .inspector(IconInspector::new(lab))
}

fn create_preview<B: Backend>(lab: &IconLabState) -> VStack<Message, B> {
    let mut icon = Icon::<B>::new(lab.selected_icon.clone()).size(lab.size);
    if let Some(color) = lab.color {
        icon = icon.color(color);
    }

    let has_color = lab.color.is_some();

    VStack::new_generic()
        .spacing(24.0)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .push(
            vstack::<Message, B>()
                .push(
                    text::<B>(format!("Preview: {}", lab.selected_icon))
                        .caption2()
                        .secondary(),
                )
                .push(crate::core::ProxyView::new(move |ctx| {
                    if !has_color {
                        icon.clone().primary().view(ctx)
                    } else {
                        icon.view(ctx)
                    }
                }))
                .spacing(16.0)
                .align_x(Alignment::Center),
        )
}

fn generate_code(lab: &IconLabState) -> String {
    let mut code = format!("Icon::new(\"{}\")", lab.selected_icon);

    if (lab.size - 32.0).abs() > 0.1 {
        code.push_str(&format!("\n    .size({:.1})", lab.size));
    }

    if lab.color.is_some() {
        code.push_str("\n    .color(my_color)");
    } else {
        code.push_str("\n    .primary()");
    }

    code
}

struct IconInspector {
    lab: IconLabState,
}

impl IconInspector {
    fn new(lab: &IconLabState) -> Self {
        Self { lab: lab.clone() }
    }
}

impl View<Message, IcedBackend> for IconInspector {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let content = vstack![
            vstack![
                Text::<IcedBackend>::new("Selected Icon")
                    .caption2()
                    .bold()
                    .secondary(),
                TextInput::<Message, IcedBackend>::new(
                    self.lab.selected_icon.clone(),
                    "Icon name...",
                    |s| Message::UpdateIconLabIcon(s),
                ),
            ]
            .spacing(8.0),
            vstack![
                Text::<IcedBackend>::new("Size")
                    .caption2()
                    .bold()
                    .secondary(),
                hstack![
                    Slider::<Message, IcedBackend>::new(12.0..=128.0, self.lab.size, |v| {
                        Message::UpdateIconLabSize(v)
                    },)
                    .width(Length::Fill),
                    Text::<IcedBackend>::new(format!("{:.0}", self.lab.size))
                        .caption2()
                        .secondary(),
                ]
                .spacing(12.0),
            ]
            .spacing(8.0),
        ]
        .spacing(24.0)
        .padding(Padding::from([20, 20]))
        .width(Length::Fill);

        IcedBackend::scroll_view(
            content.view(context),
            Length::Fill,
            Length::Fill,
            None,
            false,
            ScrollDirection::Vertical,
            context,
        )
    }
}
