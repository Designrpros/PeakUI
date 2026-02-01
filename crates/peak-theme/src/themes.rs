use crate::colors::PeakColors;
use crate::ThemeTone;
use iced::Color;
use peak_core::registry::ShellMode;

/// Available themes in PeakUI
///
/// Each theme provides a complete visual identity with semantic colors
/// that can be applied to any OS mode.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub enum PeakTheme {
    /// macOS-inspired clean design (current Desktop theme)
    #[serde(alias = "cupertino", alias = "Cupertino")]
    Cupertino,
    /// Modern control interface (SmartHome mode)
    #[serde(alias = "smart", alias = "Smart")]
    Smart,
    /// Google Material Design
    #[serde(alias = "material", alias = "Material")]
    Material,
    /// Microsoft Fluent Design
    #[serde(alias = "fluent", alias = "Fluent")]
    Fluent,
    /// High contrast for accessibility
    #[serde(
        alias = "highcontrast",
        alias = "HighContrast",
        alias = "high_contrast"
    )]
    HighContrast,
    /// Cinematic mountain aesthetic
    #[serde(alias = "mountain", alias = "Mountain")]
    Mountain,
    /// Warm, beige/stone aesthetic
    #[serde(alias = "peak", alias = "Peak")]
    Peak,
    /// Monochrome black and white aesthetic
    #[serde(alias = "mono", alias = "Mono")]
    Mono,
}

impl PeakTheme {
    /// Get the Peak Colors for this theme
    /// Get the Peak Colors for this theme
    pub fn colors(self, tone: ThemeTone) -> PeakColors {
        match self {
            PeakTheme::Cupertino => Self::cupertino_colors(tone),
            PeakTheme::Smart => Self::smart_colors(tone),
            PeakTheme::Material => Self::material_colors(tone),
            PeakTheme::Fluent => Self::fluent_colors(tone),
            PeakTheme::HighContrast => Self::high_contrast_colors(tone),
            PeakTheme::Mountain => Self::mountain_colors(tone),
            PeakTheme::Peak => Self::peak_colors(tone),
            PeakTheme::Mono => Self::mono_colors(tone),
        }
    }

    /// Get the recommended default theme for a shell mode
    /// Get the recommended default theme for a shell mode
    pub fn default_for_mode(mode: ShellMode) -> Self {
        match mode {
            ShellMode::Desktop => PeakTheme::Mono,
            ShellMode::Mobile => PeakTheme::Mono,
            ShellMode::Console => PeakTheme::Mono,
            ShellMode::TV => PeakTheme::Mono,
            ShellMode::Robot => PeakTheme::Mono,
            ShellMode::Auto => PeakTheme::Mono,
            ShellMode::Fireplace => PeakTheme::Mono,
            ShellMode::SmartHome => PeakTheme::Mono,
            ShellMode::Kiosk => PeakTheme::Mono,
            ShellMode::Server => PeakTheme::Mono,
        }
    }

    /// Get display name for UI
    pub fn display_name(self) -> &'static str {
        match self {
            PeakTheme::Cupertino => "Cupertino",
            PeakTheme::Smart => "Smart",
            PeakTheme::Material => "Material",
            PeakTheme::Fluent => "Fluent",
            PeakTheme::HighContrast => "High Contrast",
            PeakTheme::Mountain => "Mountain",
            PeakTheme::Peak => "Peak",
            PeakTheme::Mono => "Mono",
        }
    }

    /// Get all available themes
    pub fn all() -> &'static [PeakTheme] {
        &[
            PeakTheme::Mono,
            PeakTheme::Peak,
            PeakTheme::Cupertino,
            PeakTheme::Smart,
            PeakTheme::Material,
            PeakTheme::Fluent,
            PeakTheme::Mountain,
            PeakTheme::HighContrast,
        ]
    }

    // === Theme Color Implementations ===

    fn peak_colors(tone: ThemeTone) -> PeakColors {
        match tone {
            ThemeTone::Light => PeakColors {
                // Anthropic-inspired Beige Theme
                primary: Color::from_rgb8(180, 140, 100), // Warm beige-brown accent
                on_primary: Color::WHITE,
                primary_container: Color::from_rgb8(245, 240, 230),
                on_primary_container: Color::from_rgb8(90, 70, 50),

                secondary: Color::from_rgb8(140, 130, 120), // Stone gray
                on_secondary: Color::WHITE,
                secondary_container: Color::from_rgb8(235, 230, 225),
                on_secondary_container: Color::from_rgb8(60, 55, 50),

                accent: Color::from_rgb8(45, 60, 110), // Professional Deep Indigo-Navy
                on_accent: Color::WHITE,

                success: Color::from_rgb8(100, 160, 100), // Muted organic green
                warning: Color::from_rgb8(220, 180, 80),  // Muted gold
                danger: Color::from_rgb8(200, 100, 100),  // Clay red
                info: Color::from_rgb8(100, 150, 200),    // Faded denim

                surface: Color::from_rgb8(252, 250, 245), // Off-white cream paper
                on_surface: Color::from_rgb8(60, 55, 50), // Warm dark gray text
                surface_variant: Color::from_rgb8(245, 240, 230),
                on_surface_variant: Color::from_rgb8(100, 95, 90),

                background: Color::from_rgb8(250, 248, 242), // Very light beige
                on_background: Color::from_rgb8(60, 55, 50),

                border: Color::from_rgba8(160, 140, 120, 40.0),
                divider: Color::from_rgba8(160, 140, 120, 0.2),
                overlay: Color::from_rgba8(60, 55, 50, 0.1),

                text_primary: Color::from_rgb8(40, 35, 30),
                text_secondary: Color::from_rgb8(70, 65, 60), // Darker gray for better visibility
                text_tertiary: Color::from_rgba8(60, 55, 50, 0.5),
                text_disabled: Color::from_rgba8(60, 55, 50, 0.3),
            },
            ThemeTone::Dark => PeakColors {
                // Stone Warm Dark Black
                primary: Color::from_rgb8(180, 160, 140), // Light stone
                on_primary: Color::BLACK,
                primary_container: Color::from_rgb8(60, 55, 50),
                on_primary_container: Color::from_rgb8(230, 220, 210),

                secondary: Color::from_rgb8(120, 115, 110),
                on_secondary: Color::WHITE,
                secondary_container: Color::from_rgb8(50, 48, 45),
                on_secondary_container: Color::from_rgb8(200, 195, 190),

                accent: Color::from_rgb8(210, 150, 100), // Warm clay
                on_accent: Color::BLACK,

                success: Color::from_rgb8(120, 180, 120),
                warning: Color::from_rgb8(220, 200, 120),
                danger: Color::from_rgb8(220, 120, 120),
                info: Color::from_rgb8(120, 160, 200),

                surface: Color::from_rgb8(35, 33, 30), // Warm charcoal
                on_surface: Color::from_rgb8(235, 230, 225),
                surface_variant: Color::from_rgb8(45, 43, 40),
                on_surface_variant: Color::from_rgb8(200, 195, 190),

                background: Color::from_rgb8(25, 23, 20), // Deep warm black
                on_background: Color::from_rgb8(235, 230, 225),

                border: Color::from_rgba8(200, 190, 180, 0.12),
                divider: Color::from_rgba8(200, 190, 180, 0.06),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.6),

                text_primary: Color::from_rgb8(235, 230, 225),
                text_secondary: Color::from_rgba8(235, 230, 225, 0.7),
                text_tertiary: Color::from_rgba8(235, 230, 225, 0.5),
                text_disabled: Color::from_rgba8(235, 230, 225, 0.3),
            },
        }
    }

    fn cupertino_colors(tone: ThemeTone) -> PeakColors {
        match tone {
            ThemeTone::Light => PeakColors {
                primary: Color::from_rgb8(50, 110, 200), // Muted Azure
                on_primary: Color::WHITE,
                primary_container: Color::from_rgb8(235, 242, 255),
                on_primary_container: Color::from_rgb8(0, 50, 100),

                secondary: Color::from_rgb8(110, 110, 130), // Soft Slate
                on_secondary: Color::WHITE,
                secondary_container: Color::from_rgb8(242, 242, 247),
                on_secondary_container: Color::from_rgb8(60, 60, 67),

                accent: Color::from_rgb8(240, 80, 110), // Soft Coral
                on_accent: Color::WHITE,

                success: Color::from_rgb8(70, 180, 100),
                warning: Color::from_rgb8(255, 170, 50),
                danger: Color::from_rgb8(240, 80, 80),
                info: Color::from_rgb8(70, 130, 200),

                surface: Color::WHITE,
                on_surface: Color::from_rgb8(20, 20, 25),
                surface_variant: Color::from_rgb8(242, 242, 247),
                on_surface_variant: Color::from_rgb8(60, 60, 67),

                background: Color::from_rgb8(250, 250, 252), // Very slight blue undertone
                on_background: Color::from_rgb8(20, 20, 25),

                border: Color::from_rgba8(0, 0, 0, 12.0),
                divider: Color::from_rgba8(0, 0, 0, 8.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.2),

                text_primary: Color::from_rgb8(20, 20, 25),
                text_secondary: Color::from_rgba8(60, 60, 67, 160.0),
                text_tertiary: Color::from_rgba8(60, 60, 67, 100.0),
                text_disabled: Color::from_rgba8(60, 60, 67, 70.0),
            },
            ThemeTone::Dark => PeakColors {
                primary: Color::from_rgb8(80, 140, 220), // Soft Light Blue
                on_primary: Color::WHITE,
                primary_container: Color::from_rgb8(20, 60, 100),
                on_primary_container: Color::from_rgb8(200, 230, 255),

                secondary: Color::from_rgb8(160, 160, 180),
                on_secondary: Color::BLACK,
                secondary_container: Color::from_rgb8(50, 50, 60),
                on_secondary_container: Color::from_rgb8(220, 220, 230),

                accent: Color::from_rgb8(255, 100, 130),
                on_accent: Color::WHITE,

                success: Color::from_rgb8(80, 200, 120),
                warning: Color::from_rgb8(255, 190, 80),
                danger: Color::from_rgb8(255, 100, 90),
                info: Color::from_rgb8(80, 160, 240),

                surface: Color::from_rgb8(30, 30, 35),
                on_surface: Color::from_rgb8(245, 245, 250),
                surface_variant: Color::from_rgb8(45, 45, 50),
                on_surface_variant: Color::from_rgb8(200, 200, 210),

                background: Color::from_rgb8(15, 15, 20),
                on_background: Color::from_rgb8(245, 245, 250),

                border: Color::from_rgba8(255, 255, 255, 12.0),
                divider: Color::from_rgba8(255, 255, 255, 12.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.5),

                text_primary: Color::from_rgb8(245, 245, 250),
                text_secondary: Color::from_rgba8(235, 235, 245, 160.0),
                text_tertiary: Color::from_rgba8(235, 235, 245, 100.0),
                text_disabled: Color::from_rgba8(235, 235, 245, 70.0),
            },
        }
    }

    fn smart_colors(tone: ThemeTone) -> PeakColors {
        match tone {
            ThemeTone::Light => PeakColors {
                primary: Color::from_rgb8(110, 180, 210), // Desaturated Cyan
                on_primary: Color::WHITE,
                primary_container: Color::from_rgb8(235, 248, 255),
                on_primary_container: Color::from_rgb8(20, 60, 80),

                secondary: Color::from_rgb8(140, 160, 180),
                on_secondary: Color::WHITE,
                secondary_container: Color::from_rgb8(235, 240, 245),
                on_secondary_container: Color::from_rgb8(40, 50, 60),

                accent: Color::from_rgb8(140, 210, 230),
                on_accent: Color::WHITE,

                success: Color::from_rgb8(100, 190, 120),
                warning: Color::from_rgb8(240, 180, 60),
                danger: Color::from_rgb8(240, 90, 80),
                info: Color::from_rgb8(110, 180, 210),

                surface: Color::WHITE,
                on_surface: Color::from_rgb8(30, 40, 50),
                surface_variant: Color::from_rgb8(242, 246, 250),
                on_surface_variant: Color::from_rgb8(60, 70, 80),

                background: Color::from_rgb8(250, 252, 255),
                on_background: Color::from_rgb8(30, 40, 50),

                border: Color::from_rgba8(110, 180, 210, 30.0),
                divider: Color::from_rgba8(0, 0, 0, 12.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.2),

                text_primary: Color::from_rgb8(30, 40, 50),
                text_secondary: Color::from_rgba8(30, 40, 50, 170.0),
                text_tertiary: Color::from_rgba8(30, 40, 50, 120.0),
                text_disabled: Color::from_rgba8(30, 40, 50, 80.0),
            },
            ThemeTone::Dark => PeakColors {
                primary: Color::from_rgb8(120, 200, 230), // Soft Cyan
                on_primary: Color::BLACK,
                primary_container: Color::from_rgb8(20, 60, 80),
                on_primary_container: Color::from_rgb8(200, 240, 255),

                secondary: Color::from_rgb8(140, 170, 190),
                on_secondary: Color::BLACK,
                secondary_container: Color::from_rgb8(30, 50, 70),
                on_secondary_container: Color::from_rgb8(210, 230, 240),

                accent: Color::from_rgb8(160, 220, 240),
                on_accent: Color::BLACK,

                success: Color::from_rgb8(100, 200, 120),
                warning: Color::from_rgb8(250, 200, 80),
                danger: Color::from_rgb8(250, 100, 90),
                info: Color::from_rgb8(120, 200, 230),

                surface: Color::from_rgb8(25, 30, 35),
                on_surface: Color::from_rgb8(230, 240, 250),
                surface_variant: Color::from_rgb8(40, 45, 50),
                on_surface_variant: Color::from_rgb8(180, 200, 210),

                background: Color::from_rgb8(15, 20, 25),
                on_background: Color::from_rgb8(230, 240, 250),

                border: Color::from_rgba8(120, 200, 230, 40.0),
                divider: Color::from_rgba8(255, 255, 255, 20.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.5),

                text_primary: Color::from_rgb8(230, 240, 250),
                text_secondary: Color::from_rgba8(230, 240, 250, 180.0),
                text_tertiary: Color::from_rgba8(230, 240, 250, 120.0),
                text_disabled: Color::from_rgba8(230, 240, 250, 80.0),
            },
        }
    }

    fn material_colors(tone: ThemeTone) -> PeakColors {
        match tone {
            ThemeTone::Light => PeakColors {
                primary: Color::from_rgb8(140, 120, 190), // Pastel Purple
                on_primary: Color::WHITE,
                primary_container: Color::from_rgb8(235, 230, 245),
                on_primary_container: Color::from_rgb8(80, 60, 120),

                secondary: Color::from_rgb8(100, 200, 200), // Soft Teal
                on_secondary: Color::BLACK,
                secondary_container: Color::from_rgb8(220, 245, 245),
                on_secondary_container: Color::from_rgb8(0, 60, 60),

                accent: Color::from_rgb8(240, 130, 160), // Soft Pink
                on_accent: Color::WHITE,

                success: Color::from_rgb8(90, 180, 100),
                warning: Color::from_rgb8(255, 180, 60),
                danger: Color::from_rgb8(240, 90, 80),
                info: Color::from_rgb8(80, 170, 240),

                surface: Color::WHITE,
                on_surface: Color::from_rgba8(0, 0, 0, 222.0),
                surface_variant: Color::from_rgb8(248, 248, 250),
                on_surface_variant: Color::from_rgba8(0, 0, 0, 160.0),

                background: Color::from_rgb8(252, 252, 252),
                on_background: Color::from_rgba8(0, 0, 0, 222.0),

                border: Color::from_rgba8(0, 0, 0, 20.0),
                divider: Color::from_rgba8(0, 0, 0, 12.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.3),

                text_primary: Color::from_rgba8(0, 0, 0, 222.0),
                text_secondary: Color::from_rgba8(0, 0, 0, 150.0),
                text_tertiary: Color::from_rgba8(0, 0, 0, 110.0),
                text_disabled: Color::from_rgba8(0, 0, 0, 80.0),
            },
            ThemeTone::Dark => PeakColors {
                primary: Color::from_rgb8(190, 160, 230), // Lighter Pastel Purple
                on_primary: Color::BLACK,
                primary_container: Color::from_rgb8(80, 60, 120),
                on_primary_container: Color::from_rgb8(235, 230, 245),

                secondary: Color::from_rgb8(130, 220, 220),
                on_secondary: Color::BLACK,
                secondary_container: Color::from_rgb8(30, 80, 80),
                on_secondary_container: Color::from_rgb8(210, 240, 240),

                accent: Color::from_rgb8(255, 150, 180),
                on_accent: Color::BLACK,

                success: Color::from_rgb8(120, 200, 130),
                warning: Color::from_rgb8(250, 200, 100),
                danger: Color::from_rgb8(240, 120, 110),
                info: Color::from_rgb8(120, 190, 250),

                surface: Color::from_rgb8(30, 30, 30),
                on_surface: Color::WHITE,
                surface_variant: Color::from_rgb8(45, 45, 45),
                on_surface_variant: Color::from_rgb8(210, 210, 210),

                background: Color::from_rgb8(18, 18, 18),
                on_background: Color::WHITE,

                border: Color::from_rgba8(255, 255, 255, 20.0),
                divider: Color::from_rgba8(255, 255, 255, 15.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.5),

                text_primary: Color::WHITE,
                text_secondary: Color::from_rgba8(255, 255, 255, 190.0),
                text_tertiary: Color::from_rgba8(255, 255, 255, 140.0),
                text_disabled: Color::from_rgba8(255, 255, 255, 100.0),
            },
        }
    }

    fn fluent_colors(tone: ThemeTone) -> PeakColors {
        match tone {
            ThemeTone::Light => PeakColors {
                primary: Color::from_rgb8(60, 130, 190), // Soft Blue
                on_primary: Color::WHITE,
                primary_container: Color::from_rgb8(220, 240, 255),
                on_primary_container: Color::from_rgb8(20, 70, 100),

                secondary: Color::from_rgb8(120, 120, 120),
                on_secondary: Color::WHITE,
                secondary_container: Color::from_rgb8(240, 240, 240),
                on_secondary_container: Color::from_rgb8(40, 40, 40),

                accent: Color::from_rgb8(60, 150, 210),
                on_accent: Color::WHITE,

                success: Color::from_rgb8(60, 160, 80),
                warning: Color::from_rgb8(250, 180, 50),
                danger: Color::from_rgb8(220, 70, 70),
                info: Color::from_rgb8(60, 130, 190),

                surface: Color::from_rgb8(248, 248, 248),
                on_surface: Color::from_rgb8(30, 30, 30),
                surface_variant: Color::from_rgb8(252, 252, 252),
                on_surface_variant: Color::from_rgb8(80, 80, 80),

                background: Color::from_rgb8(255, 255, 255),
                on_background: Color::from_rgb8(30, 30, 30),

                border: Color::from_rgba8(0, 0, 0, 25.0),
                divider: Color::from_rgba8(0, 0, 0, 15.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.3),

                text_primary: Color::from_rgb8(30, 30, 30),
                text_secondary: Color::from_rgba8(0, 0, 0, 150.0),
                text_tertiary: Color::from_rgba8(0, 0, 0, 110.0),
                text_disabled: Color::from_rgba8(0, 0, 0, 90.0),
            },
            ThemeTone::Dark => PeakColors {
                primary: Color::from_rgb8(100, 170, 230), // Lighter Soft Blue
                on_primary: Color::BLACK,
                primary_container: Color::from_rgb8(20, 80, 130),
                on_primary_container: Color::from_rgb8(220, 240, 255),

                secondary: Color::from_rgb8(160, 160, 160),
                on_secondary: Color::BLACK,
                secondary_container: Color::from_rgb8(70, 70, 70),
                on_secondary_container: Color::from_rgb8(230, 230, 230),

                accent: Color::from_rgb8(100, 170, 230),
                on_accent: Color::BLACK,

                success: Color::from_rgb8(110, 210, 120),
                warning: Color::from_rgb8(255, 210, 80),
                danger: Color::from_rgb8(255, 100, 100),
                info: Color::from_rgb8(100, 170, 230),

                surface: Color::from_rgb8(35, 35, 35),
                on_surface: Color::from_rgb8(245, 245, 245),
                surface_variant: Color::from_rgb8(50, 50, 50),
                on_surface_variant: Color::from_rgb8(210, 210, 210),

                background: Color::from_rgb8(26, 26, 26),
                on_background: Color::from_rgb8(245, 245, 245),

                border: Color::from_rgba8(255, 255, 255, 25.0),
                divider: Color::from_rgba8(255, 255, 255, 15.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.5),

                text_primary: Color::from_rgb8(245, 245, 245),
                text_secondary: Color::from_rgba8(255, 255, 255, 190.0),
                text_tertiary: Color::from_rgba8(255, 255, 255, 140.0),
                text_disabled: Color::from_rgba8(255, 255, 255, 110.0),
            },
        }
    }

    fn high_contrast_colors(tone: ThemeTone) -> PeakColors {
        match tone {
            ThemeTone::Light => PeakColors {
                primary: Color::BLACK,
                on_primary: Color::WHITE,
                primary_container: Color::from_rgb8(220, 220, 220),
                on_primary_container: Color::BLACK,

                secondary: Color::from_rgb8(60, 60, 60),
                on_secondary: Color::WHITE,
                secondary_container: Color::from_rgb8(200, 200, 200),
                on_secondary_container: Color::BLACK,

                accent: Color::from_rgb8(0, 0, 200),
                on_accent: Color::WHITE,

                success: Color::from_rgb8(0, 150, 0),
                warning: Color::from_rgb8(200, 150, 0),
                danger: Color::from_rgb8(200, 0, 0),
                info: Color::from_rgb8(0, 0, 200),

                surface: Color::WHITE,
                on_surface: Color::BLACK,
                surface_variant: Color::from_rgb8(240, 240, 240),
                on_surface_variant: Color::BLACK,

                background: Color::WHITE,
                on_background: Color::BLACK,

                border: Color::BLACK,
                divider: Color::BLACK,
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.5),

                text_primary: Color::BLACK,
                text_secondary: Color::from_rgb8(60, 60, 60),
                text_tertiary: Color::from_rgb8(100, 100, 100),
                text_disabled: Color::from_rgb8(150, 150, 150),
            },
            ThemeTone::Dark => PeakColors {
                primary: Color::WHITE,
                on_primary: Color::BLACK,
                primary_container: Color::from_rgb8(60, 60, 60),
                on_primary_container: Color::WHITE,

                secondary: Color::from_rgb8(200, 200, 200),
                on_secondary: Color::BLACK,
                secondary_container: Color::from_rgb8(80, 80, 80),
                on_secondary_container: Color::WHITE,

                accent: Color::from_rgb8(100, 200, 255),
                on_accent: Color::BLACK,

                success: Color::from_rgb8(100, 255, 100),
                warning: Color::from_rgb8(255, 220, 100),
                danger: Color::from_rgb8(255, 100, 100),
                info: Color::from_rgb8(100, 200, 255),

                surface: Color::BLACK,
                on_surface: Color::WHITE,
                surface_variant: Color::from_rgb8(30, 30, 30),
                on_surface_variant: Color::WHITE,

                background: Color::BLACK,
                on_background: Color::WHITE,

                border: Color::WHITE,
                divider: Color::WHITE,
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.7),

                text_primary: Color::WHITE,
                text_secondary: Color::from_rgb8(200, 200, 200),
                text_tertiary: Color::from_rgb8(150, 150, 150),
                text_disabled: Color::from_rgb8(100, 100, 100),
            },
        }
    }

    fn mountain_colors(tone: ThemeTone) -> PeakColors {
        match tone {
            ThemeTone::Light => PeakColors {
                primary: Color::from_rgb8(60, 130, 180), // Softer mountain blue
                on_primary: Color::WHITE,
                primary_container: Color::from_rgb8(230, 240, 250),
                on_primary_container: Color::from_rgb8(20, 60, 90),

                secondary: Color::from_rgb8(140, 150, 160), // Softer Slate
                on_secondary: Color::WHITE,
                secondary_container: Color::from_rgb8(240, 242, 245),
                on_secondary_container: Color::from_rgb8(50, 60, 70),

                accent: Color::from_rgb8(220, 140, 80), // Muted Sunset orange
                on_accent: Color::WHITE,

                success: Color::from_rgb8(80, 170, 100),
                warning: Color::from_rgb8(220, 180, 60),
                danger: Color::from_rgb8(210, 80, 80),
                info: Color::from_rgb8(80, 140, 200),

                surface: Color::WHITE,
                on_surface: Color::from_rgb8(30, 40, 50),
                surface_variant: Color::from_rgb8(245, 248, 250),
                on_surface_variant: Color::from_rgb8(80, 90, 100),

                background: Color::from_rgb8(250, 252, 255),
                on_background: Color::from_rgb8(30, 40, 50),

                border: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
                divider: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.2),

                text_primary: Color::from_rgb8(30, 40, 50),
                text_secondary: Color::from_rgb8(80, 90, 100),
                text_tertiary: Color::from_rgb8(148, 163, 184),
                text_disabled: Color::from_rgb8(200, 210, 220),
            },
            ThemeTone::Dark => PeakColors {
                primary: Color::from_rgb8(100, 160, 200), // Muted Sky blue
                on_primary: Color::BLACK,
                primary_container: Color::from_rgb8(20, 60, 80),
                on_primary_container: Color::from_rgb8(200, 230, 250),

                secondary: Color::from_rgb8(140, 150, 160), // Slate
                on_secondary: Color::BLACK,
                secondary_container: Color::from_rgb8(40, 50, 60),
                on_secondary_container: Color::from_rgb8(200, 210, 220),

                accent: Color::from_rgb8(220, 150, 90), // Muted Sunset glow
                on_accent: Color::BLACK,

                success: Color::from_rgb8(100, 180, 110),
                warning: Color::from_rgb8(230, 190, 80),
                danger: Color::from_rgb8(220, 100, 100),
                info: Color::from_rgb8(100, 160, 220),

                surface: Color::from_rgba8(25, 30, 40, 0.8),
                on_surface: Color::WHITE,
                surface_variant: Color::from_rgba8(40, 50, 60, 0.6),
                on_surface_variant: Color::from_rgb8(200, 210, 220),

                background: Color::from_rgb8(15, 20, 25), // Dark deep blue-grey
                on_background: Color::WHITE,

                border: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
                divider: Color::from_rgba(1.0, 1.0, 1.0, 0.05),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.5),

                text_primary: Color::WHITE,
                text_secondary: Color::from_rgb8(200, 210, 220),
                text_tertiary: Color::from_rgb8(140, 150, 160),
                text_disabled: Color::from_rgb8(80, 90, 100),
            },
        }
    }

    fn mono_colors(tone: ThemeTone) -> PeakColors {
        match tone {
            ThemeTone::Light => PeakColors {
                primary: Color::BLACK,
                on_primary: Color::WHITE,
                primary_container: Color::from_rgb8(240, 240, 240),
                on_primary_container: Color::BLACK,

                secondary: Color::from_rgb8(60, 60, 60),
                on_secondary: Color::WHITE,
                secondary_container: Color::from_rgb8(245, 245, 245),
                on_secondary_container: Color::BLACK,

                accent: Color::BLACK,
                on_accent: Color::WHITE,

                success: Color::from_rgb8(34, 197, 94), // Vibrant Green
                warning: Color::from_rgb8(100, 100, 100),
                danger: Color::from_rgb8(150, 0, 0), // Small hint of red for danger
                info: Color::from_rgb8(0, 0, 0),

                surface: Color::WHITE,
                on_surface: Color::BLACK,
                surface_variant: Color::from_rgb8(248, 248, 248),
                on_surface_variant: Color::from_rgb8(60, 60, 60),

                background: Color::WHITE,
                on_background: Color::BLACK,

                border: Color::from_rgba8(0, 0, 0, 0.15),
                divider: Color::from_rgba8(0, 0, 0, 0.1),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.5),

                text_primary: Color::BLACK,
                text_secondary: Color::from_rgb8(80, 80, 80),
                text_tertiary: Color::from_rgb8(140, 140, 140),
                text_disabled: Color::from_rgb8(180, 180, 180),
            },
            ThemeTone::Dark => PeakColors {
                primary: Color::WHITE,
                on_primary: Color::BLACK,
                primary_container: Color::from_rgb8(40, 40, 40),
                on_primary_container: Color::WHITE,

                secondary: Color::from_rgb8(180, 180, 180),
                on_secondary: Color::BLACK,
                secondary_container: Color::from_rgb8(30, 30, 30),
                on_secondary_container: Color::WHITE,

                accent: Color::WHITE,
                on_accent: Color::BLACK,

                success: Color::from_rgb8(34, 197, 94), // Vibrant Green
                warning: Color::from_rgb8(150, 150, 150),
                danger: Color::from_rgb8(255, 100, 100),
                info: Color::WHITE,

                surface: Color::from_rgb8(15, 15, 15),
                on_surface: Color::WHITE,
                surface_variant: Color::from_rgb8(25, 25, 25),
                on_surface_variant: Color::from_rgb8(180, 180, 180),

                background: Color::BLACK,
                on_background: Color::WHITE,

                border: Color::from_rgba8(255, 255, 255, 0.15),
                divider: Color::from_rgba8(255, 255, 255, 0.1),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.7),

                text_primary: Color::WHITE,
                text_secondary: Color::from_rgb8(180, 180, 180),
                text_tertiary: Color::from_rgb8(120, 120, 120),
                text_disabled: Color::from_rgb8(80, 80, 80),
            },
        }
    }
}
