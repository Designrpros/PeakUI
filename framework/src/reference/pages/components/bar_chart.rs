use crate::core::{AIBackend, Backend, IcedBackend, SpatialBackend, TermBackend};
use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::{Message, RenderMode};
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(ctx: &Context, render_mode: RenderMode) -> PageResult<Message> {
    let preview_view = create_preview::<IcedBackend>();
    let terminal_preview = create_preview::<TermBackend>().view(ctx);
    let neural_preview = create_preview::<AIBackend>().view(ctx);
    let spatial_preview = create_preview::<SpatialBackend>().view(ctx);

    let code_snippet = "Chart::new(\n    ChartType::Bar,\n    vec![\n        ChartDataPoint { label: \"Compute\".to_string(), value: 84.0 },\n        ChartDataPoint { label: \"Relay\".to_string(), value: 52.0 },\n        ChartDataPoint { label: \"Storage\".to_string(), value: 91.0 },\n        ChartDataPoint { label: \"Neural\".to_string(), value: 68.0 },\n    ]\n)\n.title(\"Infrastructure Load (Total)\")".to_string();

    let doc = ComponentDoc::new(
        "Bar Chart",
        "A highly-performant categorical visualization component for comparing discrete data sets across kernels.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Discrete Comparison\nBar charts in PeakUI are rendered as semantic geometry. Each bar is a logical entity that transitions smoothly between different telemetry states.\n\n- **GPU-Backing**: Uses scaled rectangles with sub-pixel alignment.\n- **Semantic Perception**: AI kernels perceive the exact height and label values."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.title(str)` | `String` | Optional header title for the chart. |\n| `.color(color)` | `Color` | Main accent color for bars. |\n| `.height(len)` | `Length` | Fixed or relative height of the chart area. |"
    );

    PageResult::new(doc)
}

fn create_preview<B: Backend>() -> Chart<Message, B> {
    Chart::<Message, B>::new(
        ChartType::Bar,
        vec![
            ChartDataPoint {
                label: "Compute".to_string(),
                value: 84.0,
            },
            ChartDataPoint {
                label: "Relay".to_string(),
                value: 52.0,
            },
            ChartDataPoint {
                label: "Storage".to_string(),
                value: 91.0,
            },
            ChartDataPoint {
                label: "Neural".to_string(),
                value: 68.0,
            },
        ],
    )
    .title("Infrastructure Load (Total)")
    .palette(vec![
        Color::from_rgb(0.31, 0.98, 0.48), // Peak Green
        Color::from_rgb(0.31, 0.78, 0.98), // Peak Blue
        Color::from_rgb(0.78, 0.31, 0.98), // Peak Purple
        Color::from_rgb(0.98, 0.31, 0.48), // Peak Red
    ])
    .height(Length::Fixed(200.0))
}
