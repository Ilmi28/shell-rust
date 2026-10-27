use crate::{commands::ShellContext, console, file};

use super::Command;

pub struct TypeCommand;

impl Command for TypeCommand {
    fn execute(&self, args: &[&str], ctx: &ShellContext) {
        let cmd = args[0];
        if (ctx.is_builtin)(cmd) {
            console::write_line(&format!("{cmd} is a shell builtin"));
            return;
        }

        if let Some(path) = file::get_command_path(cmd) {
            console::write_line(&format!("{cmd} is {path}"));
            return;
        }

        console::write_line(&format!("{cmd}: not found"));
    }
}