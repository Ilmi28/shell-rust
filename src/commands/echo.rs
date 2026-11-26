use std::io;

use crate::{commands::ShellContext, utils::writer};
use super::Command;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: &[String], ctx: &mut ShellContext) -> io::Result<()> {
        let message = args.join(" ");
        writer::write_stdout(&format!("{message}\n"), ctx.out)?;
        Ok(())
    }
}
