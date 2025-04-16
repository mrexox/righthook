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
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinSet;

pub struct Runner {
    hook: Hook,
    git: Arc<Mutex<Git>>,
}

struct JobStatus {
    job: Job,
    ok: bool,
    output: output::Output,
}

impl Runner {
    pub fn new(hook: Hook, git: Git) -> Runner {
        Runner {
            hook,
            git: Arc::new(Mutex::new(git)),
        }
    }

    pub async fn run(&self) -> Result<()> {
        if let Some(true) = self.hook.parallel {
            self.run_jobs(true).await
        } else {
            self.run_jobs(false).await
        }
    }

    async fn run_jobs(&self, parallel: bool) -> Result<()> {
        let mut set = JoinSet::new();

        let mut failed = Vec::with_capacity(self.hook.jobs.len());
        if parallel {
            for job in self.hook.jobs.clone().into_iter() {
                let git = Arc::clone(&self.git);
                set.spawn(async move { run_job(git, job).await });
            }
            while let Some(result) = set.join_next().await {
                match result.unwrap() {
                    Ok(job_status) => {
                        if let Some(name) = handle_result(&job_status) {
                            failed.push(name);
                        }
                    }
                    Err(err) => {
                        error!("Error running job: {}", err);
                    }
                }
            }
        } else {
            for job in self.hook.jobs.clone().into_iter() {
                let git = Arc::clone(&self.git);
                match run_job(git, job).await {
                    Ok(job_status) => {
                        if let Some(name) = handle_result(&job_status) {
                            failed.push(name);
                        }
                    }
                    Err(err) => {
                        // TODO: fix
                        // error!("{} {}\n{}", "❯", job.run, err);
                        // Some(job.run.clone())
                        error!("Error running job: {}", err);
                    }
                }
            }
        };

        if !failed.is_empty() {
            return Err(eyre::eyre!("jobs failed: {}", failed.join(", ")));
        }

        Ok(())
    }
}

async fn run_job(git: Arc<Mutex<Git>>, job: Job) -> Result<JobStatus> {
    let mut cmd = job.run.clone();
    if cmd.contains("{staged_files}") {
        let staged_files = {
            git.lock()
                .await
                .staged_files()?
                .iter()
                .map(|path| path.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" ")
        };
        cmd = cmd.replace("{staged_files}", &staged_files);
    }

    if cmd.contains("{push_files}") {
        let push_files = {
            git.lock()
                .await
                .push_files()?
                .iter()
                .map(|path| path.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" ")
        };
        cmd = cmd.replace("{push_files}", &push_files);
    }

    trace!("running {}", &cmd);

    let (ok, output) = os::execute(&cmd, "").await?;

    Ok(JobStatus { ok, output, job })
}

fn handle_result(job_status: &JobStatus) -> Option<String> {
    if job_status.ok {
        info!(
            "{} {}\n{}",
            "❯",
            job_status.job.name().green(),
            job_status.output.stdout
        );
        // Debug logs
        // println!("{}", output.stderr);
        return None;
    }

    info!(
        "{} {}\n{}{}",
        "❯",
        job_status.job.name().red(),
        job_status.output.stdout,
        job_status.output.stderr.red(),
    );
    Some(job_status.job.name())
}
