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

impl ControlSize {
    pub fn resolve(&self, is_mobile: bool) -> Self {
        if is_mobile {
            match self {
                ControlSize::Small => ControlSize::Medium,
                ControlSize::Medium => ControlSize::Large,
                ControlSize::Large => ControlSize::XLarge,
                ControlSize::XLarge => ControlSize::XLarge,
            }
        } else {
            *self
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IdealWidth(pub f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Locked(pub bool);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Billboard(pub bool);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicalDepth(pub f32);
