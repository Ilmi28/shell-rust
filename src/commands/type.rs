use std::{env, path::Path};
use is_executable::IsExecutable;

use crate::{commands::ShellContext, console};

use super::Command;

pub struct TypeCommand;

impl Command for TypeCommand {
    fn execute(&self, args: &[&str], ctx: &ShellContext) {
        let cmd = args[0];
        if (ctx.is_builtin)(cmd) {
            console::write_line(&format!("{cmd} is a shell builtin"));
            return;
        }
        let path_env = env::var("PATH").unwrap();
        let paths: Vec<&str> = path_env.split([':', ';']).collect();
        for path in paths {
            let full_path = &format!("{path}/{cmd}");
            let path = Path::new(full_path);
            if path.exists() && path.is_executable() {
                console::write_line(&format!("{cmd} is {full_path}"));
                return;
            }
        }

        console::write_line(&format!("{cmd}: not found"));
    }
}