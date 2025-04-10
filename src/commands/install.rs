use crate::Result;
use crate::config::Config;
use crate::git::Git;
use std::fs::{set_permissions, write};
use std::io;

const HOOK_TEMPLATE: &str = include_str!("../templates/hook.sh");

pub fn install(force: bool) -> Result<()> {
    let git = Git::new(".")?;
    let config = match Config::parse(&git) {
        Ok(config) => config,
        Err(err) => {
            if let Some(io_err) = err.downcast_ref::<io::Error>() {
                if io_err.kind() == io::ErrorKind::NotFound {
                    Config::create(&git)?
                } else {
                    return Err(err);
                }
            } else {
                return Err(err);
            }
        }
    };

    let mut installed_hooks: Vec<String> = Vec::new();
    for hook_name in config.hooks.keys() {
        let hook_path = git.hooks.join(hook_name);
        if hook_path.exists() && !force {
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

fn render_hook(hook_name: &str) -> String {
    HOOK_TEMPLATE.replace("{{hook_name}}", hook_name)
}
