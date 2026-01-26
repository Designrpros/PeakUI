use super::super::app::{App, Message};
use super::super::model::Page;
use super::{CanvasView, SidebarView, TabBarView};
use crate::nav_split_view::NavigationSplitView;
use crate::prelude::*;

pub struct ContentView {
    pub active_tab: Page,
    pub show_search: bool,
    pub show_inspector: bool,
    pub show_sidebar: bool,
    pub show_user_profile: bool,
    pub navigation_mode: String,
    pub search_query: String,
    pub expanded_sections: std::collections::HashSet<String>,
    pub button_lab: super::super::app::ButtonLabState,
    pub typography_lab: super::super::app::TypographyLabState,
    pub layout_lab: super::super::app::LayoutLabState,
    pub sizing_lab: super::super::app::SizingLabState,
    pub render_mode: super::super::app::RenderMode,
    pub sidebar_width: f32,
    pub inspector_width: f32,
    pub is_resizing_sidebar: bool,
    pub is_resizing_inspector: bool,
}

impl ContentView {
    pub fn new(app: &App) -> Self {
        Self {
            active_tab: app.active_tab.clone(), // Now it is Page
            show_search: app.show_search,
            show_inspector: app.show_inspector,
            show_sidebar: app.show_sidebar,
            show_user_profile: app.show_user_profile,
            navigation_mode: app.navigation_mode.clone(),
            search_query: app.search_query.clone(),
            expanded_sections: app.expanded_sections.clone(),
            button_lab: app.button_lab.clone(),
            typography_lab: app.typography_lab.clone(),
            layout_lab: app.layout_lab.clone(),
            sizing_lab: app.sizing_lab.clone(),
            render_mode: app.render_mode,
            sidebar_width: app.sidebar_width,
            inspector_width: app.inspector_width,
            is_resizing_sidebar: app.is_resizing_sidebar,
            is_resizing_inspector: app.is_resizing_inspector,
        }
    }

    pub fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let is_mobile = context.is_slim();

        // --- 1. Sub-Views (Data Collection) ---
        let canvas_manager = CanvasView::new(
            self.active_tab.clone(),
            self.navigation_mode.clone(),
            self.button_lab.clone(),
            self.typography_lab.clone(),
            self.layout_lab.clone(),
            self.sizing_lab.clone(),
            self.render_mode,
        );

        let sidebar = SidebarView::new(
            self.active_tab.clone(),
            self.navigation_mode.clone(),
            self.expanded_sections.clone(),
        );
        let tabbar = TabBarView::new(self.navigation_mode.clone());

        // --- 2. Main Layout (Three-Column Split) ---
        // We inject this into the context so ScrollViews can respect it.
        let mut content_context = context.clone();

        // Define standard safe areas
        let top_safe = if is_mobile { 40.0 } else { 80.0 };
        let bottom_safe = if is_mobile { 40.0 } else { 100.0 };

        content_context = content_context.with_safe_area(Padding {
            top: top_safe,       // Protect from the notch bar / top edge
            bottom: bottom_safe, // Protect from the floating dock / bottom edge
            ..context.safe_area
        });

        // Render page with the safe-area-aware context
        let page = canvas_manager.render_page(&content_context);

        let mut split_view = NavigationSplitView::new(sidebar, ScrollView::from_boxed(page.view))
            .force_sidebar_on_slim(self.show_sidebar && is_mobile)
            .sidebar_width(self.sidebar_width)
            .inspector_width(self.inspector_width)
            .on_resize_sidebar(|w| Message::ResizeSidebar(w))
            .on_resize_inspector(|w| Message::ResizeInspector(w))
            .on_start_resize_sidebar(Message::StartResizingSidebar)
            .on_stop_resize_sidebar(Message::StopResizingSidebar)
            .on_start_resize_inspector(Message::StartResizingInspector)
            .on_stop_resize_inspector(Message::StopResizingInspector)
            .is_resizing_sidebar(self.is_resizing_sidebar)
            .is_resizing_inspector(self.is_resizing_inspector)
            .on_none(Message::None);

        // Decide which inspector to show: strictly page-specific
        let has_inspector = page.inspector.is_some();
        let active_inspector = page.inspector;

        if self.show_inspector {
            if let Some(inspector) = active_inspector {
                split_view = split_view.inspector(inspector);
            }
        }

        // --- 3. UI Overlays (The Dynamic Notch) ---
        let show_search = self.show_search;
        let show_inspector = self.show_inspector;
        let query = self.search_query.clone();

        let mut notch_content = VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Shrink)
            .spacing(12.0);

        if show_search {
            // Expanded Notch
            notch_content = notch_content.push(
                VStack::<Message, IcedBackend>::new_generic()
                    .spacing(16.0)
                    .push(
                        HStack::<Message, IcedBackend>::new_generic()
                            .spacing(12.0)
                            .align_y(Alignment::Center)
                            .push(Icon::<IcedBackend>::new("search").secondary())
                            .push(TextInput::<Message>::new(
                                query.clone(),
                                "Search docs...",
                                |s| Message::Search(s),
                            ))
                            .push(ToolbarItem::new().icon("x").on_press(Message::ToggleSearch)),
                    )
                    .push(if !query.is_empty() {
                        // SEARCH: Results should ideally be driven by search_config or a dedicated results provider
                        // For now, we remove the hardcoded global results as requested.
                        VStack::<Message, IcedBackend>::new_generic().push(
                            Text::<IcedBackend>::new("Results will appear here...")
                                .caption1()
                                .secondary(),
                        )
                    } else {
                        VStack::<Message, IcedBackend>::new_generic().push(
                            Text::<IcedBackend>::new("Start typing to search...")
                                .caption1()
                                .secondary(),
                        )
                    }),
            );
        } else {
            // Idle Pill Notch
            let mut notch_row = HStack::<Message, IcedBackend>::new_generic()
                .width(Length::Shrink)
                .spacing(24.0)
                .align_y(Alignment::Center);

            // SIDEBAR: Show burger menu if sidebar_toggle is set AND we are on mobile
            if let Some(toggle_msg) = page.sidebar_toggle {
                if is_mobile {
                    notch_row = notch_row.push(
                        ToolbarItem::new()
                            .icon("menu")
                            .active(self.show_sidebar)
                            .on_press(toggle_msg),
                    );
                }
            }

            // SEARCH: Only show search icon if the page is searchable
            if page.search_config.is_some() {
                notch_row = notch_row.push(
                    ToolbarItem::new()
                        .icon("search")
                        .on_press(Message::ToggleSearch),
                );
            }

            // AUTO-MERGE: Add page-specific toolbar items
            for item in page.toolbar_items {
                notch_row = notch_row.push(item);
            }

            // Global Inspector Toggle (only if page has an inspector)
            if has_inspector {
                notch_row = notch_row.push(
                    ToolbarItem::new()
                        .icon("sidebar")
                        .active(show_inspector)
                        .on_press(Message::ToggleInspector),
                );
            }

            notch_content = notch_content.push(notch_row);
        }

        let peak_theme = context.theme;
        let notch_container = container(notch_content.view(context))
            .padding(if show_search {
                Padding::from(16)
            } else {
                Padding::from([8, 20])
            })
            .width(if show_search {
                Length::Fixed(480.0)
            } else {
                Length::Shrink
            })
            .align_x(Alignment::Center)
            .style(move |_| {
                let bg_color = peak_theme.colors.surface;
                let border_color = peak_theme.colors.border.scale_alpha(0.1);
                container::Style {
                    background: Some(bg_color.into()),
                    border: Border {
                        radius: 32.0.into(),
                        color: border_color,
                        width: 1.0,
                    },
                    shadow: Shadow {
                        color: Color {
                            a: 0.1,
                            ..Color::BLACK
                        },
                        offset: Vector::new(0.0, 4.0),
                        blur_radius: 12.0,
                    },
                    ..Default::default()
                }
            });

        // --- 4. Final Assembly ---
        let final_view: Element<'static, Message> = stack![
            split_view.view(&content_context),
            // Floating Notch Bar
            container(notch_container)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::End)
                .align_y(Alignment::Start)
                .padding(12),
            // Bottom Dock
            container(
                HStack::<Message, IcedBackend>::new_generic()
                    .width(Length::Fill)
                    .push(Space::<IcedBackend>::new(Length::Fill, 0.0.into()))
                    .push(tabbar)
                    .push(Space::<IcedBackend>::new(Length::Fill, 0.0.into()))
                    .view(context)
            )
            .padding(Padding {
                top: 0.0,
                right: 20.0,
                bottom: 32.0,
                left: 20.0
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Alignment::End),
        ]
        .into();

        final_view
    }

    pub fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        let is_mobile = context.is_slim();

        let canvas_manager = CanvasView::new(
            self.active_tab.clone(),
            self.navigation_mode.clone(),
            self.button_lab.clone(),
            self.typography_lab.clone(),
            self.layout_lab.clone(),
            self.sizing_lab.clone(),
            self.render_mode,
        );

        let sidebar = SidebarView::new(
            self.active_tab.clone(),
            self.navigation_mode.clone(),
            self.expanded_sections.clone(),
        );

        let page = canvas_manager.render_page(context);

        let mut split_view = NavigationSplitView::new(sidebar, ScrollView::from_boxed(page.view))
            .force_sidebar_on_slim(self.show_sidebar && is_mobile)
            .sidebar_width(self.sidebar_width)
            .inspector_width(self.inspector_width);

        if self.show_inspector {
            if let Some(inspector) = page.inspector {
                split_view = split_view.inspector(inspector);
            }
        }

        crate::core::SemanticNode {
            role: "content_view".to_string(),
            label: Some(format!("Page: {:?}", self.active_tab)),
            content: None,
            children: vec![split_view.describe(context)],
            neural_tag: None,
            documentation: None,
        }
    }
}
