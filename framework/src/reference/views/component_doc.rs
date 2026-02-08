use crate::atoms::Text;
use crate::controls::Button;
use crate::core::{Backend, Context, ScrollDirection, View};
use crate::layout::{HStack, VStack};
use crate::style::Variant;
use crate::views::{CodeBlock, MarkdownView};
use iced::{Alignment, Length, Padding};
use std::marker::PhantomData;
use std::sync::Arc;

pub struct ComponentDoc<Message: 'static, B: Backend> {
    title: String,
    description: String,
    theory: Option<String>,
    code_snippet: String,
    preview: Arc<dyn View<Message, B>>,
    terminal_preview: Option<String>,
    neural_preview: Option<crate::core::SemanticNode>,
    spatial_preview: Option<crate::core::SpatialNode<()>>,
    render_mode: crate::reference::app::RenderMode,
    on_render_mode_change:
        Option<Arc<dyn Fn(crate::reference::app::RenderMode) -> Message + Send + Sync>>,
    props_table: Option<String>,
    extra_content: Option<Arc<dyn View<Message, B>>>,
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
            extra_content: None,
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

    pub fn spatial<M: Clone>(mut self, spatial: crate::core::SpatialNode<M>) -> Self {
        self.spatial_preview = Some(spatial.to_empty());
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

    pub fn extra_content(mut self, view: impl View<Message, B> + 'static) -> Self {
        self.extra_content = Some(Arc::new(view));
        self
    }
}

impl<Message: Clone + Send + Sync + 'static, B: Backend> View<Message, B>
    for ComponentDoc<Message, B>
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let theme = context.theme;

        // 1. Header with Title and Description
        // IMPORTANT: We set width(Length::Fill) on the VStack to ensure text can expand
        let header = VStack::<Message, B>::new_generic()
            .spacing(8.0)
            .width(Length::Fill)
            .align_x(Alignment::Start)
            .push(
                Text::<B>::new(self.title.clone())
                    .large_title()
                    .bold()
                    .align_start()
                    .width(Length::Fill)
                    .color(theme.colors.text_primary),
            )
            .push(
                Text::<B>::new(self.description.clone())
                    .body()
                    .align_start()
                    .color(theme.colors.text_secondary)
                    .width(Length::Fill),
            );

        // 2. Playground / Preview Area (The Lab)
        let render_mode = self.render_mode;
        let on_render_mode_change = self.on_render_mode_change.clone();

        let scrollable_tabs = move |ctx: &Context| {
            let mut mode_tabs = HStack::<Message, B>::new_generic()
                .spacing(12.0)
                .width(Length::Shrink); // Must be Shrink for horizontal scrollable

            if let Some(on_change) = &on_render_mode_change {
                let on_change = on_change.clone();
                mode_tabs = mode_tabs
                    .push(
                        Button::<Message, B>::label("Canvas")
                            .variant(
                                if render_mode == crate::reference::app::RenderMode::Canvas {
                                    Variant::Solid
                                } else {
                                    Variant::Ghost
                                },
                            )
                            .on_press((on_change)(crate::reference::app::RenderMode::Canvas))
                            .neural_tag("lab_tab_canvas"),
                    )
                    .push(
                        Button::<Message, B>::label("Terminal")
                            .variant(
                                if render_mode == crate::reference::app::RenderMode::Terminal {
                                    Variant::Solid
                                } else {
                                    Variant::Ghost
                                },
                            )
                            .on_press((on_change)(crate::reference::app::RenderMode::Terminal))
                            .neural_tag("lab_tab_terminal"),
                    )
                    .push(
                        Button::<Message, B>::label("Neural")
                            .variant(
                                if render_mode == crate::reference::app::RenderMode::Neural {
                                    Variant::Solid
                                } else {
                                    Variant::Ghost
                                },
                            )
                            .on_press((on_change)(crate::reference::app::RenderMode::Neural))
                            .neural_tag("lab_tab_neural"),
                    )
                    .push(
                        Button::<Message, B>::label("Spatial")
                            .variant(
                                if render_mode == crate::reference::app::RenderMode::Spatial {
                                    Variant::Solid
                                } else {
                                    Variant::Ghost
                                },
                            )
                            .on_press((on_change)(crate::reference::app::RenderMode::Spatial))
                            .neural_tag("lab_tab_spatial"),
                    );
            }

            B::scroll_view(
                mode_tabs.view(ctx),
                Length::Fill,
                Length::Shrink,
                None,
                false,
                ScrollDirection::Horizontal,
                ctx,
            )
        };

        let preview_area = match self.render_mode {
            crate::reference::app::RenderMode::Canvas => {
                let preview = self.preview.clone();
                crate::containers::Section::<Message, B>::new_generic(
                    "The Lab",
                    VStack::<Message, B>::new_generic()
                        .spacing(16.0)
                        .push(crate::core::ProxyView::new(scrollable_tabs.clone()))
                        .push(
                            crate::containers::Card::<Message, B>::new_generic(
                                crate::core::ProxyView::new(move |ctx| {
                                    let preview_view = preview.view(ctx);
                                    B::scroll_view(
                                        preview_view,
                                        Length::Fill,
                                        Length::Fill,
                                        None,
                                        false,
                                        ScrollDirection::Vertical,
                                        ctx,
                                    )
                                }),
                            )
                            .height(Length::Fixed(240.0)),
                        ),
                )
            }
            crate::reference::app::RenderMode::Terminal => {
                let ansi = self
                    .terminal_preview
                    .as_deref()
                    .unwrap_or("No terminal representation available.")
                    .to_string();
                crate::containers::Section::<Message, B>::new_generic(
                    "The Lab",
                    VStack::<Message, B>::new_generic()
                        .spacing(16.0)
                        .push(crate::core::ProxyView::new(scrollable_tabs.clone()))
                        .push(
                            crate::containers::Card::<Message, B>::new_generic(
                                VStack::<Message, B>::new_generic()
                                    .padding(24)
                                    .push(CodeBlock::<Message>::new(ansi)),
                            )
                            .background(iced::Color::from_rgb8(30, 30, 30)),
                        ),
                )
            }
            crate::reference::app::RenderMode::Neural => {
                let json = if let Some(node) = &self.neural_preview {
                    serde_json::to_string_pretty(node)
                        .unwrap_or_else(|_| "Error serializing neural node".to_string())
                } else {
                    "No neural representation available.".to_string()
                };
                crate::containers::Section::<Message, B>::new_generic(
                    "The Lab",
                    VStack::<Message, B>::new_generic()
                        .spacing(16.0)
                        .push(crate::core::ProxyView::new(scrollable_tabs.clone()))
                        .push(
                            crate::containers::Card::<Message, B>::new_generic(
                                VStack::<Message, B>::new_generic()
                                    .padding(24)
                                    .push(CodeBlock::<Message>::new(json)),
                            )
                            .background(iced::Color::from_rgb8(20, 20, 20)),
                        ),
                )
            }
            crate::reference::app::RenderMode::Spatial => {
                let spatial_node = self.spatial_preview.clone();
                crate::containers::Section::<Message, B>::new_generic(
                    "The Lab",
                    VStack::<Message, B>::new_generic()
                        .spacing(16.0)
                        .push(crate::core::ProxyView::new(scrollable_tabs.clone()))
                        .push(
                            crate::containers::Card::<Message, B>::new_generic(
                                crate::core::ProxyView::new(move |ctx| {
                                    if let Some(node) = &spatial_node {
                                        crate::core::View::<Message, B>::view(
                                            &crate::reference::views::SimulatorView::<Message>::new(
                                                node.clone(),
                                            ),
                                            ctx,
                                        )
                                    } else {
                                        B::text(
                                            "No spatial representation available.".to_string(),
                                            14.0,
                                            None,
                                            false,
                                            true,
                                            None,
                                            None,
                                            Length::Shrink,
                                            iced::Alignment::Center,
                                            ctx,
                                        )
                                    }
                                }),
                            )
                            .height(Length::Fixed(240.0)),
                        ),
                )
            }
        }
        .width(Length::Fill);

        // 3. Code Block with Copy (Using the Shared CodeBlock component)
        let code_snippet = self.code_snippet.clone();

        let code_area = crate::containers::Section::<Message, B>::new_generic(
            "Usage",
            crate::core::ProxyView::new(move |ctx| {
                View::<Message, B>::view(&CodeBlock::<Message>::rust(code_snippet.clone()), ctx)
            }),
        )
        .width(Length::Fill);

        // Assemble
        let mut doc_content = VStack::<Message, B>::new_generic()
            .spacing(24.0)
            .padding(Padding {
                top: context.safe_area.top + 8.0, // Minimal gap after header
                right: if context.is_slim() { 24.0 } else { 32.0 },
                bottom: context.safe_area.bottom + 48.0, // Gap for dock
                left: if context.is_slim() { 24.0 } else { 32.0 },
            })
            .width(Length::Fill)
            .push(header)
            .push(preview_area)
            .push(code_area);

        // Add Theory section if present
        if let Some(theory) = &self.theory {
            doc_content = doc_content.push(
                crate::containers::Section::<Message, B>::new_generic(
                    "Theory",
                    MarkdownView::new(theory.clone()),
                )
                .width(Length::Fill),
            );
        }

        // Add Props Table if present
        if let Some(props) = &self.props_table {
            doc_content = doc_content.push(
                crate::containers::Section::<Message, B>::new_generic(
                    "Props",
                    MarkdownView::new(props.clone()),
                )
                .width(Length::Fill),
            );
        }

        // Add Extra Content if present (below Usage/Theory/Props)
        if let Some(extra) = &self.extra_content {
            let extra = extra.clone();
            doc_content = doc_content.push(crate::core::ProxyView::new(move |ctx| extra.view(ctx)));
        }

        doc_content.view(context)
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("component_doc")
            .with_label(self.title.clone())
            .with_content(self.description.clone())
    }
}
