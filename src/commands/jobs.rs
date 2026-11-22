use crate::{commands::{Command, ShellContext}, utils::redirect};

pub struct JobsCommand;

impl Command for JobsCommand {
    fn execute(&self, _args: &[String], ctx: &mut ShellContext) -> std::io::Result<()> {
        ctx.jobs.update_statuses();
        let jobs = ctx.jobs.get_all();
        let last_num = ctx.jobs.get_next_number() - 1;
        for job in jobs {
            let plus_chr = if last_num == job.number {
                "+"
            } else {
                ""
            };
            redirect::write_stdout(&format!("[{}]{}  {:<24}{}\n", job.number, plus_chr, job.status, job.cmd), ctx.out)?;
        }
        Ok(())
    }
}