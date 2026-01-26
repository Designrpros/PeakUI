use serde::Serialize;
use std::fs;

#[derive(Serialize, Clone, Debug)]
pub struct SystemTelemetry {
    pub cpu_temp: f32,     // Degrees Celsius
    pub battery_level: u8, // 0-100
    pub memory_used: u64,  // in MB
    pub memory_total: u64, // in MB
    pub uptime: u64,       // Seconds
    pub load_avg: f32,     // 1-minute load
    pub is_charging: bool,
}

impl SystemTelemetry {
    /// The "Deep Core" Read - Direct fs access for max speed/min overhead
    pub fn snapshot() -> Self {
        SystemTelemetry {
            cpu_temp: read_cpu_temp(),
            battery_level: read_battery_capacity(),
            memory_used: read_mem_used(),
            memory_total: read_mem_total(), // Could cache this
            uptime: read_uptime(),
            load_avg: read_load_avg(),
            is_charging: read_charging_status(),
        }
    }
}

// --- /sys & /proc Parsers ---

fn read_cpu_temp() -> f32 {
    // Try standard thermal zone. On PinePhone/Arch, this is usually correct.
    // Fallback paths can be added.
    fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
        .ok()
        .and_then(|s| s.trim().parse::<f32>().ok())
        .map(|t| t / 1000.0)
        .unwrap_or(0.0)
}

fn read_battery_capacity() -> u8 {
    fs::read_to_string("/sys/class/power_supply/BAT0/capacity") // Standard Linux
        .or_else(|_| fs::read_to_string("/sys/class/power_supply/axp20x-battery/capacity")) // PinePhone specific
        .ok()
        .and_then(|s| s.trim().parse::<u8>().ok())
        .unwrap_or(100) // Default to 100 if no battery (Desktop)
}

fn read_charging_status() -> bool {
    let status = fs::read_to_string("/sys/class/power_supply/BAT0/status")
        .or_else(|_| fs::read_to_string("/sys/class/power_supply/axp20x-battery/status"))
        .unwrap_or_default();
    status.trim() == "Charging"
}

fn read_mem_total() -> u64 {
    // Simple parser for /proc/meminfo
    // format: MemTotal:       16306540 kB
    if let Ok(contents) = fs::read_to_string("/proc/meminfo") {
        for line in contents.lines() {
            if line.starts_with("MemTotal:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return parts[1].parse::<u64>().unwrap_or(0) / 1024; // KB -> MB
                }
            }
        }
    }
    0
}

fn read_mem_used() -> u64 {
    // Calculated as Total - Available
    let mut total = 0;
    let mut available = 0;

    if let Ok(contents) = fs::read_to_string("/proc/meminfo") {
        for line in contents.lines() {
            if line.starts_with("MemTotal:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    total = parts[1].parse::<u64>().unwrap_or(0);
                }
            }
            if line.starts_with("MemAvailable:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    available = parts[1].parse::<u64>().unwrap_or(0);
                }
            }
        }
    }
    (total - available) / 1024 // KB -> MB
}

fn read_uptime() -> u64 {
    fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|s| s.split_whitespace().next().map(|v| v.parse::<f32>().ok()))
        .flatten()
        .map(|v| v as u64)
        .unwrap_or(0)
}

fn read_load_avg() -> f32 {
    fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|s| s.split_whitespace().next().map(|v| v.parse::<f32>().ok()))
        .flatten()
        .unwrap_or(0.0)
}
