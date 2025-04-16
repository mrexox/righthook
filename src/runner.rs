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
    ok: bool,
    output: output::Output,
    cmd: String,
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

        let job_results: Vec<_> = if parallel {
            for job in self.hook.jobs.clone().into_iter() {
                let git = Arc::clone(&self.git);
                set.spawn(async move { run_job(git, job).await });
            }
            set.join_all().await
        } else {
            let mut results = Vec::with_capacity(self.hook.jobs.len());
            for job in self.hook.jobs.clone().into_iter() {
                let git = Arc::clone(&self.git);
                results.push(run_job(git, job).await);
            }
            results
        };

        let failed: Vec<_> = job_results
            .iter()
            .filter_map(|result| {
                match result {
                    Ok(JobStatus {
                        ok: true,
                        cmd,
                        output,
                    }) => {
                        info!("{} {}\n{}", "❯", cmd.green(), output.stdout);
                        // Debug logs
                        // println!("{}", output.stderr);
                        None
                    }
                    Ok(JobStatus {
                        ok: false,
                        cmd,
                        output,
                    }) => {
                        info!(
                            "{} {}\n{}{}",
                            "❯",
                            cmd.red(),
                            output.stdout,
                            output.stderr.red(),
                        );
                        Some(cmd.clone())
                    }
                    Err(_err) => {
                        // TODO: fix
                        // error!("{} {}\n{}", "❯", job.run, err);
                        // Some(job.run.clone())
                        None
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

    trace!("run {}", &cmd);

    let (ok, output) = os::execute(&cmd, "").await?;

    Ok(JobStatus {
        ok,
        output,
        cmd: job.run.clone(),
    })
}
