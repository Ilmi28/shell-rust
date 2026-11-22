use std::io;

use crate::{jobs::{Job, JobRegistry, JobStatus, job_helper}, utils::redirect::{self, OutputRedirect}};

pub fn write_job_table(jobs: &[&Job], out: &OutputRedirect) -> io::Result<()> {
    let n = jobs.len();
    for (index, job) in jobs.iter().enumerate() {
        let special_chr = if n > 0 && index == n - 1 {
            "+"
        } else if n > 1 && index == n - 2 {
            "-"
        } else {
            " "
        };
        let background_chr = if job.status == JobStatus::Running {
            "&"
        } else {
            ""
        };
        redirect::write_stdout(&format!("[{}]{special_chr}  {:<24}{} {background_chr}\n", job.number, job.status, job.cmd), out)?;
    }
    Ok(())
}

pub fn write_done_jobs(reg: &mut JobRegistry) -> io::Result<()> {
    let out = OutputRedirect::default();
    reg.update_statuses();
    let done_jobs = reg.get_done();
    job_helper::write_job_table(&done_jobs, &out)?;
    reg.clear_jobs();
    Ok(())
}