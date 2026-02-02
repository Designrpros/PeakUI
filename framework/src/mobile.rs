use iced::advanced::layout::{self, Layout};
use iced::advanced::mouse;
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::{Element, Length, Point, Rectangle, Size};

/// A wrapper widget that distinguishes between intention to scroll and intention to action.
/// It buffers events until a threshold is reached.
pub struct GestureArena<'a, Message, Theme, Renderer> {
    inner: Element<'a, Message, Theme, Renderer>,
    threshold: f32,
}

impl<'a, Message, Theme, Renderer> GestureArena<'a, Message, Theme, Renderer> {
    pub fn new(inner: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            inner: inner.into(),
            threshold: 10.0, // Default 10px threshold
        }
    }

    pub fn threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }
}

#[derive(Default)]
struct State {
    drag_start: Option<Point>,
    is_scrolling: bool,
    is_intercepted: bool,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for GestureArena<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        self.inner.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.inner.as_widget_mut().layout(tree, renderer, limits)
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.inner
            .as_widget()
            .draw(tree, renderer, theme, style, layout, cursor, viewport)
    }

    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State::default())
    }

    fn children(&self) -> Vec<widget::Tree> {
        self.inner.as_widget().children()
    }

    fn diff(&self, tree: &mut widget::Tree) {
        self.inner.as_widget().diff(tree)
    }

    fn update(
        &mut self,
        tree: &mut widget::Tree,
        event: &iced::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();

        match event {
            iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | iced::Event::Touch(iced::touch::Event::FingerPressed { .. }) => {
                if let Some(cursor_position) = cursor.position_in(layout.bounds()) {
                    state.drag_start = Some(cursor_position);
                    state.is_scrolling = false;
                    state.is_intercepted = false;
                }
            }
            iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | iced::Event::Touch(iced::touch::Event::FingerLifted { .. })
            | iced::Event::Touch(iced::touch::Event::FingerLost { .. }) => {
                state.drag_start = None;
                if state.is_scrolling {
                    // shell.capture_event(); // Do NOT capture, let it scroll!
                    return;
                }
            }
            iced::Event::Mouse(mouse::Event::CursorMoved { position })
            | iced::Event::Touch(iced::touch::Event::FingerMoved { position, .. }) => {
                if let Some(start) = state.drag_start {
                    let delta = *position - start;
                    let distance = (delta.x.powi(2) + delta.y.powi(2)).sqrt();

                    if !state.is_scrolling && !state.is_intercepted {
                        if distance > self.threshold {
                            if delta.y.abs() > delta.x.abs() * 1.5 {
                                state.is_scrolling = true;
                                log::trace!("GestureArena: Vertical scroll detected");
                            } else {
                                state.is_intercepted = true;
                            }
                        }
                    }

                    if state.is_scrolling {
                        // shell.capture_event(); // Do NOT capture, let it scroll!
                        return;
                    }
                }
            }
            _ => {}
        }

        if state.is_scrolling {
            // shell.capture_event(); // Do NOT capture, let it scroll!
            return;
        }

        self.inner.as_widget_mut().update(
            tree, event, layout, cursor, renderer, clipboard, shell, viewport,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.inner
            .as_widget()
            .mouse_interaction(tree, layout, cursor, viewport, renderer)
    }

    fn operate(
        &mut self,
        tree: &mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.inner
            .as_widget_mut()
            .operate(tree, layout, renderer, operation)
    }
}

impl<'a, Message: 'static, Theme: 'a, Renderer> From<GestureArena<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'static,
{
    fn from(arena: GestureArena<'a, Message, Theme, Renderer>) -> Self {
        Self::new(arena)
    }
}
