use crate::core::{AIBackend, Backend, IcedBackend, SpatialBackend, TermBackend};
use crate::engine::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::{Message, RenderMode};
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(ctx: &Context, render_mode: RenderMode) -> PageResult<Message> {
    let preview_view = create_preview::<IcedBackend>();
    let terminal_preview = create_preview::<TermBackend>().view(ctx);
    let neural_preview = create_preview::<AIBackend>().view(ctx);
    let spatial_preview = create_preview::<SpatialBackend>().view(ctx);

    let code_snippet = "Chart::new(\n    ChartType::Pie,\n    vec![\n        ChartDataPoint { label: \"Compute\".to_string(), value: 450.0 },\n        ChartDataPoint { label: \"Relays\".to_string(), value: 230.0 },\n        ChartDataPoint { label: \"Storage\".to_string(), value: 890.0 },\n        ChartDataPoint { label: \"Neural\".to_string(), value: 120.0 },\n    ]\n)\n.title(\"Infrastructure Distribution\")".to_string();

    let doc = ComponentDoc::new(
        "Pie Chart",
        "A categorical visualization optimized for displaying proportional data relationships and semantic distribution.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Proportion & Balance\nPie charts provide a high-level overview of resource distribution. \n\n- **Semantic Grouping**: Automatically clusters small values for better readability.\n- **Donut Transition**: Supports inner-radius customization for modern 'donut' style looks."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `.title(str)` | `String` | Optional header title for the chart. |\n| `.color(color)` | `Color` | Base color for segment generation. |\n| `.height(len)` | `Length` | Fixed or relative height of the chart area. |"
    );

    PageResult::new(doc)
}

fn create_preview<B: Backend>() -> Chart<Message, B> {
    Chart::<Message, B>::new(
        ChartType::Pie,
        vec![
            ChartDataPoint {
                label: "Compute".to_string(),
                value: 450.0,
            },
            ChartDataPoint {
                label: "Relays".to_string(),
                value: 230.0,
            },
            ChartDataPoint {
                label: "Storage".to_string(),
                value: 890.0,
            },
            ChartDataPoint {
                label: "Neural".to_string(),
                value: 120.0,
            },
        ],
    )
    .title("Infrastructure Distribution")
    .palette(vec![
        Color::from_rgb(0.31, 0.98, 0.48), // Peak Green
        Color::from_rgb(0.31, 0.78, 0.98), // Peak Blue
        Color::from_rgb(0.78, 0.31, 0.98), // Peak Purple
        Color::from_rgb(0.98, 0.31, 0.48), // Peak Red
    ])
    .height(Length::Fixed(200.0))
}
