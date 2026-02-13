use anyhow::{anyhow, Result};
use colored::Colorize;
use inquire::{Confirm, Text};
use std::fs;
use std::path::PathBuf;

pub async fn exec(name: Option<String>, dir: Option<String>) -> Result<()> {
    println!(
        "{}",
        "🏔️  Welcome to PeakUI! Let's initialize your project."
            .bold()
            .cyan()
    );

    let project_name = match name {
        Some(n) => n,
        None => Text::new("What is your project name?")
            .with_default("my-peakui-app")
            .prompt()?,
    };

    let target_dir = match dir {
        Some(d) => PathBuf::from(d),
        None => PathBuf::from(&project_name),
    };

    if target_dir.exists() {
        let overwrite = Confirm::new(&format!(
            "Directory {:?} already exists. Overwrite?",
            target_dir
        ))
        .with_default(false)
        .prompt()?;

        if !overwrite {
            return Err(anyhow!("Initialization cancelled by user."));
        }
        fs::remove_dir_all(&target_dir)?;
    }

    fs::create_dir_all(&target_dir)?;

    println!(
        "{}",
        format!(
            "🛠️  Scaffolding project '{}' in {:?}...",
            project_name, target_dir
        )
        .yellow()
    );

    // TODO: Implement actual template extraction
    scaffold_minimal(&target_dir, &project_name)?;

    println!("{}", "✅ Project initialized successfully!".green().bold());
    println!("Next steps:");
    println!("  cd {}", project_name);
    println!("  cargo peakui run");

    Ok(())
}

fn scaffold_minimal(path: &std::path::Path, name: &str) -> Result<()> {
    // Basic Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
peak-ui = {{ git = "https://github.com/Designrpros/PeakUI.git" }}
iced = {{ version = "0.14" }}
serde = {{ version = "1.0", features = ["derive"] }}

[features]
default = ["native"]
native = ["peak-ui/native"]
wasm = ["peak-ui/wasm"]
"#,
        name
    );

    fs::write(path.join("Cargo.toml"), cargo_toml)?;

    // Basic src/main.rs
    fs::create_dir_all(path.join("src"))?;
    let main_rs = r#"use peak_ui::prelude::*;

fn main() -> iced::Result {
    println!("Welcome to PeakUI!");
    // Minimal app entry point
    Ok(())
}
"#;
    fs::write(path.join("src/main.rs"), main_rs)?;

    Ok(())
}
