use clap::{Parser, Subcommand};
use peakui_cli::{init, runner};

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    Peakui(PeakuiArgs),
}

#[derive(Parser)]
#[command(author, version, about = "PeakUI Project Management CLI", long_about = None)]
struct PeakuiArgs {
    #[command(subcommand)]
    command: Option<PeakuiCommands>,
}

#[derive(Subcommand)]
enum PeakuiCommands {
    /// Initialize a new PeakUI project
    Init {
        /// Name of the project
        name: Option<String>,
        /// Directory to initialize in
        #[arg(short, long)]
        dir: Option<String>,
    },
    /// Alias for init
    New {
        /// Name of the project
        name: Option<String>,
        /// Directory to initialize in
        #[arg(short, long)]
        dir: Option<String>,
    },
    /// Run the PeakUI project
    Run {
        /// Run on web platform
        #[arg(short, long)]
        web: bool,
        /// Run on native platform (default if no flags provided)
        #[arg(short, long)]
        native: bool,
        /// Run on both web and native platforms
        #[arg(short, long)]
        all: bool,
        /// Release mode
        #[arg(short, long)]
        release: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let CargoCli::Peakui(args) = CargoCli::parse();

    match args.command {
        Some(PeakuiCommands::Init { name, dir }) | Some(PeakuiCommands::New { name, dir }) => {
            init::exec(name, dir).await?;
        }
        Some(PeakuiCommands::Run { web, native, all, release }) => {
            runner::exec(web, native, all, release).await?;
        }
        None => {
            // Default behavior if no command is provided: maybe run?
            println!("Usage: cargo peakui <COMMAND>");
            println!("Commands: init, run");
        }
    }

    Ok(())
}
