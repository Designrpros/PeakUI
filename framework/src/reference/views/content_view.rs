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
    pub is_thinking: bool,
    pub chat_messages: Vec<crate::views::chat::ChatMessage>,
    pub chat_input: String,
    pub sidebar_width: f32,
    pub inspector_width: f32,
    pub is_resizing_sidebar: bool,
    pub is_resizing_inspector: bool,
    pub inspector_tab: super::super::app::InspectorTab,

    // Chat State for Global Inspector
    pub api_key: String,
    pub ai_provider: super::super::app::AIProviderChoice,
    pub icon_limit: usize,
    pub pending_sudo_action: Option<super::super::app::SudoAction>,
    pub db_records: Vec<crate::core::SemanticRecord>,
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
            is_thinking: app.is_thinking,
            chat_messages: app.chat_messages.clone(),
            chat_input: app.chat_input.clone(),
            sidebar_width: app.sidebar_width,
            inspector_width: app.inspector_width,
            is_resizing_sidebar: app.is_resizing_sidebar,
            is_resizing_inspector: app.is_resizing_inspector,
            inspector_tab: app.inspector_tab,
            api_key: app.api_key.clone(),
            ai_provider: app.ai_provider,
            icon_limit: app.icon_limit,
            pending_sudo_action: app.pending_sudo_action.clone(),
            db_records: app.db.get_all(),
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
            self.api_key.clone(),
            self.ai_provider,
            self.search_query.clone(),
            self.icon_limit,
            self.db_records.clone(),
        );

        let sidebar = SidebarView::new(
            self.active_tab.clone(),
            self.navigation_mode.clone(),
            self.expanded_sections.clone(),
        );
        let tabbar = TabBarView::new(self.navigation_mode.clone());

        // --- 2. Main Layout (Three-Column Split) ---
        // Render page base result
        let mut page = canvas_manager.render_page(context);

        // Wrap the page view in a ProxyView that applies the content-specific safe area offsets.
        // This ensures the sidebar and header (shell) stay tight to the edges,
        // while the page content naturally clears them.
        let inner_page_view = page.view;
        let content_view = crate::core::ProxyView::new(move |ctx| {
            let mut content_context = ctx.clone();
            content_context.safe_area.top += 48.0; // Clear floating header
            content_context.safe_area.bottom += 80.0; // Clear floating dock

            let view = inner_page_view.view(&content_context);

            // Apply safe area as physical padding
            IcedBackend::container(
                view,
                Padding {
                    top: content_context.safe_area.top,
                    bottom: content_context.safe_area.bottom,
                    left: 0.0,
                    right: 0.0,
                },
                Length::Fill,
                Length::Shrink,
                None,
                0.0,
                0.0,
                None,
                None,
                Alignment::Start,
                Alignment::Start,
                &content_context,
            )
        });

        // --- 3. Integrated Header --
        let _ = (); // Discard old notch content logic variable

        let query = self.search_query.clone();
        let show_inspector = self.show_inspector;
        let sidebar_toggle = page.sidebar_toggle.take();
        let toolbar_items = std::mem::take(&mut page.toolbar_items);
        let search_config = page.search_config.clone();

        let header_view = crate::core::ProxyView::new(move |context| {
            let query = query.clone();

            // Main Header Row (Single Row for Left Alignment)
            // Use the dynamic safe area top padding
            let top_pad = context.safe_area.top;

            let mut header_row = iced::widget::row!()
                .padding(Padding {
                    top: top_pad, // Use safe area directly
                    right: 24.0,
                    bottom: 12.0,
                    left: 24.0,
                })
                .spacing(16)
                .align_y(Alignment::Center)
                .width(Length::Fill);

            // 1. Sidebar Toggle (Mobile Only)
            if is_mobile {
                if let Some(toggle_msg) = sidebar_toggle.clone() {
                    header_row = header_row.push(
                        ToolbarItem::new()
                            .icon("chevron-left") // Stack Navigation: "Back" to Sidebar
                            .on_press(toggle_msg)
                            .view(context),
                    );
                }
            }

            // 2. Search Field (Conditioned on Page)
            if let Some(config) = search_config.clone() {
                let search_input =
                    TextInput::<Message>::new(query.clone(), &config.placeholder, |s| {
                        Message::Search(s)
                    })
                    .variant(Variant::Ghost)
                    .neural("header-search")
                    .view(context);

                // Styled "Search Field" Container
                let search_container = container(
                    iced::widget::row!()
                        .spacing(8)
                        .align_y(Alignment::Center)
                        .push(
                            Icon::<IcedBackend>::new("search")
                                .size(14.0)
                                .secondary()
                                .view(context),
                        )
                        .push(search_input),
                )
                .padding([6, 12])
                .width(if is_mobile {
                    Length::Fill
                } else {
                    Length::Fixed(320.0)
                })
                .max_width(320.0)
                .style(move |theme: &iced::Theme| {
                    let palette = theme.extended_palette();
                    container::Style {
                        // "remove the background" - User Request
                        background: None,
                        // "only a border radius searchfield should be visible"
                        border: Border {
                            radius: 8.0.into(),
                            width: 1.0,
                            color: palette.background.strong.color.scale_alpha(0.2),
                        },
                        ..Default::default()
                    }
                });

                header_row = header_row.push(search_container);
            }

            // 3. Spacer (Push everything else right)
            header_row = header_row.push(iced::widget::Space::new().width(Length::Fill));

            // 4. Page Toolbar Items
            for item in toolbar_items.iter() {
                header_row = header_row.push(item.view(context));
            }

            // 5. Inspector Toggle
            header_row = header_row.push(
                ToolbarItem::new()
                    .icon("panel-right-open")
                    .active(show_inspector)
                    .on_press(Message::ToggleInspector)
                    .view(context),
            );

            // Transparent Floating Header Container
            container(header_row)
                .width(Length::Fill)
                .height(Length::Shrink) // ✅ Only take space needed, don't block clicks below
                .style(move |_| container::Style {
                    background: None,
                    border: Border::default(),
                    shadow: Shadow::default(),
                    ..Default::default()
                })
                .into()
        });

        // Combine Header + Content using ZStack for "Floating" effect
        // ScrollView is at the bottom (first), Header is on top (second).
        let content_layout = crate::layout::ZStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(ScrollView::new(content_view))
            .push(header_view);

        let mut split_view = NavigationSplitView::new(sidebar, content_layout)
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

        if self.show_inspector {
            // Determine available inspectors
            let ai_inspector = crate::views::chat::AIChatView::new(
                self.chat_messages.clone(),
                self.chat_input.clone(),
                self.is_thinking,
                Message::Chat,
            );

            // Logic:
            // 1. If only page inspector -> show page inspector
            // 2. If only AI inspector -> show AI inspector (default if no page inspector)
            // 3. If both -> show Tab Bar + Selected Content

            let inspector_content: Box<dyn View<Message, IcedBackend>> = if let Some(p_inspector) =
                page.inspector.take()
            // Take ownership since page is mutable
            {
                // Both available - Show Tabs
                let tab_bar = HStack::new()
                    .width(Length::Fill)
                    .padding(16.0)
                    .spacing(8.0)
                    .push(
                        Button::label("Feature")
                            .variant(
                                if self.inspector_tab == super::super::app::InspectorTab::Feature {
                                    Variant::Soft
                                } else {
                                    Variant::Ghost
                                },
                            )
                            .width(Length::Shrink)
                            .on_press(Message::SetInspectorTab(
                                super::super::app::InspectorTab::Feature,
                            )),
                    )
                    .push(
                        Button::label("Assistant")
                            .variant(
                                if self.inspector_tab == super::super::app::InspectorTab::App {
                                    Variant::Soft
                                } else {
                                    Variant::Ghost
                                },
                            )
                            .width(Length::Shrink)
                            .on_press(Message::SetInspectorTab(
                                super::super::app::InspectorTab::App,
                            )),
                    );

                let content: Box<dyn View<Message, IcedBackend>> = match self.inspector_tab {
                    super::super::app::InspectorTab::Feature => Box::new(p_inspector),
                    super::super::app::InspectorTab::App => Box::new(ai_inspector),
                };

                Box::new(
                    VStack::new()
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .push(tab_bar)
                        .push(content),
                )
            } else {
                // Only AI
                Box::new(ai_inspector)
            };

            split_view = split_view
                .inspector(inspector_content)
                .on_dismiss_inspector(Message::ToggleInspector);
        }

        // --- 4. Final Assembly ---
        // Unified Floating Dock: Both mobile and desktop use stack layout for a consistent floating dock experience
        let final_view: Element<'static, Message> = iced::widget::stack![
            split_view.view(context),
            container(tabbar.view(context))
                .width(Length::Fill)
                .height(Length::Fill) // Need full height for align_y to position at bottom
                .align_x(Alignment::Center)
                .align_y(Alignment::End)
                .padding(Padding {
                    top: 0.0,
                    right: 20.0,
                    bottom: context.safe_area.bottom,
                    left: 20.0,
                })
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
            self.api_key.clone(),
            self.ai_provider,
            self.search_query.clone(),
            self.icon_limit,
            self.db_records.clone(),
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
            children: vec![split_view.describe(context)],
            ..Default::default()
        }
    }
}
