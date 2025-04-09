use crate::repo::Repo;
use anyhow::Result;
use std::fs::{File, read_dir, remove_file};
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn uninstall() -> Result<()> {
    let repo = Repo::new(".")?;

    read_dir(&repo.hooks)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_ok_and(|ft| ft.is_file()))
        .map(|entry| entry.path())
        .filter(|path| !path.ends_with(".sample"))
        .filter(|path| is_righthook_file(path).unwrap_or(false))
        .for_each(|path| {
            if let Err(err) = remove_file(&path) {
                eprintln!("Failed to remove file {}: {}", path.display(), err);
            } else {
                println!("Removed hook: {}", path.display());
            }
        });

    Ok(())
}

fn is_righthook_file(path: &Path) -> Result<bool> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.contains("call_righthook") {
            return Ok(true);
        }
    }

    Ok(false)
}
