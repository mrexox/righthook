use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Job {
    pub run: String,
}
