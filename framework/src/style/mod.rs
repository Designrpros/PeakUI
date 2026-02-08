use crate::localization::Localization;
use iced::{Color, Length, Padding, Shadow, Size, Vector};
pub use peak_core::registry::ShellMode;
pub use peak_theme::ThemeTokens;
use std::sync::Arc;

pub use crate::modifiers::ControlSize;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub enum Variant {
    #[default]
    Solid, // Full background color
    Soft,    // Light background, dark text
    Outline, // Border only
    Ghost,   // No background until hover
    Compact, // No background, minimal spacing
    Plain,   // No background, no padding, no styling (click-only)
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub enum Intent {
    #[default]
    Primary,
    Secondary,
    Accent,
    Success,
    Warning,
    Danger,
    Info,
    Neutral,
}

/// Runtime context for rendering and layout.
///
/// `Context` provides environmental data such as the current theme, screen size,
/// and target shell mode (e.g., Desktop vs Mobile), allowing views to adapt
/// dynamically to their environment.
#[derive(Clone, Debug)]
pub struct Context {
    /// Theme tokens for colors, spacing, and typography.
    pub theme: ThemeTokens,
    /// The current shell environment.
    pub mode: ShellMode,
    /// The type of hardware device.
    pub device: DeviceType,
    /// The size of the window or container.
    pub size: Size,
    /// Padding for safe areas (e.g. notches).
    pub safe_area: Padding,
    /// The ID of the currently focused element.
    pub focused_id: Option<Arc<str>>,
    /// System localization settings.
    pub localization: Localization,
    /// A unique identifier for the current Peak session.
    pub peak_id: Arc<str>,
    /// An optional override for the foreground color.
    pub foreground: Option<Color>,
    /// Whether billboarding is active in spatial environments.
    pub billboarding: bool,
    /// Whether we are currently inside a ScrollView (to prevent unlimited height paradox).
    pub is_inside_scrollable: bool,
    /// A monotonically increasing tick count for animations and dynamic state.
    pub tick: u64,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            mode: ShellMode::Desktop,
            theme: ThemeTokens::default(),
            device: DeviceType::Desktop,
            size: Size::ZERO,
            safe_area: Padding::ZERO,
            focused_id: None,
            localization: Localization::default(),
            peak_id: "".into(),
            foreground: None,
            billboarding: false,
            is_inside_scrollable: false,
            tick: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DeviceType {
    #[default]
    Desktop,
    Mobile,
    TV,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollDirection {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

impl Context {
    pub fn new(
        mode: ShellMode,
        theme: ThemeTokens,
        size: Size,
        localization: Localization,
    ) -> Self {
        let device = match mode {
            ShellMode::Desktop => DeviceType::Desktop,
            ShellMode::Mobile => DeviceType::Mobile,
            ShellMode::TV | ShellMode::Console | ShellMode::Fireplace | ShellMode::Spatial => {
                DeviceType::TV
            }
            _ => DeviceType::Desktop,
        };

        Self {
            theme,
            mode,
            device,
            size,
            safe_area: Self::auto_padding(mode, size),
            focused_id: None,
            localization,
            peak_id: "".into(),
            foreground: None,
            billboarding: false,
            is_inside_scrollable: false,
            tick: 0,
        }
    }

    pub fn is_focused(&self, id: &str) -> bool {
        self.focused_id.as_deref() == Some(id)
    }

    pub fn with_focus(mut self, id: impl Into<Arc<str>>) -> Self {
        self.focused_id = Some(id.into());
        self
    }

    pub fn is_slim(&self) -> bool {
        self.size.width < 900.0
    }

    pub fn with_nested_scroll(mut self) -> Self {
        self.is_inside_scrollable = true;
        self
    }

    /// Automatically calculates standard safe area padding for different shell modes.
    pub fn auto_padding(mode: ShellMode, size: Size) -> Padding {
        let is_slim = size.width < 900.0;

        match mode {
            ShellMode::Desktop => {
                Padding {
                    top: 12.0, // Slight padding top for traffic lights
                    right: 0.0,
                    bottom: 12.0, // Dock remains at absolute edge
                    left: 0.0,
                }
            }
            ShellMode::Mobile => {
                Padding {
                    top: if is_slim { 36.0 } else { 24.0 }, // Tightened notch
                    right: 0.0,
                    bottom: 24.0, // Tightened mobile dock
                    left: 0.0,
                }
            }
            _ => Padding::default(),
        }
    }

    pub fn is_wide(&self) -> bool {
        self.size.width > 1200.0
    }

    pub fn is_mobile(&self) -> bool {
        self.device == DeviceType::Mobile
    }

    pub fn shadow(&self, color: Color, offset: impl Into<Vector>, blur_radius: f32) -> Shadow {
        if cfg!(target_arch = "wasm32") {
            Shadow::default()
        } else {
            Shadow {
                color,
                offset: offset.into(),
                blur_radius,
            }
        }
    }

    pub fn radius(&self, radius: f32) -> iced::border::Radius {
        radius.into()
    }

    pub fn with_safe_area(mut self, padding: Padding) -> Self {
        self.safe_area = padding;
        self
    }

    pub fn t(&self, key: &str) -> String {
        self.localization.simple(key)
    }

    pub fn scale_length(&self, l: Length) -> Length {
        match l {
            Length::Fixed(f) => Length::Fixed(f * self.theme.scaling),
            _ => l,
        }
    }
}
