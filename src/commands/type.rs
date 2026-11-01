use std::io;

use crate::{commands::ShellContext, utils::{output, file}};

use super::Command;

pub struct TypeCommand;

impl Command for TypeCommand {
    fn execute(&self, args: &Vec<String>, ctx: &ShellContext, stdout: Option<&str>) -> io::Result<()> {
        let cmd = &args[0];
        if (ctx.is_builtin)(&cmd) {
            output::write_stdout(&format!("{cmd} is a shell builtin\n"), stdout)?;
            return Ok(());
        }

        if let Some(path) = file::get_command_path(&cmd) {
            output::write_stdout(&format!("{cmd} is {path}"), stdout)?;
            return Ok(());
        }

        output::write_stdout(&format!("{cmd}: not found"), stdout)?;
        Ok(())
    }
}