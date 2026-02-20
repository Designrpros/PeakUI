use crate::prelude::*;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DataTablePreset {
    #[default]
    Professional,
    Minimal,
    Glass,
}

pub struct DataTableColumn<M, B>
where
    M: 'static + Send + Sync,
    B: Backend + Send + Sync,
{
    pub label: String,
    pub width: Length,
    pub alignment: Alignment,
    pub is_sortable: bool,
    pub on_sort: Option<Arc<dyn Fn(bool) -> M + Send + Sync>>,
    pub _phantom: std::marker::PhantomData<B>,
}

pub struct DataTableRow<M, B>
where
    M: 'static + Send + Sync,
    B: Backend + Send + Sync,
{
    pub id: Option<String>,
    pub cells: Vec<Box<dyn View<M, B> + Send + Sync>>,
    pub on_press: Option<M>,
}

pub struct DataTable<M, B>
where
    M: 'static + Send + Sync,
    B: Backend + Send + Sync,
{
    pub columns: Vec<DataTableColumn<M, B>>,
    pub rows: Vec<DataTableRow<M, B>>,
    pub min_width: Option<f32>,
    pub preset: DataTablePreset,
    pub show_grid: bool,
    pub emphasize_first_column: bool,
    pub alternate_rows: bool,
    pub sort_column: Option<usize>,
    pub sort_ascending: bool,
    // Pagination
    pub page: usize,
    pub page_size: usize,
    pub total_rows: Option<usize>,
    pub on_page_change: Option<Arc<dyn Fn(usize) -> M + Send + Sync>>,
    // Selection
    pub selected_ids: HashSet<String>,
    pub on_selection_change: Option<Arc<dyn Fn(HashSet<String>) -> M + Send + Sync>>,
}

impl<M, B> DataTable<M, B>
where
    M: 'static + Clone + Send + Sync,
    B: Backend + Send + Sync + 'static,
{
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
            page: 1,
            page_size: 10,
            total_rows: None,
            on_page_change: None,
            selected_ids: HashSet::new(),
            on_selection_change: None,
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
            alignment: Alignment::Start,
            is_sortable: false,
            on_sort: None,
            _phantom: std::marker::PhantomData,
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
            alignment: Alignment::Start,
            is_sortable: true,
            on_sort: Some(Arc::new(on_sort)),
            _phantom: std::marker::PhantomData,
        });
        self
    }

    pub fn align_center(mut self) -> Self {
        if let Some(col) = self.columns.last_mut() {
            col.alignment = Alignment::Center;
        }
        self
    }

    pub fn align_end(mut self) -> Self {
        if let Some(col) = self.columns.last_mut() {
            col.alignment = Alignment::End;
        }
        self
    }

    pub fn row(mut self, cells: Vec<Box<dyn View<M, B> + Send + Sync>>) -> Self {
        self.rows.push(DataTableRow {
            id: None,
            cells,
            on_press: None,
        });
        self
    }

    pub fn row_with_id(
        mut self,
        id: impl Into<String>,
        cells: Vec<Box<dyn View<M, B> + Send + Sync>>,
    ) -> Self {
        self.rows.push(DataTableRow {
            id: Some(id.into()),
            cells,
            on_press: None,
        });
        self
    }

    pub fn row_with_action(
        mut self,
        cells: Vec<Box<dyn View<M, B> + Send + Sync>>,
        on_press: M,
    ) -> Self {
        self.rows.push(DataTableRow {
            id: None,
            cells,
            on_press: Some(on_press),
        });
        self
    }

    pub fn row_with_id_and_action(
        mut self,
        id: impl Into<String>,
        cells: Vec<Box<dyn View<M, B> + Send + Sync>>,
        on_press: M,
    ) -> Self {
        self.rows.push(DataTableRow {
            id: Some(id.into()),
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

    pub fn page(mut self, page: usize) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, size: usize) -> Self {
        self.page_size = size;
        self
    }

    pub fn total_rows(mut self, total: usize) -> Self {
        self.total_rows = Some(total);
        self
    }

    pub fn on_page_change(
        mut self,
        on_change: impl Fn(usize) -> M + Send + Sync + 'static,
    ) -> Self {
        self.on_page_change = Some(Arc::new(on_change));
        self
    }

    pub fn selected_ids(mut self, ids: HashSet<String>) -> Self {
        self.selected_ids = ids;
        self
    }

    pub fn on_selection_change(
        mut self,
        on_change: impl Fn(HashSet<String>) -> M + Send + Sync + 'static,
    ) -> Self {
        self.on_selection_change = Some(Arc::new(on_change));
        self
    }

    fn header_cell(
        &self,
        context: &Context,
        col_index: usize,
        col: &DataTableColumn<M, B>,
    ) -> B::AnyView<M> {
        let is_sorted = self.sort_column == Some(col_index);
        let align_x = match col.alignment {
            Alignment::Start => iced::Alignment::Start,
            Alignment::Center => iced::Alignment::Center,
            Alignment::End => iced::Alignment::End,
        };

        let mut content_children = Vec::new();

        // If alignment is End, we might want icon on the left?
        // For now, keep icon on right but align the whole HStack

        content_children.push(B::text(
            col.label.clone(),
            12.0,  // Caption1 size equivalent
            None,  // Inherit color
            true,  // Bold
            false, // Dim
            None,  // Intent
            None,  // Font
            Length::Shrink,
            iced::Alignment::Start,
            context,
        ));

        if is_sorted {
            content_children.push(B::icon(
                if self.sort_ascending {
                    "chevron-up".to_string()
                } else {
                    "chevron-down".to_string()
                },
                12.0,
                Some(context.theme.colors.primary),
                context,
            ));
        }

        let content = B::hstack(
            content_children,
            8.0,
            Padding::default(),
            Length::Shrink,
            Length::Shrink,
            iced::Alignment::Start,
            iced::Alignment::Center,
            context,
        );

        let cell = B::container(
            content,
            [12, 12].into(),
            col.width,
            Length::Shrink,
            None,
            0.0,
            0.0,
            None,
            None,
            align_x,
            iced::Alignment::Start,
            context,
        );

        if let Some(on_sort) = &col.on_sort {
            let msg = on_sort(is_sorted && !self.sort_ascending);
            B::button(
                cell,
                Some(msg),
                Variant::Ghost,
                Intent::Secondary,
                Length::Shrink,
                Length::Shrink,
                true,
                context,
            )
        } else {
            cell
        }
    }

    fn row_cell(
        &self,
        context: &Context,
        cell: &Box<dyn View<M, B> + Send + Sync>,
        width: Length,
        is_first: bool,
        alignment: Alignment,
    ) -> B::AnyView<M> {
        let palette = context.theme.colors;
        let emphasize = self.emphasize_first_column && is_first;
        let align_x = match alignment {
            Alignment::Start => iced::Alignment::Start,
            Alignment::Center => iced::Alignment::Center,
            Alignment::End => iced::Alignment::End,
        };

        B::container(
            cell.view(context),
            [12, 12].into(),
            width,
            Length::Shrink,
            if emphasize {
                Some(palette.primary.scale_alpha(0.05))
            } else {
                None
            },
            0.0,
            0.0,
            None,
            None,
            align_x,
            iced::Alignment::Center,
            context,
        )
    }
}

impl<M, B> View<M, B> for DataTable<M, B>
where
    M: 'static + Clone + Send + Sync,
    B: Backend + Send + Sync + 'static,
{
    fn view(&self, context: &Context) -> B::AnyView<M> {
        let theme = context.theme;
        let palette = theme.colors;

        let (header_bg, border_color, border_width, radius) = match self.preset {
            DataTablePreset::Professional => (
                Some(palette.surface.scale_alpha(0.3)),
                Some(palette.border.scale_alpha(0.1)),
                1.0,
                8.0,
            ),
            DataTablePreset::Minimal => (None, Some(palette.border.scale_alpha(0.1)), 0.0, 0.0),
            DataTablePreset::Glass => (
                Some(palette.surface.scale_alpha(0.1)),
                Some(palette.border.scale_alpha(0.2)),
                1.0,
                12.0,
            ),
        };

        let grid_color = palette.border.scale_alpha(0.15);

        // --- Render Header ---
        let mut header_children = Vec::new();

        // Selection Checkbox Header (All)
        if self.on_selection_change.is_some() {
            header_children.push(B::container(
                B::space(Length::Fixed(1.0), Length::Fixed(1.0), context), // Placeholder
                [12, 12].into(),
                Length::Fixed(40.0),
                Length::Shrink,
                None,
                0.0,
                0.0,
                None,
                None,
                iced::Alignment::Center,
                iced::Alignment::Center,
                context,
            ));
        }

        let col_count = self.columns.len();
        for (i, col) in self.columns.iter().enumerate() {
            header_children.push(self.header_cell(context, i, col));

            if self.show_grid && i < col_count - 1 {
                header_children.push(B::container(
                    B::space(Length::Fixed(0.0), Length::Fixed(0.0), context),
                    Padding::default(),
                    Length::Fixed(1.0),
                    Length::Fill,
                    Some(grid_color),
                    0.0,
                    0.0,
                    None,
                    None,
                    iced::Alignment::Start,
                    iced::Alignment::Start,
                    context,
                ));
            }
        }

        let header = B::hstack(
            header_children,
            0.0,
            Padding::default(),
            Length::Fill,
            Length::Shrink,
            iced::Alignment::Start,
            iced::Alignment::Center,
            context,
        );

        let header_container = B::container(
            header,
            Padding::default(),
            Length::Fill,
            Length::Shrink,
            header_bg,
            0.0,
            0.0,
            None,
            None,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        );

        // --- Render Rows ---
        let mut rows_children = Vec::new();
        let row_count = self.rows.len();

        // Header separator
        rows_children.push(B::container(
            B::space(Length::Fixed(0.0), Length::Fixed(0.0), context),
            Padding::default(),
            Length::Fill,
            Length::Fixed(1.0),
            Some(grid_color),
            0.0,
            0.0,
            None,
            None,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        ));

        for (i, row_data) in self.rows.iter().enumerate() {
            let mut row_children = Vec::new();

            // Selection Checkbox (Row)
            if let Some(on_select) = &self.on_selection_change {
                if let Some(id) = &row_data.id {
                    let is_selected = self.selected_ids.contains(id);
                    let id_clone = id.clone();
                    let on_select = on_select.clone();
                    let mut current_selection = self.selected_ids.clone();

                    let icon_name = if is_selected {
                        "check-square"
                    } else {
                        "square"
                    };
                    let icon_color = if is_selected {
                        Some(palette.primary)
                    } else {
                        Some(palette.text_primary.scale_alpha(0.4))
                    };

                    let icon = B::icon(icon_name.to_string(), 16.0, icon_color, context);

                    let msg = {
                        if is_selected {
                            current_selection.remove(&id_clone);
                        } else {
                            current_selection.insert(id_clone);
                        }
                        on_select(current_selection)
                    };

                    let checkbox_container = B::container(
                        icon,
                        [12, 12].into(),
                        Length::Fixed(40.0),
                        Length::Shrink,
                        None,
                        0.0,
                        0.0,
                        None,
                        None,
                        iced::Alignment::Center,
                        iced::Alignment::Center,
                        context,
                    );

                    row_children.push(B::button(
                        checkbox_container,
                        Some(msg),
                        Variant::Ghost,
                        Intent::Neutral,
                        Length::Shrink,
                        Length::Shrink,
                        true,
                        context,
                    ));
                } else {
                    row_children.push(B::container(
                        B::space(Length::Fixed(0.0), Length::Fixed(0.0), context),
                        Padding::default(),
                        Length::Fixed(40.0),
                        Length::Shrink,
                        None,
                        0.0,
                        0.0,
                        None,
                        None,
                        iced::Alignment::Start,
                        iced::Alignment::Start,
                        context,
                    ));
                }
            }

            for (j, cell) in row_data.cells.iter().enumerate() {
                let (width, alignment) = self
                    .columns
                    .get(j)
                    .map(|c| (c.width, c.alignment))
                    .unwrap_or((Length::Shrink, Alignment::Start));
                row_children.push(self.row_cell(context, cell, width, j == 0, alignment));

                if self.show_grid && j < col_count - 1 {
                    row_children.push(B::container(
                        B::space(Length::Fixed(0.0), Length::Fixed(0.0), context),
                        Padding::default(),
                        Length::Fixed(1.0),
                        Length::Fill,
                        Some(grid_color),
                        0.0,
                        0.0,
                        None,
                        None,
                        iced::Alignment::Start,
                        iced::Alignment::Start,
                        context,
                    ));
                }
            }

            let row_content = B::hstack(
                row_children,
                0.0,
                Padding::default(),
                Length::Fill,
                Length::Shrink,
                iced::Alignment::Start,
                iced::Alignment::Center,
                context,
            );

            let row_bg = if self.alternate_rows && i % 2 != 0 {
                Some(palette.surface.scale_alpha(0.1))
            } else {
                None
            };

            let row_container = B::container(
                row_content,
                Padding::default(),
                Length::Fill,
                Length::Shrink,
                row_bg,
                0.0,
                0.0,
                None,
                None,
                iced::Alignment::Start,
                iced::Alignment::Start,
                context,
            );

            if let Some(on_press) = &row_data.on_press {
                // If using explicit button, the container bg might be handled by the button style or wrapping
                rows_children.push(B::button(
                    row_container,
                    Some(on_press.clone()),
                    Variant::Plain,
                    Intent::Primary,
                    Length::Fill,
                    Length::Shrink,
                    true,
                    context,
                ));
            } else {
                rows_children.push(row_container);
            }

            // Add internal horizontal grid lines
            if i < row_count - 1 && self.show_grid {
                rows_children.push(B::container(
                    B::space(Length::Fixed(0.0), Length::Fixed(0.0), context),
                    Padding::default(),
                    Length::Fill,
                    Length::Fixed(1.0),
                    Some(grid_color),
                    0.0,
                    0.0,
                    None,
                    None,
                    iced::Alignment::Start,
                    iced::Alignment::Start,
                    context,
                ));
            }
        }

        let rows_content = B::vstack(
            rows_children,
            0.0,
            Padding::default(),
            Length::Fill,
            Length::Shrink,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        );

        let mut table_children = vec![header_container, rows_content];

        // --- Render Pagination Footer ---
        if let Some(total) = self.total_rows {
            if let Some(on_page) = &self.on_page_change {
                let total_pages = (total as f32 / self.page_size as f32).ceil() as usize;

                let start_idx = ((self.page - 1) * self.page_size) + 1;
                let end_idx = (self.page * self.page_size).min(total);

                let mut footer_children = Vec::new();

                footer_children.push(B::text(
                    format!("Showing {}-{} of {}", start_idx, end_idx, total),
                    12.0,
                    None,
                    false,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    iced::Alignment::Start,
                    context,
                ));

                // Spacer
                footer_children.push(B::space(Length::Fill, Length::Shrink, context));

                // Controls
                let mut controls_children = Vec::new();
                controls_children.push(B::button(
                    B::text(
                        "Previous".to_string(),
                        12.0,
                        None,
                        false,
                        false,
                        None,
                        None,
                        Length::Shrink,
                        iced::Alignment::Center,
                        context,
                    ),
                    if self.page > 1 {
                        Some(on_page(self.page - 1))
                    } else {
                        None
                    },
                    Variant::Ghost,
                    Intent::Secondary,
                    Length::Shrink,
                    Length::Shrink,
                    self.page > 1,
                    context,
                ));

                controls_children.push(B::text(
                    format!("Page {}", self.page),
                    12.0,
                    None,
                    false,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    iced::Alignment::Start,
                    context,
                ));

                controls_children.push(B::button(
                    B::text(
                        "Next".to_string(),
                        12.0,
                        None,
                        false,
                        false,
                        None,
                        None,
                        Length::Shrink,
                        iced::Alignment::Center,
                        context,
                    ),
                    if self.page < total_pages {
                        Some(on_page(self.page + 1))
                    } else {
                        None
                    },
                    Variant::Ghost,
                    Intent::Secondary,
                    Length::Shrink,
                    Length::Shrink,
                    self.page < total_pages,
                    context,
                ));

                footer_children.push(B::hstack(
                    controls_children,
                    8.0,
                    Padding::default(),
                    Length::Shrink,
                    Length::Shrink,
                    iced::Alignment::Start,
                    iced::Alignment::Center,
                    context,
                ));

                let footer = B::hstack(
                    footer_children,
                    0.0,
                    [12, 12].into(),
                    Length::Fill,
                    Length::Shrink,
                    iced::Alignment::Start,
                    iced::Alignment::Center,
                    context,
                );

                table_children.push(B::container(
                    B::space(Length::Fixed(0.0), Length::Fixed(0.0), context),
                    Padding::default(),
                    Length::Fill,
                    Length::Fixed(1.0),
                    Some(grid_color),
                    0.0,
                    0.0,
                    None,
                    None,
                    iced::Alignment::Start,
                    iced::Alignment::Start,
                    context,
                ));
                table_children.push(footer);
            }
        }

        let table_content = B::vstack(
            table_children,
            0.0,
            Padding::default(),
            if let Some(min_w) = self.min_width {
                Length::Fixed(min_w)
            } else {
                Length::Fill
            },
            Length::Shrink,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        );

        // Wrap in outer container with border and radius
        let final_table_content = if self.min_width.is_some() {
            B::scroll_view(
                table_content,
                Length::Fill,
                Length::Shrink,
                None,
                true,
                ScrollDirection::Horizontal,
                context,
            )
        } else {
            table_content
        };

        B::container(
            final_table_content,
            Padding::default(),
            Length::Fill,
            Length::Shrink,
            None,
            radius,
            border_width,
            border_color,
            None,
            iced::Alignment::Start,
            iced::Alignment::Start,
            context,
        )
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::IcedBackend;
    use std::collections::HashSet;

    #[derive(Debug, Clone, PartialEq)]
    enum TestMessage {
        PageChanged(usize),
        SelectionChanged(HashSet<String>),
    }

    #[test]
    fn test_datatable_defaults() {
        let table: DataTable<TestMessage, IcedBackend> = DataTable::new();
        assert_eq!(table.page, 1);
        assert_eq!(table.page_size, 10);
        assert!(table.selected_ids.is_empty());
    }

    #[test]
    fn test_pagination_builder() {
        let table: DataTable<TestMessage, IcedBackend> =
            DataTable::new().page(2).page_size(20).total_rows(100);

        assert_eq!(table.page, 2);
        assert_eq!(table.page_size, 20);
        assert_eq!(table.total_rows, Some(100));
    }

    #[test]
    fn test_selection_builder() {
        let mut selected = HashSet::new();
        selected.insert("row1".to_string());
        selected.insert("row2".to_string());

        let table: DataTable<TestMessage, IcedBackend> =
            DataTable::new().selected_ids(selected.clone());

        assert_eq!(table.selected_ids, selected);
    }

    #[test]
    fn test_callbacks_integration() {
        // Verify callbacks can be attached (compile-time check mainly)
        let _table: DataTable<TestMessage, IcedBackend> = DataTable::new()
            .on_page_change(|p| TestMessage::PageChanged(p))
            .on_selection_change(|ids| TestMessage::SelectionChanged(ids));
    }
}
