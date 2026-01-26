use iced::Color;

/// Peak Colors: Semantic color system for PeakUI
///
/// This provides a consistent color vocabulary across all themes.
/// Each theme implements these semantic roles with different actual colors.
///
/// Inspired by Material Design 3 color system and Apple's semantic colors.
#[derive(Debug, Clone, Copy)]
pub struct PeakColors {
    // === Primary Colors ===
    /// Main brand color, used for primary actions
    pub primary: Color,
    /// Text/icons on primary color background
    pub on_primary: Color,
    /// Lighter variant of primary for containers
    pub primary_container: Color,
    /// Text/icons on primary container
    pub on_primary_container: Color,

    // === Secondary Colors ===
    /// Secondary brand color, used for less prominent actions
    pub secondary: Color,
    /// Text/icons on secondary color background
    pub on_secondary: Color,
    /// Lighter variant of secondary for containers
    pub secondary_container: Color,
    /// Text/icons on secondary container
    pub on_secondary_container: Color,

    // === Accent/Tertiary ===
    /// Accent color for highlights and emphasis
    pub accent: Color,
    /// Text/icons on accent color background
    pub on_accent: Color,

    // === Semantic Status Colors ===
    /// Success/positive state (usually green)
    pub success: Color,
    /// Warning/caution state (usually yellow/orange)
    pub warning: Color,
    /// Error/danger state (usually red)
    pub danger: Color,
    /// Informational state (usually blue)
    pub info: Color,

    // === Surface Colors ===
    /// Base surface color (e.g., cards, panels)
    pub surface: Color,
    /// Text/icons on surface
    pub on_surface: Color,
    /// Variant surface (slightly different from base)
    pub surface_variant: Color,
    /// Text/icons on surface variant
    pub on_surface_variant: Color,

    // === Background ===
    /// Main background color
    pub background: Color,
    /// Text/icons on background
    pub on_background: Color,

    // === Structural Colors ===
    /// Border color for outlined elements
    pub border: Color,
    /// Divider/separator lines
    pub divider: Color,
    /// Overlay/scrim color (for modals, dropdowns)
    pub overlay: Color,

    // === Text Colors (aliases for convenience) ===
    /// Primary text color (alias to on_background)
    pub text_primary: Color,
    /// Secondary/muted text
    pub text_secondary: Color,
    /// Tertiary/hint text
    pub text_tertiary: Color,
    /// Disabled text
    pub text_disabled: Color,
}

impl PeakColors {
    /// Create a basic color palette from a minimal set of colors
    ///
    /// This is a helper for quick theme prototyping. Most themes should
    /// explicitly define all colors for best results.
    pub fn from_simple(primary: Color, background: Color, surface: Color, is_dark: bool) -> Self {
        let on_primary = if is_dark { Color::BLACK } else { Color::WHITE };

        let on_background = if is_dark { Color::WHITE } else { Color::BLACK };

        let on_surface = on_background;

        // Generate secondary as a variant of primary (simple approach)
        let secondary = Self::adjust_color(primary, if is_dark { 0.8 } else { 1.2 });

        Self {
            primary,
            on_primary,
            primary_container: Self::adjust_color(primary, 1.3),
            on_primary_container: on_background,

            secondary,
            on_secondary: on_primary,
            secondary_container: Self::adjust_color(secondary, 1.3),
            on_secondary_container: on_background,

            accent: primary,
            on_accent: on_primary,

            success: Color::from_rgb8(76, 175, 80),
            warning: Color::from_rgb8(255, 152, 0),
            danger: Color::from_rgb8(244, 67, 54),
            info: Color::from_rgb8(33, 150, 243),

            surface,
            on_surface,
            surface_variant: Self::adjust_color(surface, if is_dark { 1.1 } else { 0.95 }),
            on_surface_variant: Self::fade_color(on_surface, 0.7),

            background,
            on_background,

            border: Self::fade_color(on_background, 0.12),
            divider: Self::fade_color(on_background, 0.08),
            overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.5),

            text_primary: on_background,
            text_secondary: Self::fade_color(on_background, 0.6),
            text_tertiary: Self::fade_color(on_background, 0.38),
            text_disabled: Self::fade_color(on_background, 0.26),
        }
    }

    /// Adjust color brightness
    fn adjust_color(color: Color, factor: f32) -> Color {
        Color {
            r: (color.r * factor).min(1.0),
            g: (color.g * factor).min(1.0),
            b: (color.b * factor).min(1.0),
            a: color.a,
        }
    }

    /// Fade color by adjusting alpha
    fn fade_color(color: Color, alpha: f32) -> Color {
        Color { a: alpha, ..color }
    }

    /// Check if this is a dark color scheme
    pub fn is_dark(&self) -> bool {
        // Simple luminance check on background
        let bg = self.background;
        let luminance = 0.299 * bg.r + 0.587 * bg.g + 0.114 * bg.b;
        luminance < 0.5
    }
}
