use anyhow::Result;
use serde_json::{json, Value};

#[cfg(feature = "native")]
use std::fs;
#[allow(unused_imports)]
#[cfg(feature = "native")]
use std::process::Command;
#[cfg(feature = "native")]
use sysinfo::{Pid, System};
#[cfg(feature = "native")]
use walkdir::WalkDir;

#[cfg(feature = "native")]
pub fn list_processes() -> Result<Value> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut processes: Vec<_> = sys.processes().values().collect();

    // Sort by memory usage descending
    processes.sort_by_key(|b| std::cmp::Reverse(b.memory()));

    // Take top 50 to prevent payload explosion
    let top_processes: Vec<Value> = processes
        .into_iter()
        .take(50)
        .map(|process| {
            json!({
                "pid": process.pid().to_string(),
                "name": process.name(),
                "memory": process.memory(),
                "cpu": process.cpu_usage(),
            })
        })
        .collect();

    Ok(json!(top_processes))
}

#[cfg(not(feature = "native"))]
pub fn list_processes() -> Result<Value> {
    Ok(json!([]))
}

#[cfg(feature = "native")]
pub fn kill_process(pid_str: &str) -> Result<Value> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Parse PID
    if let Ok(pid_int) = pid_str.parse::<usize>() {
        let pid = Pid::from(pid_int);

        if let Some(process) = sys.process(pid) {
            if process.kill() {
                Ok(json!({ "status": "success", "message": format!("Process {} killed", pid_str) }))
            } else {
                Ok(json!({ "status": "error", "message": "Failed to kill process (permission?)" }))
            }
        } else {
            Ok(json!({ "status": "error", "message": "Process not found" }))
        }
    } else {
        Ok(json!({ "status": "error", "message": "Invalid PID format" }))
    }
}

#[cfg(not(feature = "native"))]
pub fn kill_process(_pid_str: &str) -> Result<Value> {
    Err(anyhow::anyhow!("Process management not supported on web"))
}

pub fn read_file(path: &str) -> Result<Value> {
    #[cfg(feature = "native")]
    {
        let content = fs::read_to_string(path)?;
        Ok(json!(content))
    }
    #[cfg(not(feature = "native"))]
    {
        let _ = path;
        Err(anyhow::anyhow!("File system access not supported on web"))
    }
}

pub fn write_file(path: &str, content: &str) -> Result<Value> {
    #[cfg(feature = "native")]
    {
        fs::write(path, content)?;
        Ok(json!("Successfully wrote file"))
    }
    #[cfg(not(feature = "native"))]
    {
        let _ = (path, content);
        Err(anyhow::anyhow!("File system access not supported on web"))
    }
}

pub fn read_dir(path: &str) -> Result<Value> {
    #[cfg(feature = "native")]
    {
        let mut entries = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            entries.push(json!({
                "name": entry.file_name().to_string_lossy(),
                "is_dir": metadata.is_dir(),
                "size": metadata.len(),
                "path": entry.path().to_string_lossy()
            }));
        }

        // Sort: Directories first, then alphabetical
        entries.sort_by(|a, b| {
            let a_dir = a["is_dir"].as_bool().unwrap_or(false);
            let b_dir = b["is_dir"].as_bool().unwrap_or(false);
            if a_dir == b_dir {
                a["name"]
                    .as_str()
                    .unwrap_or("")
                    .cmp(b["name"].as_str().unwrap_or(""))
            } else {
                b_dir.cmp(&a_dir)
            }
        });

        Ok(json!(entries))
    }
    #[cfg(not(feature = "native"))]
    {
        let _ = path;
        Ok(json!([]))
    }
}

pub fn scan_wifi() -> Result<Value> {
    #[cfg(all(feature = "native", target_os = "linux"))]
    {
        let output = Command::new("nmcli")
            .args(["-t", "-f", "SSID,SIGNAL,SECURITY", "dev", "wifi", "list"])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let mut networks = Vec::new();

                for line in stdout.lines() {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() >= 2 {
                        networks.push(json!({
                            "ssid": parts[0],
                            "signal": parts[1].parse::<i32>().unwrap_or(0),
                            "security": parts.get(2).unwrap_or(&""),
                        }));
                    }
                }
                return Ok(json!(networks));
            }
        }
    }

    #[cfg(feature = "native")]
    {
        // Fallback/Mock for Dev/macOS
        let mock_networks = vec![
            json!({ "ssid": "PeakOS_Internal", "signal": 98, "security": "WPA2" }),
            json!({ "ssid": "Coffee_Shop_Free", "signal": 45, "security": "" }),
            json!({ "ssid": "Starlink_999", "signal": 72, "security": "WPA3" }),
            json!({ "ssid": "Hidden_Network", "signal": 20, "security": "WEP" }),
        ];
        Ok(json!(mock_networks))
    }
    #[cfg(not(feature = "native"))]
    {
        Ok(json!([]))
    }
}

pub fn search_files(query: &str, base_path: &str) -> Result<Value> {
    #[cfg(feature = "native")]
    {
        let mut results = Vec::new();
        let max_results = 20;
        let query_lower = query.to_lowercase();

        for entry in WalkDir::new(base_path)
            .max_depth(3) // Stay shallow for performance
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let name = entry.file_name().to_string_lossy();
            if name.to_lowercase().contains(&query_lower) {
                let metadata = entry.metadata()?;
                results.push(json!({
                    "name": name,
                    "path": entry.path().to_string_lossy(),
                    "is_dir": metadata.is_dir(),
                    "size": metadata.len()
                }));
                if results.len() >= max_results {
                    break;
                }
            }
        }

        Ok(json!(results))
    }
    #[cfg(not(feature = "native"))]
    {
        let _ = (query, base_path);
        Ok(json!([]))
    }
}

pub fn connect_wifi(ssid: &str, _password: &str) -> Result<Value> {
    #[cfg(all(feature = "native", target_os = "linux"))]
    {
        let output = Command::new("nmcli")
            .args(["dev", "wifi", "connect", ssid, "password", password])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                return Ok(
                    json!({ "status": "success", "message": format!("Connected to {}", ssid) }),
                );
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Ok(json!({ "status": "error", "message": stderr.to_string() }));
            }
        }
    }

    #[cfg(feature = "native")]
    {
        // Mock connection
        Ok(json!({ "status": "success", "message": format!("[MOCK] Connected to {}", ssid) }))
    }
    #[cfg(not(feature = "native"))]
    {
        let _ = (ssid, password);
        Err(anyhow::anyhow!("WiFi management not supported on web"))
    }
}
