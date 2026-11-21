use std::io;

use crate::{CommandRegistry, utils::{file, redirect::{self, OutputRedirect}}};

use super::Command;

pub struct TypeCommand;

impl Command for TypeCommand {
    fn execute(&self, args: &[String], registry: &CommandRegistry, output_redirect: &OutputRedirect) -> io::Result<()> {
        let cmd = &args[0];
        if registry.is_builtin(cmd) {
            redirect::write_stdout(&format!("{cmd} is a shell builtin\n"), output_redirect)?;
            return Ok(());
        }

        if let Some(path) = file::get_command_path(cmd) {
            redirect::write_stdout(&format!("{cmd} is {path}\n"), output_redirect)?;
            return Ok(());
        }

        redirect::write_stdout(&format!("{cmd}: not found\n"), output_redirect)?;
        Ok(())
    }
}