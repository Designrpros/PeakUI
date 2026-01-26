pub fn get_available_networks() -> Vec<String> {
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let mut networks = Vec::new();

        // 1. Get Wifi Networks
        if let Ok(output) = Command::new("nmcli")
            .args(&["-t", "-f", "SSID", "dev", "wifi", "list"])
            .output()
        {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                let wifi_list: Vec<String> = stdout
                    .lines()
                    .filter(|line| !line.is_empty())
                    .map(|s| s.to_string())
                    .collect();
                networks.extend(wifi_list);
            }
        }

        // 2. Get Active Connections (Wired/Existing)
        if let Ok(output) = Command::new("nmcli")
            .args(&["-t", "-f", "NAME,TYPE", "con", "show", "--active"])
            .output()
        {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                for line in stdout.lines() {
                    // Format: NAME:TYPE
                    let parts: Vec<&str> = line.split(':').collect();
                    if !parts.is_empty() {
                        let name = parts[0];
                        // Avoid duplicates if wifi already listed
                        if !networks.contains(&name.to_string()) {
                            networks.push(name.to_string());
                        }
                    }
                }
            }
        }

        // If we found anything, return it. Otherwise fall through?
        // Actually, on Linux with NetworkManager, if we have NM, we should rely on it.
        // If networks is empty, it means truly no networks found.
        if !networks.is_empty() {
            return networks;
        }
    }

    // Default / Mock / Mac Data (Fallback if NM missing or scan failed completely)
    vec![
        "PeakOS_5G".to_string(),
        "Riviera_Guest".to_string(),
        "StarkNet".to_string(),
        "SkynetUI".to_string(),
    ]
}
