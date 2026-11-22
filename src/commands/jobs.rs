
use crate::{commands::{Command, ShellContext}, jobs::job_helper};

pub struct JobsCommand;

impl Command for JobsCommand {
    fn execute(&self, _args: &[String], ctx: &mut ShellContext) -> std::io::Result<()> {
        ctx.jobs.update_statuses();
        let jobs= ctx.jobs.get_all();
        job_helper::write_job_table(&jobs, ctx.out)?;
        ctx.jobs.clear_jobs();
        Ok(())
    }
}