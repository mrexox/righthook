use crate::config::{Config, Job};
use crate::repo;
use anyhow::Result;
use colored::Colorize;
use std::process::Command;
use std::str::from_utf8;

pub fn run(hook_name: String) -> Result<()> {
    let repo = repo::Repo::new(".")?;
    let config = Config::parse(&repo)?;

    println!("righthook {} | hook: {} ", crate::VERSION, hook_name);

    match config.hooks.get(&hook_name) {
        Some(hook) => {
            if hook.parallel {
                run_jobs_parallel(&hook.jobs)?;
            } else {
                run_jobs(&hook.jobs)?;
            }
        }
        None => {
            return Err(anyhow::anyhow!(format!("Hook '{}' not found", hook_name)));
        }
    }

    Ok(())
}

fn run_jobs(jobs: &[Job]) -> Result<()> {
    let mut failed: Vec<String> = vec![];
    for job in jobs {
        if let Err(error) = run_job(job) {
            eprintln!("{}", error.to_string().red());
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

fn run_jobs_parallel(jobs: &[Job]) -> Result<()> {
    // TODO: add parallelization
    run_jobs(jobs)
}

fn run_job(job: &Job) -> Result<()> {
    let output = Command::new("sh").arg("-c").arg(&job.run).output()?;
    let stdout = from_utf8(&output.stdout).expect("Invalid UTF-8");
    let stderr = from_utf8(&output.stderr).expect("Invalid UTF-8");
    let status = output.status;

    if output.status.success() {
        println!("❯ {}\n{}{}", job.run.green(), stdout, stderr.red());
        Ok(())
    } else {
        println!("❯ {}\n{}{}", job.run.red(), stdout, stderr.red());
        Err(anyhow::anyhow!(format!(
            "Command '{}' failed with status: {}",
            job.run, status
        )))
    }
}
