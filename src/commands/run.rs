use crate::Result;
use crate::config::Config;
use crate::git::Git;
use crate::runner::Runner;

pub async fn run(hook_name: String) -> Result<()> {
    let git = Git::new(".")?;
    let config = Config::parse(&git)?;

    println!("righthook {} | hook: {} ", crate::VERSION, hook_name);

    match config.hooks.get(&hook_name) {
        Some(hook) => {
            let runner = Runner::new(hook.clone(), git);
            runner.run().await
        }
        None => Err(eyre::eyre!("Hook '{}' not found", hook_name)),
    }
}
