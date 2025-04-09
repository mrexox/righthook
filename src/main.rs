mod cli;
mod commands;
mod config;
mod repo;
mod templates;

use colored::Colorize;
use std::process::exit;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let cli = cli::parse();

    match cli.command {
        Some(cli::Commands::Run { hook }) => {
            commands::run::run(hook).unwrap_or_else(|err| {
                eprintln!("{}", err.to_string().red());

                exit(1);
            });
        }
        Some(cli::Commands::Install { force }) => {
            commands::install::install(force).unwrap_or_else(|err| {
                eprintln!("{}", err.to_string().red());

                exit(1);
            });
        }
        Some(cli::Commands::Uninstall) => {
            commands::uninstall::uninstall().unwrap_or_else(|err| {
                eprintln!("{}", err.to_string().red());

                exit(1);
            });
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}
