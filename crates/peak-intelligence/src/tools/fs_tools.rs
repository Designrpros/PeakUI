use crate::tools::{read_dir, read_file, search_files, write_file, IntelligenceTool};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct ReadFileTool;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl IntelligenceTool for ReadFileTool {
    fn name(&self) -> &'static str {
        "read_file"
    }
    fn description(&self) -> &'static str {
        "Read content of a file from the system."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Absolute path to file" }
            },
            "required": ["path"]
        })
    }
    async fn execute(&self, args: Value) -> Result<Value> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'path' argument"))?;
        read_file(path)
    }
}

pub struct WriteFileTool;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl IntelligenceTool for WriteFileTool {
    fn name(&self) -> &'static str {
        "write_file"
    }
    fn description(&self) -> &'static str {
        "Write content to a file."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Absolute path to file" },
                "content": { "type": "string", "description": "Content to write" }
            },
            "required": ["path", "content"]
        })
    }
    async fn execute(&self, args: Value) -> Result<Value> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'path' argument"))?;
        let content = args
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'content' argument"))?;
        write_file(path, content)
    }
}

pub struct ReadDirTool;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl IntelligenceTool for ReadDirTool {
    fn name(&self) -> &'static str {
        "read_dir"
    }
    fn description(&self) -> &'static str {
        "List files and directories in a path."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Directory path" }
            },
            "required": ["path"]
        })
    }
    async fn execute(&self, args: Value) -> Result<Value> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'path' argument"))?;
        read_dir(path)
    }
}

pub struct SearchFilesTool;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl IntelligenceTool for SearchFilesTool {
    fn name(&self) -> &'static str {
        "search_files"
    }
    fn description(&self) -> &'static str {
        "Search for files and directories by name."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": "Search term" },
                "base_path": { "type": "string", "description": "Path to search from" }
            },
            "required": ["query", "base_path"]
        })
    }
    async fn execute(&self, args: Value) -> Result<Value> {
        let query = args
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'query' argument"))?;
        let base_path = args
            .get("base_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'base_path' argument"))?;
        search_files(query, base_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_fs_tools() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test.txt");
        let file_path_str = file_path.to_str().unwrap();

        // Test WriteFileTool
        let write_tool = WriteFileTool;
        write_tool
            .execute(json!({
                "path": file_path_str,
                "content": "hello world"
            }))
            .await?;

        // Test ReadFileTool
        let read_tool = ReadFileTool;
        let content = read_tool
            .execute(json!({
                "path": file_path_str
            }))
            .await?;
        assert_eq!(content.as_str().unwrap(), "hello world");

        // Test ReadDirTool
        let read_dir_tool = ReadDirTool;
        let entries = read_dir_tool
            .execute(json!({
                "path": dir.path().to_str().unwrap()
            }))
            .await?;
        assert!(entries.as_array().unwrap().iter().any(|e| e
            .as_str()
            .map(|s| s.contains("test.txt"))
            .unwrap_or(false)
            || e.get("name")
                .and_then(|v| v.as_str())
                .map(|s| s.contains("test.txt"))
                .unwrap_or(false)));

        // Test SearchFilesTool
        let search_tool = SearchFilesTool;
        let results = search_tool
            .execute(json!({
                "query": "test",
                "base_path": dir.path().to_str().unwrap()
            }))
            .await?;
        assert!(results.as_array().unwrap().len() > 0);

        Ok(())
    }
}
