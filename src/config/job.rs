use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Job {
    pub run: String,
}
