use crate::{commands::ShellContext, console};

use super::Command;

pub struct TypeCommand;

impl Command for TypeCommand {
    fn execute(&self, args: &[&str], ctx: &ShellContext) {
        let cmd = args[0];
        if (ctx.is_builtin)(cmd) {
            console::write_line(&format!("{cmd} is a shell builtin"));
        }
        else {
            console::write_line(&format!("{cmd}: not found"));
        }
    }
}