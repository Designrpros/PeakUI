use crate::terminal::TerminalManager;
use crate::tools::IntelligenceTool;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::sync::mpsc;

pub struct TerminalOpenTool {
    pub manager: &'static TerminalManager,
    pub tx: mpsc::Sender<String>,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl IntelligenceTool for TerminalOpenTool {
    fn name(&self) -> &'static str {
        "terminal_open"
    }
    fn description(&self) -> &'static str {
        "Open a new terminal PTY session."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "rows": { "type": "number", "default": 24 },
                "cols": { "type": "number", "default": 80 }
            }
        })
    }
    async fn execute(&self, args: Value) -> Result<Value> {
        let rows = args.get("rows").and_then(|v| v.as_u64()).unwrap_or(24) as u16;
        let cols = args.get("cols").and_then(|v| v.as_u64()).unwrap_or(80) as u16;
        self.manager.open(rows, cols, self.tx.clone())
    }
}

pub struct TerminalWriteTool {
    pub manager: &'static TerminalManager,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl IntelligenceTool for TerminalWriteTool {
    fn name(&self) -> &'static str {
        "terminal_write"
    }
    fn description(&self) -> &'static str {
        "Write data to the active terminal."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "data": { "type": "string" }
            },
            "required": ["data"]
        })
    }
    async fn execute(&self, args: Value) -> Result<Value> {
        let data = args.get("data").and_then(|v| v.as_str()).unwrap_or("");
        self.manager.write(data)
    }
}

pub struct TerminalResizeTool {
    pub manager: &'static TerminalManager,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl IntelligenceTool for TerminalResizeTool {
    fn name(&self) -> &'static str {
        "terminal_resize"
    }
    fn description(&self) -> &'static str {
        "Resize the active terminal."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "rows": { "type": "number" },
                "cols": { "type": "number" }
            },
            "required": ["rows", "cols"]
        })
    }
    async fn execute(&self, args: Value) -> Result<Value> {
        let rows = args
            .get("rows")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'rows'"))? as u16;
        let cols = args
            .get("cols")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'cols'"))? as u16;
        self.manager.resize(rows, cols)
    }
}
