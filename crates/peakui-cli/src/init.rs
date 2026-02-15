use anyhow::{anyhow, Result};
use colored::Colorize;
use convert_case::{Case, Casing};
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
    // 1. Cargo.toml
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
default = []
native = ["peak-ui/native"]
wasm = ["peak-ui/wasm"]
"#,
        name
    );
    fs::write(path.join("Cargo.toml"), cargo_toml)?;

    // 2. src/main.rs
    fs::create_dir_all(path.join("src"))?;
    let title = name.to_case(Case::Title);
    let main_rs = format!(
        r#"use peak_ui::prelude::*;

fn main() -> iced::Result {{
    iced::application(|| (State::default(), Task::none()), update, view)
        .title("{title} - PeakUI")
        .run()
}}

#[derive(Debug, Clone)]
enum Message {{
    ButtonPressed,
}}

struct State {{
    count: i32,
}}

impl Default for State {{
    fn default() -> Self {{
        Self {{ count: 0 }}
    }}
}}

fn update(state: &mut State, message: Message) -> Task<Message> {{
    match message {{
        Message::ButtonPressed => {{
            state.count += 1;
        }}
    }}
    Task::none()
}}

fn view(state: &State) -> Element<'_, Message> {{
    let context = Context::default();

    let logo = ProxyView::<Message, IcedBackend>::new(|_| {{
        iced::widget::image(
            iced::widget::image::Handle::from_bytes(include_bytes!("../assets/app_logo.png").as_slice())
        )
        .width(120)
        .height(120)
        .into()
    }});

    vstack![
        logo,
        text("Welcome to {title}!")
            .size(36.0),
        text(format!("Count: {{}}", state.count))
            .size(24.0),
        button(text("Increment Count"))
            .on_press(Message::ButtonPressed)
    ]
    .spacing(24.0)
    .align_x(Alignment::Center)
    .padding(40.0)
    .view(&context)
}}
"#,
        title = title
    );
    fs::write(path.join("src/main.rs"), main_rs)?;

    // 3. Trunk.toml
    let trunk_toml = r#"[build]
target = "index.html"
dist = "dist"
cargo-features = ["wasm"]

[[hooks]]
stage = "post_build"
command = "sh"
command_arguments = ["-c", "echo 'Build complete!'"]

[serve]
port = 8080
"#;
    fs::write(path.join("Trunk.toml"), trunk_toml)?;

    // 4. index.html
    let index_html = format!(
        r#"<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8" />
        <title>{} - PeakUI</title>
        <link rel="icon" href="assets/favicon.ico" />
        <link data-trunk rel="icon" href="assets/favicon.ico" />
        <link data-trunk rel="copy-file" href="assets/app_logo.png" />
    </head>
    <body>
        <div id="iced-canvas"></div>
    </body>
</html>
"#,
        name.to_case(Case::Title)
    );
    fs::write(path.join("index.html"), index_html)?;

    // 5. Assets
    let assets_dir = path.join("assets");
    fs::create_dir_all(&assets_dir)?;
    fs::write(assets_dir.join("app_logo.png"), crate::LOGO_PNG)?;
    fs::write(assets_dir.join("favicon.ico"), crate::FAVICON_ICO)?;

    Ok(())
}
