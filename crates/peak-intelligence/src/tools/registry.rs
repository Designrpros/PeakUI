use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait IntelligenceTool: Send + Sync {
    /// Unique name of the tool (e.g., "read_file")
    fn name(&self) -> &'static str;

    /// Human-readable description of what the tool does
    fn description(&self) -> &'static str;

    /// JSON schema for the tool's input arguments
    fn input_schema(&self) -> Value;

    /// Execute the tool with the provided arguments
    async fn execute(&self, args: Value) -> Result<Value>;

    /// Check if the tool is available in the current environment
    fn is_available(&self) -> bool {
        true
    }
}

pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn IntelligenceTool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: impl IntelligenceTool + 'static) {
        self.tools.insert(tool.name().to_string(), Arc::new(tool));
    }

    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn IntelligenceTool>> {
        self.tools.get(name).cloned()
    }

    pub fn list_tools(&self) -> Vec<Value> {
        self.tools
            .values()
            .filter(|t| t.is_available())
            .map(|t| {
                json!({
                    "name": t.name(),
                    "description": t.description(),
                    "input_schema": t.input_schema(),
                })
            })
            .collect()
    }

    pub async fn call(&self, name: &str, args: Value) -> Result<Value> {
        if let Some(tool) = self.get_tool(name) {
            if !tool.is_available() {
                return Err(anyhow::anyhow!(
                    "Tool '{}' is not available in this environment",
                    name
                ));
            }
            tool.execute(args).await
        } else {
            Err(anyhow::anyhow!("Tool '{}' not found", name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockTool;
    #[async_trait]
    impl IntelligenceTool for MockTool {
        fn name(&self) -> &'static str {
            "mock_tool"
        }
        fn description(&self) -> &'static str {
            "A mock tool for testing"
        }
        fn input_schema(&self) -> Value {
            json!({ "type": "object" })
        }
        async fn execute(&self, args: Value) -> Result<Value> {
            Ok(json!({ "received": args }))
        }
    }

    #[tokio::test]
    async fn test_registry_registration_and_call() -> Result<()> {
        let mut registry = ToolRegistry::new();
        registry.register(MockTool);

        assert_eq!(registry.list_tools().len(), 1);
        assert_eq!(registry.list_tools()[0]["name"], "mock_tool");

        let args = json!({ "foo": "bar" });
        let result = registry.call("mock_tool", args.clone()).await?;
        assert_eq!(result["received"], args);

        Ok(())
    }

    #[tokio::test]
    async fn test_registry_tool_not_found() {
        let registry = ToolRegistry::new();
        let result = registry.call("non_existent", json!({})).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Tool 'non_existent' not found"
        );
    }
}
