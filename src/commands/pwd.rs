use std::env;

use crate::utils::console;

use super::Command;

pub struct PwdCommand;

impl Command for PwdCommand {
    fn execute(&self, _args: &[&str], _ctx: &super::ShellContext) {
        if let Ok(value) = env::current_dir() {
            console::write_line(&value.to_str().unwrap());
        }
    }
}