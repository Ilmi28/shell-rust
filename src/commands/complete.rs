use crate::commands::Command;



pub struct CompleteCommand;

impl Command for CompleteCommand {
    fn execute(&self, args: &Vec<String>, ctx: &super::ShellContext, out: &crate::utils::redirect::OutputRedirect) -> std::io::Result<()> {
        todo!()
    }
}