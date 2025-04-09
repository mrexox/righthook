use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "righthook")]
#[command(version = crate::VERSION)]
#[command(about = "An alternative of lefthook written in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Runs the specified hook
    Run {
        /// Hook name
        hook: String,
    },
    /// Install Git hooks
    Install {
        /// Overwrite existing hooks
        #[arg(short, long)]
        force: bool,
    },
    /// Uninstall righthook hooks
    Uninstall,
}

pub fn parse() -> Cli {
    Cli::parse()
}
