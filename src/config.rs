mod hook;
mod job;

pub use job::Job;

use crate::repo::Repo;
use anyhow::Result;
use hook::Hook;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(flatten)]
    pub hooks: HashMap<String, Hook>,
}

impl Config {
    pub fn parse(repo: &Repo) -> Result<Self> {
        let config_path = repo.root.join(".righthook.yml");
        let yaml = fs::read_to_string(&config_path)?;
        let config: Config = serde_yaml::from_str(&yaml)?;

        Ok(config)
    }
}
