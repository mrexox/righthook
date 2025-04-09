mod hook;
mod job;

pub use hook::Hook;
pub use job::Job;

use crate::repo::Repo;
use crate::templates::render_config;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

const CONFIG_NAME: &str = ".righthook.yml";

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(flatten)]
    pub hooks: HashMap<String, Hook>,
}

impl Config {
    pub fn parse(repo: &Repo) -> Result<Self> {
        let config_path = repo.root.join(CONFIG_NAME);
        let yaml = fs::read_to_string(&config_path)?;
        let config: Config = serde_yaml::from_str(&yaml)?;

        Ok(config)
    }

    pub fn create(repo: &Repo) -> Result<Self> {
        let config_path = repo.root.join(CONFIG_NAME);
        fs::write(&config_path, render_config())?;

        Self::parse(repo)
    }
}
