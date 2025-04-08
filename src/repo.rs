use anyhow::Result;
use git2::Repository;
use std::path::PathBuf;

pub struct Repo {
    pub root: PathBuf,
    repository: Repository,
}

impl Repo {
    pub fn new(path: &str) -> Result<Self> {
        let repository = Repository::discover(path)?;
        let root = repository.path().parent().unwrap().to_path_buf();
        Ok(Repo { repository, root })
    }
}
