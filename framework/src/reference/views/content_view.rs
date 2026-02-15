use super::super::app::{App, Message};
use super::{CanvasView, SidebarView, TabBarView};
use crate::layout::nav_split_view::NavigationSplitView;
use crate::prelude::*;

use super::state::ViewState;

#[derive(Clone)]
pub struct ContentView {
    pub state: ViewState,
}

impl View<Message, IcedBackend> for ContentView {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        self.view(context)
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        self.describe(context)
    }
}

impl ContentView {
    pub fn new(app: &App) -> Self {
        Self {
            state: ViewState::new(app),
        }
    }

    pub fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let is_mobile = context.is_slim();

        // --- 1. Sub-Views (Data Collection) ---
        let canvas_manager = CanvasView::new(self.state.clone());

        let sidebar = SidebarView::new(
            self.state.active_tab.clone(),
            self.state.navigation_mode.clone(),
        );
        let tab_bar = TabBarView::new(self.state.navigation_mode.clone());

        // --- 2. Main Layout (Three-Column Split) ---
        // Render page base result
        let mut page = canvas_manager.render_page(context);

        // Wrap the page view in a ProxyView that applies the content-specific safe area offsets.
        // This ensures the sidebar and header (shell) stay tight to the edges,
        // while the page content naturally clears them.
        let inner_page_view = page.view;
        let content_view = crate::core::ProxyView::new(move |ctx| {
            let mut content_context = ctx.clone();
            content_context.safe_area.top += 84.0; // Optimal clear floating header
            content_context.safe_area.bottom += 80.0; // Clear floating dock

            let view = inner_page_view.view(&content_context);

            // Apply horizontal safe area as physical padding.
            // Verticals are handled by components using ctx.safe_area.
            IcedBackend::container(
                view,
                Padding {
                    top: 0.0,
                    bottom: 0.0,
                    left: content_context.safe_area.left,
                    right: content_context.safe_area.right,
                },
                Length::Fill,
                Length::Fill,
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

        let query = self.state.search_query.clone();
        let show_inspector = self.state.show_inspector;
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

            // 2. Search Field (Conditioned on AppPage)
            if let Some(config) = search_config.clone() {
                let search_input =
                    TextInput::<Message>::new(query.clone(), config.placeholder.clone(), |s| {
                        Message::Search(s)
                    })
                    .variant(Variant::Ghost)
                    .neural_tag("header-search")
                    .view(context);

                let colors = &context.theme.colors;
                let search_container = IcedBackend::container(
                    iced::widget::row!()
                        .spacing(8)
                        .align_y(iced::Alignment::Center)
                        .push(
                            Icon::<IcedBackend>::new("search")
                                .size(14.0)
                                .secondary()
                                .view(context),
                        )
                        .push(search_input)
                        .into(),
                    iced::Padding::from([6, 12]),
                    if is_mobile {
                        Length::Fill
                    } else {
                        Length::Fixed(320.0)
                    },
                    Length::Shrink,
                    None,                                 // Transparent
                    1.0,                                  // border width
                    8.0,                                  // radius
                    Some(colors.border.scale_alpha(0.6)), // Using border color with alpha
                    None,
                    iced::Alignment::Start,
                    iced::Alignment::Start,
                    context,
                );

                header_row = header_row.push(search_container);
            }

            // 3. Spacer (Push everything else right)
            header_row = header_row.push(iced::widget::Space::new().width(Length::Fill));

            // 4. AppPage Toolbar Items
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
            IcedBackend::container(
                header_row.into(),
                iced::Padding::ZERO,
                Length::Fill,
                Length::Shrink,
                None,
                0.0,
                0.0,
                None,
                None,
                iced::Alignment::Start,
                iced::Alignment::Start,
                context,
            )
        });

        // Combine Header + Content using ZStack for "Floating" effect
        // ScrollView is at the bottom (first), Header is on top (second).
        let content_layout = crate::layout::ZStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(ScrollView::new(content_view))
            .push(header_view);

        let mut split_view = NavigationSplitView::new(sidebar, content_layout)
            .force_sidebar_on_slim(self.state.show_sidebar && is_mobile)
            .sidebar_width(self.state.sidebar_width)
            .inspector_width(self.state.inspector_width)
            .on_resize_sidebar(|w| Message::ResizeSidebar(w))
            .on_resize_inspector(|w| Message::ResizeInspector(w))
            .on_start_resize_sidebar(Message::StartResizingSidebar)
            .on_stop_resize_sidebar(Message::StopResizingSidebar)
            .on_start_resize_inspector(Message::StartResizingInspector)
            .on_stop_resize_inspector(Message::StopResizingInspector)
            .is_resizing_sidebar(self.state.is_resizing_sidebar)
            .is_resizing_inspector(self.state.is_resizing_inspector)
            .on_none(Message::None);

        if self.state.show_inspector {
            let ai_inspector = crate::views::chat::AIChatView::<Message>::new(
                #[cfg(feature = "intelligence")]
                self.state.chat_messages.clone(),
                #[cfg(feature = "intelligence")]
                self.state.chat_input.clone(),
                #[cfg(feature = "intelligence")]
                self.state.is_thinking,
                #[cfg(feature = "intelligence")]
                Message::Chat,
            );

            let inspector_content: Option<Box<dyn View<Message, IcedBackend>>> =
                if let Some(p_inspector) = page.inspector.take() {
                    #[cfg(feature = "intelligence")]
                    {
                        // Both available - Show Tabs
                        let tab_bar = HStack::new()
                            .width(Length::Fill)
                            .padding(16.0)
                            .spacing(8.0)
                            .push(
                                Button::label("Feature")
                                    .variant(
                                        if self.state.inspector_tab
                                            == super::super::app::InspectorTab::Feature
                                        {
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
                                        if self.state.inspector_tab
                                            == super::super::app::InspectorTab::App
                                        {
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

                        let content: Box<dyn View<Message, IcedBackend>> =
                            match self.state.inspector_tab {
                                super::super::app::InspectorTab::Feature => Box::new(p_inspector),
                                super::super::app::InspectorTab::App => Box::new(ai_inspector),
                            };

                        Some(Box::new(
                            VStack::new()
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .push(tab_bar)
                                .push(content),
                        ))
                    }
                    #[cfg(not(feature = "intelligence"))]
                    {
                        // Only Feature available
                        Some(Box::new(p_inspector))
                    }
                } else {
                    // No page inspector - try AI
                    Some(Box::new(ai_inspector))
                };

            if let Some(content) = inspector_content {
                split_view = split_view
                    .inspector(content)
                    .on_dismiss_inspector(Message::ToggleInspector);
            }
        }

        // --- 4. Final Assembly ---
        // Unified Floating Dock: Both mobile and desktop use stack layout for a consistent floating dock experience
        let final_view: Element<'static, Message> = iced::widget::stack![
            split_view.view(context),
            IcedBackend::container(
                tab_bar.view(context),
                Padding {
                    top: 0.0,
                    right: 20.0,
                    bottom: context.safe_area.bottom,
                    left: 20.0,
                },
                Length::Fill,
                Length::Fill,
                None, // Transparent outer
                0.0,
                0.0,
                None,
                None,
                Alignment::Center,
                Alignment::End,
                context,
            )
        ]
        .into();

        final_view
    }

    pub fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        let is_mobile = context.is_slim();

        let canvas_manager = CanvasView::new(self.state.clone());

        let sidebar = SidebarView::new(
            self.state.active_tab.clone(),
            self.state.navigation_mode.clone(),
        );

        let page = canvas_manager.render_page(context);

        let mut split_view = NavigationSplitView::new(sidebar, ScrollView::from_boxed(page.view))
            .force_sidebar_on_slim(self.state.show_sidebar && is_mobile)
            .sidebar_width(self.state.sidebar_width)
            .inspector_width(self.state.inspector_width);

        if self.state.show_inspector {
            if let Some(inspector) = page.inspector {
                split_view = split_view.inspector(inspector);
            }
        }

        crate::core::SemanticNode::new("content_view")
            .with_label(format!("AppPage: {:?}", self.state.active_tab))
            .push_child(split_view.describe(context))
    }
}
