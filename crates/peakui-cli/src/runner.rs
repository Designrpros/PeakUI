use anyhow::{anyhow, Result};
use colored::Colorize;
use tokio::process::Command;

pub async fn exec(web: bool, native: bool, all: bool, _release: bool) -> Result<()> {
    if all || (web && native) {
        println!(
            "{}",
            "🚀 Starting PeakUI on all platforms...".green().bold()
        );
        let native_task = tokio::spawn(run_native());
        let web_task = tokio::spawn(run_web());

        let (res_native, res_web) = tokio::join!(native_task, web_task);
        res_native??;
        res_web??;
        Ok(())
    } else if web {
        run_web().await
    } else {
        // Default to native if specifically requested or no flags
        run_native().await
    }
}

async fn run_native() -> Result<()> {
    println!("{}", "💻 Running Native build...".blue().bold());
    let status = Command::new("cargo")
        .args(["run", "--features", "native"])
        .status()
        .await?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("Native run failed"))
    }
}

async fn run_web() -> Result<()> {
    println!("{}", "🌐 Running Web build (Trunk)...".cyan().bold());
    // Check if trunk is installed
    if Command::new("trunk").arg("-V").output().await.is_err() {
        return Err(anyhow!(
            "'trunk' is not installed. Please install it with 'cargo install trunk'"
        ));
    }

    let status = Command::new("trunk").args(["serve"]).status().await?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("Web run failed"))
    }
}
