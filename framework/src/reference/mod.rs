pub mod app;
pub use app::message::Message;
pub use app::state::App;
pub mod data;
pub mod intelligence;
pub mod models;
pub mod pages;
pub mod views;

// Unique aliases for the reference app to avoid name collisions with framework prelude
pub use models::result::PageResult as AppPageResult;
pub use models::Page as AppPage;

// Backward compatibility (deprecated, use AppPage/AppPageResult)
pub use models as model;
