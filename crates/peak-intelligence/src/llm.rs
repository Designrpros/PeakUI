use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelProvider {
    Ollama,
    LlamaCpp,
    OpenRouter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Clone)]
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

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn provider(&self) -> ModelProvider {
        self.provider
    }

    pub async fn chat(&self, messages: Vec<Message>) -> Result<String, String> {
        match self.provider {
            ModelProvider::Ollama => self.chat_ollama(messages).await,
            ModelProvider::LlamaCpp => self.chat_llamacpp(messages).await,
            ModelProvider::OpenRouter => self.chat_openrouter(messages).await,
        }
    }

    pub async fn embeddings(&self, text: &str) -> Result<Vec<f32>, String> {
        match self.provider {
            ModelProvider::Ollama => self.embeddings_ollama(text).await,
            ModelProvider::LlamaCpp => self.embeddings_llamacpp(text).await,
            ModelProvider::OpenRouter => Err("Embeddings not yet supported for OpenRouter".to_string()),
        }
    }

    async fn embeddings_ollama(&self, text: &str) -> Result<Vec<f32>, String> {
        let url = "http://localhost:11434/api/embeddings";
        let body = serde_json::json!({
            "model": self.model,
            "prompt": text
        });

        let res = crate::http::HttpClient::post_json(url, &body)
            .await
            .map_err(|e| e.to_string())?;

        if res.status != 200 {
            return Err(format!("Ollama embeddings error: {}", res.status));
        }

        let json: Value = res.json().map_err(|e| e.to_string())?;
        
        json["embedding"]
            .as_array()
            .ok_or("Invalid response format from Ollama embeddings")?
            .iter()
            .map(|v| v.as_f64().map(|f| f as f32).ok_or("Invalid float in embedding".to_string()))
            .collect()
    }

    async fn embeddings_llamacpp(&self, text: &str) -> Result<Vec<f32>, String> {
        let url = "http://localhost:8080/embedding";
        let body = serde_json::json!({
            "content": text
        });

        let res = crate::http::HttpClient::post_json(url, &body)
            .await
            .map_err(|e| e.to_string())?;

        if res.status != 200 {
            return Err(format!("Llama.cpp embeddings error: {}", res.status));
        }

        let json: Value = res.json().map_err(|e| e.to_string())?;
        
        json["embedding"]
            .as_array()
            .ok_or("Invalid response format from Llama.cpp embeddings")?
            .iter()
            .map(|v| v.as_f64().map(|f| f as f32).ok_or("Invalid float in embedding".to_string()))
            .collect()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn chat_stream(
        &self,
        messages: Vec<Message>,
    ) -> impl futures::Stream<Item = Result<String, String>> + Send {
        let client = self.clone();
        async_stream::stream! {
            match client.provider {
                ModelProvider::Ollama => {
                    let stream = client.chat_ollama_stream(messages).await;
                    match stream {
                        Ok(s) => {
                            use futures::StreamExt;
                            let mut s = Box::pin(s);
                            while let Some(chunk) = s.next().await {
                                match chunk {
                                    Ok(text) => yield Ok(text),
                                    Err(e) => yield Err(e),
                                }
                            }
                        }
                        Err(e) => yield Err(e),
                    }
                }
                ModelProvider::LlamaCpp => {
                    let stream = client.chat_llamacpp_stream(messages).await;
                    match stream {
                        Ok(s) => {
                            use futures::StreamExt;
                            let mut s = Box::pin(s);
                            while let Some(chunk) = s.next().await {
                                match chunk {
                                    Ok(text) => yield Ok(text),
                                    Err(e) => yield Err(e),
                                }
                            }
                        }
                        Err(e) => yield Err(e),
                    }
                }
                ModelProvider::OpenRouter => {
                    let stream = client.chat_openrouter_stream(messages).await;
                    match stream {
                        Ok(s) => {
                            use futures::StreamExt;
                            let mut s = Box::pin(s);
                            while let Some(chunk) = s.next().await {
                                match chunk {
                                    Ok(text) => yield Ok(text),
                                    Err(e) => yield Err(e),
                                }
                            }
                        }
                        Err(e) => yield Err(e),
                    }
                }
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn chat_stream(
        &self,
        messages: Vec<Message>,
    ) -> impl futures::Stream<Item = Result<String, String>> {
        let client = self.clone();
        async_stream::stream! {
            match client.provider {
                ModelProvider::Ollama => {
                    let stream = client.chat_ollama_stream(messages).await;
                    match stream {
                        Ok(s) => {
                            use futures::StreamExt;
                            let mut s = Box::pin(s);
                            while let Some(chunk) = s.next().await {
                                match chunk {
                                    Ok(text) => yield Ok(text),
                                    Err(e) => yield Err(e),
                                }
                            }
                        }
                        Err(e) => yield Err(e),
                    }
                }
                ModelProvider::LlamaCpp => {
                    let stream = client.chat_llamacpp_stream(messages).await;
                    match stream {
                        Ok(s) => {
                            use futures::StreamExt;
                            let mut s = Box::pin(s);
                            while let Some(chunk) = s.next().await {
                                match chunk {
                                    Ok(text) => yield Ok(text),
                                    Err(e) => yield Err(e),
                                }
                            }
                        }
                        Err(e) => yield Err(e),
                    }
                }
                ModelProvider::OpenRouter => {
                    let stream = client.chat_openrouter_stream(messages).await;
                    match stream {
                        Ok(s) => {
                            use futures::StreamExt;
                            let mut s = Box::pin(s);
                            while let Some(chunk) = s.next().await {
                                match chunk {
                                    Ok(text) => yield Ok(text),
                                    Err(e) => yield Err(e),
                                }
                            }
                        }
                        Err(e) => yield Err(e),
                    }
                }
            }
        }
    }

    async fn chat_ollama_stream(
        &self,
        messages: Vec<Message>,
    ) -> Result<impl futures::Stream<Item = Result<String, String>>, String> {
        let url = "http://localhost:11434/api/chat";
        let body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "stream": true
        });

        let stream =
            crate::http::HttpClient::post_json_stream(url, &body, std::collections::HashMap::new());

        use futures::StreamExt;
        let mapped = stream.map(|res| {
            res.map_err(|e| e.to_string()).map(|bytes| {
                let s = String::from_utf8_lossy(bytes.as_ref());
                let mut full_content = String::new();

                for line in s.lines() {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    if let Ok(json) = serde_json::from_str::<Value>(line) {
                        if let Some(content) = json["message"]["content"].as_str() {
                            full_content.push_str(content);
                        }
                    }
                }
                full_content
            })
        });

        Ok(mapped)
    }

    async fn chat_llamacpp_stream(
        &self,
        messages: Vec<Message>,
    ) -> Result<impl futures::Stream<Item = Result<String, String>>, String> {
        let url = "http://localhost:8080/v1/chat/completions";

        let body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "stream": true
        });

        let stream =
            crate::http::HttpClient::post_json_stream(url, &body, std::collections::HashMap::new());

        use futures::StreamExt;
        let mapped = stream.map(|res| {
            res.map_err(|e| e.to_string()).map(|bytes| {
                let s = String::from_utf8_lossy(bytes.as_ref());
                let mut full_content = String::new();

                for line in s.lines() {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    if line.starts_with("data: ") {
                        let json_str = &line[6..].trim();
                        if *json_str == "[DONE]" {
                            continue;
                        }

                        if let Ok(json) = serde_json::from_str::<Value>(json_str) {
                            if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                                full_content.push_str(content);
                            }
                        }
                    }
                }
                full_content
            })
        });

        Ok(mapped)
    }

    async fn chat_openrouter_stream(
        &self,
        messages: Vec<Message>,
    ) -> Result<impl futures::Stream<Item = Result<String, String>>, String> {
        let url = "https://openrouter.ai/api/v1/chat/completions";
        let api_key = self.api_key.as_ref().ok_or("OpenRouter API key required")?;

        let body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "stream": true
        });

        let mut headers = std::collections::HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        #[cfg(not(target_arch = "wasm32"))]
        {
            headers.insert("HTTP-Referer".to_string(), "https://peakos.dev".to_string());
            headers.insert("X-Title".to_string(), "PeakOS Intelligence".to_string());
        }

        let stream = crate::http::HttpClient::post_json_stream(url, &body, headers);

        use futures::StreamExt;
        let mapped = stream.map(|res| {
            res.map_err(|e| e.to_string()).map(|bytes| {
                let s = String::from_utf8_lossy(bytes.as_ref());
                let mut full_content = String::new();

                for line in s.lines() {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    if line.starts_with("data: ") {
                        let json_str = &line[6..].trim();
                        if *json_str == "[DONE]" {
                            continue;
                        }

                        if let Ok(json) = serde_json::from_str::<Value>(json_str) {
                            if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                                full_content.push_str(content);
                            }
                        }
                    }
                }
                full_content
            })
        });

        Ok(mapped)
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

    async fn chat_llamacpp(&self, messages: Vec<Message>) -> Result<String, String> {
        let url = "http://localhost:8080/v1/chat/completions";
        let body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "stream": false
        });

        let res = crate::http::HttpClient::post_json(url, &body)
            .await
            .map_err(|e| e.to_string())?;

        if res.status != 200 {
            return Err(format!("Llama.cpp error: {}", res.status));
        }

        let json: Value = res.json().map_err(|e| e.to_string())?;

        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or("Invalid response format from Llama.cpp".to_string())
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

        #[cfg(not(target_arch = "wasm32"))]
        {
            headers.insert("HTTP-Referer".to_string(), "https://peakos.dev".to_string());
            headers.insert("X-Title".to_string(), "PeakOS Intelligence".to_string());
        }

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
