pub mod assistant;
pub mod chat;
pub mod model;
pub mod plan;
pub mod settings;
pub mod web;

pub use assistant::Assistant;
pub use chat::Chat;
pub use model::Model;
pub use plan::Plan;
pub use settings::Settings;
pub use url::Url;

pub(crate) mod directory;
pub(crate) mod request;

use std::io;
use std::sync::Arc;
#[cfg(feature = "native")]
use tokio::task;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("request failed: {0}")]
    RequestFailed(String),
    #[error("io operation failed: {0}")]
    IOFailed(Arc<io::Error>),
    #[error("docker operation failed: {0}")]
    DockerFailed(&'static str),
    #[cfg(feature = "llm")]
    #[error("llama-server failed: {0:?}")]
    ExecutorFailed(llama_server::Error),
    #[error("JSON deserialization failed: {0}")]
    InvalidJson(Arc<serde_json::Error>),
    #[error("TOML deserialization failed: {0}")]
    InvalidToml(Arc<toml::de::Error>),
    #[error("TOML serialization impossible: {0}")]
    ImpossibleToml(Arc<toml::ser::Error>),
    #[error("deserialization failed")]
    DecoderFailed(Arc<decoder::Error>),
    #[cfg(feature = "native")]
    #[error("task join failed: {0}")]
    JoinFailed(Arc<task::JoinError>),
    #[error("no suitable executor was found: neither llama-server nor docker are installed")]
    NoExecutorAvailable,
    #[error("operation not supported on WASM: {0}")]
    WasmError(String),
}

#[cfg(feature = "native")]
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::RequestFailed(error.to_string())
    }
}

impl From<crate::http::HttpError> for Error {
    fn from(error: crate::http::HttpError) -> Self {
        Self::RequestFailed(error.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IOFailed(Arc::new(error))
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::InvalidJson(Arc::new(error))
    }
}

impl From<toml::ser::Error> for Error {
    fn from(error: toml::ser::Error) -> Self {
        Self::ImpossibleToml(Arc::new(error))
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Self::InvalidToml(Arc::new(error))
    }
}

impl From<decoder::Error> for Error {
    fn from(error: decoder::Error) -> Self {
        Self::DecoderFailed(Arc::new(error))
    }
}

#[cfg(feature = "native")]
impl From<task::JoinError> for Error {
    fn from(error: task::JoinError) -> Self {
        Self::JoinFailed(Arc::new(error))
    }
}

#[cfg(feature = "llm")]
impl From<llama_server::Error> for Error {
    fn from(error: llama_server::Error) -> Self {
        Self::ExecutorFailed(error)
    }
}
