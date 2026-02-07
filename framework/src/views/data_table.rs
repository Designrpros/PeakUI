use crate::prelude::*;
use iced::widget::container;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DataTablePreset {
    #[default]
    Professional,
    Minimal,
    Glass,
}

pub struct DataTableColumn<M> {
    pub label: String,
    pub width: Length,
    pub is_sortable: bool,
    pub on_sort: Option<Arc<dyn Fn(bool) -> M + Send + Sync>>,
}

pub struct DataTableRow<M> {
    pub cells: Vec<Box<dyn View<M, IcedBackend>>>,
    pub on_press: Option<M>,
}

pub struct DataTable<M> {
    pub columns: Vec<DataTableColumn<M>>,
    pub rows: Vec<DataTableRow<M>>,
    pub min_width: Option<f32>,
    pub preset: DataTablePreset,
    pub show_grid: bool,
    pub emphasize_first_column: bool,
    pub alternate_rows: bool,
    pub sort_column: Option<usize>,
    pub sort_ascending: bool,
}

impl<M: 'static + Clone> DataTable<M> {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            min_width: None,
            preset: DataTablePreset::default(),
            show_grid: false,
            emphasize_first_column: false,
            alternate_rows: true,
            sort_column: None,
            sort_ascending: true,
        }
    }

    pub fn preset(mut self, preset: DataTablePreset) -> Self {
        self.preset = preset;
        self
    }

    pub fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    pub fn emphasize_first_column(mut self, emphasize: bool) -> Self {
        self.emphasize_first_column = emphasize;
        self
    }

    pub fn alternate_rows(mut self, alternate: bool) -> Self {
        self.alternate_rows = alternate;
        self
    }

    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    pub fn column(mut self, label: impl Into<String>, width: Length) -> Self {
        self.columns.push(DataTableColumn {
            label: label.into(),
            width,
            is_sortable: false,
            on_sort: None,
        });
        self
    }

    pub fn sortable_column(
        mut self,
        label: impl Into<String>,
        width: Length,
        on_sort: impl Fn(bool) -> M + Send + Sync + 'static,
    ) -> Self {
        self.columns.push(DataTableColumn {
            label: label.into(),
            width,
            is_sortable: true,
            on_sort: Some(Arc::new(on_sort)),
        });
        self
    }

    pub fn row(mut self, cells: Vec<Box<dyn View<M, IcedBackend>>>) -> Self {
        self.rows.push(DataTableRow {
            cells,
            on_press: None,
        });
        self
    }

    pub fn row_with_action(
        mut self,
        cells: Vec<Box<dyn View<M, IcedBackend>>>,
        on_press: M,
    ) -> Self {
        self.rows.push(DataTableRow {
            cells,
            on_press: Some(on_press),
        });
        self
    }

    pub fn sort(mut self, column_index: usize, ascending: bool) -> Self {
        self.sort_column = Some(column_index);
        self.sort_ascending = ascending;
        self
    }

    fn header_cell(
        &self,
        context: &Context,
        col_index: usize,
        col: &DataTableColumn<M>,
    ) -> Element<'static, M, Theme, Renderer> {
        let is_sorted = self.sort_column == Some(col_index);

        let mut content = HStack::new()
            .spacing(8.0)
            .align_y(iced::Alignment::Center)
            .push(
                Text::<IcedBackend>::new(&col.label)
                    .caption1()
                    .bold()
                    .secondary(),
            );

        if is_sorted {
            content = content.push(
                Icon::<IcedBackend>::new(if self.sort_ascending {
                    "chevron-up"
                } else {
                    "chevron-down"
                })
                .size(12.0)
                .color(context.theme.colors.primary),
            );
        }

        let cell = container(content.view(context))
            .width(col.width)
            .padding([12, 12]);

        if let Some(on_sort) = &col.on_sort {
            let msg = on_sort(is_sorted && !self.sort_ascending);
            IcedBackend::button(
                cell.into(),
                Some(msg),
                Variant::Ghost,
                Intent::Secondary,
                Length::Shrink,
                Length::Shrink,
                true,
                context,
            )
        } else {
            cell.into()
        }
    }

    fn row_cell(
        &self,
        context: &Context,
        cell: &Box<dyn View<M, IcedBackend>>,
        width: Length,
        is_first: bool,
    ) -> Element<'static, M, Theme, Renderer> {
        let palette = context.theme.colors;
        let emphasize = self.emphasize_first_column && is_first;

        container(cell.view(context))
            .width(width)
            .padding([12, 12])
            .align_y(iced::Alignment::Center)
            .style(move |_| container::Style {
                background: if emphasize {
                    Some(palette.primary.scale_alpha(0.05).into())
                } else {
                    None
                },
                ..Default::default()
            })
            .into()
    }
}

impl<M: 'static + Clone> View<M, IcedBackend> for DataTable<M> {
    fn view(&self, context: &Context) -> Element<'static, M, Theme, Renderer> {
        let theme = context.theme;
        let palette = theme.colors;

        let (header_bg, border_color, border_width, radius) = match self.preset {
            DataTablePreset::Professional => (
                Some(palette.surface.scale_alpha(0.3).into()),
                palette.border.scale_alpha(0.1),
                1.0,
                8.0,
            ),
            DataTablePreset::Minimal => (None, palette.border.scale_alpha(0.1), 0.0, 0.0),
            DataTablePreset::Glass => (
                Some(palette.surface.scale_alpha(0.1).into()),
                palette.border.scale_alpha(0.2),
                1.0,
                12.0,
            ),
        };

        let grid_color = palette.border.scale_alpha(0.15);

        // --- Render Header ---
        let mut header_row = iced::widget::row!()
            .spacing(0)
            .align_y(iced::Alignment::Center);
        let col_count = self.columns.len();
        for (i, col) in self.columns.iter().enumerate() {
            header_row = header_row.push(self.header_cell(context, i, col));

            if self.show_grid && i < col_count - 1 {
                header_row = header_row.push(
                    container(iced::widget::Space::new())
                        .width(Length::Fixed(1.0))
                        .height(Length::Fill)
                        .style(move |_| container::Style {
                            background: Some(grid_color.into()),
                            ..Default::default()
                        }),
                );
            }
        }

        let header = container(header_row)
            .width(Length::Fill)
            .style(move |_| container::Style {
                background: header_bg,
                ..Default::default()
            });

        // --- Render Rows ---
        let mut rows_column = iced::widget::column!().spacing(0);
        let row_count = self.rows.len();
        let alternate_rows = self.alternate_rows;

        // Header separator (always present if we have a header)
        rows_column = rows_column.push(
            container(iced::widget::Space::new())
                .width(Length::Fill)
                .height(Length::Fixed(1.0))
                .style(move |_| container::Style {
                    background: Some(grid_color.into()),
                    ..Default::default()
                }),
        );

        for (i, row_data) in self.rows.iter().enumerate() {
            let mut row_ui = iced::widget::row!()
                .spacing(0)
                .align_y(iced::Alignment::Center);
            for (j, cell) in row_data.cells.iter().enumerate() {
                let width = self
                    .columns
                    .get(j)
                    .map(|c| c.width)
                    .unwrap_or(Length::Shrink);
                row_ui = row_ui.push(self.row_cell(context, cell, width, j == 0));

                if self.show_grid && j < col_count - 1 {
                    row_ui = row_ui.push(
                        container(iced::widget::Space::new())
                            .width(Length::Fixed(1.0))
                            .height(Length::Fill)
                            .style(move |_| container::Style {
                                background: Some(grid_color.into()),
                                ..Default::default()
                            }),
                    );
                }
            }

            let mut row_container =
                container(row_ui)
                    .width(Length::Fill)
                    .style(move |_| container::Style {
                        background: if alternate_rows && i % 2 != 0 {
                            Some(palette.surface.scale_alpha(0.1).into())
                        } else {
                            None
                        },
                        ..Default::default()
                    });

            if let Some(on_press) = &row_data.on_press {
                row_container = row_container.style(move |_t| {
                    let mut s = container::Style::default();
                    if alternate_rows && i % 2 != 0 {
                        s.background = Some(palette.surface.scale_alpha(0.1).into());
                    }
                    s
                });

                rows_column = rows_column.push(IcedBackend::button(
                    row_container.into(),
                    Some(on_press.clone()),
                    Variant::Plain,
                    Intent::Primary,
                    Length::Fill,
                    Length::Shrink,
                    true,
                    context,
                ));
            } else {
                rows_column = rows_column.push(row_container);
            }

            // Add internal horizontal grid lines
            if i < row_count - 1 && self.show_grid {
                rows_column = rows_column.push(
                    container(iced::widget::Space::new())
                        .width(Length::Fill)
                        .height(Length::Fixed(1.0))
                        .style(move |_| container::Style {
                            background: Some(grid_color.into()),
                            ..Default::default()
                        }),
                );
            }
        }

        let mut table_content = iced::widget::column![header, rows_column].width(Length::Fill);

        if let Some(min_w) = self.min_width {
            table_content = table_content.width(Length::Fixed(min_w));
        }

        // Wrap in outer container with border and radius
        let final_table_content: iced::Element<'static, M, Theme, Renderer> =
            if self.min_width.is_some() {
                IcedBackend::scroll_view(
                    table_content.into(),
                    Length::Fill,
                    Length::Shrink, // We want it to be as high as its content, but scrollable horizontally
                    None,
                    true, // Show indicators for horizontal scrolling
                    ScrollDirection::Horizontal,
                    context,
                )
            } else {
                table_content.into()
            };

        container(final_table_content)
            .width(Length::Fill)
            .style(move |_| container::Style {
                border: Border {
                    width: border_width,
                    color: border_color,
                    radius: radius.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn describe(&self, context: &Context) -> SemanticNode {
        let rows = self.rows.iter().map(|r| {
            SemanticNode::new("row").extend_children(r.cells.iter().map(|c| c.describe(context)))
        });

        SemanticNode::new("data_table")
            .with_label(format!(
                "Table with {} columns and {} rows",
                self.columns.len(),
                self.rows.len()
            ))
            .extend_children(rows)
    }
}
