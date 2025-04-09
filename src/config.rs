mod hook;
mod job;

pub use hook::Hook;
pub use job::Job;

use crate::Result;
use crate::git::Git;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

const CONFIG_NAME: &str = ".righthook.yml";
const CONFIG_TEMPLATE: &str = include_str!("./templates/config.yml");

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(flatten)]
    pub hooks: HashMap<String, Hook>,
}

impl Config {
    pub fn parse(git: &Git) -> Result<Self> {
        let config_path = git.root.join(CONFIG_NAME);
        let yaml = fs::read_to_string(&config_path)?;
        let config: Config = serde_yaml::from_str(&yaml)?;

        Ok(config)
    }

    pub fn create(git: &Git) -> Result<Self> {
        let config_path = git.root.join(CONFIG_NAME);
        fs::write(&config_path, render_config())?;

        Self::parse(git)
    }
}

fn render_config() -> String {
    CONFIG_TEMPLATE.to_string()
}
