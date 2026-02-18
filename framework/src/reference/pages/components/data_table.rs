use crate::core::{AIBackend, Context, IcedBackend, SpatialBackend};

use crate::prelude::*;
use crate::reference::app::{LabMessage, Message, RenderMode};
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(ctx: &Context, render_mode: RenderMode) -> AppPageResult {
    // --- 1. Preview Construction ---
    let preview_view = create_canvas_preview(ctx);
    let terminal_preview = "DataTable[Rows: 3, Columns: 3]".to_string();
    let neural_preview = vstack::<Message, AIBackend>()
        .push(text("DataTable"))
        .describe(ctx);
    let spatial_preview = vstack::<Message, SpatialBackend>()
        .push(text("DataTable"))
        .view(ctx);

    // --- 2. Code Snippet ---
    let code_snippet = r#"data_table![
    preset(DataTablePreset::Professional),
    column("ID", Length::Fixed(60.0)),
    column("Name", Length::Fill),
    column("Status", Length::Fixed(120.0)),
    row(vec![
        Box::new(Text::new("001")),
        Box::new(Text::new("Primary Node").bold()),
        Box::new(Badge::new("Active").intent(Intent::Success)),
    ])
]"#
    .to_string();

    // --- 3. Component Documentation Object ---
    let doc = ComponentDoc::new(
        "DataTable",
        "A highly-optimized grid for displaying structured data with support for presets, column emphasis, and multi-kernel adapted rendering.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::Lab(LabMessage::SetRenderMode(mode)))
    .theory(
       "### High-Density Data\nData tables in PeakUI are more than just grids. They are semantic structures that handle complex layout logic across different display kernels.\n\n- **Kernel Agnostic**: The same `data_table!` DSL manifests as a rich interactive grid on Canvas, a character-based table in Terminal, and a structured array in Neural mode.\n- **Performance**: Built with virtualization in mind (planned) to handle thousands of rows without dropping frames."
    )
    .props_table(
        "| Modifier | Type | Description |\n| :--- | :--- | :--- |\n| `preset(p)` | `DataTablePreset` | Professional, Minimal, or Custom styles. |\n| `column(n, w)` | `&str, Length` | Defines a table column and its width. |\n| `sortable_column(n, w, f)` | `&str, Length, Fn` | Defines a sortable column with a callback. |\n| `row(cells)` | `Vec<Box<dyn View>>`| Adds a row of content. |\n| `row_with_action(cells, m)` | `Vec<...>, Msg` | Adds an interactive row. |\n| `show_grid(b)` | `bool` | Toggles internal grid lines. |"
    );

    AppPageResult::new(doc)
}

fn create_canvas_preview(_ctx: &Context) -> VStack<Message, IcedBackend> {
    let sample_rows = || {
        vec![
            ("001", "Primary Node C", "Active", "99.9%", Intent::Success),
            ("002", "Secondary Relay", "Standby", "98.5%", Intent::Info),
            ("003", "Storage Cluster", "Syncing", "100%", Intent::Warning),
        ]
    };

    vstack::<Message, IcedBackend>().spacing(32.0).push(
        vstack::<Message, IcedBackend>()
            .spacing(8.0)
            .push(text::<IcedBackend>("Sample Render").caption2().secondary())
            .push({
                let mut table = crate::data_table![
                    preset(DataTablePreset::Professional),
                    min_width(400.0),
                    column("ID", Length::Fixed(60.0)),
                ]
                .sortable_column("Name", Length::Fill, |asc| {
                    Message::Unknown(format!("Sort Name {}", if asc { "Asc" } else { "Desc" }))
                })
                .column("Status", Length::Fixed(100.0));

                for (id, name, status, _, intent) in sample_rows() {
                    table = table.row_with_action(
                        vec![
                            Box::new(text::<IcedBackend>(id).secondary())
                                as Box<dyn View<Message, IcedBackend> + Send + Sync>,
                            Box::new(text::<IcedBackend>(name).bold())
                                as Box<dyn View<Message, IcedBackend> + Send + Sync>,
                            Box::new(Badge::new(status).intent(intent))
                                as Box<dyn View<Message, IcedBackend> + Send + Sync>,
                        ],
                        Message::Unknown(format!("Selected {}", name)),
                    );
                }
                table
            }),
    )
}
