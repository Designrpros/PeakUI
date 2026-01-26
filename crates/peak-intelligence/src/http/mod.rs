// Platform-agnostic HTTP client abstraction
// This module provides a unified interface for HTTP requests that works on both
// native and WASM targets.

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct HttpClient;

#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[cfg(target_arch = "wasm32")]
    #[error("Wasm error: {0}")]
    WasmError(String),
}

impl HttpClient {
    pub fn new() -> Self {
        Self
    }

    /// Perform a GET request
    pub async fn get(url: &str) -> Result<HttpResponse, HttpError> {
        Self::get_with_headers(url, std::collections::HashMap::new()).await
    }

    /// Perform a GET request with custom headers
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_with_headers(
        url: &str,
        headers: std::collections::HashMap<String, String>,
    ) -> Result<HttpResponse, HttpError> {
        native::get_with_headers(url, headers).await
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_with_headers(
        url: &str,
        headers: std::collections::HashMap<String, String>,
    ) -> Result<HttpResponse, HttpError> {
        wasm::get_with_headers(url, headers).await
    }

    /// Perform a POST request with JSON body
    pub async fn post_json<T: Serialize>(url: &str, body: &T) -> Result<HttpResponse, HttpError> {
        Self::post_json_with_headers(url, body, std::collections::HashMap::new()).await
    }

    /// Perform a POST request with JSON body and custom headers
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn post_json_with_headers<T: Serialize>(
        url: &str,
        body: &T,
        headers: std::collections::HashMap<String, String>,
    ) -> Result<HttpResponse, HttpError> {
        native::post_json_with_headers(url, body, headers).await
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn post_json_with_headers<T: Serialize>(
        url: &str,
        body: &T,
        headers: std::collections::HashMap<String, String>,
    ) -> Result<HttpResponse, HttpError> {
        wasm::post_json_with_headers(url, body, headers).await
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpResponse {
    pub fn text(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }

    pub fn json<T: for<'de> Deserialize<'de>>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.body)
    }

    pub fn bytes(&self) -> &[u8] {
        &self.body
    }
}
