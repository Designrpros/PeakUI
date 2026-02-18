use crate::prelude::*;
use crate::reference::app::{InteractionMessage, LabMessage, Message, RenderMode};
use crate::reference::AppPageResult;
use peak_theme::{PeakTheme, ThemeTone};
use std::borrow::Cow;

pub fn view(ctx: &Context, render_mode: RenderMode) -> AppPageResult {
    let palette = ctx.theme.colors;
    let is_narrow = ctx.size.width < 1000.0;

    // --- 1. Hero Section ---
    let hero = vstack![
        text("Theming")
            .size(if is_narrow { 42.0 } else { 56.0 })
            .bold()
            .color(palette.text_primary),
        text("PeakUI's runtime theming system allows you to adapt the interface to any brand, preference, or environment instantly.")
            .size(18.0)
            .color(palette.text_secondary),
    ]
    .spacing(12.0)
    .align_x(iced::Alignment::Start)
    .width(Length::Fill);

    // --- 2. Lab Section ---
    let theme_tabs = ScrollView::new(
        hstack![
            render_theme_tab("Mono", PeakTheme::Mono, ctx.theme.colors.primary, ctx),
            render_theme_tab(
                "Peak",
                PeakTheme::Peak,
                PeakTheme::Peak.colors(ThemeTone::Dark).primary,
                ctx
            ),
            render_theme_tab(
                "Cupertino",
                PeakTheme::Cupertino,
                PeakTheme::Cupertino.colors(ThemeTone::Dark).primary,
                ctx
            ),
            render_theme_tab(
                "Smart",
                PeakTheme::Smart,
                PeakTheme::Smart.colors(ThemeTone::Dark).primary,
                ctx
            ),
            render_theme_tab(
                "Material",
                PeakTheme::Material,
                PeakTheme::Material.colors(ThemeTone::Dark).primary,
                ctx
            ),
            render_theme_tab(
                "Fluent",
                PeakTheme::Fluent,
                PeakTheme::Fluent.colors(ThemeTone::Dark).primary,
                ctx
            ),
            render_theme_tab(
                "Mountain",
                PeakTheme::Mountain,
                PeakTheme::Mountain.colors(ThemeTone::Dark).primary,
                ctx
            ),
        ]
        .spacing(12.0)
        .align_y(iced::Alignment::Center)
        .width(Length::Shrink),
    )
    .direction(ScrollDirection::Horizontal)
    .height(Length::Shrink)
    .hide_indicators();

    let tone_tabs = hstack![
        render_tone_tab("Light", ThemeTone::Light, ctx),
        render_tone_tab("Dark", ThemeTone::Dark, ctx),
    ]
    .spacing(8.0)
    .align_y(iced::Alignment::Center);

    let mode_tabs = hstack![
        render_mode_tab("Canvas", RenderMode::Canvas, render_mode),
        render_mode_tab("Terminal", RenderMode::Terminal, render_mode),
        render_mode_tab("Neural", RenderMode::Neural, render_mode),
        render_mode_tab("Spatial", RenderMode::Spatial, render_mode),
    ]
    .spacing(8.0)
    .align_y(iced::Alignment::Center);

    let preview_content: Box<dyn View<Message, IcedBackend> + Send + Sync> = match render_mode {
        RenderMode::Canvas => {
            crate::layout::containers::Card::new(create_preview::<IcedBackend>(ctx))
                .padding(32)
                .width(Length::Fill)
                .height(Length::Shrink)
                .into_box()
        }
        RenderMode::Terminal => {
            let ansi = create_preview::<TermBackend>(ctx).view(ctx);
            crate::layout::containers::Card::new(
                CodeBlock::new(ansi)
                    .transparent()
                    .on_copy(|c| Message::Interaction(InteractionMessage::CopyCode(c))),
            )
            .background(iced::Color::from_rgb8(30, 30, 30))
            .padding(0)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into_box()
        }
        RenderMode::Neural => {
            let node = create_preview::<AIBackend>(ctx).view(ctx);
            let json = serde_json::to_string_pretty(&node).unwrap_or_default();
            crate::layout::containers::Card::new(
                CodeBlock::new(json)
                    .transparent()
                    .on_copy(|c| Message::Interaction(InteractionMessage::CopyCode(c))),
            )
            .background(iced::Color::from_rgb8(30, 30, 30))
            .padding(0)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into_box()
        }
        RenderMode::Spatial => {
            let spatial_node = create_preview::<crate::core::SpatialBackend>(ctx).view(ctx);
            let empty_node = spatial_node.to_empty();
            crate::layout::containers::Card::new(crate::reference::views::SimulatorView::new(
                empty_node,
            ))
            .background(iced::Color::from_rgb8(30, 30, 30))
            .padding(0)
            .width(Length::Fill)
            .height(Length::Fixed(400.0))
            .into_box()
        }
    };

    let lab_section = vstack![
        Text::<IcedBackend>::new("The Lab").title2(),
        vstack![
            text("Render Mode").caption1().secondary(),
            ScrollView::new(mode_tabs)
                .direction(ScrollDirection::Horizontal)
                .height(Length::Shrink)
                .hide_indicators(),
        ]
        .spacing(8.0),
        vstack![
            text("Appearance").caption1().secondary(),
            ScrollView::new(tone_tabs)
                .direction(ScrollDirection::Horizontal)
                .height(Length::Shrink)
                .hide_indicators(),
        ]
        .spacing(8.0),
        vstack![
            text("Theme").caption1().secondary(),
            theme_tabs, // theme_tabs is already a ScrollView
        ]
        .spacing(8.0),
        preview_content
    ]
    .spacing(32.0)
    .width(Length::Fill);

    // --- 3. Usage Section ---
    let usage = vstack![
        text("Usage").title2().bold(),
        text("Themes are typically applied at the root of the application, but can also be overridden for specific sub-trees using a `ThemedView`.")
            .color(palette.text_secondary),
        CodeBlock::rust(
            r#"// Changing theme for the entire app
// Message::Interaction(InteractionMessage::SetThemeKind(PeakTheme::Cupertino))

// Accessing tokens in your own components
fn view(&self, ctx: &Context) -> Element {
    let spacing = ctx.theme.spacing_unit;
    let radius = ctx.theme.radius;
    
    container(content)
        .padding(spacing)
        .style(|_| container::Style {
            border: Border {
                radius: radius.into(),
                ..Default::default()
            },
            ..Default::default()
        })
}"#
        )
        .on_copy(|c| Message::Interaction(InteractionMessage::CopyCode(c)))
    ]
    .spacing(24.0)
    .width(Length::Fill);

    // --- 4. Theory Section ---
    let theory = vstack![
        text("Theory").title2().bold(),
        text("PeakUI's theming is built on four core pillars: Colors, Spacing, Radius, and Scaling. These are unified into a single `ThemeTokens` struct.")
            .color(palette.text_secondary),
        vstack![
            theory_item("Runtime Switching", "Themes are not baked; they can be swapped instantly without re-rendering the entire app state."),
            theory_item("Scaling", "The `scaling` token allows for global UI density adjustments (e.g., 0.8 for power users, 1.2 for accessibility)."),
            theory_item("Glassmorphism", "The system includes native support for glass effects, which are automatically optimized or disabled for performance-critical backends."),
            theory_item("Tone Aware", "Every theme defines both Light and Dark palettes, ensuring a consistent experience across all system appearances."),
        ]
        .spacing(12.0)
    ]
    .spacing(24.0)
    .width(Length::Fill);

    AppPageResult::new(
        vstack![hero, lab_section, usage, theory]
            .spacing(64.0)
            .padding(if is_narrow { 24 } else { 48 })
            .align_x(iced::Alignment::Start)
            .width(Length::Fill),
    )
}

fn render_theme_tab(
    label: impl Into<Cow<'static, str>>,
    theme: PeakTheme,
    color: Color,
    ctx: &Context,
) -> impl View<Message, IcedBackend> {
    let is_active = ctx.theme.colors.primary == color; // Approximation since we don't have theme name in Context

    Button::<Message, IcedBackend>::label(label)
        .variant(if is_active {
            Variant::Solid
        } else {
            Variant::Ghost
        })
        .on_press(Message::Interaction(InteractionMessage::SetThemeKind(
            theme,
        )))
}

fn render_tone_tab(
    label: impl Into<Cow<'static, str>>,
    tone: ThemeTone,
    ctx: &Context,
) -> impl View<Message, IcedBackend> {
    let is_active = match tone {
        ThemeTone::Light => !ctx.theme.colors.is_dark(),
        ThemeTone::Dark => ctx.theme.colors.is_dark(),
    };

    Button::<Message, IcedBackend>::label(label)
        .variant(if is_active {
            Variant::Soft
        } else {
            Variant::Ghost
        })
        .size(ControlSize::Small)
        .on_press(Message::Interaction(InteractionMessage::SetTheme(tone)))
}

fn render_mode_tab(
    label: impl Into<Cow<'static, str>>,
    mode: RenderMode,
    current: RenderMode,
) -> impl View<Message, IcedBackend> {
    let is_active = mode == current;
    Button::<Message, IcedBackend>::label(label)
        .variant(if is_active {
            Variant::Solid
        } else {
            Variant::Ghost
        })
        .size(ControlSize::Small)
        .on_press(Message::Lab(LabMessage::SetRenderMode(mode)))
}

fn theory_item<B: Backend>(
    title: impl Into<Cow<'static, str>>,
    desc: impl Into<Cow<'static, str>>,
) -> impl View<Message, B> + 'static {
    VStack::<Message, B>::new_generic()
        .spacing(4.0)
        .push(Text::<B>::new(title).bold())
        .push(Text::<B>::new(desc).secondary())
}

fn create_preview<B: Backend>(ctx: &Context) -> VStack<Message, B> {
    let tokens = ctx.theme;

    VStack::new_generic()
        .spacing(32.0)
        .width(Length::Fill)
        .push(
            hstack![
                Icon::<B>::new("palette").size(24.0).primary(),
                text("Live Token Preview").bold().size(16.0)
            ]
            .spacing(12.0),
        )
        .push(
            VStack::new_generic()
                .spacing(32.0)
                .width(Length::Fill)
                .push(
                    VStack::new_generic()
                        .spacing(16.0)
                        .push(token_display::<B>(
                            "Radius",
                            format!("{:.1}px", tokens.radius),
                        ))
                        .push(token_display::<B>(
                            "Spacing",
                            format!("{:.1}px", tokens.spacing_unit),
                        ))
                        .push(token_display::<B>(
                            "Blur",
                            format!("{:.1}px", tokens.blur_radius),
                        ))
                        .push(token_display::<B>(
                            "Scaling",
                            format!("{:.2}x", tokens.scaling),
                        )),
                )
                .push(Divider::<B>::new())
                .push(
                    VStack::new_generic()
                        .spacing(16.0)
                        .width(Length::Fill)
                        .push(
                            VStack::new_generic()
                                .spacing(8.0)
                                .push(Text::<B>::new("Component Preview").secondary().size(12.0))
                                .push(
                                    hstack![
                                        Button::<Message, B>::label("Primary")
                                            .variant(Variant::Solid)
                                            .intent(Intent::Primary),
                                        Button::<Message, B>::label("Secondary")
                                            .variant(Variant::Ghost),
                                        Button::<Message, B>::label("")
                                            .icon("settings")
                                            .variant(Variant::Soft),
                                    ]
                                    .spacing(12.0),
                                )
                                .push(
                                    HStack::new_generic()
                                        .spacing(12.0)
                                        .push(
                                            Icon::<B>::new("info")
                                                .size(18.0)
                                                .color(tokens.colors.info),
                                        )
                                        .push(
                                            Icon::<B>::new("check-circle")
                                                .size(18.0)
                                                .color(tokens.colors.success),
                                        )
                                        .push(
                                            Icon::<B>::new("alert-triangle")
                                                .size(18.0)
                                                .color(tokens.colors.warning),
                                        )
                                        .push(
                                            Icon::<B>::new("x-circle")
                                                .size(18.0)
                                                .color(tokens.colors.danger),
                                        ),
                                ),
                        ),
                ),
        )
}

fn token_display<B: Backend>(
    label: impl Into<Cow<'static, str>>,
    value: impl Into<Cow<'static, str>>,
) -> impl View<Message, B> + 'static {
    HStack::<Message, B>::new_generic()
        .spacing(12.0)
        .push(Text::<B>::new(label).secondary().width(Length::Fixed(80.0)))
        .push(Text::<B>::new(value).bold())
}
