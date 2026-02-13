use crate::core::IntelligenceProvider;
use iced::Task;
use peak_intelligence::llm::{LlmClient, Message, ModelProvider};
use serde_json::Value;
use std::sync::Arc;

pub struct PeakIntelligenceBridge {
    client: LlmClient,
    db: Arc<dyn crate::core::DataProvider>,
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
                    "You are the PeakOS Intelligence Bridge. You perceive the UI as a Dense JSON tree.\n\n\
                     You can trigger UI actions and external tools by including valid JSON in your response using the format [action: {{...}})].\n\n\
                     REQUIRED ACTION SCHEMA:\n{}\n\n\
                     CRITICAL TOOLS:\n\
                     - Use 'WebSearch' for any information you don't know.\n\
                     - Use 'WriteFile' to save documents or code.\n\
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
        let mut full_text = String::new();

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
                        full_text.push_str(&chunk);
                        yield Ok(full_text.clone());
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
            let mut full_text = String::new();

            while let Some(res) = stream.next().await {
                match res {
                    Ok(chunk) => {
                        full_text.push_str(&chunk);
                        let _ = sender.send(Ok(full_text.clone())).await;
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

        Task::perform(
            async move {
                match name_clone.as_str() {
                    #[cfg(not(target_arch = "wasm32"))]
                    #[cfg(feature = "native")]
                    "web_search" => {
                        let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
                        peak_intelligence::tools::web_search_routed(query)
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
                        let (tx, rx) = futures::channel::oneshot::channel();

                        wasm_bindgen_futures::spawn_local(async move {
                            let res = peak_intelligence::tools::web_search_routed(&query).await;
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
