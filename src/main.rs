#[macro_use]
extern crate log;

mod cli;
mod commands;
mod config;
mod env;
mod git;
mod logger;
mod runner;

pub use eyre::Result;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    logger::init();
    let cli = cli::parse();

    match cli.command {
        Some(cli::Commands::Run { hook }) => {
            commands::run::run(hook).await.unwrap_or_else(|err| {
                error!("{}", err);
                std::process::exit(1);
            });
        }
        Some(cli::Commands::Install { force }) => {
            commands::install::install(force).unwrap_or_else(|err| {
                error!("{}", err);
                std::process::exit(1);
            });
        }
        Some(cli::Commands::Uninstall) => {
            commands::uninstall::uninstall().unwrap_or_else(|err| {
                error!("{}", err);
                std::process::exit(1);
            });
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}
