use crate::{commands::{Command, ShellContext}, jobs::JobStatus, utils::redirect};

pub struct JobsCommand;

impl Command for JobsCommand {
    fn execute(&self, _args: &[String], ctx: &mut ShellContext) -> std::io::Result<()> {
        ctx.jobs.update_statuses();
        let jobs = ctx.jobs.get_all();
        let n = jobs.len() - 1;
        for (index, job) in jobs.iter().enumerate() {
            let special_chr = if index == n {
                "+"
            } else if index == n - 1 {
                "-"
            } else {
                " "
            };
            let background_chr = if job.status == JobStatus::Running {
                "&"
            } else {
                ""
            };
            redirect::write_stdout(&format!("[{}]{special_chr}  {:<24}{} {background_chr}\n", job.number, job.status, job.cmd), ctx.out)?;
        }
        ctx.jobs.clear_jobs();
        Ok(())
    }
}