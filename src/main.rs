use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A CLI tool to detect copy-paste and edit errors across programming languages
#[derive(Parser)]
#[command(name = "copyedit-check")]
#[command(about = "Detect copy-paste and edit errors", long_about = None)]
struct Cli {
    /// One or more paths to search (default is current directory)
    #[arg(default_value = ".", global = true)]
    paths: Vec<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check for copy-edit errors in code
    Check {
        /// Number of neighboring lines to compare (default: 5)
        #[arg(long, default_value_t = 5, value_name = "WINDOW")]
        window: usize,

        /// File extensions to include (default: all)
        #[arg(long, value_name = "EXTENSIONS")]
        extensions: Option<String>,

        /// Output results as JSON
        #[arg(long)]
        json: bool,

        /// Skip specific folders
        #[arg(long, value_name = "PATH")]
        ignore: Vec<PathBuf>,
    },
}

mod analysis;
mod pattern;
mod utilities;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Check { window, extensions, json, ignore } => {
            if let Err(e) = analysis::check_copy_edit_errors(&cli.paths, *window, extensions.as_deref(), json, ignore) {
                eprintln!("Error checking copy-edit errors: {}", e);
            }
        }
    }
}
