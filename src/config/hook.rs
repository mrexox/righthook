use super::job::Job;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Hook {
    pub parallel: Option<bool>,
    pub jobs: Vec<Job>,
}
