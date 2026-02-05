use crate::core::{AIBackend, Backend, ScrollDirection, SpatialBackend};
use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::{Message, RenderMode};
use serde_json; // Added missing import

pub fn view(ctx: &Context, render_mode: RenderMode) -> PageResult<Message> {
    let palette = ctx.theme.colors;
    let is_narrow = ctx.size.width < 1000.0;

    // --- 1. Hero Section ---
    let hero = vstack![
        text("Colors")
            .size(if is_narrow { 42.0 } else { 56.0 })
            .bold()
            .color(palette.text_primary),
        text("The semantic color system defines the visual language of the application, ensuring consistency and accessibility.")
            .size(18.0)
            .color(palette.text_secondary),
    ]
    .spacing(12.0)
    .align_x(iced::Alignment::Start)
    .width(Length::Fill);

    // --- 2. Lab Section ---
    let lab_tabs = ScrollView::new(
        hstack![
            render_mode_tab("Canvas", RenderMode::Canvas, render_mode),
            render_mode_tab("Terminal", RenderMode::Terminal, render_mode),
            render_mode_tab("Neural", RenderMode::Neural, render_mode),
            render_mode_tab("Spatial", RenderMode::Spatial, render_mode),
        ]
        .spacing(12.0)
        .align_y(iced::Alignment::Center)
        .width(Length::Shrink),
    )
    .direction(ScrollDirection::Horizontal)
    .height(Length::Shrink)
    .hide_indicators();

    let preview_content: Box<dyn View<Message, IcedBackend>> = match render_mode {
        RenderMode::Canvas => crate::containers::Card::new(create_preview::<IcedBackend>(palette))
            .padding(24)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into_box(),
        RenderMode::Terminal => {
            let ansi = create_preview::<TermBackend>(palette).view(ctx);
            crate::containers::Card::new(CodeBlock::new(ansi).transparent())
                .background(iced::Color::from_rgb8(30, 30, 30))
                .padding(0)
                .width(Length::Fill)
                .height(Length::Shrink)
                .into_box()
        }
        RenderMode::Neural => {
            let node = create_preview::<AIBackend>(palette).view(ctx);
            let json = serde_json::to_string_pretty(&node).unwrap_or_default();
            crate::containers::Card::new(CodeBlock::new(json).transparent())
                .background(iced::Color::from_rgb8(30, 30, 30))
                .padding(0)
                .width(Length::Fill)
                .height(Length::Shrink)
                .into_box()
        }
        RenderMode::Spatial => {
            let spatial_node = create_preview::<crate::core::SpatialBackend>(palette).view(ctx);
            let empty_node = spatial_node.to_empty();
            crate::containers::Card::new(crate::reference::views::SimulatorView::new(empty_node))
                .background(iced::Color::from_rgb8(30, 30, 30))
                .padding(0)
                .width(Length::Fill)
                .height(Length::Fixed(400.0))
                .into_box()
        }
    };

    let lab_section = vstack![text("The Lab").title2().bold(), lab_tabs, preview_content]
        .spacing(24.0)
        .width(Length::Fill);

    // --- 3. Usage Section ---
    let usage = vstack![
        text("Usage").title2().bold(),
        CodeBlock::rust(
            r#"// Access semantic colors via the theme in Context
let palette = ctx.theme.colors;

// Use in atoms or containers
Text::new("Hello")
    .color(palette.primary)

Rectangle::new(Length::Fixed(100.0), Length::Fixed(100.0))
    .color(palette.surface_variant)"#
        )
    ]
    .spacing(24.0)
    .width(Length::Fill);

    // --- 4. Theory Section ---
    let theory = vstack![
        text("Theory").title2().bold(),
        text("PeakUI uses a semantic color system based on roles rather than raw values. This allows themes to be swapped effortlessly while maintaining legibility and intent.")
            .color(palette.text_secondary),
        vstack![
            theory_item("Primary", "The main brand color, used for key actions and active states."),
            theory_item("Secondary", "Used for supporting elements and less prominent actions."),
            theory_item("Surface", "Background color for cards, menus, and elevated sections."),
            theory_item("Text Primary", "Highest contrast text for maximum readability."),
        ]
        .spacing(12.0)
    ]
    .spacing(24.0)
    .width(Length::Fill);

    PageResult::new(
        vstack![hero, lab_section, usage, theory]
            .spacing(64.0)
            .padding(if is_narrow { 24 } else { 48 })
            .align_x(iced::Alignment::Start)
            .width(Length::Fill),
    )
}

fn render_mode_tab(
    label: impl Into<String>,
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
        .on_press(Message::SetRenderMode(mode))
}

fn theory_item<B: Backend>(
    title: impl Into<String>,
    desc: impl Into<String>,
) -> impl View<Message, B> + 'static {
    VStack::<Message, B>::new_generic()
        .spacing(4.0)
        .push(Text::<B>::new(title).bold())
        .push(Text::<B>::new(desc).secondary())
}

fn create_preview<B: Backend>(palette: peak_theme::PeakColors) -> VStack<Message, B> {
    VStack::new_generic()
        .spacing(32.0)
        .padding(0.0)
        .width(Length::Fill)
        .push(
            // Main Colors
            VStack::new_generic()
                .spacing(16.0)
                .push(Text::<B>::new("Main Colors").title3().bold())
                .push(
                    HStack::new_generic()
                        .spacing(16.0)
                        .push(color_swatch::<B>("Primary", palette.primary, palette))
                        .push(color_swatch::<B>("Secondary", palette.secondary, palette))
                        .push(color_swatch::<B>("Accent", palette.accent, palette)),
                ),
        )
        .push(
            // Status Colors
            VStack::new_generic()
                .spacing(16.0)
                .push(Text::<B>::new("Status Colors").title3().bold())
                .push(
                    HStack::new_generic()
                        .spacing(16.0)
                        .push(color_swatch::<B>("Success", palette.success, palette))
                        .push(color_swatch::<B>("Warning", palette.warning, palette))
                        .push(color_swatch::<B>("Danger", palette.danger, palette))
                        .push(color_swatch::<B>("Info", palette.info, palette)),
                ),
        )
        .push(
            // Surfaces & Backgrounds
            VStack::new_generic()
                .spacing(16.0)
                .push(Text::<B>::new("Surfaces").title3().bold())
                .push(
                    HStack::new_generic()
                        .spacing(16.0)
                        .push(color_swatch::<B>("Surface", palette.surface, palette))
                        .push(color_swatch::<B>(
                            "Variant",
                            palette.surface_variant,
                            palette,
                        ))
                        .push(color_swatch::<B>("Background", palette.background, palette)),
                ),
        )
}

fn color_swatch<B: Backend>(
    name: impl Into<String>,
    color: Color,
    palette: peak_theme::PeakColors,
) -> impl View<Message, B> + 'static {
    VStack::<Message, B>::new_generic()
        .spacing(12.0)
        .push(
            crate::atoms::Rectangle::<B>::new(Length::Fixed(120.0), Length::Fixed(80.0))
                .color(color)
                .radius(12.0)
                .border(1.0, palette.text_secondary.scale_alpha(0.1)),
        )
        .push(
            VStack::new_generic()
                .spacing(2.0)
                .push(Text::<B>::new(name).bold())
                .push(
                    Text::<B>::new(format!(
                        "#{:02x}{:02x}{:02x}",
                        (color.r * 255.0) as u8,
                        (color.g * 255.0) as u8,
                        (color.b * 255.0) as u8
                    ))
                    .size(11.0)
                    .color(palette.text_secondary),
                ),
        )
}
