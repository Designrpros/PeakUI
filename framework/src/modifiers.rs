#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum ControlSize {
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum Variant {
    #[default]
    Solid, // Full background color
    Soft,    // Light background, dark text
    Outline, // Border only
    Ghost,   // No background until hover
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum Intent {
    #[default]
    Primary,
    Secondary,
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
