#[cfg(not(target_arch = "wasm32"))]
pub mod exposure;
pub mod protocol;
pub mod bridge;
pub mod mcp;
pub mod ai;

pub use protocol::{Action, ActionParser, ContentPart};
pub use bridge::PeakIntelligenceBridge;
pub use ai::OpenRouterClient;
pub use mcp::get_framework_schema;
