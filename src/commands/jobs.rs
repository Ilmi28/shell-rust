use crate::commands::{Command, ShellContext};

pub struct JobsCommand;

impl Command for JobsCommand {
    fn execute(&self, _args: &[String], _ctx: &mut ShellContext) -> std::io::Result<()> {
        Ok(())
    }
}