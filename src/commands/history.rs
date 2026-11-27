use crate::{commands::Command, utils::writer};

pub struct HistoryCommand;

impl Command for HistoryCommand {
    fn execute(&self, args: &[String], ctx: &mut super::ShellContext) -> std::io::Result<()> {
        let mut history = ctx.history.get_all();
        if !args.is_empty() && let Ok(last) = args[0].parse::<usize>() {
            let n = history.len();
            history = history[n - last..n].to_vec();
        }
        for rec in history {
            writer::write_stdout(&format!("    {}  {}\n", rec.number, rec.command), ctx.out)?;
        }
        Ok(())
    }
}