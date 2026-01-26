#[cfg(feature = "native")]
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
#[cfg(feature = "native")]
use serde_json::json;
#[cfg(feature = "native")]
use std::io::{Read, Write};
#[cfg(feature = "native")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "native")]
use std::thread;

#[cfg(feature = "native")]
pub struct TerminalManager {
    pty_pair: Arc<Mutex<Option<portable_pty::PtyPair>>>,
    writer: Arc<Mutex<Option<Box<dyn Write + Send>>>>,
}

#[cfg(not(feature = "native"))]
pub struct TerminalManager;

impl Default for TerminalManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalManager {
    #[cfg(feature = "native")]
    pub fn new() -> Self {
        Self {
            pty_pair: Arc::new(Mutex::new(None)),
            writer: Arc::new(Mutex::new(None)),
        }
    }

    #[cfg(not(feature = "native"))]
    pub fn new() -> Self {
        Self
    }

    #[cfg(feature = "native")]
    pub fn open(
        &self,
        rows: u16,
        cols: u16,
        tx: tokio::sync::mpsc::Sender<String>,
    ) -> anyhow::Result<serde_json::Value> {
        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let shell = if cfg!(target_os = "windows") {
            "powershell.exe"
        } else {
            "bash"
        };

        let cmd = CommandBuilder::new(shell);
        let _child = pair.slave.spawn_command(cmd)?;

        let reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        let mut pty_pair_lock = self.pty_pair.lock().unwrap();
        *pty_pair_lock = Some(pair);

        let mut writer_lock = self.writer.lock().unwrap();
        *writer_lock = Some(writer);

        let tx = tx.clone();
        // Spawn reader thread
        thread::spawn(move || {
            let mut reader = reader;
            let mut buffer = [0u8; 1024];
            loop {
                match reader.read(&mut buffer) {
                    Ok(n) if n > 0 => {
                        let data = String::from_utf8_lossy(&buffer[..n]).to_string();
                        let notification = json!({
                            "jsonrpc": "2.0",
                            "method": "terminal/output",
                            "params": { "data": data }
                        });
                        let _ = tx.blocking_send(notification.to_string());
                    }
                    Ok(_) => break, // EOF
                    Err(_) => break,
                }
            }
        });

        Ok(json!("Terminal opened"))
    }

    #[cfg(not(feature = "native"))]
    pub fn open(
        &self,
        _rows: u16,
        _cols: u16,
        _tx: tokio::sync::mpsc::Sender<String>,
    ) -> anyhow::Result<serde_json::Value> {
        Err(anyhow::anyhow!("Terminal not supported on web"))
    }

    #[cfg(feature = "native")]
    pub fn write(&self, data: &str) -> anyhow::Result<serde_json::Value> {
        let mut writer_lock = self.writer.lock().unwrap();
        if let Some(writer) = writer_lock.as_mut() {
            writer.write_all(data.as_bytes())?;
            writer.flush()?;
            Ok(json!("Data written"))
        } else {
            Err(anyhow::anyhow!("Terminal not open"))
        }
    }

    #[cfg(not(feature = "native"))]
    pub fn write(&self, _data: &str) -> anyhow::Result<serde_json::Value> {
        Err(anyhow::anyhow!("Terminal not supported on web"))
    }

    #[cfg(feature = "native")]
    pub fn resize(&self, rows: u16, cols: u16) -> anyhow::Result<serde_json::Value> {
        let pty_pair_lock = self.pty_pair.lock().unwrap();
        if let Some(pair) = pty_pair_lock.as_ref() {
            pair.master.resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })?;
            Ok(json!("Terminal resized"))
        } else {
            Err(anyhow::anyhow!("Terminal not open"))
        }
    }

    #[cfg(not(feature = "native"))]
    pub fn resize(&self, _rows: u16, _cols: u16) -> anyhow::Result<serde_json::Value> {
        Err(anyhow::anyhow!("Terminal not supported on web"))
    }
}
