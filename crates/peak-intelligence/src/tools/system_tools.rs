use crate::tools::{get_system_snapshot, kill_process, list_processes, IntelligenceTool};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct ListProcessesTool;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl IntelligenceTool for ListProcessesTool {
    fn name(&self) -> &'static str {
        "list_processes"
    }
    fn description(&self) -> &'static str {
        "List all running processes with PID, CPU, and Memory usage."
    }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    async fn execute(&self, _args: Value) -> Result<Value> {
        list_processes()
    }
}

pub struct KillProcessTool;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl IntelligenceTool for KillProcessTool {
    fn name(&self) -> &'static str {
        "kill_process"
    }
    fn description(&self) -> &'static str {
        "Terminate a system process by PID."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pid": { "type": "string", "description": "Process ID to kill" }
            },
            "required": ["pid"]
        })
    }
    async fn execute(&self, args: Value) -> Result<Value> {
        let pid = args
            .get("pid")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'pid' argument"))?;
        kill_process(pid)
    }
}

pub struct SystemSnapshotTool;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl IntelligenceTool for SystemSnapshotTool {
    fn name(&self) -> &'static str {
        "get_system_snapshot"
    }
    fn description(&self) -> &'static str {
        "Get a comprehensive snapshot of system health and metrics."
    }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    async fn execute(&self, _args: Value) -> Result<Value> {
        get_system_snapshot()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_system_tools() -> Result<()> {
        // Test ListProcessesTool
        let list_tool = ListProcessesTool;
        let processes = list_tool.execute(json!({})).await?;
        assert!(processes.as_array().unwrap().len() > 0);

        // Test SystemSnapshotTool
        let snapshot_tool = SystemSnapshotTool;
        let snapshot = snapshot_tool.execute(json!({})).await?;
        assert!(snapshot.get("os_name").is_some());

        Ok(())
    }
}
