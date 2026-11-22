use crate::{commands::{Command, ShellContext}, utils::redirect};

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
            redirect::write_stdout(&format!("[{}]{}  {:<24}{}\n", job.number, special_chr, job.status, job.cmd), ctx.out)?;
        }
        Ok(())
    }
}