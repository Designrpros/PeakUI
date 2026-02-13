use crate::tools::{web_search_routed, IntelligenceTool};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct WebSearchTool;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl IntelligenceTool for WebSearchTool {
    fn name(&self) -> &'static str {
        "web_search"
    }
    fn description(&self) -> &'static str {
        "Search the web for real-time information."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": "The search query" },
                "brave_key": { "type": "string", "description": "Optional Brave Search API key" },
                "tavily_key": { "type": "string", "description": "Optional Tavily Search API key" }
            },
            "required": ["query"]
        })
    }
    async fn execute(&self, args: Value) -> Result<Value> {
        let query = args
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'query' argument"))?;
        let brave_key = args
            .get("brave_key")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let tavily_key = args
            .get("tavily_key")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        web_search_routed(query, brave_key, tavily_key).await
    }
}
