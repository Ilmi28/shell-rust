use std::io;

use crate::{commands::ShellContext, utils::{file, writer::{self}}};

use super::Command;

pub struct TypeCommand;

impl Command for TypeCommand {
    fn execute(&self, args: &[String], ctx: &mut ShellContext) -> io::Result<()> {
        let cmd = &args[0];
        if ctx.commands.is_builtin(cmd) {
            writer::write_stdout(&format!("{cmd} is a shell builtin\n"), ctx.out)?;
            return Ok(());
        }

        if let Some(path) = file::get_command_path(cmd) {
            writer::write_stdout(&format!("{cmd} is {path}\n"), ctx.out)?;
            return Ok(());
        }

        writer::write_stdout(&format!("{cmd}: not found\n"), ctx.out)?;
        Ok(())
    }
}