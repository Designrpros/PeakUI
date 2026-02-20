use crate::reference::app::{Command, Message};

use crate::reference::intelligence::mcp;
use iced::futures::channel::mpsc::Sender;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn run_server(sender: Sender<Message>) {
    let listener = match TcpListener::bind("127.0.0.1:8081").await {
        Ok(l) => l,
        Err(e) => {
            log::error!("Failed to bind Exposure API server: {}", e);
            return;
        }
    };

    log::info!("Neural Exposure API listening on 127.0.0.1:8081");

    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                log::info!("Exposure API connection from {}", addr);
                let mut sender = sender.clone();

                tokio::spawn(async move {
                    let mut buffer = [0; 8192];
                    match socket.read(&mut buffer).await {
                        Ok(n) if n > 0 => {
                            let request = String::from_utf8_lossy(&buffer[..n]);

                            if request.starts_with("GET /schema") {
                                let schema = mcp::get_framework_schema();
                                let body = serde_json::to_string_pretty(&schema).unwrap();
                                let response = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                    body.len(),
                                    body
                                );
                                let _ = socket.write_all(response.as_bytes()).await;
                            } else if request.starts_with("GET /instructions") {
                                let instructions = r#"
# PeakUI Neural Exposure Protocol

You are interacting with the PeakUI framework via a direct network socket.

## Chat
POST /chat
Body: {"message": "Hello"}
Response: {"response": "Hi there!"}

## Navigation
Use the `SetTab` command to navigate between pages. 
Available pages: Colors, Typography, Layout, Icons, Buttons, etc.

## Interaction
- You can trigger actions by sending JSON commands.
- Commands are mapped 1:1 to the application's `Command` enum.
- Example: `{"SetTab": "Icons"}`

## Neural Sudo
Certain system-level or destructive actions are "Protected". 
If you attempt a protected action, the user will see a "Neural Sudo" prompt to approve it. 
Always provide a clear reason when requested.

## Search
Use the `Search` command to filter components or search for documentation within the app.
"#;
                                let response = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                    instructions.len(),
                                    instructions
                                );
                                let _ = socket.write_all(response.as_bytes()).await;
                            } else if request.starts_with("GET /view") {
                                let view_json = std::fs::read_to_string(".peak/current_view.json")
                                    .unwrap_or_else(|_| {
                                        "{\"error\": \"View not exported yet\"}".to_string()
                                    });
                                let response = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                    view_json.len(),
                                    view_json
                                );
                                let _ = socket.write_all(response.as_bytes()).await;
                            } else if request.starts_with("POST /chat") {
                                if let Some(body_start) = request.find("\r\n\r\n") {
                                    let json_body = &request[body_start + 4..];

                                    let message = if let Ok(json) =
                                        serde_json::from_str::<serde_json::Value>(json_body.trim())
                                    {
                                        json.get("message")
                                            .and_then(|s| s.as_str())
                                            .map(|s| s.to_string())
                                    } else {
                                        None
                                    };

                                    if let Some(msg) = message {
                                        // 1. Send user message to UI for visibility
                                        let _ = sender.try_send(Message::Intelligence(
                                            crate::reference::app::IntelligenceMessage::Chat(
                                                crate::views::chat::ChatViewMessage::InputChanged(
                                                    msg.clone(),
                                                ),
                                            ),
                                        ));

                                        // 2. Call Bridge Directly if enabled
                                        if let Some(bridge) =
                                            crate::reference::intelligence::bridge::GLOBAL_BRIDGE
                                                .get()
                                        {
                                            let chat_msg = crate::core::ChatCompletionMessage {
                                                role: "user".to_string(),
                                                content: msg,
                                            };

                                            let response_text =
                                                match bridge.chat_direct(vec![chat_msg]).await {
                                                    Ok(res) => res,
                                                    Err(e) => format!("Error: {}", e),
                                                };

                                            // 3. Parse and Execute Actions
                                            let actions = crate::reference::intelligence::ActionParser::parse_text(&response_text);
                                            for action in actions {
                                                let command = match action {
                                                    crate::reference::intelligence::Action::Navigate(page) => Some(Command::SetTab(page)),
                                                    crate::reference::intelligence::Action::SetThemeTone(tone) => Some(Command::SetTheme(tone)),
                                                    crate::reference::intelligence::Action::SetThemeKind(kind) => Some(Command::SetThemeKind(kind)),
                                                    crate::reference::intelligence::Action::SetLabMode(mode) => Some(Command::SetRenderMode(mode)),
                                                    crate::reference::intelligence::Action::SetButtonVariant(v) => Some(Command::UpdateButtonVariant(v)),
                                                    crate::reference::intelligence::Action::SetButtonIntent(i) => Some(Command::UpdateButtonIntent(i)),
                                                    _ => None,
                                                };

                                                if let Some(cmd) = command {
                                                    let _ = sender.try_send(cmd.into_message());
                                                }
                                            }

                                            let response_json = serde_json::json!({
                                                "response": response_text
                                            });
                                            let body = serde_json::to_string_pretty(&response_json)
                                                .unwrap();

                                            let response = format!(
                                                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                                body.len(),
                                                body
                                            );
                                            let _ = socket.write_all(response.as_bytes()).await;
                                        } else {
                                            let response = "HTTP/1.1 503 Service Unavailable\r\nConnection: close\r\n\r\nIntelligence Bridge not initialized";
                                            let _ = socket.write_all(response.as_bytes()).await;
                                        }
                                    } else {
                                        let response = "HTTP/1.1 400 Bad Request\r\nConnection: close\r\n\r\nMissing 'message' field";
                                        let _ = socket.write_all(response.as_bytes()).await;
                                    }
                                }
                            } else if request.starts_with("POST /command") {
                                // Find end of headers
                                if let Some(body_start) = request.find("\r\n\r\n") {
                                    let json_body = &request[body_start + 4..];
                                    if let Ok(cmd) =
                                        serde_json::from_str::<Command>(json_body.trim())
                                    {
                                        let _ = sender.try_send(cmd.into_message());
                                        let response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nOK";
                                        let _ = socket.write_all(response.as_bytes()).await;
                                    } else {
                                        log::warn!(
                                            "Failed to parse Exposure API command: {}",
                                            json_body
                                        );
                                        let response = "HTTP/1.1 400 Bad Request\r\nConnection: close\r\n\r\nInvalid Command JSON";
                                        let _ = socket.write_all(response.as_bytes()).await;
                                    }
                                } else {
                                    let response = "HTTP/1.1 400 Bad Request\r\nConnection: close\r\n\r\nMissing Body";
                                    let _ = socket.write_all(response.as_bytes()).await;
                                }
                            } else {
                                let response =
                                    "HTTP/1.1 404 Not Found\r\nConnection: close\r\n\r\n";
                                let _ = socket.write_all(response.as_bytes()).await;
                            }
                        }
                        _ => {}
                    }
                    let _ = socket.shutdown().await;
                });
            }
            Err(e) => {
                log::error!("Exposure API accept error: {}", e);
            }
        }
    }
}
