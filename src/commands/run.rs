use crate::config::Config;
use crate::repo::Repo;
use crate::runner::run_hook;
use anyhow::Result;

pub fn run(hook_name: String) -> Result<()> {
    let repo = Repo::new(".")?;
    let config = Config::parse(&repo)?;

    println!("righthook {} | hook: {} ", crate::VERSION, hook_name);

    match config.hooks.get(&hook_name) {
        Some(hook) => run_hook(hook),
        None => Err(anyhow::anyhow!(format!("Hook '{}' not found", hook_name))),
    }
}
