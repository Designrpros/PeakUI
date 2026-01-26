use crate::atoms::Text;
use crate::controls::Button;
use crate::core::{Backend, Context, IcedBackend, View};
use crate::layout::{HStack, VStack};
use crate::modifiers::Variant;
use crate::views::{CodeBlock, MarkdownView};
use iced::{Length, Padding};
use std::marker::PhantomData;
use std::sync::Arc;

pub struct ComponentDoc<Message: 'static, B: Backend = IcedBackend> {
    title: String,
    description: String,
    theory: Option<String>,
    code_snippet: String,
    preview: Arc<dyn View<Message, B>>,
    terminal_preview: Option<String>,
    neural_preview: Option<crate::core::SemanticNode>,
    spatial_preview: Option<String>,
    render_mode: crate::reference::app::RenderMode,
    on_render_mode_change:
        Option<Arc<dyn Fn(crate::reference::app::RenderMode) -> Message + Send + Sync>>,
    props_table: Option<String>,
    _phantom: PhantomData<B>,
}

impl<Message: 'static, B: Backend> ComponentDoc<Message, B> {
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        code_snippet: impl Into<String>,
        preview: Arc<dyn View<Message, B>>,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            theory: None,
            code_snippet: code_snippet.into(),
            preview,
            terminal_preview: None,
            neural_preview: None,
            spatial_preview: None,
            render_mode: crate::reference::app::RenderMode::Canvas,
            on_render_mode_change: None,
            props_table: None,
            _phantom: PhantomData,
        }
    }

    pub fn terminal(mut self, terminal: String) -> Self {
        self.terminal_preview = Some(terminal);
        self
    }

    pub fn neural(mut self, neural: crate::core::SemanticNode) -> Self {
        self.neural_preview = Some(neural);
        self
    }

    pub fn spatial(mut self, spatial: crate::core::SpatialNode) -> Self {
        self.spatial_preview = Some(
            serde_json::to_string_pretty(&spatial)
                .unwrap_or_else(|_| "Error serializing spatial node".to_string()),
        );
        self
    }

    pub fn render_mode(mut self, mode: crate::reference::app::RenderMode) -> Self {
        self.render_mode = mode;
        self
    }

    pub fn on_render_mode_change(
        mut self,
        f: impl Fn(crate::reference::app::RenderMode) -> Message + Send + Sync + 'static,
    ) -> Self {
        self.on_render_mode_change = Some(Arc::new(f));
        self
    }

    pub fn theory(mut self, theory: impl Into<String>) -> Self {
        self.theory = Some(theory.into());
        self
    }

    pub fn props_table(mut self, props_table: impl Into<String>) -> Self {
        self.props_table = Some(props_table.into());
        self
    }
}

impl<Message: Clone + 'static> View<Message, IcedBackend> for ComponentDoc<Message, IcedBackend> {
    fn view(
        &self,
        context: &Context,
    ) -> iced::Element<'static, Message, iced::Theme, iced::Renderer> {
        let theme = context.theme;

        // 1. Header with Title and Description
        // IMPORTANT: We set width(Length::Fill) on the VStack to ensure text can expand
        let header = VStack::<Message, IcedBackend>::new_generic()
            .spacing(12.0)
            .width(Length::Fill)
            .push(
                Text::<IcedBackend>::new(self.title.clone())
                    .large_title()
                    .bold()
                    .color(theme.colors.text_primary),
            )
            .push(
                Text::<IcedBackend>::new(self.description.clone())
                    .body()
                    .color(theme.colors.text_secondary)
                    .width(Length::Fill),
            );

        // 2. Playground / Preview Area (The Lab)
        let render_mode = self.render_mode;
        let on_render_mode_change = self.on_render_mode_change.clone();

        let scrollable_tabs = move |ctx: &Context| {
            let mut mode_tabs = HStack::<Message, IcedBackend>::new_generic()
                .spacing(12.0)
                .width(Length::Shrink); // Must be Shrink for horizontal scrollable

            if let Some(on_change) = &on_render_mode_change {
                let on_change = on_change.clone();
                mode_tabs = mode_tabs
                    .push(
                        Button::<Message, IcedBackend>::label("Canvas")
                            .variant(
                                if render_mode == crate::reference::app::RenderMode::Canvas {
                                    Variant::Solid
                                } else {
                                    Variant::Ghost
                                },
                            )
                            .on_press((on_change)(crate::reference::app::RenderMode::Canvas)),
                    )
                    .push(
                        Button::<Message, IcedBackend>::label("Terminal")
                            .variant(
                                if render_mode == crate::reference::app::RenderMode::Terminal {
                                    Variant::Solid
                                } else {
                                    Variant::Ghost
                                },
                            )
                            .on_press((on_change)(crate::reference::app::RenderMode::Terminal)),
                    )
                    .push(
                        Button::<Message, IcedBackend>::label("Neural")
                            .variant(
                                if render_mode == crate::reference::app::RenderMode::Neural {
                                    Variant::Solid
                                } else {
                                    Variant::Ghost
                                },
                            )
                            .on_press((on_change)(crate::reference::app::RenderMode::Neural)),
                    )
                    .push(
                        Button::<Message, IcedBackend>::label("Spatial")
                            .variant(
                                if render_mode == crate::reference::app::RenderMode::Spatial {
                                    Variant::Solid
                                } else {
                                    Variant::Ghost
                                },
                            )
                            .on_press((on_change)(crate::reference::app::RenderMode::Spatial)),
                    );
            }

            iced::widget::scrollable(mode_tabs.view(ctx))
                .direction(iced::widget::scrollable::Direction::Horizontal(
                    iced::widget::scrollable::Scrollbar::new()
                        .width(4)
                        .scroller_width(4)
                        .margin(2),
                ))
                .width(Length::Fill)
                .into()
        };

        let preview_area = match self.render_mode {
            crate::reference::app::RenderMode::Canvas => {
                let preview = self.preview.clone();
                crate::containers::Section::<Message, IcedBackend>::new_generic(
                    "The Lab",
                    VStack::<Message, IcedBackend>::new_generic()
                        .spacing(24.0)
                        .push(crate::core::ProxyView::new(scrollable_tabs.clone()))
                        .push(
                            crate::containers::Card::<Message, IcedBackend>::new_generic(
                                crate::core::ProxyView::new(move |ctx| {
                                    let preview_view = preview.view(ctx);
                                    crate::scroll_view::ScrollView::apply_style(
                                        iced::widget::scrollable(
                                            iced::widget::container(preview_view)
                                                .width(Length::Shrink),
                                        ),
                                        &ctx.theme,
                                        true,
                                    )
                                    .direction(iced::widget::scrollable::Direction::Horizontal(
                                        iced::widget::scrollable::Scrollbar::new()
                                            .width(4)
                                            .scroller_width(4)
                                            .margin(2),
                                    ))
                                    .width(Length::Fill)
                                    .into()
                                }),
                            ),
                        ),
                )
            }
            crate::reference::app::RenderMode::Terminal => {
                let ansi = self
                    .terminal_preview
                    .as_deref()
                    .unwrap_or("No terminal representation available.")
                    .to_string();
                crate::containers::Section::<Message, IcedBackend>::new_generic(
                    "The Lab",
                    VStack::<Message, IcedBackend>::new_generic()
                        .spacing(24.0)
                        .push(crate::core::ProxyView::new(scrollable_tabs.clone()))
                        .push(CodeBlock::<Message>::new(ansi)),
                )
            }
            crate::reference::app::RenderMode::Neural => {
                let json = if let Some(node) = &self.neural_preview {
                    serde_json::to_string_pretty(node)
                        .unwrap_or_else(|_| "Error serializing neural node".to_string())
                } else {
                    "No neural representation available.".to_string()
                };
                crate::containers::Section::<Message, IcedBackend>::new_generic(
                    "The Lab",
                    VStack::<Message, IcedBackend>::new_generic()
                        .spacing(24.0)
                        .push(crate::core::ProxyView::new(scrollable_tabs.clone()))
                        .push(CodeBlock::<Message>::new(json)),
                )
            }
            crate::reference::app::RenderMode::Spatial => {
                let json = self
                    .spatial_preview
                    .as_deref()
                    .unwrap_or("No spatial representation available.")
                    .to_string();
                crate::containers::Section::<Message, IcedBackend>::new_generic(
                    "The Lab",
                    VStack::<Message, IcedBackend>::new_generic()
                        .spacing(24.0)
                        .push(crate::core::ProxyView::new(scrollable_tabs.clone()))
                        .push(CodeBlock::<Message>::new(json)),
                )
            }
        }
        .width(Length::Fill);

        // 3. Code Block with Copy (Using the Shared CodeBlock component)
        let code_snippet = self.code_snippet.clone();

        let code_area = crate::containers::Section::<Message, IcedBackend>::new_generic(
            "Usage",
            crate::core::ProxyView::new(move |ctx| {
                CodeBlock::<Message>::rust(code_snippet.clone()).view(ctx)
            }),
        )
        .width(Length::Fill);

        // Assemble
        let mut doc_content = VStack::<Message, IcedBackend>::new_generic()
            .spacing(40.0)
            .padding(Padding {
                top: context.safe_area.top.max(48.0),
                right: if context.is_slim() { 24.0 } else { 48.0 },
                bottom: context.safe_area.bottom.max(48.0),
                left: if context.is_slim() { 24.0 } else { 48.0 },
            })
            .width(Length::Fill)
            .push(header)
            .push(preview_area)
            .push(code_area);

        // Add Theory section if present
        if let Some(theory) = &self.theory {
            doc_content = doc_content.push(
                crate::containers::Section::<Message, IcedBackend>::new_generic(
                    "Theory",
                    MarkdownView::new(theory.clone()),
                )
                .width(Length::Fill),
            );
        }

        // Add Props Table if present
        if let Some(props) = &self.props_table {
            doc_content = doc_content.push(
                crate::containers::Section::<Message, IcedBackend>::new_generic(
                    "Props",
                    MarkdownView::new(props.clone()),
                )
                .width(Length::Fill),
            );
        }

        doc_content.view(context)
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode {
            role: "component_doc".to_string(),
            label: Some(self.title.clone()),
            content: Some(self.description.clone()),
            children: Vec::new(),
            neural_tag: None,
            documentation: None,
        }
    }
}
