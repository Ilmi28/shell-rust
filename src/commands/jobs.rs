use crate::commands::Command;

pub struct JobsCommand;

impl Command for JobsCommand {
    fn execute(&self, _args: &Vec<String>, _registry: &super::CommandRegistry, _out: &crate::utils::redirect::OutputRedirect) -> std::io::Result<()> {
        Ok(())
    }
}