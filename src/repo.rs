use anyhow::Result;
use git2::Repository;
use std::path::PathBuf;

pub struct Repo {
    pub root: PathBuf,
    pub hooks: PathBuf,
    repository: Repository,
}

impl Repo {
    pub fn new(path: &str) -> Result<Self> {
        let repository = Repository::discover(path)?;
        let root = repository.path().parent().unwrap().to_path_buf();
        let hooks: PathBuf;

        let config = repository.config()?;
        if let Ok(hooks_path) = config.get_string("core.hooksPath") {
            hooks = PathBuf::from(hooks_path);
        } else {
            hooks = repository.path().join("hooks");
        }

        Ok(Repo {
            repository,
            root,
            hooks,
        })
    }
}
