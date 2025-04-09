#[macro_use]
mod log;

mod cli;
mod commands;
mod config;
mod git;
mod runner;
mod templates;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let cli = cli::parse();

    match cli.command {
        Some(cli::Commands::Run { hook }) => {
            commands::run::run(hook).unwrap_or_else(|err| {
                error!("{}", err.to_string());
            });
        }
        Some(cli::Commands::Install { force }) => {
            commands::install::install(force).unwrap_or_else(|err| {
                error!("{}", err.to_string());
            });
        }
        Some(cli::Commands::Uninstall) => {
            commands::uninstall::uninstall().unwrap_or_else(|err| {
                error!("{}", err.to_string());
            });
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}
