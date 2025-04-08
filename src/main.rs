mod commands;
mod config;
mod repo;

use clap::{Parser, Subcommand};
use colored::Colorize;
use std::process::exit;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "righthook")]
#[command(version = VERSION)]
#[command(about = "An alternative of lefthook written in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs the specified hook
    Run {
        /// Hook name
        hook: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run { hook }) => {
            commands::run::run(hook).unwrap_or_else(|err| {
                eprintln!("{}", err.to_string().red());

                exit(1);
            });
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}
