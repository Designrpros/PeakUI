use crate::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn benchmark_green_ai_reduction() {
        let ctx = Context::default();

        // 1. Build a complex realistic view using the Macro DSL pattern
        // We help inference by using turbofish on the first element
        let view = vstack![
            text::<IcedBackend>("PeakOS Industrial Interface")
                .large_title()
                .bold(),
            hstack![
                glass_card(vstack![
                    text("System Pressure").secondary(),
                    text("84.2 PSI").title2().intent(Intent::Success),
                ]),
                glass_card(vstack![
                    text("Core Temperature").secondary(),
                    text("42.5°C").title2(),
                ]),
            ]
            .spacing(16.0),
            section(
                "Active Processes",
                vstack![
                    toggle("Hydraulic Pump", true, |_| ()),
                    toggle("Cooling Fan", false, |_| ()),
                    button_label("Emergency Stop")
                        .intent(Intent::Danger)
                        .variant(Variant::Solid)
                        .sudo("High-risk industrial operation")
                ]
                .spacing(12.0)
            )
        ]
        .spacing(24.0)
        .padding(32.0);

        // Define type anchor naturally through assignment to a known trait bound
        let semantic_tree = View::<(), IcedBackend>::describe(&view, &ctx);

        // 2. Measure PeakUI Semantic Size
        let json_output = serde_json::to_string(&semantic_tree).unwrap();
        let semantic_bytes = json_output.len() as f64;
        let semantic_kb = semantic_bytes / 1024.0;

        // 3. Calculate Traditional Vision Size (1080p RGBA)
        let width = 1920.0;
        let height = 1080.0;
        let bytes_per_pixel = 4.0;
        let screenshot_bytes = width * height * bytes_per_pixel;
        let screenshot_mb = screenshot_bytes / (1024.0 * 1024.0);

        // 4. Calculate Reduction
        let reduction_ratio = (1.0 - (semantic_bytes / screenshot_bytes)) * 100.0;

        println!("\n--- PEAKUI GREEN AI BENCHMARK (DSL PATTERN) ---");
        println!("Task: Industrial Dashboard Semantic Serialization");
        println!(
            "PeakUI Semantic Size: {:.2} KB ({} bytes)",
            semantic_kb, semantic_bytes
        );
        println!(
            "Vision Equivalent (1080p): {:.2} MB ({} bytes)",
            screenshot_mb, screenshot_bytes
        );
        println!("Data Reduction: {:.4}%", reduction_ratio);
        let energy_factor = if semantic_bytes > 0.0 {
            screenshot_bytes / semantic_bytes
        } else {
            0.0
        };
        println!(
            "Energy Factor Estimate: ~{}x reduction in data handling",
            energy_factor as u64
        );
        println!("---------------------------------\n");

        // 5. Assertions (The core "proof")
        assert!(
            semantic_kb < 50.0,
            "Semantic tree should be very small (under 50KB)"
        );
        assert!(
            reduction_ratio > 99.9,
            "Reduction should be greater than 99.9% for raw data"
        );
    }

    #[test]
    fn benchmark_neural_sudo_validation() {
        let ctx = Context::default();

        // The .sudo() modifier is the idiomatic way to protect an element
        let view = button_label::<(), IcedBackend>("Delete All Data").sudo("Irreversible action");

        let node = view.describe(&ctx);

        assert!(
            node.is_protected,
            "Neural Sudo must mark the node as protected"
        );
        assert_eq!(
            node.protection_reason,
            Some("Irreversible action".to_string())
        );

        let json = serde_json::to_string(&node).unwrap();
        assert!(
            json.contains("\"p\":true"),
            "Serialized JSON must contain protected flag 'p'"
        );
    }
}
