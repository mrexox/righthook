use crate::config::Config;
use crate::repo::Repo;
use crate::templates::render_hook;
use anyhow::Result;
use std::fs::{set_permissions, write};

pub fn install(force: bool) -> Result<()> {
    let repo = Repo::new(".")?;
    let config = Config::parse(&repo)?;

    let mut installed_hooks: Vec<String> = Vec::new();
    for hook_name in config.hooks.keys() {
        let hook_path = repo.hooks.join(hook_name);
        if std::path::Path::new(&hook_path).exists() && !force {
            println!(
                "Hook {} already exists. Use --force to overwrite.",
                hook_name
            );
            continue;
        }

        write(&hook_path, render_hook(hook_name))?;
        set_permissions(
            &hook_path,
            std::os::unix::fs::PermissionsExt::from_mode(0o755),
        )?;
        installed_hooks.push(hook_name.clone());
    }

    if !installed_hooks.is_empty() {
        println!("installed hooks: {}", installed_hooks.join(", "));
    }

    Ok(())
}
