use crate::Result;
use eyre::WrapErr;
use git2::{ObjectType, Repository, StatusOptions, StatusShow};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Git {
    pub root: PathBuf,
    pub hooks: PathBuf,
    repo: Repository,
    files_cache: RefCell<HashMap<Templates, Vec<PathBuf>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Templates {
    StagedFiles,
    PushFiles,
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

        let files_cache = RefCell::new(HashMap::new());

        Ok(Git {
            repo,
            root,
            hooks,
            files_cache,
        })
    }

    pub fn staged_files(&self) -> Result<Vec<PathBuf>> {
        if let Some(files) = self.files_cache.borrow().get(&Templates::StagedFiles) {
            return Ok(files.clone());
        }

        let mut status_options = StatusOptions::new();
        status_options.show(StatusShow::Index);

        let statuses = self
            .repo
            .statuses(Some(&mut status_options))
            .wrap_err("failed to get statuses")?;

        let paths: Vec<PathBuf> = statuses
            .iter()
            .filter_map(|s| s.path().map(PathBuf::from))
            .filter(|p| p.exists())
            .collect();

        self.files_cache
            .borrow_mut()
            .insert(Templates::StagedFiles, paths.clone());

        Ok(paths)
    }

    pub fn push_files(&self) -> Result<Vec<PathBuf>> {
        if let Some(files) = self.files_cache.borrow().get(&Templates::PushFiles) {
            return Ok(files.clone());
        }

        // Get the HEAD commit
        let head_ref = self.repo.head()?;
        let head_commit = head_ref.peel(ObjectType::Commit)?.into_commit().unwrap();

        // Get the commit referenced by @{push}
        let push_ref = self.repo.find_reference("refs/remotes/origin/HEAD")?;
        let push_commit = push_ref
            .peel(ObjectType::Commit)?
            .into_commit()
            .map_err(|_| {
                eyre::eyre!("Please setup the push ref, e.g.: git remote set-head origin main")
            })?;

        // Create a diff between the HEAD commit and the @{push} commit
        let diff = self.repo.diff_tree_to_tree(
            Some(&head_commit.tree()?),
            Some(&push_commit.tree()?),
            None,
        )?;

        // Collect the names of the files that differ
        let mut paths = Vec::new();
        diff.foreach(
            &mut |file, _| {
                // Get the file path
                if let Some(path) = file.new_file().path() {
                    paths.push(path.to_owned());
                }
                true // Continue iteration
            },
            None,
            None,
            None,
        )?;
        Ok(paths)
    }
}
