use crate::core::{Backend, Context, IcedBackend, TermBackend, View};
use iced::widget::{column, container, row};
use iced::{Element, Length, Renderer, Theme};
use std::sync::Arc;

pub struct NavigationSplitView<Message: 'static + Send + Sync, B: Backend = IcedBackend> {
    sidebar: Box<dyn View<Message, B>>,
    content: Box<dyn View<Message, B>>,
    inspector: Option<Box<dyn View<Message, B>>>,
    force_sidebar_on_slim: bool,
    on_back: Option<Message>,
    sidebar_width: f32,
    inspector_width: f32,
    on_resize_sidebar: Option<Arc<dyn Fn(f32) -> Message + Send + Sync>>,
    on_resize_inspector: Option<Arc<dyn Fn(f32) -> Message + Send + Sync>>,
    on_start_resize_sidebar: Option<Message>,
    on_stop_resize_sidebar: Option<Message>,
    on_start_resize_inspector: Option<Message>,
    on_stop_resize_inspector: Option<Message>,
    is_resizing_sidebar: bool,
    is_resizing_inspector: bool,
    sidebar_locked: bool,
    inspector_locked: bool,
    on_none: Option<Message>,
    on_dismiss_inspector: Option<Message>,
}

impl<Message: Clone + Send + Sync + 'static> NavigationSplitView<Message, IcedBackend> {
    pub fn new(
        sidebar: impl View<Message, IcedBackend> + 'static,
        content: impl View<Message, IcedBackend> + 'static,
    ) -> Self {
        Self::new_generic(sidebar, content)
    }
}

impl<Message: Clone + Send + Sync + 'static> NavigationSplitView<Message, TermBackend> {
    pub fn new_tui(
        sidebar: impl View<Message, TermBackend> + 'static,
        content: impl View<Message, TermBackend> + 'static,
    ) -> Self {
        Self::new_generic(sidebar, content)
    }
}

impl<Message: Clone + Send + Sync + 'static, B: Backend> NavigationSplitView<Message, B> {
    pub fn new_generic(
        sidebar: impl View<Message, B> + 'static,
        content: impl View<Message, B> + 'static,
    ) -> Self {
        Self {
            sidebar: Box::new(sidebar),
            content: Box::new(content),
            inspector: None,
            force_sidebar_on_slim: false,
            on_back: None,
            sidebar_width: 240.0,
            inspector_width: 300.0,
            on_resize_sidebar: None,
            on_resize_inspector: None,
            on_start_resize_sidebar: None,
            on_stop_resize_sidebar: None,
            on_start_resize_inspector: None,
            on_stop_resize_inspector: None,
            is_resizing_sidebar: false,
            is_resizing_inspector: false,
            sidebar_locked: false,
            inspector_locked: false,
            on_none: None,
            on_dismiss_inspector: None,
        }
    }

    pub fn inspector(mut self, inspector: impl View<Message, B> + 'static) -> Self {
        self.inspector = Some(Box::new(inspector));
        self
    }

    pub fn force_sidebar_on_slim(mut self, force: bool) -> Self {
        self.force_sidebar_on_slim = force;
        self
    }

    pub fn on_back(mut self, msg: Message) -> Self {
        self.on_back = Some(msg);
        self
    }

    pub fn sidebar_width(mut self, width: f32) -> Self {
        self.sidebar_width = width;
        self
    }

    pub fn inspector_width(mut self, width: f32) -> Self {
        self.inspector_width = width;
        self
    }

    pub fn on_resize_sidebar(mut self, f: impl Fn(f32) -> Message + Send + Sync + 'static) -> Self {
        self.on_resize_sidebar = Some(Arc::new(f));
        self
    }

    pub fn on_resize_inspector(
        mut self,
        f: impl Fn(f32) -> Message + Send + Sync + 'static,
    ) -> Self {
        self.on_resize_inspector = Some(Arc::new(f));
        self
    }

    pub fn on_start_resize_sidebar(mut self, msg: Message) -> Self {
        self.on_start_resize_sidebar = Some(msg);
        self
    }

    pub fn on_stop_resize_sidebar(mut self, msg: Message) -> Self {
        self.on_stop_resize_sidebar = Some(msg);
        self
    }

    pub fn on_start_resize_inspector(mut self, msg: Message) -> Self {
        self.on_start_resize_inspector = Some(msg);
        self
    }

    pub fn on_stop_resize_inspector(mut self, msg: Message) -> Self {
        self.on_stop_resize_inspector = Some(msg);
        self
    }

    pub fn is_resizing_sidebar(mut self, resizing: bool) -> Self {
        self.is_resizing_sidebar = resizing;
        self
    }

    pub fn is_resizing_inspector(mut self, resizing: bool) -> Self {
        self.is_resizing_inspector = resizing;
        self
    }

    pub fn sidebar_locked(mut self, locked: bool) -> Self {
        self.sidebar_locked = locked;
        self
    }

    pub fn inspector_locked(mut self, locked: bool) -> Self {
        self.inspector_locked = locked;
        self
    }

    pub fn on_none(mut self, msg: Message) -> Self {
        self.on_none = Some(msg);
        self
    }

    pub fn on_dismiss_inspector(mut self, msg: Message) -> Self {
        self.on_dismiss_inspector = Some(msg);
        self
    }
}

#[allow(unused_imports)]
use crate::scroll_view::ScrollView;

impl<Message: Clone + Send + Sync + 'static> View<Message, IcedBackend>
    for NavigationSplitView<Message, IcedBackend>
{
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;

        if context.is_slim() {
            if self.force_sidebar_on_slim {
                // Mobile Sidebar View
                container(self.sidebar.view(context))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style({
                        let bg_color = theme.colors.background;
                        let text_color = theme.colors.text_primary;
                        move |_| container::Style {
                            background: Some(bg_color.into()),
                            text_color: Some(text_color),
                            ..Default::default()
                        }
                    })
                    .into()
            } else {
                // Mobile Content View (with optional back button)
                let mut content_col = column![]
                    .spacing(0)
                    .width(Length::Fill)
                    .height(Length::Fill);

                if let Some(back_msg) = self.on_back.clone() {
                    let back_button =
                        crate::controls::Button::<Message, IcedBackend>::label("Back")
                            .icon("chevron_left")
                            .variant(crate::style::Variant::Ghost)
                            .on_press(back_msg)
                            .width(Length::Shrink)
                            .view(context);

                    content_col = content_col.push(
                        container(back_button)
                            .padding([16, 20])
                            .width(Length::Fill)
                            .style({
                                let bg_color = theme.colors.background;
                                move |_| container::Style {
                                    background: Some(bg_color.into()),
                                    ..Default::default()
                                }
                            }),
                    );
                }

                let content_view = self.content.view(context);

                content_col = content_col.push(
                    container(content_view)
                        .width(Length::Fill)
                        .height(Length::Fill),
                );

                let base_content = container(content_col)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style({
                        let bg_color = theme.colors.background;
                        let text_color = theme.colors.text_primary;
                        move |_| container::Style {
                            background: Some(bg_color.into()),
                            text_color: Some(text_color),
                            ..Default::default()
                        }
                    });

                // Use Stack to support Inspector Overlay
                let mut stack = iced::widget::stack![base_content]
                    .width(Length::Fill)
                    .height(Length::Fill);

                if let Some(inspector) = &self.inspector {
                    // Dimmed Background - Now interactive to dismiss
                    let overlay = container(
                        iced::widget::Space::new()
                            .width(Length::Fill)
                            .height(Length::Fill),
                    )
                    .style(|_| container::Style {
                        background: Some(
                            iced::Color {
                                a: 0.5,
                                ..iced::Color::BLACK
                            }
                            .into(),
                        ),
                        ..Default::default()
                    });

                    if let Some(dismiss_msg) = self.on_dismiss_inspector.clone() {
                        stack = stack.push(IcedBackend::mouse_area(
                            overlay.into(),
                            None,
                            Some(dismiss_msg),
                            None,
                            context,
                        ));
                    } else {
                        stack = stack.push(overlay);
                    }

                    let radius = context.radius(16.0);
                    let bg_color = theme.colors.surface_variant;
                    let sheet = container(inspector.view(context))
                        .width(Length::Fill)
                        .height(Length::FillPortion(1)) // Take up half screen? Or fixed height?
                        .padding(16)
                        .style({
                            let bg = bg_color;
                            let r = radius;
                            move |_| container::Style {
                                background: Some(bg.into()),
                                border: iced::Border {
                                    radius: r, // Top corners rounded - simplified to all for now
                                    ..Default::default()
                                },
                                ..Default::default()
                            }
                        });

                    // Align to bottom
                    stack = stack.push(
                        container(column![
                            // Push content down
                            iced::widget::Space::new()
                                .width(Length::Fill)
                                .height(Length::FillPortion(1)),
                            // The Sheet
                            sheet.height(Length::FillPortion(1))
                        ])
                        .height(Length::Fill)
                        .align_y(iced::alignment::Vertical::Bottom),
                    );
                }

                stack.into()
            }
        } else {
            // Desktop Layout
            let window_width = context.size.width;
            let sidebar_width = self.sidebar_width * context.theme.scaling;
            let inspector_width = self.inspector_width * context.theme.scaling;

            let mut main_row = row![
                // 1. Sidebar
                container(self.sidebar.view(context))
                    .width(Length::Fixed(sidebar_width))
                    .height(Length::Fill)
                    .style({
                        let bg = if theme.colors.background.r < 0.1 {
                            iced::Color::from_rgb8(28, 28, 30)
                        } else {
                            let mut c = theme.colors.surface_variant;
                            c.a = 0.5; // High transparency for glass
                            c
                        };
                        let text_color = theme.colors.text_primary;
                        move |_| container::Style {
                            background: Some(bg.into()),
                            text_color: Some(text_color),
                            ..Default::default()
                        }
                    }),
            ]
            .width(Length::Fill)
            .height(Length::Fill);

            // Right border divider handle
            if !self.sidebar_locked {
                main_row = main_row.push(IcedBackend::mouse_area(
                    container(
                        iced::widget::Space::new()
                            .width(Length::Fixed(6.0))
                            .height(Length::Fill),
                    )
                    .style({
                        let div_color = if self.is_resizing_sidebar {
                            theme.colors.primary
                        } else {
                            theme.colors.divider
                        };
                        move |_| container::Style {
                            background: Some(div_color.into()),
                            ..Default::default()
                        }
                    })
                    .into(),
                    None, // We don't use move here, we use global move below
                    self.on_start_resize_sidebar.clone(),
                    self.on_stop_resize_sidebar.clone(),
                    context,
                ));
            } else {
                // Fixed divider if locked
                main_row = main_row.push(
                    container(
                        iced::widget::Space::new()
                            .width(Length::Fixed(1.0))
                            .height(Length::Fill),
                    )
                    .style({
                        let div_color = theme.colors.divider;
                        move |_| container::Style {
                            background: Some(div_color.into()),
                            ..Default::default()
                        }
                    }),
                );
            }

            main_row = main_row.push(
                container(self.content.view(context))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style({
                        let bg_color = theme.colors.background;
                        let text_color = theme.colors.text_primary;
                        move |_| container::Style {
                            background: Some(bg_color.into()),
                            text_color: Some(text_color),
                            ..Default::default()
                        }
                    }),
            );

            // 3. Inspector (Optional)
            if let Some(inspector) = &self.inspector {
                if !self.inspector_locked {
                    main_row = main_row.push(IcedBackend::mouse_area(
                        container(
                            iced::widget::Space::new()
                                .width(Length::Fixed(6.0))
                                .height(Length::Fill),
                        )
                        .style({
                            let div_color = if self.is_resizing_inspector {
                                theme.colors.primary
                            } else {
                                theme.colors.divider
                            };
                            move |_| container::Style {
                                background: Some(div_color.into()),
                                ..Default::default()
                            }
                        })
                        .into(),
                        None,
                        self.on_start_resize_inspector.clone(),
                        self.on_stop_resize_inspector.clone(),
                        context,
                    ));
                } else {
                    main_row = main_row.push(
                        container(
                            iced::widget::Space::new()
                                .width(Length::Fixed(1.0))
                                .height(Length::Fill),
                        )
                        .style({
                            let div_color = theme.colors.divider;
                            move |_| container::Style {
                                background: Some(div_color.into()),
                                ..Default::default()
                            }
                        }),
                    );
                }

                main_row = main_row.push(
                    container(inspector.view(context))
                        .width(Length::Fixed(inspector_width))
                        .height(Length::Fill)
                        .style({
                            let bg = if theme.colors.background.r < 0.1 {
                                iced::Color::from_rgb8(28, 28, 30)
                            } else {
                                let mut c = theme.colors.surface;
                                c.a = 0.5; // Slightly more transparent than sidebar
                                c
                            };
                            let div_color = theme.colors.divider;
                            let text_color = theme.colors.text_primary;
                            move |_| container::Style {
                                background: Some(bg.into()),
                                border: iced::Border {
                                    color: div_color,
                                    width: 1.0,
                                    ..Default::default()
                                },
                                text_color: Some(text_color),
                                ..Default::default()
                            }
                        }),
                );
            }

            if (self.is_resizing_sidebar || self.is_resizing_inspector) && self.on_none.is_some() {
                let on_resize_sidebar = self.on_resize_sidebar.clone();
                let on_resize_inspector = self.on_resize_inspector.clone();
                let is_resizing_sidebar = self.is_resizing_sidebar;
                let is_resizing_inspector = self.is_resizing_inspector;
                let on_stop = if is_resizing_sidebar {
                    self.on_stop_resize_sidebar.clone()
                } else {
                    self.on_stop_resize_inspector.clone()
                };
                if let Some(on_none) = self.on_none.clone() {
                    IcedBackend::mouse_area(
                        container(main_row)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .padding(0)
                            .into(),
                        Some(Arc::new(move |p: iced::Point| {
                            if is_resizing_sidebar {
                                if let Some(f) = &on_resize_sidebar {
                                    return f(p.x);
                                }
                            } else if is_resizing_inspector {
                                if let Some(f) = &on_resize_inspector {
                                    return f(window_width - p.x);
                                }
                            }
                            on_none.clone()
                        })
                            as Arc<dyn Fn(iced::Point) -> Message + Send + Sync>),
                        None, // Divider handles start
                        on_stop,
                        context,
                    )
                } else {
                    container(main_row)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .into()
                }
            } else {
                container(main_row)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style({
                        let bg_color = theme.colors.background;
                        move |_| container::Style {
                            background: Some(bg_color.into()),
                            ..Default::default()
                        }
                    })
                    .into()
            }
        }
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        let mut children = vec![
            self.sidebar.describe(context),
            self.content.describe(context),
        ];

        if let Some(inspector) = &self.inspector {
            children.push(inspector.describe(context));
        }

        crate::core::SemanticNode::new("navigation_split_view")
            .with_label("Main Layout")
            .extend_children(children)
    }
}

impl<Message: Clone + Send + Sync + 'static> View<Message, TermBackend>
    for NavigationSplitView<Message, TermBackend>
{
    fn view(&self, context: &Context) -> String {
        let mut out = String::new();
        out.push_str("=== SIDEBAR ===\n");
        out.push_str(&self.sidebar.view(context));
        out.push_str("\n=== CONTENT ===\n");
        out.push_str(&self.content.view(context));
        out
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        let mut children = vec![
            self.sidebar.describe(context),
            self.content.describe(context),
        ];

        if let Some(inspector) = &self.inspector {
            children.push(inspector.describe(context));
        }

        crate::core::SemanticNode::new("navigation_split_view")
            .with_label("Main Layout")
            .extend_children(children)
    }
}
