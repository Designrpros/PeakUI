use crate::app_traits::{PeakApp, ShellContext};
use crate::theme::Theme;
use iced::{Element, Task};
#[cfg(feature = "native")]
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};

#[cfg(feature = "native")]
use std::io::{Read, Write};

#[cfg(feature = "native")]
use std::sync::{Arc, Mutex};

#[cfg(feature = "native")]
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum TerminalMessage {
    OutputReceived(String),
    InputChanged(String),
    InputSubmitted,
    RunCommand(String),
}

pub struct TerminalApp {
    pub content: String,
    pub input_buffer: String,
    #[cfg(feature = "native")]
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    #[cfg(feature = "native")]
    receiver: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<String>>>,
    pub is_open: bool,
}

impl Default for TerminalApp {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalApp {
    pub fn new() -> Self {
        #[cfg(feature = "native")]
        {
            let pty_system = NativePtySystem::default();
            let pair = pty_system
                .openpty(PtySize {
                    rows: 24,
                    cols: 80,
                    pixel_width: 0,
                    pixel_height: 0,
                })
                .expect("Failed to create PTY");

            let shell = if std::path::Path::new("/bin/bash").exists() {
                "bash"
            } else {
                "sh"
            };
            let cmd = CommandBuilder::new(shell);
            let _child = pair
                .slave
                .spawn_command(cmd)
                .expect("Failed to spawn shell");

            let mut reader = pair.master.try_clone_reader().unwrap();
            let writer = pair.master.take_writer().unwrap();

            let (tx, rx) = mpsc::unbounded_channel();

            std::thread::spawn(move || {
                let mut buf = [0u8; 1024];
                loop {
                    match reader.read(&mut buf) {
                        Ok(n) if n > 0 => {
                            let text = String::from_utf8_lossy(&buf[..n]).to_string();
                            if tx.send(text).is_err() {
                                break;
                            }
                        }
                        _ => break,
                    }
                }
            });

            Self {
                content: String::from("PeakOS Terminal v0.1\n> "),
                input_buffer: String::new(),
                writer: Arc::new(Mutex::new(writer)),
                receiver: Arc::new(tokio::sync::Mutex::new(rx)),
                is_open: false,
            }
        }

        #[cfg(target_arch = "wasm32")]
        Self {
            content: String::from("Terminal is not supported on WASM"),
            input_buffer: String::new(),
            is_open: false,
        }
    }
}

impl PeakApp for TerminalApp {
    type Message = TerminalMessage;

    fn title(&self) -> String {
        String::from("Terminal")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _context: &dyn ShellContext,
    ) -> Task<Self::Message> {
        match message {
            TerminalMessage::OutputReceived(text) => {
                let cleaned = strip_ansi(&text);
                self.content.push_str(&cleaned);

                if self.content.len() > 10000 {
                    let to_remove = self.content.len() - 10000;
                    self.content.drain(..to_remove);
                }
            }
            TerminalMessage::InputChanged(val) => {
                self.input_buffer = val;
            }
            TerminalMessage::InputSubmitted => {
                #[cfg(feature = "native")]
                {
                    let cmd = format!("{}\n", self.input_buffer);
                    if let Ok(mut writer) = self.writer.lock() {
                        let _ = write!(writer, "{}", cmd);
                    }
                }
                self.input_buffer.clear();
            }
            TerminalMessage::RunCommand(cmd) => {
                // Execute external command by writing to PTY
                #[cfg(feature = "native")]
                {
                    if let Ok(mut writer) = self.writer.lock() {
                        let _ = writeln!(writer, "{}", cmd);
                    }
                }
                #[cfg(target_arch = "wasm32")]
                let _ = cmd;
            }
        }
        Task::none()
    }

    fn view(&self, _theme: &Theme) -> Element<'_, Self::Message> {
        // Implementation will be handled by peak-desktop for now
        // until we move all view logic to core
        iced::widget::text("Terminal View (Stub)").into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        #[cfg(feature = "native")]
        {
            iced::Subscription::run_with_id(
                "terminal_listener",
                iced::futures::stream::unfold(self.receiver.clone(), |receiver| async move {
                    let mut rx = receiver.lock().await;
                    rx.recv()
                        .await
                        .map(|text| (TerminalMessage::OutputReceived(text), receiver.clone()))
                }),
            )
        }

        #[cfg(target_arch = "wasm32")]
        iced::Subscription::none()
    }
}

pub fn strip_ansi(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut in_escape = false;
    let mut in_csi = false;

    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if in_escape {
            if b == b'[' {
                in_csi = true;
            }
            in_escape = false;
        } else if in_csi {
            if (0x40..=0x7E).contains(&b) {
                in_csi = false;
            }
        } else if b == 0x1B {
            in_escape = true;
        } else {
            result.push(b as char);
        }
        i += 1;
    }
    result
}
