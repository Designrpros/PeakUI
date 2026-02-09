use crate::core::{AIBackend, Backend, IcedBackend, SpatialBackend, TermBackend};
use crate::prelude::*;
use crate::reference::app::{Message, RenderMode, SpacerLabState};
use crate::reference::views::ComponentDoc;
use crate::reference::AppPageResult;
use std::sync::Arc;

pub fn view(ctx: &Context, lab: &SpacerLabState, render_mode: RenderMode) -> AppPageResult {
    // --- 1. Preview Construction ---
    let preview_view = create_preview::<IcedBackend>(lab, ctx);
    let terminal_preview = create_preview::<TermBackend>(lab, ctx).view(ctx);
    let neural_preview = create_preview::<AIBackend>(lab, ctx).view(ctx);
    let spatial_preview = create_preview::<SpatialBackend>(lab, ctx).view(ctx);

    // --- 2. Code Snippet ---
    let code_snippet = generate_code(lab);

    // --- 3. Component Documentation ---
    let doc = ComponentDoc::new(
        "Spacer",
        "A fundamental layout primitive used to create empty space between elements. Spacers can have fixed dimensions or expand to fill available space depending on the container.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Spatial Dynamics\nThe **Spacer** element in PeakUI is more than just padding; it is a structural element that defines the 'negative space' of an interface.\n\n- **Fixed vs. Elastic**: When used with fixed dimensions, it acts as a precise gutter. Within stacks, it can be set to `Length::Fill` to push elements apart.\n- **Semantic Neutrality**: AI agents recognize spacers as layout boundaries, helping them understand grouping and hierarchy without being distracted by visual-only decorators.\n- **Efficient Rendering**: Unlike empty containers, the `Space` element is optimized for minimal overhead in all backends."
    )
    .props_table(
        "| Modifier | Description |\n| :--- | :--- |\n| `.new(width, height)` | Initialize with width and height (Length). |\n| `.view(ctx)` | Renders the spacer into the current context. |"
    );

    AppPageResult::new(doc).inspector(SpacerInspector::new(lab))
}

fn create_preview<B: Backend>(lab: &SpacerLabState, ctx: &Context) -> VStack<Message, B> {
    let box_color = ctx.theme.colors.primary.scale_alpha(0.3);
    let spacer_tint = ctx.theme.colors.accent.scale_alpha(0.1);

    crate::dev::dsl::vstack::<Message, B>()
        .spacing(24.0)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .push(
            crate::dev::dsl::text::<B>("The blue area represents the Space element")
                .caption2()
                .secondary(),
        )
        .push(
            crate::dev::dsl::hstack::<Message, B>()
                .push(
                    crate::dev::dsl::container::<Message, B>(crate::dev::dsl::text::<B>("Box A"))
                        .padding(16)
                        .background(box_color)
                        .radius(8.0),
                )
                .push(
                    crate::dev::dsl::container::<Message, B>(crate::dev::dsl::space::<B>(
                        lab.width,
                        lab.height,
                    ))
                    .background(spacer_tint),
                )
                .push(
                    crate::dev::dsl::container::<Message, B>(crate::dev::dsl::text::<B>("Box B"))
                        .padding(16)
                        .background(box_color)
                        .radius(8.0),
                )
                .align_y(Alignment::Center),
        )
}

fn generate_code(lab: &SpacerLabState) -> String {
    format!(
        "Space::new(Length::Fixed({:.0}), Length::Fixed({:.0}))",
        lab.width, lab.height
    )
}

struct SpacerInspector {
    lab: SpacerLabState,
}

impl SpacerInspector {
    fn new(lab: &SpacerLabState) -> Self {
        Self { lab: lab.clone() }
    }
}

impl View<Message, IcedBackend> for SpacerInspector {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let content = vstack![
            vstack![
                Text::<IcedBackend>::new("Width")
                    .caption2()
                    .bold()
                    .secondary(),
                hstack![
                    Slider::<Message, IcedBackend>::new(0.0..=400.0, self.lab.width, |v| {
                        Message::UpdateSpacerWidth(v)
                    },)
                    .width(Length::Fill),
                    Text::<IcedBackend>::new(format!("{:.0}", self.lab.width))
                        .caption2()
                        .secondary(),
                ]
                .spacing(12.0),
            ]
            .spacing(8.0),
            vstack![
                Text::<IcedBackend>::new("Height")
                    .caption2()
                    .bold()
                    .secondary(),
                hstack![
                    Slider::<Message, IcedBackend>::new(0.0..=400.0, self.lab.height, |v| {
                        Message::UpdateSpacerHeight(v)
                    },)
                    .width(Length::Fill),
                    Text::<IcedBackend>::new(format!("{:.0}", self.lab.height))
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
