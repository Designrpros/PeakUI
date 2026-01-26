use crate::core::{Backend, Context, IcedBackend, TermBackend, View};
use iced::{Element, Length, Renderer, Theme};

/// A scrollable container that wraps content and provides styled scrollbars.
pub struct ScrollView<Message: 'static, B: Backend = IcedBackend> {
    content: Box<dyn View<Message, B>>,
    width: Length,
    height: Length,
    show_indicators: bool,
}

impl<Message: 'static> ScrollView<Message, IcedBackend> {
    /// Creates a new `ScrollView` for the Iced backend.
    pub fn new(content: impl View<Message, IcedBackend> + 'static) -> Self {
        Self::new_generic(content)
    }

    /// Creates a new `ScrollView` for the Iced backend from a boxed view.
    pub fn from_boxed(content: Box<dyn View<Message, IcedBackend>>) -> Self {
        Self {
            content,
            width: Length::Fill,
            height: Length::Fill,
            show_indicators: true,
        }
    }
}

impl<Message: 'static> ScrollView<Message, TermBackend> {
    /// Creates a new `ScrollView` for the Term backend (TUI).
    pub fn new_tui(content: impl View<Message, TermBackend> + 'static) -> Self {
        Self::new_generic(content)
    }
}

impl<Message: 'static, B: Backend> ScrollView<Message, B> {
    /// Creates a new generic `ScrollView` with the given content.
    pub fn new_generic(content: impl View<Message, B> + 'static) -> Self {
        Self {
            content: Box::new(content),
            width: Length::Fill,
            height: Length::Fill,
            show_indicators: true,
        }
    }

    /// Sets the width of the `ScrollView`.
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the `ScrollView`.
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Hides the scroll indicators (scrollbars).
    pub fn hide_indicators(mut self) -> Self {
        self.show_indicators = false;
        self
    }

    /// Alias for `hide_indicators()`.
    pub fn hide(self) -> Self {
        self.hide_indicators()
    }
}

impl<Message: 'static> View<Message, IcedBackend> for ScrollView<Message, IcedBackend> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let content = iced::widget::container(self.content.view(context)).width(Length::Fill);

        Self::apply_style(
            iced::widget::scrollable(content),
            &context.theme,
            self.show_indicators,
        )
        .width(self.width)
        .height(self.height)
        .into()
    }
}

impl<Message: 'static> View<Message, TermBackend> for ScrollView<Message, TermBackend> {
    fn view(&self, context: &Context) -> String {
        self.content.view(context)
    }
}

impl<Message: 'static> ScrollView<Message, IcedBackend> {
    /// Applies custom styling to the Iced scrollable widget.
    pub fn apply_style<'a>(
        s: iced::widget::Scrollable<'a, Message, Theme, Renderer>,
        theme: &peak_theme::ThemeTokens,
        show_indicators: bool,
    ) -> iced::widget::Scrollable<'a, Message, Theme, Renderer> {
        let text_color = theme.colors.text_primary;

        if !show_indicators {
            return s.style(|_, _| iced::widget::scrollable::Style {
                container: iced::widget::container::Style::default(),
                vertical_rail: iced::widget::scrollable::Rail {
                    background: None,
                    border: iced::Border::default(),
                    scroller: iced::widget::scrollable::Scroller {
                        color: iced::Color::TRANSPARENT,
                        border: iced::Border::default(),
                    },
                },
                horizontal_rail: iced::widget::scrollable::Rail {
                    background: None,
                    border: iced::Border::default(),
                    scroller: iced::widget::scrollable::Scroller {
                        color: iced::Color::TRANSPARENT,
                        border: iced::Border::default(),
                    },
                },
                gap: None,
            });
        }

        s.style(move |_, status| {
            let scroller_alpha = match status {
                iced::widget::scrollable::Status::Hovered { .. } => 0.3,
                iced::widget::scrollable::Status::Dragged { .. } => 0.5,
                _ => 0.05, // Very faint when idle
            };

            let _rail_width = match status {
                iced::widget::scrollable::Status::Hovered { .. }
                | iced::widget::scrollable::Status::Dragged { .. } => 6.0,
                _ => 3.0, // Thinner when idle
            };

            iced::widget::scrollable::Style {
                container: iced::widget::container::Style::default(),
                vertical_rail: iced::widget::scrollable::Rail {
                    background: None,
                    border: iced::Border::default(),
                    scroller: iced::widget::scrollable::Scroller {
                        color: iced::Color {
                            a: scroller_alpha,
                            ..text_color
                        },
                        border: iced::Border {
                            radius: if cfg!(target_arch = "wasm32") {
                                0.0
                            } else {
                                2.0 // Even tighter radius for thinner look
                            }
                            .into(),
                            width: 0.0,
                            ..Default::default()
                        },
                    },
                },
                horizontal_rail: iced::widget::scrollable::Rail {
                    background: None,
                    border: iced::Border::default(),
                    scroller: iced::widget::scrollable::Scroller {
                        color: iced::Color {
                            a: scroller_alpha,
                            ..text_color
                        },
                        border: iced::Border {
                            radius: if cfg!(target_arch = "wasm32") {
                                0.0
                            } else {
                                2.0
                            }
                            .into(),
                            width: 0.0,
                            ..Default::default()
                        },
                    },
                },
                gap: None,
            }
        })
    }
}
