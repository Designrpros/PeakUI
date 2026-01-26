use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelProvider {
    Ollama,
    OpenRouter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub struct LlmClient {
    provider: ModelProvider,
    model: String,
    api_key: Option<String>,
}

impl LlmClient {
    pub fn new(provider: ModelProvider, model: String, api_key: Option<String>) -> Self {
        Self {
            provider,
            model,
            api_key,
        }
    }

    pub async fn chat(&self, messages: Vec<Message>) -> Result<String, String> {
        match self.provider {
            ModelProvider::Ollama => self.chat_ollama(messages).await,
            ModelProvider::OpenRouter => self.chat_openrouter(messages).await,
        }
    }

    async fn chat_ollama(&self, messages: Vec<Message>) -> Result<String, String> {
        let url = "http://localhost:11434/api/chat";
        let body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "stream": false
        });

        let res = crate::http::HttpClient::post_json(url, &body)
            .await
            .map_err(|e| e.to_string())?;

        if res.status != 200 {
            return Err(format!("Ollama error: {}", res.status));
        }

        let json: Value = res.json().map_err(|e| e.to_string())?;

        json["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or("Invalid response format from Ollama".to_string())
    }

    async fn chat_openrouter(&self, messages: Vec<Message>) -> Result<String, String> {
        let url = "https://openrouter.ai/api/v1/chat/completions";
        let api_key = self.api_key.as_ref().ok_or("OpenRouter API key required")?;

        let body = serde_json::json!({
            "model": self.model,
            "messages": messages
        });

        let mut headers = std::collections::HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let res = crate::http::HttpClient::post_json_with_headers(url, &body, headers)
            .await
            .map_err(|e| e.to_string())?;

        if res.status != 200 {
            return Err(format!("OpenRouter error: {}", res.status));
        }

        let json: Value = res.json().map_err(|e| e.to_string())?;

        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or("Invalid response format from OpenRouter".to_string())
    }
}
