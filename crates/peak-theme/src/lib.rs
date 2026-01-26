mod colors;
mod themes;

pub use colors::PeakColors;
pub use themes::PeakTheme;

use iced::Color;
use peak_core::registry::ShellMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ThemeTone {
    Light,
    Dark,
}

/// Theme tokens for PeakUI components
///
/// This combines semantic colors (PeakColors) with visual properties
/// like blur, shadows, and spacing.
#[derive(Debug, Clone, Copy)]
pub struct ThemeTokens {
    // === Semantic Colors ===
    /// Peak Colors semantic color system
    pub colors: PeakColors,
    /// Theme tone (Light or Dark)
    pub tone: ThemeTone,

    // === Visual Properties ===
    /// Glassmorphism opacity (0.0 - 1.0)
    pub glass_opacity: f32,
    /// Blur radius for glass effects
    pub blur_radius: f32,
    /// Border radius for rounded corners
    pub radius: f32,
    /// Shadow color
    pub shadow_color: Color,
    /// Shadow offset [x, y]
    pub shadow_offset: [f32; 2],
    /// Shadow blur radius
    pub shadow_blur: f32,
    /// Base spacing unit (for consistent spacing)
    pub spacing_unit: f32,
    /// Global UI scaling factor (e.g., 0.8 for compact desktop)
    pub scaling: f32,
}

impl ThemeTokens {
    /// Create theme tokens from a PeakTheme and tone
    pub fn new(theme: PeakTheme, tone: ThemeTone) -> Self {
        let colors = theme.colors(tone);

        // Theme-specific visual properties
        let (glass_opacity, blur_radius, radius, shadow_blur, spacing_unit) = match theme {
            PeakTheme::Cupertino => (0.7, 40.0, 12.0, 16.0, 8.0),
            PeakTheme::Gaming => (0.6, 15.0, 8.0, 16.0, 12.0),
            PeakTheme::MediaCenter => (0.8, 30.0, 16.0, 40.0, 16.0),
            PeakTheme::Ambient => (0.4, 25.0, 30.0, 20.0, 16.0),
            PeakTheme::Terminal => (0.9, 0.0, 0.0, 0.0, 8.0),
            PeakTheme::Automotive => (1.0, 0.0, 40.0, 10.0, 16.0),
            PeakTheme::Smart => (0.9, 20.0, 24.0, 15.0, 12.0),
            PeakTheme::Material => (1.0, 0.0, 4.0, 8.0, 8.0),
            PeakTheme::Fluent => (0.9, 30.0, 4.0, 8.0, 8.0),
            PeakTheme::HighContrast => (1.0, 0.0, 0.0, 0.0, 8.0),
            PeakTheme::Mountain => (0.5, 60.0, 20.0, 30.0, 10.0),
            PeakTheme::Peak => (0.6, 20.0, 16.0, 20.0, 12.0),
        };

        let shadow_offset = match theme {
            PeakTheme::MediaCenter => [0.0, 20.0],
            PeakTheme::Ambient => [0.0, 10.0],
            PeakTheme::Terminal | PeakTheme::HighContrast | PeakTheme::Automotive => [0.0, 0.0],
            _ => [0.0, 4.0],
        };

        let shadow_color = if colors.is_dark() {
            Color::from_rgba(0.0, 0.0, 0.0, 0.5)
        } else {
            Color::from_rgba(0.0, 0.0, 0.0, 0.15)
        };

        #[allow(unused_mut)]
        let mut tokens = Self {
            colors,
            tone,
            glass_opacity,
            blur_radius,
            radius,
            shadow_color,
            shadow_offset,
            shadow_blur,
            spacing_unit,
            scaling: 1.0, // Default to 1.0, can be overridden
        };

        #[cfg(target_arch = "wasm32")]
        {
            tokens.glass_opacity = 1.0;
            tokens.blur_radius = 0.0;
            tokens.radius = 0.0;
            tokens.shadow_blur = 0.0;
            tokens.shadow_offset = [0.0, 0.0];
        }

        tokens
    }

    /// Get theme tokens for a shell mode using its default theme
    ///
    /// This is the backward-compatible constructor that existing code uses.
    pub fn get(mode: ShellMode, tone: ThemeTone) -> Self {
        let theme = PeakTheme::default_for_mode(mode);
        Self::new(theme, tone)
    }

    /// Get theme tokens with explicit theme choice
    pub fn with_theme(theme: PeakTheme, tone: ThemeTone) -> Self {
        Self::new(theme, tone)
    }
}

impl Default for ThemeTokens {
    fn default() -> Self {
        Self::get(ShellMode::Desktop, ThemeTone::Light)
    }
}
