use super::job::Job;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Hook {
    pub parallel: Option<bool>,
    pub jobs: Vec<Job>,
}
