use clap::{Parser, Subcommand};

use crate::launcher;

#[derive(Parser)]
#[command(name = "launch_tool")]
#[command(about = "A fast Linux app launcher in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch an application by name
    Launch {
        /// Name of the application to launch (e.g. firefox, terminal)
        name: String,
    },
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Launch { name } => {
            if let Err(err) = launcher::launch(name) {
                eprintln!("❌ Failed to launch '{}': {}", name, err);
            }
        }
    }
}
