# Neural Control API Commands 🧠🎮

This document lists all available commands for the PeakUI Neural Control API. Commands are sent by writing a JSON object to `.peak/command.json`.

## Navigation Commands

### SetTab
Navigates to a specific page in the application.

**Format:**
```json
{
  "SetTab": "PageVariant"
}
```

**Common Page Variants:**
- `Introduction`
- `VStack`
- `HStack`
- `ZStack`
- `Button`
- `ScrollView`
- `PeakDB`
- `Appearance`

## Global UI Commands

### ToggleSearch / ToggleInspector / ToggleSidebar / ToggleUserProfile
Toggles various UI panels.
**Format:** `{"ToggleSearch": null}`, etc.

### SetNavigationMode
Sets the navigation category (e.g., "Start", "Catalog", "Data", "Settings").
**Format:** `{"SetNavigationMode": "Catalog"}`

### Search
Performs a search throughout the documentation.
**Format:** `{"Search": "button"}`

### SetTheme / SetThemeKind
Controls the appearance of the application.
**Format:**
- `{"SetTheme": "Dark"}` (or `Light`)
- `{"SetThemeKind": "Peak"}` (or `Mountain`)

---

## Component Lab Commands

### Button Lab
- `UpdateButtonLabel`: `{"UpdateButtonLabel": "Submit"}`
- `UpdateButtonVariant`: `{"UpdateButtonVariant": "Solid"}` (Options: `Solid`, `Soft`, `Outline`, `Ghost`)
- `UpdateButtonIntent`: `{"UpdateButtonIntent": "Success"}` (Options: `Primary`, `Secondary`, `Success`, `Warning`, `Danger`, `Info`, `Neutral`)
- `ToggleButtonFullWidth`: `{"ToggleButtonFullWidth": true}`
- `ToggleButtonDisabled`: `{"ToggleButtonDisabled": true}`
- `ToggleButtonFocused`: `{"ToggleButtonFocused": true}`

### Typography Lab
- `UpdateTypographyText`: `{"UpdateTypographyText": "Hello Peek"}`
- `UpdateTypographySize`: `{"UpdateTypographySize": 24.5}`
- `ToggleTypographyBold`: `{"ToggleTypographyBold": true}`
- `ToggleTypographyItalic`: `{"ToggleTypographyItalic": true}`

### Layout Lab
- `UpdateLayoutOuterSpacing`: `{"UpdateLayoutOuterSpacing": 20}`
- `UpdateLayoutInnerSpacing`: `{"UpdateLayoutInnerSpacing": 32}`
- `UpdateLayoutChildCount`: `{"UpdateLayoutChildCount": 5}`
- `UpdateLayoutAlignment`: `{"UpdateLayoutAlignment": "Center"}` (Options: `Start`, `Center`, `End`)

### Sizing Lab
- `UpdateSizingWidthType` / `UpdateSizingHeightType`: `{"UpdateSizingWidthType": "Fill"}` (Options: `Fixed`, `Fill`, `Shrink`)
- `UpdateSizingFixedWidth`: `{"UpdateSizingFixedWidth": 300}`
- `UpdateSizingFixedHeight`: `{"UpdateSizingFixedHeight": 100}`

### Emoji Lab
- `UpdateEmojiScale`: `{"UpdateEmojiScale": 1.5}`
- `UpdateEmojiEmoji`: `{"UpdateEmojiEmoji": "🚀"}`

### Spacer Lab
- `UpdateSpacerWidth`: `{"UpdateSpacerWidth": 20.0}`
- `UpdateSpacerHeight`: `{"UpdateSpacerHeight": 40.0}`

---

## Usage Example
To put the app in Dark Mode, navigate to the Button Lab, and set the label to "DESTRUCT":
```bash
echo '{"SetTheme": "Dark"}' > .peak/command.json
sleep 0.2
echo '{"SetTab": "Button"}' > .peak/command.json
sleep 0.2
echo '{"UpdateButtonLabel": "DESTRUCT"}' > .peak/command.json
echo '{"UpdateButtonIntent": "Danger"}' > .peak/command.json
```

## Usage Example
To navigate to the VStack documentation:
```bash
echo '{"SetTab": "VStack"}' > .peak/command.json
```

## Neural Tagging & Targeting 🏷️

The `.neural(tag)` modifier provides stable, logical identifiers for AI agents. These tags are preserved in the semantic tree regardless of visual changes (styling, position, etc.).

### Targeting with Tags
While standard commands target by role or global state, future API updates will allow direct targeting by `neural_tag`.

Current Lab implementation:
```rust
Button::label("Click Me")
    .neural("lab-primary-action") // stable identifier
```

---
*Note: This API is currently enabled only in the Desktop build.*
