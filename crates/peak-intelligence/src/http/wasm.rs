// WASM HTTP implementation using browser Fetch API

use super::{HttpError, HttpResponse};
use serde::Serialize;
// use std::collections::HashMap; // Removed unused import
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

#[allow(dead_code)]
pub async fn get(url: &str) -> Result<HttpResponse, HttpError> {
    get_with_headers(url, std::collections::HashMap::new()).await
}

pub async fn get_with_headers(
    url: &str,
    headers_map: std::collections::HashMap<String, String>,
) -> Result<HttpResponse, HttpError> {
    let window =
        web_sys::window().ok_or_else(|| HttpError::WasmError("No window object".to_string()))?;

    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let headers = Headers::new()
        .map_err(|e| HttpError::WasmError(format!("Failed to create headers: {:?}", e)))?;
    for (key, value) in headers_map {
        headers
            .set(&key, &value)
            .map_err(|e| HttpError::WasmError(format!("Failed to set header {}: {:?}", key, e)))?;
    }
    opts.set_headers(&headers);

    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|e| HttpError::WasmError(format!("Failed to create request: {:?}", e)))?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| HttpError::WasmError(format!("Fetch failed: {:?}", e)))?;

    let resp: Response = resp_value
        .dyn_into()
        .map_err(|_| HttpError::WasmError("Response is not a Response object".to_string()))?;

    let status = resp.status() as u16;

    let array_buffer = JsFuture::from(
        resp.array_buffer()
            .map_err(|e| HttpError::WasmError(format!("Failed to get array buffer: {:?}", e)))?,
    )
    .await
    .map_err(|e| HttpError::WasmError(format!("Failed to read array buffer: {:?}", e)))?;

    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
    let body = uint8_array.to_vec();

    Ok(HttpResponse { status, body })
}

#[allow(dead_code)]
pub async fn post_json<T: Serialize>(url: &str, body: &T) -> Result<HttpResponse, HttpError> {
    post_json_with_headers(url, body, std::collections::HashMap::new()).await
}

pub async fn post_json_with_headers<T: Serialize>(
    url: &str,
    body: &T,
    headers_map: std::collections::HashMap<String, String>,
) -> Result<HttpResponse, HttpError> {
    let window =
        web_sys::window().ok_or_else(|| HttpError::WasmError("No window object".to_string()))?;

    let json_body = serde_json::to_string(body)?;

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&wasm_bindgen::JsValue::from_str(&json_body));

    // Set headers
    let headers = Headers::new()
        .map_err(|e| HttpError::WasmError(format!("Failed to create headers: {:?}", e)))?;
    headers
        .set("Content-Type", "application/json")
        .map_err(|e| HttpError::WasmError(format!("Failed to set Content-Type header: {:?}", e)))?;

    for (key, value) in headers_map {
        headers
            .set(&key, &value)
            .map_err(|e| HttpError::WasmError(format!("Failed to set header {}: {:?}", key, e)))?;
    }
    opts.set_headers(&headers);

    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|e| HttpError::WasmError(format!("Failed to create request: {:?}", e)))?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| HttpError::WasmError(format!("Fetch failed: {:?}", e)))?;

    let resp: Response = resp_value
        .dyn_into()
        .map_err(|_| HttpError::WasmError("Response is not a Response object".to_string()))?;

    let status = resp.status() as u16;

    let array_buffer = JsFuture::from(
        resp.array_buffer()
            .map_err(|e| HttpError::WasmError(format!("Failed to get array buffer: {:?}", e)))?,
    )
    .await
    .map_err(|e| HttpError::WasmError(format!("Failed to read array buffer: {:?}", e)))?;

    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
    let response_body = uint8_array.to_vec();

    Ok(HttpResponse {
        status,
        body: response_body,
    })
}
