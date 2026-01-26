mod mcp;
mod terminal;
mod tools;
mod voice;

#[cfg(feature = "voice")]
use crate::voice::VoiceManager;
#[cfg(feature = "voice")]
use voice::VOICE;

use crate::mcp::{
    CallToolParams, CallToolResult, JsonRpcRequest, JsonRpcResponse, ListToolsResult, Tool,
    ToolContent,
};
use crate::terminal::TerminalManager;
use once_cell::sync::Lazy;
use peak_intelligence::kernel;
use serde_json::json;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

static TERMINAL: Lazy<TerminalManager> = Lazy::new(TerminalManager::new);
// No longer here

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

    // We need to pass the tx to the terminal manager somehow, or tell it how to notify.
    // Actually, TerminalManager's thread can use a global channel or similar.
    // For now, let's just make it simple.

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

async fn handle_request(req: JsonRpcRequest, tx: mpsc::Sender<String>) -> JsonRpcResponse {
    match req.method.as_str() {
        "tools/list" => {
            let tools = vec![
                Tool {
                    name: "list_processes".into(),
                    description: "List all running processes with PID, CPU, and Memory usage."
                        .into(),
                    input_schema: json!({ "type": "object", "properties": {} }),
                },
                Tool {
                    name: "terminal_open".into(),
                    description: "Open a new terminal PTY session.".into(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "rows": { "type": "number", "default": 24 },
                            "cols": { "type": "number", "default": 80 }
                        }
                    }),
                },
                Tool {
                    name: "terminal_write".into(),
                    description: "Write data to the active terminal.".into(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "data": { "type": "string" }
                        },
                        "required": ["data"]
                    }),
                },
                Tool {
                    name: "terminal_resize".into(),
                    description: "Resize the active terminal.".into(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "rows": { "type": "number" },
                            "cols": { "type": "number" }
                        },
                        "required": ["rows", "cols"]
                    }),
                },
                // ... other tools ...
                Tool {
                    name: "read_file".into(),
                    description: "Read content of a file from the system.".into(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "path": { "type": "string", "description": "Absolute path to file" }
                        },
                        "required": ["path"]
                    }),
                },
                Tool {
                    name: "write_file".into(),
                    description: "Write content to a file.".into(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "path": { "type": "string", "description": "Absolute path to file" },
                            "content": { "type": "string", "description": "Content to write" }
                        },
                        "required": ["path", "content"]
                    }),
                },
                Tool {
                    name: "kill_process".into(),
                    description: "Terminate a system process by PID.".into(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "pid": { "type": "string", "description": "Process ID to kill" }
                        },
                        "required": ["pid"]
                    }),
                },
                Tool {
                    name: "read_dir".into(),
                    description: "List files and directories in a path.".into(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "path": { "type": "string", "description": "Directory path" }
                        },
                        "required": ["path"]
                    }),
                },
                Tool {
                    name: "scan_wifi".into(),
                    description: "Scan for available WiFi networks.".into(),
                    input_schema: json!({ "type": "object", "properties": {} }),
                },
                Tool {
                    name: "connect_wifi".into(),
                    description: "Connect to a WiFi network.".into(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "ssid": { "type": "string", "description": "SSID of the network" },
                            "password": { "type": "string", "description": "WiFi password" }
                        },
                        "required": ["ssid", "password"]
                    }),
                },
                Tool {
                    name: "search_files".into(),
                    description: "Search for files and directories by name.".into(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "query": { "type": "string", "description": "Search term" },
                            "base_path": { "type": "string", "description": "Path to search from" }
                        },
                        "required": ["query", "base_path"]
                    }),
                },
                Tool {
                    name: "intelligence/stt".into(),
                    description: "Convert PCM audio data (f32, 16kHz) to text using Whisper."
                        .into(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "audio": { "type": "array", "items": { "type": "number" } }
                        },
                        "required": ["audio"]
                    }),
                },
                Tool {
                    name: "intelligence/tts".into(),
                    description: "Convert text to speech samples (f32, 22kHz) using Piper.".into(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "text": { "type": "string" },
                            "voice": { "type": "string", "default": "en_US-lessac-medium" }
                        },
                        "required": ["text"]
                    }),
                },
            ];

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
                        let result = match p.name.as_str() {
                            "terminal_open" => {
                                let rows =
                                    p.arguments
                                        .as_ref()
                                        .and_then(|a| a.get("rows"))
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(24) as u16;
                                let cols =
                                    p.arguments
                                        .as_ref()
                                        .and_then(|a| a.get("cols"))
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(80) as u16;
                                TERMINAL.open(rows, cols, tx)
                            }
                            "terminal_write" => {
                                let data = p
                                    .arguments
                                    .as_ref()
                                    .and_then(|a| a.get("data"))
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("");
                                TERMINAL.write(data)
                            }
                            "terminal_resize" => {
                                let rows =
                                    p.arguments
                                        .as_ref()
                                        .and_then(|a| a.get("rows"))
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(24) as u16;
                                let cols =
                                    p.arguments
                                        .as_ref()
                                        .and_then(|a| a.get("cols"))
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(80) as u16;
                                TERMINAL.resize(rows, cols)
                            }
                            "list_processes" => tools::list_processes(),
                            "read_file" => {
                                if let Some(args) = p.arguments {
                                    if let Some(path) = args.get("path").and_then(|v| v.as_str()) {
                                        tools::read_file(path)
                                    } else {
                                        return JsonRpcResponse::error(
                                            req.id,
                                            -32602,
                                            "Missing 'path' argument".into(),
                                        );
                                    }
                                } else {
                                    return JsonRpcResponse::error(
                                        req.id,
                                        -32602,
                                        "Missing arguments".into(),
                                    );
                                }
                            }
                            "write_file" => {
                                if let Some(args) = p.arguments {
                                    let path = args.get("path").and_then(|v| v.as_str());
                                    let content = args.get("content").and_then(|v| v.as_str());
                                    if let (Some(path), Some(content)) = (path, content) {
                                        tools::write_file(path, content)
                                    } else {
                                        return JsonRpcResponse::error(
                                            req.id,
                                            -32602,
                                            "Missing 'path' or 'content' arguments".into(),
                                        );
                                    }
                                } else {
                                    return JsonRpcResponse::error(
                                        req.id,
                                        -32602,
                                        "Missing arguments".into(),
                                    );
                                }
                            }
                            "kill_process" => {
                                if let Some(args) = p.arguments {
                                    if let Some(pid) = args.get("pid").and_then(|v| v.as_str()) {
                                        tools::kill_process(pid)
                                    } else {
                                        return JsonRpcResponse::error(
                                            req.id,
                                            -32602,
                                            "Missing 'pid' argument".into(),
                                        );
                                    }
                                } else {
                                    return JsonRpcResponse::error(
                                        req.id,
                                        -32602,
                                        "Missing arguments".into(),
                                    );
                                }
                            }
                            "read_dir" => {
                                if let Some(args) = p.arguments {
                                    if let Some(path) = args.get("path").and_then(|v| v.as_str()) {
                                        tools::read_dir(path)
                                    } else {
                                        return JsonRpcResponse::error(
                                            req.id,
                                            -32602,
                                            "Missing 'path' argument".into(),
                                        );
                                    }
                                } else {
                                    return JsonRpcResponse::error(
                                        req.id,
                                        -32602,
                                        "Missing arguments".into(),
                                    );
                                }
                            }
                            "scan_wifi" => tools::scan_wifi(),
                            "connect_wifi" => {
                                if let Some(args) = p.arguments {
                                    let ssid = args.get("ssid").and_then(|v| v.as_str());
                                    let password = args.get("password").and_then(|v| v.as_str());
                                    if let (Some(ssid), Some(password)) = (ssid, password) {
                                        tools::connect_wifi(ssid, password)
                                    } else {
                                        return JsonRpcResponse::error(
                                            req.id,
                                            -32602,
                                            "Missing 'ssid' or 'password' arguments".into(),
                                        );
                                    }
                                } else {
                                    return JsonRpcResponse::error(
                                        req.id,
                                        -32602,
                                        "Missing arguments".into(),
                                    );
                                }
                            }
                            "search_files" => {
                                if let Some(args) = p.arguments {
                                    let query = args.get("query").and_then(|v| v.as_str());
                                    let base_path = args.get("base_path").and_then(|v| v.as_str());
                                    if let (Some(query), Some(base_path)) = (query, base_path) {
                                        tools::search_files(query, base_path)
                                    } else {
                                        return JsonRpcResponse::error(
                                            req.id,
                                            -32602,
                                            "Missing 'query' or 'base_path' arguments".into(),
                                        );
                                    }
                                } else {
                                    return JsonRpcResponse::error(
                                        req.id,
                                        -32602,
                                        "Missing arguments".into(),
                                    );
                                }
                            }
                            #[cfg(feature = "voice")]
                            "intelligence/stt" => {
                                let args = p
                                    .arguments
                                    .ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;
                                let audio_val = args
                                    .get("audio")
                                    .ok_or_else(|| anyhow::anyhow!("Missing 'audio' argument"))?;
                                let audio_arr = audio_val
                                    .as_array()
                                    .ok_or_else(|| anyhow::anyhow!("'audio' must be an array"))?;

                                let samples: Vec<f32> = audio_arr
                                    .iter()
                                    .filter_map(|v| v.as_f64().map(|f| f as f32))
                                    .collect();

                                let mut manager = VOICE.lock().await;
                                manager.init_whisper("tiny.en").await?;
                                manager.transcribe(&samples).await.map(|s| json!(s))
                            }
                            #[cfg(feature = "voice")]
                            "intelligence/tts" => {
                                let args = p
                                    .arguments
                                    .ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;
                                let text = args.get("text").and_then(|v| v.as_str()).unwrap_or("");
                                let voice = args
                                    .get("voice")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("en_US-lessac-medium");

                                let mut manager = VOICE.lock().await;
                                manager
                                    .synthesize(text, voice)
                                    .await
                                    .map(|samples| json!({ "samples": samples }))
                            }
                            _ => {
                                return JsonRpcResponse::error(
                                    req.id,
                                    -32601,
                                    format!("Tool not found: {}", p.name),
                                );
                            }
                        };

                        match result {
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
