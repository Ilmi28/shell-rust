use crate::{commands::{Command, ShellContext}, jobs::JobStatus, utils::redirect};

pub struct JobsCommand;

impl Command for JobsCommand {
    fn execute(&self, _args: &[String], ctx: &mut ShellContext) -> std::io::Result<()> {
        ctx.jobs.update_statuses();
        let jobs = ctx.jobs.get_all();
        let last_num = ctx.jobs.get_next_number() - 1;
        for job in jobs {
            let special_chr = if job.number == last_num {
                "+"
            } else if job.number == last_num - 1 {
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