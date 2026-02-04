use crate::data_table;
use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::Message;

pub fn view(context: &Context) -> PageResult<Message> {
    // --- Sample Data Helper ---
    let sample_rows = || {
        vec![
            ("001", "Primary Node C", "Active", "99.9%", Intent::Success),
            ("002", "Secondary Relay", "Standby", "98.5%", Intent::Info),
            ("003", "Storage Cluster", "Syncing", "100%", Intent::Warning),
        ]
    };

    let title = "Data Table & Multi-Kernel DSL";
    let description = "PeakUI uses an expressive DSL to build professional interfaces that automatically adapt across UI, TUI, Semantic, and Spatial kernels.";
    let code_snippet = r#"data_table![
    preset(DataTablePreset::Professional),
    column("ID", Length::Fixed(60.0)),
    column("Name", Length::Fill),
    row(vec![...])
]"#;

    let preview = VStack::<Message, IcedBackend>::new_generic()
        .spacing(48.0)
        .width(Length::Fill)
        .push(
            VStack::new_generic()
                .spacing(24.0)
                .push(Text::new("1. Professional Preset (DSL Syntax)").title3())
                .push(Text::new("Using the `data_table!` macro for a clean, declarative declaration.").secondary())
                .push({
                    let mut table = data_table![
                        preset(DataTablePreset::Professional),
                        min_width(600.0),
                        column("ID", Length::Fixed(60.0)),
                        column("Name", Length::Fill),
                        column("Status", Length::Fixed(120.0)),
                        column("Uptime", Length::Fixed(100.0)),
                    ];

                    for (id, name, status, uptime, intent) in sample_rows() {
                        table = table.row(vec![
                            Box::new(Text::new(id).secondary()) as Box<dyn View<Message, IcedBackend>>,
                            Box::new(Text::new(name).bold()),
                            Box::new(Badge::new(status).intent(intent)),
                            Box::new(Text::new(uptime).footnote()),
                        ]);
                    }
                    table
                })
        )
        .push(
            VStack::new_generic()
                .spacing(24.0)
                .push(Text::new("2. Multi-Kernel View Logic").title3())
                .push(Text::new("The same DSL adaptively renders based on the active Kernel mode.").secondary())
                .push(
                    ResponsiveGrid::new()
                        .columns(2)
                        .spacing(20.0)
                        .push(kernel_card("UI Mode", "Rich graphics and glassmorphism (Desktop).", "monitor", context.mode == ShellMode::Desktop))
                        .push(kernel_card("TUI Mode", "High-performance terminal rendering (Console).", "terminal", context.mode == ShellMode::Console))
                        .push(kernel_card("Semantic Mode", "Direct neural/AI mapping (Server/LLM).", "brain", context.mode == ShellMode::Server))
                        .push(kernel_card("Spatial (AR/VR)", "Volumetric layouts (Spatial).", "layers", context.mode == ShellMode::Spatial))
                )
        )
        .push(
            VStack::new_generic()
                .spacing(24.0)
                .push(Text::new("3. Building Grids").title3())
                .push(Text::new("Using `ResponsiveGrid` for auto-layout cards that adapt to screen size.").secondary())
                .push(
                    ResponsiveGrid::new()
                        .columns(3)
                        .mobile_columns(1)
                        .spacing(16.0)
                        .push(GlassCard::new(Text::new("Grid Item 1").center()).padding(24.0))
                        .push(GlassCard::new(Text::new("Grid Item 2").center()).padding(24.0))
                        .push(GlassCard::new(Text::new("Grid Item 3").center()).padding(24.0))
                )
        )
        .push(
            VStack::new_generic()
                .spacing(24.0)
                .push(Text::new("4. Custom Grid & Emphasis").title3())
                .push(Text::new("Granular control over grid lines and column emphasis for high-density data.").secondary())
                .push({
                    let mut table = data_table![
                        preset(DataTablePreset::Minimal),
                        show_grid(true),
                        emphasize_first_column(true),
                        min_width(600.0),
                        column("ID", Length::Fixed(60.0)),
                        column("Name", Length::Fill),
                        column("Status", Length::Fixed(120.0)),
                    ];

                    for (id, name, status, _, intent) in sample_rows() {
                        table = table.row(vec![
                            Box::new(Text::new(id).secondary()) as Box<dyn View<Message, IcedBackend>>,
                            Box::new(Text::new(name).bold()),
                            Box::new(Badge::new(status).intent(intent)),
                        ]);
                    }
                    table
                })
        );

    // Terminal representation for "The Lab"
    let terminal =
        "DataTable[\n  Filter: Active\n  Columns: [ID, Name, Status, Uptime]\n  Rows: 3\n]"
            .to_string();

    // Neural representation for "The Lab"
    let neural = crate::core::SemanticNode {
        role: "data_table".to_string(),
        label: Some("Node Status Table".to_string()),
        children: vec![
            crate::core::SemanticNode {
                role: "row".to_string(),
                content: Some("Primary Node C (Active)".to_string()),
                ..Default::default()
            },
            crate::core::SemanticNode {
                role: "row".to_string(),
                content: Some("Secondary Relay (Standby)".to_string()),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    PageResult::new(
        crate::reference::views::ComponentDoc::new(
            title,
            description,
            code_snippet,
            std::sync::Arc::new(preview),
        )
        .terminal(terminal)
        .neural(neural)
        .on_render_mode_change(Message::SetRenderMode)
        .render_mode(
            context
                .is_inside_scrollable
                .then(|| crate::reference::app::RenderMode::Canvas)
                .unwrap_or(crate::reference::app::RenderMode::Canvas),
        ), // This is a bit hacky but works
    )
}

fn kernel_card(
    title: &str,
    desc: &str,
    icon: &str,
    active: bool,
) -> impl View<Message, IcedBackend> {
    let card_icon = Icon::new(icon).size(24.0);

    let content = HStack::new()
        .spacing(16.0)
        .align_y(Alignment::Center)
        .push(if active {
            Box::new(card_icon.primary_color()) as Box<dyn View<Message, IcedBackend>>
        } else {
            Box::new(card_icon) as Box<dyn View<Message, IcedBackend>>
        })
        .push(
            VStack::new()
                .spacing(4.0)
                .push(Text::new(title).bold())
                .push(Text::new(desc).caption2().secondary()),
        );

    GlassCard::new(content).padding(16.0)
}
