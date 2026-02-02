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
pub enum ControlSize {
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IdealWidth(pub f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Locked(pub bool);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Billboard(pub bool);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicalDepth(pub f32);
