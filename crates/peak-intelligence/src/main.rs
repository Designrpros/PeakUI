#[cfg(not(target_arch = "wasm32"))]
use anyhow::Result;

#[cfg(not(target_arch = "wasm32"))]
mod mcp;
#[cfg(not(target_arch = "wasm32"))]
mod terminal;
#[cfg(not(target_arch = "wasm32"))]
mod tools;
#[cfg(not(target_arch = "wasm32"))]
mod voice;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "voice")]
use crate::voice::VoiceManager;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "voice")]
use voice::VOICE;

#[cfg(not(target_arch = "wasm32"))]
use mcp::{
    CallToolParams, CallToolResult, JsonRpcRequest, JsonRpcResponse, ListToolsResult, Tool,
    ToolContent,
};
#[cfg(not(target_arch = "wasm32"))]
use once_cell::sync::Lazy;
#[cfg(not(target_arch = "wasm32"))]
use peak_intelligence::kernel;
#[cfg(not(target_arch = "wasm32"))]
use serde_json::json;
#[cfg(not(target_arch = "wasm32"))]
use terminal::TerminalManager;
#[cfg(not(target_arch = "wasm32"))]
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::mpsc;
#[cfg(not(target_arch = "wasm32"))]
use tokio::time::{sleep, Duration};

#[cfg(not(target_arch = "wasm32"))]
use tools::fs_tools::{ReadDirTool, ReadFileTool, SearchFilesTool, WriteFileTool};
#[cfg(not(target_arch = "wasm32"))]
use tools::registry::ToolRegistry;
#[cfg(not(target_arch = "wasm32"))]
use tools::search_tool::WebSearchTool;
#[cfg(not(target_arch = "wasm32"))]
use tools::system_tools::{KillProcessTool, ListProcessesTool, SystemSnapshotTool};
#[cfg(not(target_arch = "wasm32"))]
use tools::terminal_tools::{TerminalOpenTool, TerminalResizeTool, TerminalWriteTool};

#[cfg(not(target_arch = "wasm32"))]
static TERMINAL: Lazy<TerminalManager> = Lazy::new(TerminalManager::new);
#[cfg(not(target_arch = "wasm32"))]
static TOOL_REGISTRY: Lazy<tokio::sync::RwLock<ToolRegistry>> =
    Lazy::new(|| tokio::sync::RwLock::new(ToolRegistry::new()));

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Verify Icebreaker Core Linkage
    let lib = peak_intelligence::brain::model::Library::default();
    println!("Peak Intelligence initialized.");
    println!("🤖 AI Model Directory: {:?}", lib.directory());

    // Setup Stdio
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();
    let _stdout = io::stdout();

    let (tx, mut rx) = mpsc::channel::<String>(100);

    // Initialize Tool Registry
    {
        let mut registry = TOOL_REGISTRY.write().await;

        // FS Tools
        registry.register(ReadFileTool);
        registry.register(WriteFileTool);
        registry.register(ReadDirTool);
        registry.register(SearchFilesTool);

        // System Tools
        registry.register(ListProcessesTool);
        registry.register(KillProcessTool);
        registry.register(SystemSnapshotTool);

        // Search Tool
        registry.register(WebSearchTool);

        // Terminal Tools
        registry.register(TerminalOpenTool {
            manager: &TERMINAL,
            tx: tx.clone(),
        });
        registry.register(TerminalWriteTool { manager: &TERMINAL });
        registry.register(TerminalResizeTool { manager: &TERMINAL });

        // Voice Tools
        #[cfg(feature = "voice")]
        {
            use crate::tools::voice_tools::{SttTool, TtsTool};
            registry.register(SttTool);
            registry.register(TtsTool);
        }
    }

    // Stdout writer task
    let _stdout_tx = tx.clone();
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let _ = io::stdout()
                .write_all(format!("{}\n", msg).as_bytes())
                .await;
            let _ = io::stdout().flush().await;
        }
    });

    // --- DEEP CORE TELEMETRY LOOP ---
    let telemetry_tx = tx.clone();
    tokio::spawn(async move {
        loop {
            // "The Deep Core Pulse" - 2 seconds
            sleep(Duration::from_secs(2)).await;

            let snapshot = kernel::SystemTelemetry::snapshot();
            let notification = json!({
                "jsonrpc": "2.0",
                "method": "system/telemetry",
                "params": snapshot
            });

            if let Ok(msg) = serde_json::to_string(&notification) {
                let _ = telemetry_tx.send(msg).await;
            }
        }
    });

    // Main Loop
    while let Some(line) = reader.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }

        let request: Result<JsonRpcRequest, _> = serde_json::from_str(&line);

        match request {
            Ok(req) => {
                let tx_clone = tx.clone();
                tokio::spawn(async move {
                    let response = handle_request(req, tx_clone.clone()).await;
                    if let Ok(response_str) = serde_json::to_string(&response) {
                        let _ = tx_clone.send(response_str).await;
                    }
                });
            }
            Err(_e) => {
                let err_res = JsonRpcResponse::error(None, -32700, "Parse error".into());
                if let Ok(err_str) = serde_json::to_string(&err_res) {
                    let _ = tx.send(err_str).await;
                }
            }
        }
    }

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
async fn handle_request(req: JsonRpcRequest, _tx: mpsc::Sender<String>) -> JsonRpcResponse {
    match req.method.as_str() {
        "tools/list" => {
            let registry = TOOL_REGISTRY.read().await;
            let tools_raw = registry.list_tools();

            // Map to MCP Tool format
            let tools: Vec<Tool> = tools_raw
                .into_iter()
                .map(|t| Tool {
                    name: t["name"].as_str().unwrap_or_default().into(),
                    description: t["description"].as_str().unwrap_or_default().into(),
                    input_schema: t["input_schema"].clone(),
                })
                .collect();

            JsonRpcResponse::success(
                req.id,
                serde_json::to_value(ListToolsResult { tools }).unwrap(),
            )
        }
        "tools/call" => {
            if let Some(params) = req.params {
                let call_params: Result<CallToolParams, _> = serde_json::from_value(params);
                match call_params {
                    Ok(p) => {
                        let registry = TOOL_REGISTRY.read().await;
                        let args = p.arguments.unwrap_or(json!({}));

                        match registry.call(&p.name, args).await {
                            Ok(tool_res) => {
                                let content = vec![ToolContent {
                                    r#type: "text".into(),
                                    text: tool_res.to_string(),
                                }];
                                JsonRpcResponse::success(
                                    req.id,
                                    serde_json::to_value(CallToolResult {
                                        content,
                                        is_error: Some(false),
                                    })
                                    .unwrap(),
                                )
                            }
                            Err(e) => {
                                let content = vec![ToolContent {
                                    r#type: "text".into(),
                                    text: format!("Error: {}", e),
                                }];
                                JsonRpcResponse::success(
                                    req.id,
                                    serde_json::to_value(CallToolResult {
                                        content,
                                        is_error: Some(true),
                                    })
                                    .unwrap(),
                                )
                            }
                        }
                    }
                    Err(_) => JsonRpcResponse::error(req.id, -32602, "Invalid params".into()),
                }
            } else {
                JsonRpcResponse::error(req.id, -32602, "Missing params".into())
            }
        }
        _ => JsonRpcResponse::error(req.id, -32601, "Method not found".into()),
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {}
