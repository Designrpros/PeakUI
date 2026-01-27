#[macro_use]
extern crate lazy_static;

pub mod app_registry;

pub mod app_traits;
pub mod apps;
pub mod icons;
pub mod integrations;
pub mod models;
pub mod registry;
pub mod styles;
pub mod systems;
pub mod theme;
pub mod utils;

// Re-export key types for convenience
pub use app_registry::AppRegistry;
pub use registry::{AppId, AppInfo, AppMetadata, ShellMode};
pub use theme::Theme;
