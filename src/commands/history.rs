use crate::{commands::Command, utils::writer};

pub struct HistoryCommand;

impl Command for HistoryCommand {
    fn execute(&self, _args: &[String], ctx: &mut super::ShellContext) -> std::io::Result<()> {
        let history = ctx.history.get_all();
        for (i, rec) in history.iter().enumerate() {
            writer::write_stdout(&format!("    {}  {rec}\n", i+1), ctx.out)?
        }
        Ok(())
    }
}