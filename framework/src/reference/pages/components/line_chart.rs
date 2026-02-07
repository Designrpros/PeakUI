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

    let code_snippet = "Chart::new(\n    ChartType::Line,\n    vec![\n        ChartDataPoint { label: \"10:00\".to_string(), value: 45.0 },\n        ChartDataPoint { label: \"10:05\".to_string(), value: 52.0 },\n        ChartDataPoint { label: \"10:10\".to_string(), value: 48.0 },\n        ChartDataPoint { label: \"10:15\".to_string(), value: 70.0 },\n        ChartDataPoint { label: \"10:20\".to_string(), value: 65.0 },\n        ChartDataPoint { label: \"10:25\".to_string(), value: 85.0 },\n        ChartDataPoint { label: \"10:30\".to_string(), value: 72.0 },\n    ]\n)\n.title(\"Network Telemetry (Mbps)\")".to_string();

    let doc = ComponentDoc::new(
        "Line Chart",
        "A visualization for trends and continuous data series with sub-pixel rendering and GPU acceleration.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Continuity & Growth\nLine charts in PeakUI prioritize smooth curves and high refresh rates. \n\n- **Sub-pixel Precision**: GPU-backed vertex buffers ensured crisp lines at any zoom level.\n- **Temporal Awareness**: Optimized for real-time telemetry streaming."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.title(str)` | `String` | Optional header title for the chart. |\n| `.color(color)` | `Color` | Main accent color for data series. |\n| `.height(len)` | `Length` | Fixed or relative height of the chart area. |"
    );

    PageResult::new(doc)
}

fn create_preview<B: Backend>() -> Chart<Message, B> {
    Chart::<Message, B>::new(
        ChartType::Line,
        vec![
            ChartDataPoint {
                label: "10:00".to_string(),
                value: 45.0,
            },
            ChartDataPoint {
                label: "10:05".to_string(),
                value: 52.0,
            },
            ChartDataPoint {
                label: "10:10".to_string(),
                value: 48.0,
            },
            ChartDataPoint {
                label: "10:15".to_string(),
                value: 70.0,
            },
            ChartDataPoint {
                label: "10:20".to_string(),
                value: 65.0,
            },
            ChartDataPoint {
                label: "10:25".to_string(),
                value: 85.0,
            },
            ChartDataPoint {
                label: "10:30".to_string(),
                value: 72.0,
            },
        ],
    )
    .title("Network Telemetry (Mbps)")
    .color(Color::from_rgb(0.31, 0.78, 0.98)) // Peak Blue
    .height(Length::Fixed(200.0))
}
