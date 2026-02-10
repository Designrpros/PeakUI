use crate::core::{AIBackend, Backend, IcedBackend, SpatialBackend, TermBackend};

use crate::prelude::*;
use crate::reference::app::{Message, RenderMode, SizingLabState, SizingType};
use crate::reference::AppPageResult;

pub fn view(ctx: &Context, lab: &SizingLabState, render_mode: RenderMode) -> AppPageResult {
    let mode = ctx.theme.colors;
    let is_narrow = ctx.size.width < 1000.0;

    // --- 1. Hero Section ---
    let hero = vstack![
        text("Basic Sizing")
            .size(if is_narrow { 42.0 } else { 56.0 })
            .bold()
            .color(mode.text_primary),
        text("Control element dimensions with fixed, fill, or relative sizing units.")
            .size(18.0)
            .color(mode.text_secondary),
    ]
    .spacing(12.0)
    .align_x(iced::Alignment::Start)
    .width(Length::Fill);

    // --- 2. Interactive Lab ---
    let mode_tabs = hstack![
        render_mode_tab("Canvas", RenderMode::Canvas, render_mode),
        render_mode_tab("Terminal", RenderMode::Terminal, render_mode),
        render_mode_tab("Neural", RenderMode::Neural, render_mode),
        render_mode_tab("Spatial", RenderMode::Spatial, render_mode),
    ]
    .spacing(8.0);

    let width_tabs = hstack![
        render_sizing_tab("Fixed", SizingType::Fixed, lab.width_type, true),
        render_sizing_tab("Fill", SizingType::Fill, lab.width_type, true),
        render_sizing_tab("Shrink", SizingType::Shrink, lab.width_type, true),
    ]
    .spacing(8.0);

    let height_tabs = hstack![
        render_sizing_tab("Fixed", SizingType::Fixed, lab.height_type, false),
        render_sizing_tab("Fill", SizingType::Fill, lab.height_type, false),
        render_sizing_tab("Shrink", SizingType::Shrink, lab.height_type, false),
    ]
    .spacing(8.0);

    let preview_content: Box<dyn View<Message, IcedBackend>> = match render_mode {
        RenderMode::Canvas => {
            crate::layout::containers::Card::new(create_preview::<IcedBackend>(ctx, lab))
                .padding(32)
                .width(Length::Fill)
                .height(Length::Shrink)
                .into_box()
        }
        RenderMode::Terminal => {
            let ansi = create_preview::<TermBackend>(ctx, lab).view(ctx);
            crate::layout::containers::Card::new(
                CodeBlock::new(ansi)
                    .transparent()
                    .on_copy(Message::CopyCode),
            )
            .background(iced::Color::from_rgb8(30, 30, 30))
            .padding(0)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into_box()
        }
        RenderMode::Neural => {
            let node = create_preview::<AIBackend>(ctx, lab).view(ctx);
            let json = serde_json::to_string_pretty(&node).unwrap_or_default();
            crate::layout::containers::Card::new(
                CodeBlock::new(json)
                    .transparent()
                    .on_copy(Message::CopyCode),
            )
            .background(iced::Color::from_rgb8(30, 30, 30))
            .padding(0)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into_box()
        }
        RenderMode::Spatial => {
            let spatial_node = create_preview::<SpatialBackend>(ctx, lab).view(ctx);
            let empty_node = spatial_node.to_empty();
            crate::layout::containers::Card::new(
                crate::reference::views::simulator::SimulatorView::new(empty_node),
            )
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
            text("Width Type").caption1().secondary(),
            ScrollView::new(width_tabs)
                .direction(ScrollDirection::Horizontal)
                .height(Length::Shrink)
                .hide_indicators(),
        ]
        .spacing(8.0),
        if lab.width_type == SizingType::Fixed {
            vstack![
                text(format!("Fixed Width: {:.0}px", lab.fixed_width))
                    .caption1()
                    .secondary(),
                Slider::new(0.0..=600.0, lab.fixed_width, |v| {
                    Message::UpdateSizingFixedWidth(v)
                })
                .width(Length::Fill),
            ]
            .spacing(8.0)
        } else {
            vstack![]
        },
        vstack![
            text("Height Type").caption1().secondary(),
            ScrollView::new(height_tabs)
                .direction(ScrollDirection::Horizontal)
                .height(Length::Shrink)
                .hide_indicators(),
        ]
        .spacing(8.0),
        if lab.height_type == SizingType::Fixed {
            vstack![
                text(format!("Fixed Height: {:.0}px", lab.fixed_height))
                    .caption1()
                    .secondary(),
                Slider::new(0.0..=400.0, lab.fixed_height, |v| {
                    Message::UpdateSizingFixedHeight(v)
                })
                .width(Length::Fill),
            ]
            .spacing(8.0)
        } else {
            vstack![]
        },
        preview_content
    ]
    .spacing(32.0)
    .width(Length::Fill);

    // --- 3. Usage Section ---
    let usage = vstack![
        text("Usage").title2().bold(),
        text("Sizing in PeakUI is managed through the Length enum, which provides a flexible balance between precise control and adaptive layouts.")
            .secondary(),
        CodeBlock::new(generate_code(lab)).on_copy(Message::CopyCode),
    ]
    .spacing(24.0);

    // --- 4. Theory Section ---
    let theory = vstack![
        text("Theory").title2().bold(),
        theory_item::<IcedBackend>(
            "Relative Sizing (Fill & Shrink)",
            "Fill expands to occupy all available space in its parent container, making it ideal for main content areas. Shrink contracts to fit its content precisely, preventing wasted space in labels and buttons."
        ),
        theory_item::<IcedBackend>(
            "Absolute Sizing (Fixed)",
            "Fixed length provides precise control using logical pixels. This is essential for iconography, toolbars, and branding elements that must remain consistent across different devices."
        ),
        theory_item::<IcedBackend>(
            "The Scaling Paradox",
            "PeakUI's sizing system is scale-aware. When the system scaling token is adjusted, all 'Fixed' values are automatically multiplied by that factor, ensuring that the entire interface grows or shrinks proportionally."
        ),
    ]
    .spacing(24.0);

    let content = vstack![hero, lab_section, usage, theory]
        .spacing(64.0)
        .padding(Padding::new(48.0))
        .width(Length::Fill);

    AppPageResult::new(content)
}

fn render_mode_tab(
    label: &str,
    mode: RenderMode,
    current: RenderMode,
) -> Button<Message, IcedBackend> {
    let active = mode == current;
    button_label(label.to_string())
        .variant(if active {
            Variant::Solid
        } else {
            Variant::Ghost
        })
        .intent(Intent::Primary)
        .on_press(Message::SetRenderMode(mode))
}

fn render_sizing_tab(
    label: &str,
    sizing: SizingType,
    current: SizingType,
    is_width: bool,
) -> Button<Message, IcedBackend> {
    let active = sizing == current;
    button_label(label.to_string())
        .variant(if active {
            Variant::Solid
        } else {
            Variant::Ghost
        })
        .intent(Intent::Primary)
        .on_press(if is_width {
            Message::UpdateSizingWidthType(sizing)
        } else {
            Message::UpdateSizingHeightType(sizing)
        })
}

fn create_preview<B: Backend>(ctx: &Context, lab: &SizingLabState) -> VStack<Message, B> {
    let width = match lab.width_type {
        SizingType::Fixed => Length::Fixed(lab.fixed_width),
        SizingType::Fill => Length::Fill,
        SizingType::Shrink => Length::Shrink,
    };

    let height = match lab.height_type {
        SizingType::Fixed => Length::Fixed(lab.fixed_height),
        SizingType::Fill => Length::Fill,
        SizingType::Shrink => Length::Shrink,
    };

    vstack![
        text("Sizing Mockup").secondary().caption2(),
        crate::elements::atoms::Container::<Message, B>::new(
            crate::elements::atoms::Container::<Message, B>::new(
                text("Element").color(Color::WHITE).size(12.0)
            )
            .width(width)
            .height(height)
            .background(ctx.theme.colors.primary)
            .radius(8.0)
            .padding(12)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
        )
        .width(Length::Fill)
        .height(Length::Shrink)
        .background(ctx.theme.colors.surface_variant.scale_alpha(0.2))
        .radius(12.0)
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center)
    ]
    .spacing(12.0)
    .width(Length::Fill)
    .align_x(iced::Alignment::Center)
}

fn theory_item<B: Backend>(title: &str, description: &str) -> VStack<Message, B> {
    vstack![
        text(title.to_string()).bold(),
        text(description.to_string()).secondary(),
    ]
    .spacing(8.0)
}

fn generate_code(lab: &SizingLabState) -> String {
    let w = match lab.width_type {
        SizingType::Fixed => format!("Length::Fixed({:.1})", lab.fixed_width),
        SizingType::Fill => "Length::Fill".to_string(),
        SizingType::Shrink => "Length::Shrink".to_string(),
    };
    let h = match lab.height_type {
        SizingType::Fixed => format!("Length::Fixed({:.1})", lab.fixed_height),
        SizingType::Fill => "Length::Fill".to_string(),
        SizingType::Shrink => "Length::Shrink".to_string(),
    };

    format!(
        "// Setting dimensions on a container
container(content)
    .width({})
    .height({})",
        w, h
    )
}
