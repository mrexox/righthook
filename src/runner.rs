#[cfg(target_family = "windows")]
#[path = "runner/windows.rs"]
mod system;

#[cfg(target_family = "unix")]
#[path = "runner/unix.rs"]
mod system;

mod output;

use crate::config::Hook;
use crate::config::Job;
use crate::Result;
use colored::Colorize;

pub fn run_hook(hook: &Hook) -> Result<()> {
    if hook.parallel {
        run_jobs_parallel(&hook.jobs)
    } else {
        run_jobs(&hook.jobs)
    }
}

fn run_jobs_parallel(jobs: &[Job]) -> Result<()> {
    // TODO: add parallelization
    run_jobs(jobs)
}

fn run_jobs(jobs: &[Job]) -> Result<()> {
    let mut failed: Vec<String> = Vec::with_capacity(jobs.len());
    for job in jobs {
        if !run_job(job)? {
            failed.push(job.run.clone());
        }
    }

    if !failed.is_empty() {
        return Err(anyhow::anyhow!(format!(
            "jobs failed: {}",
            failed.join(", ")
        )));
    }

    Ok(())
}

fn run_job(job: &Job) -> Result<bool> {
    let output = system::run(&job.run, "")?;

    if output.ok {
        println!("{} {}\n{}", "❯", job.run.green(), output.stdout);
        // Debug logs
        // println!("{}", output.stderr);
        return Ok(true);
    }

    println!(
        "{} {}\n{}{}",
        "❯".red(),
        job.run.red(),
        output.stdout,
        output.stderr.red(),
    );

    Ok(false)
}
