use anyhow::Result;
use git2::Repository;
use std::path::PathBuf;

pub struct Git {
    pub root: PathBuf,
    pub hooks: PathBuf,
    repo: Repository,
}

impl Git {
    pub fn new(path: &str) -> Result<Self> {
        let repo = Repository::discover(path)?;
        let root = repo.path().parent().unwrap().to_path_buf();
        let hooks: PathBuf;

        let config = repo.config()?;
        if let Ok(hooks_path) = config.get_string("core.hooksPath") {
            hooks = PathBuf::from(hooks_path);
        } else {
            hooks = repo.path().join("hooks");
        }

        Ok(Git { repo, root, hooks })
    }
}
