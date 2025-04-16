use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Job {
    pub name: Option<String>,
    pub run: String,
    pub glob: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

impl Job {
    pub fn name(&self) -> String {
        self.name.clone().unwrap_or(self.run.clone())
    }
}
