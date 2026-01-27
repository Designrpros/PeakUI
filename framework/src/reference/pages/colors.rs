use super::super::app::{Message, RenderMode};
use super::super::page::PageResult;
use crate::core::{Backend, SpatialBackend};
use crate::prelude::*;

use super::super::views::ComponentDoc;
use std::sync::Arc;

pub fn view(base_context: &Context, render_mode: RenderMode) -> PageResult {
    let context = base_context;

    // 1. Canvas View (Standard GUI)
    let canvas_preview = create_preview::<IcedBackend>(context);

    // 2. Terminal View (ANSI Text)
    let terminal_preview = create_preview::<TermBackend>(context).view(context);

    // 3. Neural View (Semantic JSON)
    let neural_preview = create_preview::<AIBackend>(context).view(context);

    // 4. Spatial View (3D transforms)
    let spatial_preview = create_preview::<SpatialBackend>(context).view(context);

    let doc = ComponentDoc::new(
        "Colors",
        "The semantic color system defines the visual language of the application, ensuring consistency and accessibility.",
        "// Colors are defined in the Theme and accessed via Context or Style",
        Arc::new(canvas_preview),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
        r#"
### Semantic Colors
PeakUI uses a semantic color system based on roles rather than raw values. This allows themes to be swapped effortlessly while maintaining legibility and intent.

- **Primary:** The main brand color, used for key actions and active states.
- **Secondary:** Less prominent than primary, used for supporting elements.
- **Accent:** A punchy color for highlights and special calls to action.
- **Surface:** The background color for cards, sheets, and menus.
- **Background:** The underlying color of the application window.
"#,
    );

    PageResult::new(doc)
}

fn create_preview<B: Backend>(context: &Context) -> VStack<Message, B> {
    let theme = &context.theme;
    let palette = theme.colors;

    VStack::new_generic()
        .spacing(32.0)
        .padding(20.0)
        .width(Length::Fill)
        .push(
            // Main Colors
            VStack::new_generic()
                .spacing(16.0)
                .push(Text::<B>::new("Main Colors").title3())
                .push(
                    HStack::new_generic()
                        .spacing(16.0)
                        .push(color_swatch::<B>(
                            "Primary",
                            palette.primary,
                            palette.on_primary,
                        ))
                        .push(color_swatch::<B>(
                            "Secondary",
                            palette.secondary,
                            palette.on_secondary,
                        ))
                        .push(color_swatch::<B>(
                            "Accent",
                            palette.accent,
                            palette.on_accent,
                        )),
                ),
        )
        .push(
            // Status Colors
            VStack::new_generic()
                .spacing(16.0)
                .push(Text::<B>::new("Status Colors").title3())
                .push(
                    HStack::new_generic()
                        .spacing(16.0)
                        .push(color_swatch::<B>("Success", palette.success, Color::WHITE))
                        .push(color_swatch::<B>("Warning", palette.warning, Color::BLACK))
                        .push(color_swatch::<B>("Danger", palette.danger, Color::WHITE))
                        .push(color_swatch::<B>("Info", palette.info, Color::WHITE)),
                ),
        )
        .push(
            // Surfaces & Backgrounds
            VStack::new_generic()
                .spacing(16.0)
                .push(Text::<B>::new("Surfaces").title3())
                .push(
                    HStack::new_generic()
                        .spacing(16.0)
                        .push(color_swatch::<B>(
                            "Surface",
                            palette.surface,
                            palette.on_surface,
                        ))
                        .push(color_swatch::<B>(
                            "Surface Variant",
                            palette.surface_variant,
                            palette.on_surface_variant,
                        ))
                        .push(color_swatch::<B>(
                            "Background",
                            palette.background,
                            palette.on_background,
                        )),
                ),
        )
        .push(
            // Text Colors
            VStack::new_generic()
                .spacing(16.0)
                .push(Text::<B>::new("Text Hierarchy").title3())
                .push(
                    // Removed ineffective Container styling for Text Hierarchy
                    VStack::new_generic()
                        .spacing(8.0)
                        .padding(16.0)
                        .push(Text::<B>::new("Text Primary").color(palette.text_primary))
                        .push(Text::<B>::new("Text Secondary").color(palette.text_secondary))
                        .push(Text::<B>::new("Text Tertiary").color(palette.text_tertiary))
                        .push(Text::<B>::new("Text Disabled").color(palette.text_disabled)),
                ),
        )
}

fn color_swatch<B: Backend>(name: &str, color: Color, _text_color: Color) -> impl View<Message, B> {
    VStack::<Message, B>::new_generic()
        .spacing(8.0)
        .push(
            crate::atoms::Rectangle::<B>::new(Length::Fixed(100.0), Length::Fixed(60.0))
                .color(color)
                .radius(8.0),
        )
        .push(
            Text::<B>::new(name)
                .caption2()
                .align_center()
                .width(Length::Fixed(100.0)),
        )
}
