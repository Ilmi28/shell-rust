use crate::{commands::ShellContext, utils::{console, file}};

use super::Command;

pub struct TypeCommand;

impl Command for TypeCommand {
    fn execute(&self, args: &Vec<String>, ctx: &ShellContext) {
        let cmd = &args[0];
        if (ctx.is_builtin)(&cmd) {
            console::write_line(&format!("{cmd} is a shell builtin"));
            return;
        }

        if let Some(path) = file::get_command_path(&cmd) {
            console::write_line(&format!("{cmd} is {path}"));
            return;
        }

        console::write_line(&format!("{cmd}: not found"));
    }
}