use super::super::app::{Message, RenderMode};
use super::super::page::PageResult;
use crate::core::{Backend, SpatialBackend};
use crate::prelude::*;

use super::super::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context, _is_mobile: bool, render_mode: RenderMode) -> PageResult {
    let code_snippet = r#"
// Accessing theme in a view
let theme = context.theme;
let primary = theme.colors.primary;

// Applying rounded corners
let radius = context.radius(12.0);
"#
    .to_string();

    // 1. Canvas View (Standard GUI)
    let canvas_preview = create_preview::<IcedBackend>();

    // 2. Terminal View (ANSI Text)
    let terminal_preview = create_preview::<TermBackend>().view(_context);

    // 3. Neural View (Semantic JSON)
    let neural_preview = create_preview::<AIBackend>().view(_context);

    // 4. Spatial View (3D transforms)
    let spatial_preview = create_preview::<SpatialBackend>().view(_context);

    let doc = ComponentDoc::new(
        "Customizations",
        "Learn how to customize themes, colors, and global styles.",
        code_snippet,
        Arc::new(canvas_preview),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
        r#"
### Flexible Design System
**PeakUI** uses `ThemeTokens` to control everything from colors to border radii. You can swap themes at runtime for instant UI updates.

Themes are composed of:
- **Colors:** Semantic palette (Primary, Secondary, Success, etc.)
- **Spacing:** Standardized unit gaps.
- **Radius:** Adaptive corner rounding.
- **Scaling:** Global UI scale factor.
"#,
    )
    .props_table(
        r#"
| Modifier | Description |
| :--- | :--- |
| `context.theme` | Access the current theme tokens. |
| `theme.colors` | Access semantic colors. |
| `context.radius(f32)` | Get context-aware corner radius. |
"#,
    );

    PageResult::new(doc)
}

fn create_preview<B: Backend>() -> VStack<Message, B> {
    VStack::new_generic()
        .spacing(24.0)
        .width(Length::Fill)
        .push(
            VStack::new_generic()
                .spacing(12.0)
                .push(Text::<B>::new("Runtime Theming").body().bold())
                .push(
                    Text::<B>::new(
                        "Theme tokens allow you to build interfaces that adapt to user preferences or platform branding without rewriting any logic.",
                    )
                    .secondary(),
                ),
        )
}
