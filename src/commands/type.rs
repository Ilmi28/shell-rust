use std::io;

use crate::{commands::ShellContext, utils::{file, output::{self, OutputRedirect}}};

use super::Command;

pub struct TypeCommand;

impl Command for TypeCommand {
    fn execute(&self, args: &Vec<String>, ctx: &ShellContext, output_redirect: &OutputRedirect) -> io::Result<()> {
        let cmd = &args[0];
        if (ctx.is_builtin)(&cmd) {
            output::write_stdout(&format!("{cmd} is a shell builtin\n"), &output_redirect.stdout)?;
            return Ok(());
        }

        if let Some(path) = file::get_command_path(&cmd) {
            output::write_stdout(&format!("{cmd} is {path}\n"), &output_redirect.stdout)?;
            return Ok(());
        }

        output::write_stdout(&format!("{cmd}: not found\n"), &output_redirect.stdout)?;
        Ok(())
    }
}