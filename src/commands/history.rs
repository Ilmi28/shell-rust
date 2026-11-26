use crate::commands::Command;

pub struct HistoryCommand;

impl Command for HistoryCommand {
    fn execute(&self, _args: &[String], _ctx: &mut super::ShellContext) -> std::io::Result<()> {
        todo!()
    }
}