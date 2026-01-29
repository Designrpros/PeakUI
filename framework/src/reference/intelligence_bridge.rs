use crate::core::IntelligenceProvider;
use iced::Task;
use peak_os_intelligence::llm::{LlmClient, Message, ModelProvider};
use serde_json::Value;

pub struct PeakIntelligenceBridge {
    client: LlmClient,
}

impl PeakIntelligenceBridge {
    pub fn new(provider: ModelProvider, model: impl Into<String>, api_key: Option<String>) -> Self {
        Self {
            client: LlmClient::new(provider, model.into(), api_key),
        }
    }
}

impl IntelligenceProvider for PeakIntelligenceBridge {
    fn chat(
        &self,
        messages: Vec<crate::core::ChatCompletionMessage>,
    ) -> Task<std::result::Result<String, String>> {
        let llm_messages = messages
            .into_iter()
            .map(|m| Message {
                role: m.role,
                content: m.content,
            })
            .collect();

        let client = self.client.clone();
        Task::perform(async move { client.chat(llm_messages).await }, |res| res)
    }

    fn chat_stream(
        &self,
        messages: Vec<crate::core::ChatCompletionMessage>,
    ) -> iced::futures::stream::BoxStream<'static, std::result::Result<String, String>> {
        use iced::futures::StreamExt;

        let llm_messages: Vec<Message> = messages
            .into_iter()
            .map(|m| Message {
                role: m.role,
                content: m.content,
            })
            .collect();

        let client = self.client.clone();
        let mut full_text = String::new();

        async_stream::stream! {
            let mut stream = Box::pin(client.chat_stream(llm_messages));
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

    fn execute_tool(&self, name: String, args: Value) -> Task<std::result::Result<Value, String>> {
        // For now, this is a stub. In a real scenario, this would route to PeakOS tool handlers.
        Task::perform(
            async move {
                Ok(serde_json::json!({
                    "status": "success",
                    "tool": name,
                    "args": args
                }))
            },
            |res| res,
        )
    }

    fn get_system_context(&self) -> String {
        format!("PeakOS Local AI (Ollama). Model: {}", self.client.model())
    }
}

// Add Clone to LlmClient if it doesn't have it, or wrap in Arc
// Looking at llm.rs, LlmClient doesn't have Clone. Let's fix that or wrap it.
