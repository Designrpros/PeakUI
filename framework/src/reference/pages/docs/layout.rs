use crate::core::{AIBackend, Backend, IcedBackend, ScrollDirection, SpatialBackend, TermBackend};

use crate::prelude::*;
use crate::reference::AppPageResult;
use crate::reference::app::{LayoutLabState, Message, RenderMode, SizingType};

pub fn view(ctx: &Context, lab: &LayoutLabState, render_mode: RenderMode) -> AppPageResult {
    let mode = ctx.theme.colors;
    let is_narrow = ctx.size.width < 1000.0;

    // --- 1. Hero Section ---
    let hero = vstack![
        text("Layout Engine")
            .size(if is_narrow { 42.0 } else { 56.0 })
            .bold()
            .color(mode.text_primary),
        text("Use VStack, HStack, and ZStack to compose complex, responsive layouts with ease.")
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

    let child_count_tabs = hstack![
        render_child_count_tab("1", 1, lab.child_count),
        render_child_count_tab("3", 3, lab.child_count),
        render_child_count_tab("5", 5, lab.child_count),
        render_child_count_tab("8", 8, lab.child_count),
    ]
    .spacing(8.0);

    let alignment_tabs = hstack![
        render_alignment_tab("Start", Alignment::Start, lab.alignment),
        render_alignment_tab("Center", Alignment::Center, lab.alignment),
        render_alignment_tab("End", Alignment::End, lab.alignment),
    ]
    .spacing(8.0);

    let item_sizing_tabs = hstack![
        render_item_sizing_tab("Fixed", SizingType::Fixed, lab.item_sizing),
        render_item_sizing_tab("Fill", SizingType::Fill, lab.item_sizing),
        render_item_sizing_tab("Shrink", SizingType::Shrink, lab.item_sizing),
    ]
    .spacing(8.0);

    let preview_content: Box<dyn View<Message, IcedBackend>> = match render_mode {
        RenderMode::Canvas => crate::layout::containers::Card::new(create_preview::<IcedBackend>(ctx, lab))
            .padding(32)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into_box(),
        RenderMode::Terminal => {
            let ansi = create_preview::<TermBackend>(ctx, lab).view(ctx);
            crate::layout::containers::Card::new(CodeBlock::new(ansi).transparent())
                .background(iced::Color::from_rgb8(30, 30, 30))
                .padding(0)
                .width(Length::Fill)
                .height(Length::Shrink)
                .into_box()
        }
        RenderMode::Neural => {
            let node = create_preview::<AIBackend>(ctx, lab).view(ctx);
            let json = serde_json::to_string_pretty(&node).unwrap_or_default();
            crate::layout::containers::Card::new(CodeBlock::new(json).transparent())
                .background(iced::Color::from_rgb8(30, 30, 30))
                .padding(0)
                .width(Length::Fill)
                .height(Length::Shrink)
                .into_box()
        }
        RenderMode::Spatial => {
            let spatial_node = create_preview::<SpatialBackend>(ctx, lab).view(ctx);
            let empty_node = spatial_node.to_empty();
            crate::layout::containers::Card::new(crate::reference::views::SimulatorView::new(empty_node))
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
            text("Child Count").caption1().secondary(),
            ScrollView::new(child_count_tabs)
                .direction(ScrollDirection::Horizontal)
                .height(Length::Shrink)
                .hide_indicators(),
        ]
        .spacing(8.0),
        vstack![
            text("Alignment (Cross Axis)").caption1().secondary(),
            ScrollView::new(alignment_tabs)
                .direction(ScrollDirection::Horizontal)
                .height(Length::Shrink)
                .hide_indicators(),
        ]
        .spacing(8.0),
        vstack![
            text("Item Sizing").caption1().secondary(),
            ScrollView::new(item_sizing_tabs)
                .direction(ScrollDirection::Horizontal)
                .height(Length::Shrink)
                .hide_indicators(),
        ]
        .spacing(8.0),
        vstack![
            text(format!("Inner Spacing: {:.0}px", lab.inner_spacing))
                .caption1()
                .secondary(),
            Slider::new(0.0..=64.0, lab.inner_spacing, |v| {
                Message::UpdateLayoutInnerSpacing(v)
            })
            .width(Length::Fill),
        ]
        .spacing(8.0),
        preview_content
    ]
    .spacing(32.0)
    .width(Length::Fill);

    // --- 3. Usage Section ---
    let usage = vstack![
        text("Usage").title2().bold(),
        text("Layouts are composed using three primary stacks. By nesting these stacks, you can build any interface structure while maintaining clean, declarative code.")
            .secondary(),
        CodeBlock::new(generate_code(lab)),
    ]
    .spacing(24.0);

    // --- 4. Theory Section ---
    let theory = vstack![
        text("Theory").title2().bold(),
        theory_item::<IcedBackend>(
            "VStack & HStack",
            "These stacks arrange children linearly. VStack stacks them vertically (top to bottom), while HStack stacks them horizontally (left to right). Both support spacing, alignment, and recursive nesting."
        ),
        theory_item::<IcedBackend>(
            "ZStack & Depth",
            "ZStack enables overlaying components on top of each other. This is essential for modals, notifications, floating action buttons, and complex multi-layered UI effects."
        ),
        theory_item::<IcedBackend>(
            "The Flex Mental Model",
            "PeakUI's layout engine works on a flex-box mental model. Children negotiate their size with the parent stack based on their Length settings (Fixed, Fill, Shrink), ensuring responsiveness across all device types."
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

fn render_child_count_tab(
    label: &str,
    count: usize,
    current: usize,
) -> Button<Message, IcedBackend> {
    let active = count == current;
    button_label(label.to_string())
        .variant(if active {
            Variant::Solid
        } else {
            Variant::Ghost
        })
        .intent(Intent::Primary)
        .on_press(Message::UpdateLayoutChildCount(count))
}

fn render_alignment_tab(
    label: &str,
    alignment: Alignment,
    current: Alignment,
) -> Button<Message, IcedBackend> {
    let active = alignment == current;
    button_label(label.to_string())
        .variant(if active {
            Variant::Solid
        } else {
            Variant::Ghost
        })
        .intent(Intent::Primary)
        .on_press(Message::UpdateLayoutAlignment(alignment))
}

fn render_item_sizing_tab(
    label: &str,
    sizing: SizingType,
    current: SizingType,
) -> Button<Message, IcedBackend> {
    let active = sizing == current;
    button_label(label.to_string())
        .variant(if active {
            Variant::Solid
        } else {
            Variant::Ghost
        })
        .intent(Intent::Primary)
        .on_press(Message::UpdateLayoutItemSizing(sizing))
}

fn create_preview<B: Backend>(ctx: &Context, lab: &LayoutLabState) -> VStack<Message, B> {
    let mut children = Vec::new();

    for i in 0..lab.child_count {
        let color = match i % 3 {
            0 => ctx.theme.colors.primary,
            1 => ctx.theme.colors.success,
            _ => ctx.theme.colors.warning,
        };

        children.push(
            crate::elements::atoms::Container::<Message, B>::new(
                text(format!("{}", i + 1)).bold().color(Color::WHITE),
            )
            .padding(12)
            .width(match lab.item_sizing {
                SizingType::Fixed => Length::Fixed(40.0),
                SizingType::Fill => Length::Fixed(80.0), // Give it a base width in preview so it doesn't collapse
                SizingType::Shrink => Length::Shrink,
            })
            .height(Length::Fixed(40.0))
            .background(color)
            .radius(8.0)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center),
        );
    }

    vstack![
        text("Layout Composition").secondary().caption2(),
        crate::elements::atoms::Container::<Message, B>::new(
            vstack![
                text("HStack Example").caption2().secondary(),
                ScrollView::new_generic(
                    hstack()
                        .extend(children.clone())
                        .spacing(lab.inner_spacing)
                        .align_y(lab.alignment)
                        .width(Length::Shrink) // MUST be Shrink to overflow and scroll
                )
                .direction(ScrollDirection::Horizontal)
                .height(Length::Shrink)
                .hide_indicators(),
                Divider::<B>::new(),
                text("VStack Example").caption2().secondary(),
                vstack()
                    .extend(children)
                    .spacing(lab.inner_spacing)
                    .align_x(lab.alignment)
                    .width(Length::Fill),
            ]
            .spacing(32.0)
            .padding(12)
            .width(Length::Fill)
        )
        .height(Length::Shrink)
        .width(Length::Fill)
        .background(ctx.theme.colors.surface_variant.scale_alpha(0.2))
        .radius(12.0)
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

fn generate_code(lab: &LayoutLabState) -> String {
    format!(
        "// Composition Example
VStack::new()
    .spacing({:.1})
    .align_x(Alignment::{:?})
    .push(HStack::new()
        .spacing({:.1})
        .push(child_1)
        .push(child_2)
    )",
        lab.inner_spacing, lab.alignment, lab.inner_spacing
    )
}
