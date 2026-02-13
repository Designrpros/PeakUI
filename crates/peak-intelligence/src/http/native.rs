// Native HTTP implementation using reqwest

use super::{HttpError, HttpResponse};
use serde::Serialize;

#[allow(dead_code)]
pub async fn get(url: &str) -> Result<HttpResponse, HttpError> {
    get_with_headers(url, std::collections::HashMap::new()).await
}

pub async fn get_with_headers(
    url: &str,
    headers: std::collections::HashMap<String, String>,
) -> Result<HttpResponse, HttpError> {
    let client = reqwest::Client::new();
    let mut request = client.get(url);

    for (key, value) in headers {
        request = request.header(key, value);
    }

    let response = request.send().await?;
    let status = response.status().as_u16();
    let body = response.bytes().await?.to_vec();

    Ok(HttpResponse { status, body })
}

#[allow(dead_code)]
pub async fn post_json<T: Serialize>(url: &str, body: &T) -> Result<HttpResponse, HttpError> {
    post_json_with_headers(url, body, std::collections::HashMap::new()).await
}

pub async fn post_json_with_headers<T: Serialize>(
    url: &str,
    body: &T,
    headers: std::collections::HashMap<String, String>,
) -> Result<HttpResponse, HttpError> {
    let client = reqwest::Client::new();
    let mut request = client.post(url).json(body);

    for (key, value) in headers {
        request = request.header(key, value);
    }

    let response = request.send().await?;
    let status = response.status().as_u16();
    let response_body = response.bytes().await?.to_vec();

    Ok(HttpResponse {
        status,
        body: response_body,
    })
}

pub fn post_json_stream<T: Serialize + 'static>(
    url: &str,
    body: &T,
    headers: std::collections::HashMap<String, String>,
) -> impl futures::Stream<Item = Result<String, String>> {
    let url = url.to_string();
    let headers = headers.clone();
    let body_json = serde_json::to_string(body).unwrap_or_default();

    async_stream::try_stream! {
        let client = reqwest::Client::new();
        let mut request = client.post(&url)
            .header("Content-Type", "application/json")
            .body(body_json);

        for (key, value) in headers {
            request = request.header(key, value);
        }

        let mut response = request.send().await
            .map_err(|e| e.to_string())?;

        while let Some(chunk) = response.chunk().await.map_err(|e| e.to_string())? {
            if let Ok(text) = String::from_utf8(chunk.to_vec()) {
                yield text;
            }
        }
    }
}
