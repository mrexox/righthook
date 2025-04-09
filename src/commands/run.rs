use crate::config::Config;
use crate::git::Git;
use crate::runner::run_hook;
use crate::Result;

pub fn run(hook_name: String) -> Result<()> {
    let git = Git::new(".")?;
    let config = Config::parse(&git)?;

    println!("righthook {} | hook: {} ", crate::VERSION, hook_name);

    match config.hooks.get(&hook_name) {
        Some(hook) => run_hook(hook),
        None => Err(anyhow::anyhow!(format!("Hook '{}' not found", hook_name))),
    }
}
