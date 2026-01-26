use iced::widget::{button, container, row, text};
use iced::{Element, Length};

/// A segmented picker/control component with pill-style selection
/// Similar to iOS segmented controls or material tabs
pub struct SegmentedPicker<Message, Theme = iced::Theme> {
    options: Vec<SegmentOption<Message>>,
    active_index: usize,
    width: Length,
    height: Length,
    padding: f32,
    button_padding: f32,
    text_size: f32,
    border_radius: f32,
    background_color: iced::Color,
    active_bg_color: iced::Color,
    text_color: iced::Color,
    _phantom: std::marker::PhantomData<Theme>,
}

impl<Message, Theme> Clone for SegmentedPicker<Message, Theme>
where
    Message: Clone,
    Theme: Clone,
{
    fn clone(&self) -> Self {
        Self {
            options: self.options.clone(),
            active_index: self.active_index,
            width: self.width,
            height: self.height,
            padding: self.padding,
            button_padding: self.button_padding,
            text_size: self.text_size,
            border_radius: self.border_radius,
            background_color: self.background_color,
            active_bg_color: self.active_bg_color,
            text_color: self.text_color,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct SegmentOption<Message> {
    label: String,
    on_press: Message,
}

impl<Message, Theme> SegmentedPicker<Message, Theme>
where
    Message: Clone + 'static,
{
    /// Create a new segmented picker with options
    pub fn new(options: Vec<(String, Message)>, active_index: usize) -> Self {
        Self {
            options: options
                .into_iter()
                .map(|(label, on_press)| SegmentOption { label, on_press })
                .collect(),
            active_index,
            width: Length::Fill,
            height: Length::Shrink,
            padding: 4.0,
            button_padding: 6.0,
            text_size: 13.0,
            border_radius: 24.0,
            background_color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            active_bg_color: iced::Color::from_rgba(1.0, 1.0, 1.0, 0.3),
            text_color: iced::Color::WHITE,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Set the width of the picker
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Set the height of the picker
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Set the outer padding
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the button padding
    pub fn button_padding(mut self, button_padding: f32) -> Self {
        self.button_padding = button_padding;
        self
    }

    /// Set the text size
    pub fn text_size(mut self, size: f32) -> Self {
        self.text_size = size;
        self
    }

    /// Set the border radius
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    /// Set the background color
    pub fn background_color(mut self, color: iced::Color) -> Self {
        self.background_color = color;
        self
    }

    /// Set the active button background color
    pub fn active_bg_color(mut self, color: iced::Color) -> Self {
        self.active_bg_color = color;
        self
    }

    /// Set the text color
    pub fn text_color(mut self, color: iced::Color) -> Self {
        self.text_color = color;
        self
    }

    /// Build the view (consumes self)
    pub fn build(self) -> Element<'static, Message> {
        let active_idx = self.active_index;
        let active_bg = self.active_bg_color;
        let text_col = self.text_color;
        let btn_padding = self.button_padding;
        let txt_size = self.text_size;
        let btn_radius = self.border_radius / 2.0;

        let buttons = self
            .options
            .into_iter()
            .enumerate()
            .map(|(idx, opt)| {
                button(
                    text(opt.label)
                        .size(txt_size)
                        .align_x(iced::alignment::Horizontal::Center),
                )
                .on_press(opt.on_press)
                .width(Length::Fill)
                .padding(btn_padding)
                .style(move |_theme, _status| iced::widget::button::Style {
                    background: if idx == active_idx {
                        Some(active_bg.into())
                    } else {
                        None
                    },
                    text_color: text_col,
                    border: iced::Border {
                        radius: btn_radius.into(),
                        width: 0.0,
                        color: iced::Color::TRANSPARENT,
                    },
                    ..Default::default()
                })
                .into()
            })
            .collect::<Vec<Element<'static, Message>>>();

        container(row(buttons).spacing(4).padding(self.padding))
            .style({
                let bg_color = self.background_color;
                let radius = self.border_radius;
                move |_| container::Style {
                    background: Some(bg_color.into()),
                    border: iced::Border {
                        radius: radius.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
            .width(self.width)
            .height(self.height)
            .into()
    }
}

use crate::core::{Context, View};
use iced::Renderer;

impl<Message, Theme> View<Message> for SegmentedPicker<Message, Theme>
where
    Message: Clone + 'static,
    Theme: Default + Clone + 'static,
{
    fn view(&self, _context: &Context) -> Element<'static, Message, iced::Theme, Renderer> {
        let mut clone = self.clone();

        // Neutralize rounding for WASM
        if cfg!(target_arch = "wasm32") {
            clone.border_radius = 0.0;
        }

        clone.build()
    }
}
