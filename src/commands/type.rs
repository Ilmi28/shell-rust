use std::io;

use crate::{commands::ShellContext, utils::{file, redirect::{self, OutputRedirect}}};

use super::Command;

pub struct TypeCommand;

impl Command for TypeCommand {
    fn execute(&self, args: &Vec<String>, ctx: &ShellContext, output_redirect: &OutputRedirect) -> io::Result<()> {
        let cmd = &args[0];
        if (ctx.is_builtin)(&cmd) {
            redirect::write_stdout(&format!("{cmd} is a shell builtin\n"), &output_redirect.stdout)?;
            return Ok(());
        }

        if let Some(path) = file::get_command_path(&cmd) {
            redirect::write_stdout(&format!("{cmd} is {path}\n"), &output_redirect.stdout)?;
            return Ok(());
        }

        redirect::write_stdout(&format!("{cmd}: not found\n"), &output_redirect.stdout)?;
        Ok(())
    }
}