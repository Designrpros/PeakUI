use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatCompletionMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
struct OpenRouterRequest {
    model: String,
    messages: Vec<ChatCompletionMessage>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatCompletionMessage,
}

pub struct OpenRouterClient {
    api_key: String,
}

impl OpenRouterClient {
    pub fn new(user_key: String) -> Self {
        Self { api_key: user_key }
    }

    pub async fn chat(&self, messages: Vec<ChatCompletionMessage>) -> Result<String, String> {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key))
                .map_err(|e| e.to_string())?,
        );

        let request = OpenRouterRequest {
            model: "google/gemini-3-flash-preview".to_string(),
            messages,
        };

        let response = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .headers(headers)
            .json(&request)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("API Error: {}", error_text));
        }

        let body: OpenRouterResponse = response.json().await.map_err(|e| e.to_string())?;

        body.choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| "No completion found".to_string())
    }
}
