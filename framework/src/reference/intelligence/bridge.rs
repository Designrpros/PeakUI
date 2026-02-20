#![cfg(feature = "intelligence")]
use crate::core::IntelligenceProvider;
use chrono;
use iced::Task;
use peak_intelligence::llm::{LlmClient, Message, ModelProvider};
use serde_json::Value;
use std::sync::Arc;
use uuid;

use std::sync::OnceLock;

pub static GLOBAL_BRIDGE: OnceLock<Arc<PeakIntelligenceBridge>> = OnceLock::new();

pub struct PeakIntelligenceBridge {
    client: LlmClient,
    db: Arc<dyn crate::core::DataProvider>,
}

impl std::fmt::Debug for PeakIntelligenceBridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PeakIntelligenceBridge")
            .field("client", &"LlmClient")
            .finish()
    }
}

impl Clone for PeakIntelligenceBridge {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            db: self.db.clone(),
        }
    }
}

impl Default for PeakIntelligenceBridge {
    fn default() -> Self {
        Self::new(
            ModelProvider::Ollama,
            "llama3",
            None,
            Arc::new(crate::reference::data::stub_db::StubDB::new()),
        )
    }
}

impl PeakIntelligenceBridge {
    pub fn new(
        provider: ModelProvider,
        model: impl Into<String>,
        api_key: Option<String>,
        db: Arc<dyn crate::core::DataProvider>,
    ) -> Self {
        Self {
            client: LlmClient::new(provider, model.into(), api_key),
            db,
        }
    }

    pub async fn chat_direct(
        &self,
        messages: Vec<crate::core::ChatCompletionMessage>,
    ) -> std::result::Result<String, String> {
        let client = self.client.clone();
        let db = self.db.clone();

        let mut final_messages = Vec::new();

        // Generate Action Schema
        let schema = schemars::schema_for!(crate::reference::intelligence::Action);
        let schema_json = serde_json::to_string_pretty(&schema).unwrap_or_default();

        let system_instruction = format!(
            "You are the PeakOS Intelligence AI Assistant. You perceive the UI as a Dense JSON tree.\n\n\
             You can trigger UI actions and external tools by including valid JSON in your response using the format [action: {{...}})].\n\n\
             REQUIRED ACTION SCHEMA:\n{}\n\n\
             CRITICAL TOOLS:\n\
             - Use 'WebSearch' for any information you don't know.\n\
             - Use 'WriteFile' to save documents or code. ALWAYS prefer '~/Desktop/' for user visibility. DO NOT use the OS root '/' as it is read-only.\n\
             - Use 'Navigate' to move between pages.\n\n\
             CRITICAL: You MUST terminate actions with ')]'. \n\
             Example: [action: {{\"WebSearch\": \"latest rust version\"}})]",
            schema_json
        );

        final_messages.push(Message {
            role: "system".to_string(),
            content: system_instruction,
        });

        // 1. RAG: Search for context if we have a user message
        if let Some(user_msg) = messages.iter().rev().find(|m| m.role == "user") {
            let query = user_msg.content.clone();

            let mut final_records = Vec::new();
            // Try semantic search first
            if let Ok(vector) = client.embeddings(&query).await {
                if let Ok(semantic_records) = db.async_find_semantic(vector, 10).await {
                    final_records = semantic_records;
                }
            }

            // Fallback/Supplement with keyword search
            if let Ok(keyword_records) = db.async_find(query).await {
                for r in keyword_records {
                    if !final_records.iter().any(|existing| existing.id == r.id) {
                        final_records.push(r);
                    }
                }
            }

            if !final_records.is_empty() {
                let context = final_records
                    .iter()
                    .map(|r| format!("[Memory: {}] {}", r.collection, r.content))
                    .collect::<Vec<_>>()
                    .join("\n");

                final_messages.push(Message {
                    role: "system".to_string(),
                    content: format!("### LONG-TERM MEMORY (Neural Layer)\nThe following information was retrieved from your persistent memory bank. Priority context:\n\n{}", context),
                });
            }
        }

        // 2. Append original messages
        for m in messages {
            final_messages.push(Message {
                role: m.role,
                content: m.content,
            });
        }

        client.chat(final_messages).await
    }
}

impl IntelligenceProvider for PeakIntelligenceBridge {
    fn model(&self) -> &str {
        self.client.model()
    }

    fn provider(&self) -> ModelProvider {
        self.client.provider()
    }

    fn chat(
        &self,
        messages: Vec<crate::core::ChatCompletionMessage>,
    ) -> Task<std::result::Result<String, String>> {
        let client = self.client.clone();
        let db = self.db.clone();
        let messages_clone = messages.clone();

        Task::perform(
            async move {
                let mut final_messages = Vec::new();

                // Generate Action Schema
                let schema = schemars::schema_for!(crate::reference::intelligence::Action);
                let schema_json = serde_json::to_string_pretty(&schema).unwrap_or_default();

                let system_instruction = format!(
                    "You are the PeakOS Intelligence AI Assistant. You perceive the UI as a Dense JSON tree.\n\n\
                     You can trigger UI actions and external tools by including valid JSON in your response using the format [action: {{...}})].\n\n\
                     REQUIRED ACTION SCHEMA:\n{}\n\n\
                     CRITICAL TOOLS:\n\
                     - Use 'WebSearch' for any information you don't know.\n\
                     - Use 'WriteFile' to save documents or code. ALWAYS prefer '~/Desktop/' for user visibility. DO NOT use the OS root '/' as it is read-only.\n\
                     - Use 'Navigate' to move between pages.\n\n\
                     CRITICAL: You MUST terminate actions with ')]'. \n\
                     Example: [action: {{\"WebSearch\": \"latest rust version\"}})]",
                    schema_json
                );

                final_messages.push(Message {
                    role: "system".to_string(),
                    content: system_instruction,
                });

                // 1. RAG: Search for context if we have a user message
                if let Some(user_msg) = messages_clone.iter().rev().find(|m| m.role == "user") {
                    let client = client.clone();
                    let db = db.clone();
                    let query = user_msg.content.clone();

                    let mut final_records = Vec::new();
                    // Try semantic search first
                    if let Ok(vector) = client.embeddings(&query).await {
                        if let Ok(semantic_records) = db.async_find_semantic(vector, 10).await {
                            final_records = semantic_records;
                        }
                    }

                    // Fallback/Supplement with keyword search
                    if let Ok(keyword_records) = db.async_find(query).await {
                        for r in keyword_records {
                            if !final_records.iter().any(|existing| existing.id == r.id) {
                                final_records.push(r);
                            }
                        }
                    }

                    if !final_records.is_empty() {
                        let context = final_records
                            .iter()
                            .map(|r| format!("[Memory: {}] {}", r.collection, r.content))
                            .collect::<Vec<_>>()
                            .join("\n");

                        final_messages.push(Message {
                            role: "system".to_string(),
                            content: format!("### LONG-TERM MEMORY (Neural Layer)\nThe following information was retrieved from your persistent memory bank. Priority context:\n\n{}", context),
                        });
                    }
                }

                // 2. Append original messages
                for m in messages_clone {
                    final_messages.push(Message {
                        role: m.role,
                        content: m.content,
                    });
                }

                client.chat(final_messages).await
            },
            |res| res,
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn chat_stream(
        &self,
        messages: Vec<crate::core::ChatCompletionMessage>,
    ) -> iced::futures::stream::BoxStream<'static, std::result::Result<String, String>> {
        use iced::futures::StreamExt;

        let client = self.client.clone();
        let db = self.db.clone();
        let messages_clone = messages.clone();

        async_stream::stream! {
            let mut final_messages = Vec::new();

            // Generate Action Schema
            let schema = schemars::schema_for!(crate::reference::intelligence::Action);
            let schema_json = serde_json::to_string_pretty(&schema).unwrap_or_default();

            let system_instruction = format!(
                "You are the PeakOS Intelligence Bridge. You can trigger UI actions by including valid JSON in your response using the format [action: {{...}})].\n\nREQUIRED ACTION SCHEMA:\n{}\n\nExample: To navigate to settings, output: [action: {{\"Navigate\": \"SettingsAI\"}})]",
                schema_json
            );

            final_messages.push(Message {
                role: "system".to_string(),
                content: system_instruction,
            });

            // 1. RAG: Search for context if we have a user message
            if let Some(user_msg) = messages_clone.iter().rev().find(|m| m.role == "user") {
                if let Ok(records) = db.async_find(user_msg.content.clone()).await {
                    if !records.is_empty() {
                        let context = records
                            .iter()
                            .map(|r| format!("[Memory: {}] {}", r.collection, r.content))
                            .collect::<Vec<_>>()
                            .join("\n");

                        final_messages.push(Message {
                            role: "system".to_string(),
                            content: format!("Relevant context from PeakDB:\n{}", context),
                        });
                    }
                }
            }

            // 2. Append original messages
            for m in messages_clone {
                final_messages.push(Message {
                    role: m.role,
                    content: m.content,
                });
            }

            let mut stream = Box::pin(client.chat_stream(final_messages));
            while let Some(res) = stream.next().await {
                match res {
                    Ok(chunk) => {
                        yield Ok(chunk);
                    }
                    Err(e) => {
                        yield Err(e);
                        break;
                    }
                }
            }
        }
        .boxed()
    }

    #[cfg(target_arch = "wasm32")]
    fn chat_stream(
        &self,
        messages: Vec<crate::core::ChatCompletionMessage>,
    ) -> iced::futures::stream::BoxStream<'static, std::result::Result<String, String>> {
        use iced::futures::{SinkExt, StreamExt};
        let (mut sender, receiver) = iced::futures::channel::mpsc::channel(100);

        let client = self.client.clone();
        let db = self.db.clone();
        let messages_clone = messages.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let mut final_messages = Vec::new();

            // Generate Action Schema
            let schema = schemars::schema_for!(crate::reference::intelligence::Action);
            let schema_json = serde_json::to_string_pretty(&schema).unwrap_or_default();

            let system_instruction = format!(
                "You are the PeakOS Intelligence Bridge. You can trigger UI actions by including valid JSON in your response using the format [action: {{...}})].\n\nREQUIRED ACTION SCHEMA:\n{}\n\nExample: To navigate to settings, output: [action: {{\"Navigate\": \"SettingsAI\"}})]",
                schema_json
            );

            final_messages.push(Message {
                role: "system".to_string(),
                content: system_instruction,
            });

            // 1. RAG: Search for context if we have a user message
            if let Some(user_msg) = messages_clone.iter().rev().find(|m| m.role == "user") {
                if let Ok(records) = db.async_find(user_msg.content.clone()).await {
                    if !records.is_empty() {
                        let context = records
                            .iter()
                            .map(|r| format!("[Memory: {}] {}", r.collection, r.content))
                            .collect::<Vec<_>>()
                            .join("\n");

                        final_messages.push(Message {
                            role: "system".to_string(),
                            content: format!("Relevant context from PeakDB:\n{}", context),
                        });
                    }
                }
            }

            // 2. Append original messages
            for m in messages_clone {
                final_messages.push(Message {
                    role: m.role,
                    content: m.content,
                });
            }

            let stream = client.chat_stream(final_messages);
            let mut stream = Box::pin(stream);

            while let Some(res) = stream.next().await {
                match res {
                    Ok(chunk) => {
                        let _ = sender.send(Ok(chunk)).await;
                    }
                    Err(e) => {
                        let _ = sender.send(Err(e)).await;
                        break;
                    }
                }
            }
        });

        receiver.boxed()
    }

    fn execute_tool(&self, name: String, args: Value) -> Task<std::result::Result<Value, String>> {
        let name_clone = name.clone();
        let client = self.client.clone();
        let db = self.db.clone();

        Task::perform(
            async move {
                match name_clone.as_str() {
                    #[cfg(not(target_arch = "wasm32"))]
                    #[cfg(feature = "native")]
                    "web_search" => {
                        let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
                        let brave_key = args
                            .get("brave_key")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        let tavily_key = args
                            .get("tavily_key")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        peak_intelligence::tools::web_search_routed(query, brave_key, tavily_key)
                            .await
                            .map_err(|e| e.to_string())
                    }
                    #[cfg(target_arch = "wasm32")]
                    "web_search" => {
                        let query = args
                            .get("query")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let brave_key = args
                            .get("brave_key")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        let tavily_key = args
                            .get("tavily_key")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        let (tx, rx) = futures::channel::oneshot::channel();

                        wasm_bindgen_futures::spawn_local(async move {
                            let res = peak_intelligence::tools::web_search_routed(
                                &query, brave_key, tavily_key,
                            )
                            .await;
                            let _ = tx.send(res.map_err(|e| e.to_string()));
                        });

                        rx.await.map_err(|e| e.to_string())?
                    }
                    #[cfg(feature = "native")]
                    "get_system_snapshot" => {
                        peak_intelligence::tools::get_system_snapshot().map_err(|e| e.to_string())
                    }
                    #[cfg(feature = "native")]
                    "read_file" => {
                        let path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
                        peak_intelligence::tools::read_file(path).map_err(|e| e.to_string())
                    }
                    #[cfg(feature = "native")]
                    "write_file" => {
                        let path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
                        let content = args.get("content").and_then(|v| v.as_str()).unwrap_or("");
                        peak_intelligence::tools::write_file(path, content)
                            .map_err(|e| e.to_string())
                    }
                    #[cfg(feature = "native")]
                    "list_processes" => {
                        peak_intelligence::tools::list_processes().map_err(|e| e.to_string())
                    }
                    "memorize" => {
                        let content = args.get("content").and_then(|v| v.as_str()).unwrap_or("");
                        if content.is_empty() {
                            return Ok(
                                serde_json::json!({ "status": "error", "message": "No content provided" }),
                            );
                        }

                        let db = db.clone();
                        let client = client.clone();
                        let content_owned = content.to_string();

                        // Generate embedding for the new memory
                        let vector = client.embeddings(&content_owned).await.ok();

                        let record = crate::semantic::SemanticRecord {
                            id: uuid::Uuid::new_v4().to_string(),
                            collection: "Memory".to_string(),
                            content: content_owned,
                            vector,
                            metadata: serde_json::json!({}),
                            timestamp: chrono::Utc::now().timestamp() as u64,
                        };

                        db.async_save(record).await?;

                        Ok(serde_json::json!({
                            "status": "success",
                            "message": "Information saved to memory with semantic embedding."
                        }))
                    }
                    _ => Ok(serde_json::json!({
                        "status": "success",
                        "tool": name_clone,
                        "args": args,
                        "message": "Tool stub executed (logic not yet linked or platform not supported)"
                    })),
                }
            },
            |res| res,
        )
    }

    fn get_system_context(&self) -> String {
        let provider_name = match self.client.provider() {
            ModelProvider::Ollama => "Local AI (Ollama)",
            ModelProvider::LlamaCpp => "Local AI (Llama.cpp)",
            ModelProvider::OpenRouter => "Cloud AI (OpenRouter)",
        };
        format!("PeakOS {}. Model: {}", provider_name, self.client.model())
    }
}
