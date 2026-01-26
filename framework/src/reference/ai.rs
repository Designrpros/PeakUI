use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

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

const FALLBACK_KEY: &str = "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";

impl OpenRouterClient {
    pub fn new(user_key: String) -> Self {
        let api_key = if user_key.is_empty() {
            FALLBACK_KEY.to_string()
        } else {
            user_key
        };
        Self { api_key }
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
