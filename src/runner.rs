#[cfg(target_family = "windows")]
#[path = "runner/os/windows.rs"]
mod os;

#[cfg(target_family = "unix")]
#[path = "runner/os/unix.rs"]
mod os;

mod output;

use crate::Result;
use crate::config::Hook;
use crate::config::Job;
use crate::git::Git;
use colored::Colorize;

pub struct Runner<'a> {
    hook: &'a Hook,
    git: Git,
}

impl<'a> Runner<'a> {
    pub fn new(hook: &'a Hook, git: Git) -> Runner<'a> {
        Runner { hook, git }
    }

    pub fn run(&self) -> Result<()> {
        if let Some(true) = self.hook.parallel {
            self.run_jobs_parallel()
        } else {
            self.run_jobs()
        }
    }

    fn run_jobs_parallel(&self) -> Result<()> {
        self.run_jobs() // TODO: add parallelization
    }

    fn run_jobs(&self) -> Result<()> {
        let failed: Vec<String> = self
            .hook
            .jobs
            .iter()
            .filter_map(|job| {
                match run_job(&self.git, &job) {
                    Ok((true, output)) => {
                        info!("{} {}\n{}", "❯", job.run.green(), output.stdout);
                        // Debug logs
                        // println!("{}", output.stderr);
                        None
                    }
                    Ok((false, output)) => {
                        info!(
                            "{} {}\n{}{}",
                            "❯",
                            job.run.red(),
                            output.stdout,
                            output.stderr.red(),
                        );
                        Some(job.run.clone())
                    }
                    Err(err) => {
                        error!("{} {}\n{}", "❯", job.run, err.to_string());
                        Some(job.run.clone())
                    }
                }
            })
            .collect();

        if !failed.is_empty() {
            return Err(eyre::eyre!("jobs failed: {}", failed.join(", ")));
        }

        Ok(())
    }
}

fn run_job(git: &Git, job: &Job) -> Result<(bool, output::Output)> {
    let mut cmd = job.run.clone();
    if cmd.contains("{staged_files}") {
        cmd = cmd.replace(
            "{staged_files}",
            &git.staged_files()?
                .iter()
                .map(|path| path.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" "),
        );
    }

    if cmd.contains("{push_files}") {
        cmd = cmd.replace(
            "{push_files}",
            &git.push_files()?
                .iter()
                .map(|path| path.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" "),
        );
    }

    trace!("run {}", &cmd);

    os::execute(&cmd, "")
}
