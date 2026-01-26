use super::super::app::Message;
use super::super::page::PageResult;
use crate::prelude::*;

pub fn view(_context: &Context, _is_mobile: bool) -> PageResult {
    let items = vec![
        TimelineItem::new(
            "Phase 1: Core Fundamentals",
            "Q4 2025",
            "completed",
            "The foundation of PeakUI. Robust layout primitives and a unified messaging system.",
            vec![
                "VStack, HStack, ZStack",
                "Visual Primitives (Rectangle, Circle, Capsule)",
                "Contextual Notch Protocol",
                "Unified Message Bus"
            ]
        ),
        TimelineItem::new(
            "Phase 2: Refined Controls",
            "Q1 2026",
            "active",
            "Polishing the developer experience with high-level components and deep system integration.",
            vec![
                "ThemeToken System",
                "Form Sections & Pickers",
                "Safe-Area Aware ScrollView",
                "Navigation Split Views"
            ]
        ),
        TimelineItem::new(
            "Phase 3: Ecosystem",
            "Q2 2026",
            "future",
            "Breaking out of the local environment with cloud connectivity and reactive data layers.",
            vec![
                "PeakDB Sync Integration",
                "Conflict-free State Management",
                "Multi-Backend AI Abstraction",
                "Unified Identity Protocol"
            ]
        ),
        TimelineItem::new(
            "Phase 4: Universal App Engine",
            "Q3 2026",
            "future",
            "True cross-platform power. Deploying to Mobile and Desktop with native performance.",
            vec![
                "iOS & Android Native Bridges",
                "macOS & Windows Shell Integration",
                "High-Density Grid Layout Engines",
                "Real-time Layout Debugger"
            ]
        ),
    ];

    PageResult::new(RoadmapPage { items }).sidebar_toggle(Message::ToggleSidebar)
}

#[derive(Clone)]
struct TimelineItem {
    title: String,
    date: String,
    status: String,
    description: String,
    features: Vec<String>,
}

impl TimelineItem {
    fn new(title: &str, date: &str, status: &str, description: &str, features: Vec<&str>) -> Self {
        Self {
            title: title.to_string(),
            date: date.to_string(),
            status: status.to_string(),
            description: description.to_string(),
            features: features.into_iter().map(|s| s.to_string()).collect(),
        }
    }
}

struct RoadmapPage {
    items: Vec<TimelineItem>,
}

impl View<Message, IcedBackend> for RoadmapPage {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let is_mobile = context.is_slim();

        // Root container for the whole page
        let mut page_col = VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(Padding {
                top: context.safe_area.top.max(48.0),
                right: if is_mobile { 24.0 } else { 48.0 },
                bottom: context.safe_area.bottom.max(120.0),
                left: if is_mobile { 24.0 } else { 48.0 },
            })
            .spacing(48.0);

        // Header
        page_col = page_col.push(
            VStack::<Message, IcedBackend>::new_generic()
                .width(Length::Fill)
                .spacing(12.0)
                .push(
                    Text::<IcedBackend>::new("PeakUI Framework Roadmap")
                        .large_title()
                        .bold()
                        .width(Length::Fill),
                )
                .push(
                    Text::<IcedBackend>::new(
                        "The journey toward a unified, AI-native universal design system.",
                    )
                    .body()
                    .secondary()
                    .width(Length::Fill),
                ),
        );

        // Roadmap content
        let count = self.items.len();
        let mut roadmap_col = VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .spacing(0.0);

        for (i, item) in self.items.iter().enumerate() {
            roadmap_col = roadmap_col.push(TimelineRow::new(item.clone(), i == 0, i == count - 1));
        }

        page_col.push(roadmap_col).view(context)
    }
}

struct TimelineRow {
    item: TimelineItem,
    is_first: bool,
    is_last: bool,
}

impl TimelineRow {
    fn new(item: TimelineItem, is_first: bool, is_last: bool) -> Self {
        Self {
            item,
            is_first,
            is_last,
        }
    }
}

impl View<Message, IcedBackend> for TimelineRow {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let (dot_color, line_color) = get_status_colors(&self.item.status, &context.theme);

        let pillar = {
            let line_layer = VStack::<Message, IcedBackend>::new_generic()
                .width(Length::Fixed(32.0))
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .push(if self.is_last {
                    VStack::<Message, IcedBackend>::new_generic()
                        .width(Length::Fixed(4.0))
                        .height(Length::Fill)
                        .push(Rectangle::new(4.0.into(), 28.0.into()).color(line_color))
                        .into_box()
                } else if self.is_first {
                    VStack::<Message, IcedBackend>::new_generic()
                        .width(Length::Fixed(4.0))
                        .height(Length::Fill)
                        .push(Space::new(0.0.into(), 28.0.into()))
                        .push(Rectangle::new(4.0.into(), Length::Fill).color(line_color))
                        .into_box()
                } else {
                    Rectangle::new(4.0.into(), Length::Fill)
                        .color(line_color)
                        .into_box()
                });

            let dot_layer = VStack::<Message, IcedBackend>::new_generic()
                .width(Length::Fixed(32.0))
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .push(Space::new(0.0.into(), 20.0.into()))
                .push(
                    Rectangle::new(16.0.into(), 16.0.into())
                        .color(dot_color)
                        .radius(8.0),
                );

            line_layer
                .overlay(dot_layer, Alignment::Center)
                .width(Length::Fixed(32.0))
        };

        let card_wrapper = VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .padding(Padding {
                top: 0.0,
                bottom: 12.0,
                left: 56.0,
                ..Default::default()
            })
            .push(render_card(&self.item, context));

        card_wrapper
            .overlay(pillar, Alignment::Start)
            .height(Length::Shrink)
            .view(context)
    }
}

fn get_status_colors(status: &str, theme: &ThemeTokens) -> (Color, Color) {
    match status {
        "completed" => (theme.colors.success, theme.colors.success),
        "active" => (theme.colors.primary, theme.colors.primary),
        _ => (theme.colors.border, theme.colors.border.scale_alpha(0.4)),
    }
}

fn render_card(item: &TimelineItem, context: &Context) -> impl View<Message, IcedBackend> {
    let theme = context.theme;
    let status_str = item.status.clone();
    let date_str = item.date.clone();

    let date_pill = ProxyView::new(move |ctx| {
        let t = ctx.theme;
        let s = status_str.clone();
        container(
            Text::<IcedBackend>::new(date_str.clone())
                .caption2()
                .bold()
                .color(if s == "future" {
                    t.colors.text_primary
                } else {
                    Color::WHITE
                })
                .view(ctx),
        )
        .padding([4, 10])
        .style(move |_| container::Style {
            background: Some(
                if s == "active" {
                    t.colors.primary
                } else if s == "completed" {
                    t.colors.success
                } else {
                    t.colors.surface_variant
                }
                .into(),
            ),
            border: Border {
                radius: 12.0.into(),
                color: if s == "future" {
                    t.colors.border
                } else {
                    Color::TRANSPARENT
                },
                width: if s == "future" { 1.0 } else { 0.0 },
            },
            ..Default::default()
        })
        .into()
    });

    let mut features_col = VStack::<Message, IcedBackend>::new_generic()
        .width(Length::Fill)
        .spacing(8.0);

    for feat in &item.features {
        features_col = features_col.push(
            HStack::<Message, IcedBackend>::new_generic()
                .width(Length::Fill)
                .spacing(10.0)
                .align_y(Alignment::Center)
                .push(
                    Icon::<IcedBackend>::new(if item.status == "future" {
                        "circle"
                    } else {
                        "check-circle"
                    })
                    .size(14.0)
                    .color(if item.status == "future" {
                        theme.colors.text_secondary
                    } else {
                        theme.colors.success
                    }),
                )
                .push(
                    Text::<IcedBackend>::new(feat.clone())
                        .body()
                        .color(if item.status == "future" {
                            theme.colors.text_secondary
                        } else {
                            theme.colors.text_primary
                        })
                        .width(Length::Fill),
                ),
        );
    }

    crate::containers::Card::new(
        VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .spacing(16.0)
            .push(
                HStack::<Message, IcedBackend>::new_generic()
                    .width(Length::Fill)
                    .align_y(Alignment::Center)
                    .spacing(12.0)
                    .push(
                        Text::<IcedBackend>::new(item.title.clone())
                            .title3()
                            .bold()
                            .width(Length::Fill),
                    )
                    .push(date_pill),
            )
            .push(
                Text::<IcedBackend>::new(item.description.clone())
                    .body()
                    .color(theme.colors.text_secondary)
                    .width(Length::Fill),
            )
            .push(features_col),
    )
    .padding(16.0)
    .width(Length::Fill)
}
