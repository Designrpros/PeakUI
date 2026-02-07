use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::{AccessibilityComponent, AccessibilityLabState, Message, RenderMode};
use crate::reference::pages::shared::*;
use crate::core::{AIBackend, Backend, IcedBackend, SpatialBackend, TermBackend};

pub fn view(ctx: &Context, lab: &AccessibilityLabState, render_mode: RenderMode) -> PageResult<Message> {
    let mode = ctx.theme.colors;
    let is_narrow = ctx.size.width < 1000.0;

    // --- 1. Hero Section ---
    let hero = vstack![
        text("Accessibility & Bridge")
            .size(if is_narrow { 42.0 } else { 56.0 })
            .bold()
            .color(mode.text_primary),
        text("PeakUI unifies AI-readability and human accessibility through a single semantic tree and the AccessibilityBridge.")
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

    let component_tabs = hstack![
        render_component_tab("Button", AccessibilityComponent::Button, lab.selected_component),
        render_component_tab("Slider", AccessibilityComponent::Slider, lab.selected_component),
        render_component_tab("Toggle", AccessibilityComponent::Toggle, lab.selected_component),
        render_component_tab("Container", AccessibilityComponent::Container, lab.selected_component),
    ]
    .spacing(8.0);

    let preview_content: Box<dyn View<Message, IcedBackend>> = match render_mode {
        RenderMode::Canvas => crate::containers::Card::new(create_preview::<IcedBackend>(ctx, lab))
            .padding(32)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into_box(),
        RenderMode::Terminal => {
            let ansi = create_preview::<TermBackend>(ctx, lab).view(ctx);
             Box::new(crate::containers::Card::new(CodeBlock::new(ansi).transparent())
                .background(iced::Color::from_rgb8(30, 30, 30))
                .padding(0)
                .width(Length::Fill)
                .height(Length::Shrink))
        }
        RenderMode::Neural => {
            let node = create_preview::<AIBackend>(ctx, lab).view(ctx);
            let json = serde_json::to_string_pretty(&node).unwrap_or_default();
             Box::new(crate::containers::Card::new(CodeBlock::new(json).transparent())
                .background(iced::Color::from_rgb8(30, 30, 30))
                .padding(0)
                .width(Length::Fill)
                .height(Length::Shrink))
        }
        RenderMode::Spatial => {
            let spatial_node = create_preview::<SpatialBackend>(ctx, lab).view(ctx);
            let empty_node = spatial_node.to_empty();
             Box::new(crate::containers::Card::new(crate::reference::views::simulator::SimulatorView::new(
                empty_node,
            ))
            .background(iced::Color::from_rgb8(30, 30, 30))
            .padding(0)
            .width(Length::Fill)
            .height(Length::Fixed(400.0)))
        }
    };

    let lab_section = vstack![
        text("The Lab").title2(),
        vstack![
            text("Render Mode").caption1().secondary(),
            ScrollView::new(mode_tabs)
                .direction(ScrollDirection::Horizontal)
                .height(Length::Shrink)
                .hide_indicators(),
        ]
        .spacing(8.0),
        vstack![
            text("Target Component").caption1().secondary(),
            ScrollView::new(component_tabs)
                .direction(ScrollDirection::Horizontal)
                .height(Length::Shrink)
                .hide_indicators(),
        ]
        .spacing(8.0),
        preview_content
    ]
    .spacing(32.0)
    .width(Length::Fill);

    // --- 3. Usage Section ---
    let usage = vstack![
        text("Usage").title2().bold(),
        text("Every component in PeakUI implements the describe() method to provide semantic information.")
            .secondary(),
        VStack::<Message, IcedBackend>::new_generic()
            .spacing(16.0)
            .push(theory_item_manual::<IcedBackend>(
                "Implicit Accessibility",
                "Constructing a UI using high-level atoms automatically populates the accessibility tree."
            ))
            .push(theory_item_manual::<IcedBackend>(
                "Custom Descriptions",
                "Advanced components can override describe() to provide nuanced hints for AI agents."
            ))
            .push(theory_item_manual::<IcedBackend>(
                "Screen Readers",
                "The AccessibilityBridge maps semantic roles to platform-native roles (e.g., AccessKit for Desktop)."
            )),
    ]
    .spacing(24.0);

    // --- 4. Theory Section ---
    let theory = vstack![
        text("Theory").title2().bold(),
        text("PeakUI's accessibility model is based on the Semantic Tree concept.")
            .secondary(),
        
        crate::atoms::Container::<Message, IcedBackend>::new(
            text("SemanticNode {\n  role: \"button\",\n  label: \"Save\",\n  is_focused: true,\n  ...\n}")
                .font(iced::Font {
                    family: iced::font::Family::Monospace,
                    ..Default::default()
                })
                .caption1()
                .color(ctx.theme.colors.primary)
        )
        .padding(16)
        .background(ctx.theme.colors.surface_variant)
        .radius(8.0),
        
        paragraph(
            "By decoupling the UI representation from its visual rendering, PeakUI allows different backends (Canvas, TUI, Neural) to interpret the interface in the most efficient way for their medium.",
            ctx,
        ),
    ]
    .spacing(24.0);

    let content = vstack![hero, lab_section, usage, theory]
        .spacing(64.0)
        .padding(Padding::new(48.0))
        .width(Length::Fill);

    PageResult::new(content)
}

fn create_preview<B: Backend>(
    ctx: &Context,
    lab: &AccessibilityLabState,
) -> VStack<Message, B> {
    match lab.selected_component {
        AccessibilityComponent::Button => {
            VStack::<Message, B>::new_generic()
                .spacing(12.0)
                .push(
                    crate::controls::Button::<Message, B>::new(text::<B>("Standard Button"))
                        .intent(Intent::Primary)
                )
                .push(text("Role: Button").caption2().secondary())
                .push(text("Label: \"Standard Button\"").caption2().secondary())
        }
        AccessibilityComponent::Slider => {
            VStack::<Message, B>::new_generic()
                .spacing(12.0)
                .push(crate::controls::Slider::<Message, B>::new(0.0..=100.0, 50.0, |_| {
                    Message::None
                }))
                .push(text("Role: Slider").caption2().secondary())
                .push(text("Range: 0 - 100").caption2().secondary())
        }
        AccessibilityComponent::Toggle => {
            VStack::<Message, B>::new_generic()
                .spacing(12.0)
                .push(crate::controls::Toggle::<Message, B>::new("Example Toggle", true, |_| Message::None))
                .push(text("Role: Toggle").caption2().secondary())
                .push(text("State: Checked").caption2().secondary())
        }
        AccessibilityComponent::Container => {
            VStack::<Message, B>::new_generic()
                .spacing(12.0)
                .push(
                    crate::atoms::Container::<Message, B>::new(text::<B>("Inside Container"))
                        .padding(16)
                        .background(ctx.theme.colors.surface_variant)
                        .radius(8.0),
                )
                .push(text("Role: Container").caption2().secondary())
                .push(text("Children: 1 (Text)").caption2().secondary())
        }
    }
}

fn render_mode_tab(
    label: &str,
    mode: RenderMode,
    current: RenderMode,
) -> Button<Message, IcedBackend> {
    let active = mode == current;
    crate::controls::Button::<Message, IcedBackend>::new(text::<IcedBackend>(label.to_string()))
        .variant(if active {
            Variant::Solid
        } else {
            Variant::Ghost
        })
        .intent(Intent::Primary)
        .on_press(Message::SetRenderMode(mode))
}

fn render_component_tab(
    label: &str,
    comp: AccessibilityComponent,
    current: AccessibilityComponent,
) -> Button<Message, IcedBackend> {
    let active = comp == current;
    crate::controls::Button::<Message, IcedBackend>::new(text::<IcedBackend>(label.to_string()))
        .variant(if active {
            Variant::Solid
        } else {
            Variant::Ghost
        })
        .intent(Intent::Primary)
        .on_press(Message::UpdateAccessibilityComponent(comp))
}

fn theory_item_manual<B: Backend>(title: &str, description: &str) -> VStack<Message, B> {
    VStack::<Message, B>::new_generic()
        .spacing(8.0)
        .push(text(title.to_string()).bold())
        .push(text(description.to_string()).secondary())
}
